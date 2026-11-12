//! Interface do módulo ZIP.

use iced::widget::{column, row, text};
use iced::{Alignment, Element, Length};

use crate::modules::zip::{Mode, Msg, State};
use crate::shared::theme::{BannerKind, Btn};
use crate::shared::types::Env;
use crate::shared::ui::{self, act, human_size, SEMIBOLD};

pub fn view<'a>(state: &'a State, env: &Env) -> Element<'a, Msg> {
    let header = ui::page_header(
        env,
        "🗜",
        "Compactador e Extrator ZIP",
        "Extraia arquivos .zip com verificação automática de segurança ou crie novos pacotes compactados com arquivos e pastas.",
        None,
    );

    let tabs = row![
        ui::btn(
            "📂  Extrair / Ver conteúdo de um ZIP",
            Btn::Tab(state.mode == Mode::Extract),
            Some(Msg::ModeChanged(Mode::Extract))
        ),
        ui::btn(
            "📦  Criar novo arquivo ZIP",
            Btn::Tab(state.mode == Mode::Create),
            Some(Msg::ModeChanged(Mode::Create))
        ),
    ]
    .spacing(10)
    .align_items(Alignment::Center);

    let content = match state.mode {
        Mode::Extract => extract_view(state, env),
        Mode::Create => create_view(state, env),
    };

    ui::page(env, vec![header, tabs.into(), content])
}

fn extract_view<'a>(state: &'a State, env: &Env) -> Element<'a, Msg> {
    let c = env.c();
    let has_zip = !state.zip_path.trim().is_empty();

    let zip_field = ui::field(
        env,
        "Arquivo ZIP de entrada",
        "Escolha o arquivo .zip que você quer abrir ou descomprimir.",
        ui::path_field(
            env,
            "Ex.: C:\\Users\\voce\\Downloads\\documentos.zip",
            &state.zip_path,
            Msg::ZipPathChanged,
            "📂  Escolher .zip…",
            Msg::BrowseZip,
        ),
    );

    let dest_field = ui::field(
        env,
        "Pasta de destino da extração",
        "Se deixar em branco, será criada uma pasta com o mesmo nome do arquivo ZIP.",
        ui::path_field(
            env,
            "Ex.: C:\\Users\\voce\\Downloads\\documentos",
            &state.dest_dir,
            Msg::DestDirChanged,
            "📁  Escolher pasta…",
            Msg::BrowseDest,
        ),
    );

    let btns = ui::actions(
        env,
        vec![
            act(
                "🔍  Listar conteúdo do ZIP",
                Btn::Secondary,
                (has_zip && !state.busy).then_some(Msg::InspectZip),
            ),
            act(
                "📤  Extrair arquivos agora",
                Btn::Primary,
                (has_zip && !state.busy).then_some(Msg::ExtractRun),
            ),
        ],
    );

    let mut body = column![zip_field, dest_field, btns].spacing(14);

    if state.busy {
        body = body.push(ui::busy(env, "Extraindo arquivos do ZIP"));
    }

    if let Some(entries_res) = &state.entries {
        match entries_res {
            Ok(list) => {
                let total_uncompressed: u64 = list.iter().map(|e| e.size).sum();
                let mut col = column![row![
                    ui::strong(format!("Conteúdo do ZIP ({} item(ns))", list.len()), 15)
                        .width(Length::Fill),
                    ui::badge(
                        format!("Total: {}", human_size(total_uncompressed)),
                        c.accent
                    ),
                ]
                .align_items(Alignment::Center)]
                .spacing(6);

                for item in list.iter().take(50) {
                    let icon = if item.is_dir { "📁" } else { "📄" };
                    col = col.push(ui::list_row(
                        row![
                            text(format!("{icon}  {}", item.name))
                                .size(13)
                                .width(Length::Fill),
                            ui::muted(env, human_size(item.size)),
                        ]
                        .spacing(10)
                        .align_items(Alignment::Center),
                    ));
                }
                if list.len() > 50 {
                    col = col.push(ui::muted(
                        env,
                        format!("… e mais {} item(ns) dentro do arquivo.", list.len() - 50),
                    ));
                }
                body = body.push(col);
            }
            Err(err) => {
                body = body.push(ui::info_banner(env, BannerKind::Error, "⚠", err));
            }
        }
    }

    if let Some(res) = &state.result {
        let open = res.output.clone().map(Msg::OpenPath);
        body = body.push(ui::result_banner(env, res, open));
    }

    let main = ui::section(
        env,
        "📂",
        "Extrair ou Inspecionar Arquivo ZIP",
        "Protegido contra caminhos maliciosos (Zip-Slip): nenhum arquivo pode escapar da pasta de destino.",
        body,
    );

    let side = ui::help(
        env,
        &[
            "Clique em \"Escolher .zip…\" e selecione o arquivo compactado.",
            "Confira a lista de arquivos que estão dentro do ZIP antes mesmo de extrair.",
            "Clique em \"Extrair arquivos agora\" e depois em \"Abrir no Explorer\" para acessar os arquivos.",
        ],
        Some("Ao escolher um arquivo .zip, a pasta de destino já é preenchida automaticamente para você."),
    );

    ui::two_col(env, main, side)
}

