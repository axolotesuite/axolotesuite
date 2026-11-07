//! Interface do módulo PDF.

use iced::widget::{column, row, text};
use iced::{Alignment, Element, Length};

use crate::modules::pdf::{Mode, Msg, State};
use crate::shared::theme::{BannerKind, Btn};
use crate::shared::types::Env;
use crate::shared::ui::{self, act, human_size, SEMIBOLD};

pub fn view<'a>(state: &'a State, env: &Env) -> Element<'a, Msg> {
    let header = ui::page_header(
        env,
        "📄",
        "Central de Documentos PDF",
        "Junte vários arquivos PDF na ordem que quiser, extraia páginas separadas e consulte informações técnicas do documento.",
        None,
    );

    let tabs = row![
        ui::btn("🔗  Unir vários PDFs em um só", Btn::Tab(state.mode == Mode::Merge), Some(Msg::ModeChanged(Mode::Merge))),
        ui::btn("✂  Dividir PDF em páginas / Informações", Btn::Tab(state.mode == Mode::Split), Some(Msg::ModeChanged(Mode::Split))),
    ]
    .spacing(10)
    .align_items(Alignment::Center);

    let content = match state.mode {
        Mode::Merge => merge_view(state, env),
        Mode::Split => split_view(state, env),
    };

    ui::page(env, vec![header, tabs.into(), content])
}

fn merge_view<'a>(state: &'a State, env: &Env) -> Element<'a, Msg> {
    let c = env.c();
    let can_merge = state.merge_files.len() >= 2 && !state.merge_output.trim().is_empty() && !state.busy;

    let top_btns = ui::actions(
        env,
        vec![
            act("➕  Adicionar arquivos PDF…", Btn::Primary, (!state.busy).then_some(Msg::AddMergeFiles)),
            act(
                "🗑  Limpar lista",
                Btn::Ghost,
                (!state.merge_files.is_empty() && !state.busy).then_some(Msg::ClearMerge),
            ),
        ],
    );

    let total_pages: usize = state.merge_files.iter().filter_map(|(_, p)| *p).sum();
    let mut list_col = column![row![
        ui::strong(format!("Ordem dos documentos ({})", state.merge_files.len()), 15).width(Length::Fill),
        ui::badge(format!("Total estimado: {total_pages} página(s)"), c.accent),
    ]
    .align_items(Alignment::Center)]
    .spacing(8);

    if state.merge_files.is_empty() {
        list_col = list_col.push(ui::empty_state(
            env,
            "📄",
            "Nenhum PDF adicionado",
            "Clique em \"Adicionar arquivos PDF…\" e escolha 2 ou mais documentos para juntar.",
        ));
    } else {
        let len = state.merge_files.len();
        for (i, (path, pages)) in state.merge_files.iter().enumerate() {
            let pages_badge = match pages {
                Some(n) => ui::badge(format!("{n} pág."), c.primary),
                None => ui::badge("PDF", c.muted),
            };
            list_col = list_col.push(ui::list_row(
                row![
                    ui::badge(i + 1, c.accent),
                    text(ui::file_name(path)).size(13).font(SEMIBOLD).width(Length::Fill),
                    pages_badge,
                    ui::btn("↑", Btn::Secondary, (i > 0).then_some(Msg::MoveUp(i))),
                    ui::btn("↓", Btn::Secondary, (i + 1 < len).then_some(Msg::MoveDown(i))),
                    ui::btn("✕", Btn::Ghost, Some(Msg::RemoveMergeFile(i))),
                ]
                .spacing(8)
                .align_items(Alignment::Center),
            ));
        }
    }

    let out_field = ui::field(
        env,
        "Arquivo PDF unido de saída",
        "Onde o novo PDF combinado será salvo.",
        ui::path_field(
            env,
            "Ex.: C:\\Users\\voce\\Desktop\\documento_unido.pdf",
            &state.merge_output,
            Msg::MergeOutputChanged,
            "💾  Salvar como…",
            Msg::BrowseMergeOutput,
        ),
    );

    let merge_btn = ui::actions(env, vec![act("🔗  Unir PDFs agora", Btn::Primary, can_merge.then_some(Msg::MergeRun))]);

    let mut body = column![top_btns, list_col, out_field, merge_btn].spacing(16);

    if state.busy {
        body = body.push(ui::busy(env, "Unindo documentos PDF"));
    }
    if let Some(res) = &state.result {
        let open = res.output.clone().map(Msg::OpenPath);
        body = body.push(ui::result_banner(env, res, open));
    }

    let main = ui::section(
        env,
        "🔗",
        "Unir Múltiplos PDFs",
        "Use as setas ↑ e ↓ para ajustar a ordem em que cada PDF entrará no documento final.",
        body,
    );

    let side = ui::help(
        env,
        &[
            "Clique em \"Adicionar arquivos PDF…\" e selecione 2 ou mais arquivos.",
            "Reordene os arquivos com os botões ↑ e ↓ se precisar mudar a sequência das páginas.",
            "Clique em \"Unir PDFs agora\" para gerar o documento único.",
        ],
        Some("Os arquivos originais permanecem intactos; um novo arquivo PDF é criado com todas as páginas."),
    );

    ui::two_col(env, main, side)
}

