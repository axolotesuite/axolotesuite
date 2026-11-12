//! Interface do módulo de Renomeação em Lote.

use iced::widget::{column, pick_list, row, text};
use iced::{Alignment, Element, Length};

use crate::modules::batch_rename::service::Rule;
use crate::modules::batch_rename::{Msg, State};
use crate::shared::theme::Btn;
use crate::shared::types::Env;
use crate::shared::ui::{self, act, SEMIBOLD};

pub fn view<'a>(state: &'a State, env: &Env) -> Element<'a, Msg> {
    let c = env.c();

    let header = ui::page_header(
        env,
        "🏷",
        "Renomeação em Lote",
        "Renomeie dezenas de arquivos de uma só vez com pré-visualização instantânea antes de aplicar.",
        None,
    );

    // 1. Seleção de arquivos
    let file_btns = ui::actions(
        env,
        vec![
            act(
                "➕  Escolher arquivos…",
                Btn::Primary,
                (!state.busy).then_some(Msg::AddFiles),
            ),
            act(
                "📁  Carregar pasta inteira…",
                Btn::Secondary,
                (!state.busy).then_some(Msg::AddFolder),
            ),
            act(
                "🗑  Limpar lista",
                Btn::Ghost,
                (!state.files.is_empty() && !state.busy).then_some(Msg::ClearFiles),
            ),
        ],
    );

    // 2. Regra e parâmetros
    let rule_picker = pick_list(Rule::ALL, Some(state.rule), Msg::RuleChanged)
        .padding([10, 12])
        .width(Length::Fill);

    let rule_field = ui::field(env, "Tipo de regra", state.rule.description(), rule_picker);

    let params: Element<'a, Msg> = match state.rule {
        Rule::Replace => {
            let find = ui::field(
                env,
                "Procurar por",
                "Trecho atual do nome que você quer substituir ou remover.",
                ui::input("Ex.: IMG_ ou cópia", &state.find_text, Msg::FindChanged),
            );
            let repl = ui::field(
                env,
                "Substituir por",
                "Deixe em branco se quiser apenas apagar o trecho encontrado.",
                ui::input("Ex.: Ferias_", &state.value_text, Msg::ValueChanged),
            );
            if env.narrow() {
                column![find, repl].spacing(12).into()
            } else {
                row![find, repl].spacing(12).into()
            }
        }
        Rule::Prefix => ui::field(
            env,
            "Prefixo (texto adicionado no início)",
            "Ex.: \"2026_\" transforma \"relatorio.pdf\" em \"2026_relatorio.pdf\".",
            ui::input("Ex.: 2026_ ou Cliente_", &state.value_text, Msg::ValueChanged),
        ),
        Rule::Suffix => ui::field(
            env,
            "Sufixo (texto adicionado antes da extensão)",
            "Ex.: \"_revisado\" transforma \"contrato.pdf\" em \"contrato_revisado.pdf\".",
            ui::input("Ex.: _final ou _v2", &state.value_text, Msg::ValueChanged),
        ),
        Rule::Number => ui::field(
            env,
            "Nome base (opcional)",
            "Ex.: \"Foto\" gera \"Foto_01.jpg\", \"Foto_02.jpg\"… Se deixar vazio, o número vai na frente do nome atual.",
            ui::input("Ex.: Foto_Viagem", &state.value_text, Msg::ValueChanged),
        ),
        Rule::Uppercase | Rule::Lowercase => {
            ui::muted(env, "Esta regra não precisa de texto adicional — veja a prévia abaixo.").into()
        }
    };

    // 3. Pré-visualização ao vivo
    let previews = state.preview_items();
    let changes_count = previews.iter().filter(|p| p.changed).count();

    let mut preview_col = column![row![
        ui::strong(
            format!("Pré-visualização ({} arquivo(s))", previews.len()),
            15
        )
        .width(Length::Fill),
        ui::badge(
            format!("{changes_count} será(ão) alterado(s)"),
            if changes_count > 0 {
                c.success
            } else {
                c.muted
            },
        ),
    ]
    .align_items(Alignment::Center)]
    .spacing(8);

    if previews.is_empty() {
        preview_col = preview_col.push(ui::empty_state(
            env,
            "🏷",
            "Nenhum arquivo selecionado",
            "Clique em \"Escolher arquivos…\" ou \"Carregar pasta inteira…\" acima para começar.",
        ));
    } else {
        for (idx, item) in previews.iter().enumerate().take(60) {
            let arrow_color = if item.changed { c.success } else { c.muted };
            let line: Element<'a, Msg> = if env.narrow() {
                column![
                    text(&item.old_name).size(13).style(c.muted),
                    ui::colored(format!("→  {}", item.new_name), 13, arrow_color).font(SEMIBOLD),
                ]
                .spacing(2)
                .width(Length::Fill)
                .into()
            } else {
                row![
                    text(&item.old_name)
                        .size(13)
                        .style(c.muted)
                        .width(Length::FillPortion(1)),
                    ui::colored("→", 14, arrow_color).font(ui::BOLD),
                    ui::colored(&item.new_name, 13, arrow_color)
                        .font(SEMIBOLD)
                        .width(Length::FillPortion(1)),
                ]
                .spacing(10)
                .align_items(Alignment::Center)
                .width(Length::Fill)
                .into()
            };

            preview_col = preview_col.push(ui::list_row(
                row![line, ui::btn("✕", Btn::Ghost, Some(Msg::RemoveFile(idx)))]
                    .spacing(10)
                    .align_items(Alignment::Center),
            ));
        }
        if previews.len() > 60 {
            preview_col = preview_col.push(ui::muted(
                env,
                format!("… e mais {} arquivo(s).", previews.len() - 60),
            ));
        }
    }

    let exec_btn = ui::actions(
        env,
        vec![act(
            format!("✅  Aplicar renomeação em {changes_count} arquivo(s)"),
            Btn::Primary,
            (changes_count > 0 && !state.busy).then_some(Msg::Execute),
        )],
    );

    let mut body = column![file_btns, rule_field, params, preview_col, exec_btn].spacing(16);

    if state.busy {
        body = body.push(ui::busy(env, "Renomeando arquivos"));
    }
    if let Some(res) = &state.result {
        let open = res.output.clone().map(Msg::OpenPath);
        body = body.push(ui::result_banner(env, res, open));
    }

    let main = ui::section(
        env,
        "✨",
        "Configurar Regra e Pré-visualizar",
        "Nada é alterado no disco até você clicar em \"Aplicar renomeação\".",
        body,
    );

    let side = ui::help(
        env,
        &[
            "Clique em \"Escolher arquivos…\" (ou carregue uma pasta inteira).",
            "Escolha a regra desejada: substituir texto, adicionar prefixo/sufixo, numerar ou mudar maiúsculas/minúsculas.",
            "Confira na tabela de pré-visualização exatamente como cada nome vai ficar (Antigo → Novo).",
            "Clique em \"Aplicar renomeação\" para renomear todos de uma vez.",
        ],
        Some("A extensão do arquivo (.jpg, .pdf, .mp3...) é sempre preservada automaticamente."),
    );

    ui::page(env, vec![header, ui::two_col(env, main, side)])
}