fn create_view<'a>(state: &'a State, env: &Env) -> Element<'a, Msg> {
    let c = env.c();
    let can_create =
        !state.sources.is_empty() && !state.output_zip.trim().is_empty() && !state.busy;

    let add_btns = ui::actions(
        env,
        vec![
            act(
                "➕  Adicionar arquivos…",
                Btn::Primary,
                (!state.busy).then_some(Msg::AddFiles),
            ),
            act(
                "📁  Adicionar pasta inteira…",
                Btn::Secondary,
                (!state.busy).then_some(Msg::AddFolder),
            ),
            act(
                "🗑  Limpar lista",
                Btn::Ghost,
                (!state.sources.is_empty() && !state.busy).then_some(Msg::ClearSources),
            ),
        ],
    );

    let mut list_col = column![row![ui::strong(
        format!("Itens para compactar ({})", state.sources.len()),
        15
    )
    .width(Length::Fill),]]
    .spacing(6);

    if state.sources.is_empty() {
        list_col = list_col.push(ui::empty_state(
            env,
            "📦",
            "Nenhum arquivo ou pasta na lista",
            "Adicione arquivos avulsos ou pastas completas usando os botões acima.",
        ));
    } else {
        for (i, p) in state.sources.iter().enumerate() {
            let is_dir = p.is_dir();
            let icon = if is_dir { "📁" } else { "📄" };
            let kind_badge = if is_dir {
                ui::badge("Pasta completa", c.primary)
            } else {
                let sz = std::fs::metadata(p)
                    .map(|m| human_size(m.len()))
                    .unwrap_or_default();
                ui::badge(sz, c.muted)
            };
            list_col = list_col.push(ui::list_row(
                row![
                    text(format!("{icon}  {}", p.display()))
                        .size(13)
                        .font(SEMIBOLD)
                        .width(Length::Fill),
                    kind_badge,
                    ui::btn("✕", Btn::Ghost, Some(Msg::RemoveSource(i))),
                ]
                .spacing(10)
                .align_items(Alignment::Center),
            ));
        }
    }

    let out_field = ui::field(
        env,
        "Onde salvar o novo arquivo .zip",
        "Escolha o local e o nome do arquivo compactado que será gerado.",
        ui::path_field(
            env,
            "Ex.: C:\\Users\\voce\\Desktop\\pacote.zip",
            &state.output_zip,
            Msg::OutputZipChanged,
            "💾  Salvar como…",
            Msg::BrowseOutputZip,
        ),
    );

    let create_btn = ui::actions(
        env,
        vec![act(
            "🗜  Criar arquivo ZIP agora",
            Btn::Primary,
            can_create.then_some(Msg::CreateRun),
        )],
    );

    let mut body = column![add_btns, list_col, out_field, create_btn].spacing(16);

    if state.busy {
        body = body.push(ui::busy(env, "Compactando arquivos em ZIP"));
    }
    if let Some(res) = &state.result {
        let open = res.output.clone().map(Msg::OpenPath);
        body = body.push(ui::result_banner(env, res, open));
    }

    let main = ui::section(
        env,
        "📦",
        "Criar Novo Arquivo ZIP",
        "Compactação Deflate compatível com o Explorador do Windows, WinRAR e 7-Zip.",
        body,
    );

    let side = ui::help(
        env,
        &[
            "Clique em \"Adicionar arquivos…\" ou \"Adicionar pasta inteira…\" (você pode misturar vários).",
            "Verifique ou escolha onde o novo arquivo .zip será salvo.",
            "Clique em \"Criar arquivo ZIP agora\".",
        ],
        Some("Quando você adiciona uma pasta, todas as subpastas e arquivos dela são preservados dentro do ZIP."),
    );

    ui::two_col(env, main, side)
}