fn split_view<'a>(state: &'a State, env: &Env) -> Element<'a, Msg> {
    let c = env.c();
    let has_pdf = !state.split_input.trim().is_empty() && !state.busy;

    let in_field = ui::field(
        env,
        "Documento PDF de entrada",
        "Ao selecionar o arquivo, o número de páginas e os detalhes técnicos aparecem abaixo.",
        ui::path_field(
            env,
            "Escolha um arquivo .pdf…",
            &state.split_input,
            Msg::SplitInputChanged,
            "📂  Escolher PDF…",
            Msg::BrowseSplitInput,
        ),
    );

    let mut info_block = column![].spacing(8);
    if let Some(meta_res) = &state.split_meta {
        match meta_res {
            Ok(m) => {
                info_block = info_block.push(ui::list_row(
                    row![
                        ui::badge(format!("PDF v{}", m.version), c.primary),
                        ui::strong(format!("{} página(s)", m.pages), 15).width(Length::Fill),
                        ui::badge(format!("Tamanho: {}", human_size(m.size_bytes)), c.accent),
                        ui::badge(
                            if m.encrypted { "🔒 Criptografado" } else { "🔓 Sem senha" },
                            if m.encrypted { c.warning } else { c.success },
                        ),
                    ]
                    .spacing(10)
                    .align_items(Alignment::Center),
                ));
            }
            Err(err) => {
                info_block = info_block.push(ui::info_banner(env, BannerKind::Error, "⚠", err));
            }
        }
    }

    let range_row: Element<'a, Msg> = {
        let from = ui::field(
            env,
            "Da página",
            "Primeira página a extrair (começa em 1).",
            ui::input("1", &state.page_from, Msg::PageFromChanged),
        );
        let to = ui::field(
            env,
            "Até a página",
            "Última página a extrair.",
            ui::input("Ex.: 10", &state.page_to, Msg::PageToChanged),
        );
        if env.narrow() {
            column![from, to].spacing(10).into()
        } else {
            row![from, to].spacing(12).into()
        }
    };

    let out_dir_field = ui::field(
        env,
        "Pasta onde as páginas serão salvas",
        "Cada página virará um arquivo separado (ex.: contrato_pagina_01.pdf, contrato_pagina_02.pdf...).",
        ui::path_field(
            env,
            "Automático (subpasta ao lado do PDF original)",
            &state.split_out_dir,
            Msg::SplitOutDirChanged,
            "📁  Escolher pasta…",
            Msg::BrowseSplitOutDir,
        ),
    );

    let split_btn = ui::actions(
        env,
        vec![act("✂  Extrair páginas em PDFs separados", Btn::Primary, has_pdf.then_some(Msg::SplitRun))],
    );

    let mut body = column![in_field, info_block, range_row, out_dir_field, split_btn].spacing(14);

    if state.busy {
        body = body.push(ui::busy(env, "Dividindo páginas do PDF"));
    }
    if let Some(res) = &state.result {
        let open = res.output.clone().map(Msg::OpenPath);
        body = body.push(ui::result_banner(env, res, open));
    }

    let main = ui::section(
        env,
        "✂",
        "Dividir PDF e Consultar Informações",
        "Extraia todas as páginas ou apenas um intervalo específico (por exemplo, da página 2 até a 5).",
        body,
    );

    let side = ui::help(
        env,
        &[
            "Clique em \"Escolher PDF…\" para carregar o documento e ver quantas páginas ele tem.",
            "Se quiser extrair só algumas páginas, ajuste os campos \"Da página\" e \"Até a página\".",
            "Clique em \"Extrair páginas em PDFs separados\".",
        ],
        Some("Para extrair apenas uma única página (ex.: página 3), basta colocar 3 nos dois campos."),
    );

    ui::two_col(env, main, side)
}
