//! Interface do módulo de Imagens: Revelador de Fotos ao Vivo
//! (Luz, Cor, Efeitos, Histogramas, Presets, Lote e Redimensionar/Converter).

use iced::widget::{
    button, checkbox, column, container, image as iced_image, pick_list, row, slider, text,
};
use iced::{Alignment, ContentFit, Element, Length};

use crate::modules::images::develop::Adj;
use crate::modules::images::service::{AspectCrop, OutFormat, PhotoFilter};
use crate::modules::images::{EditorTab, Mode, Msg, State};
use crate::shared::theme::{self, BannerKind, Btn};
use crate::shared::types::Env;
use crate::shared::ui::{self, act, human_size, SEMIBOLD};

pub fn view<'a>(state: &'a State, env: &Env) -> Element<'a, Msg> {
    let is_en = env.lang.is_en();
    let header = ui::page_header(
        env,
        "🖼",
        if is_en {
            "Photo Studio & Developer"
        } else {
            "Estúdio & Revelador de Fotos"
        },
        if is_en {
            "Non-destructive photo editing with live real-time preview, RGB histogram, professional Light, Color, and Effects controls, batch folder sync, and cinema looks."
        } else {
            "Edição não destrutiva com pré-visualização ao vivo, histograma RGB, controles profissionais de Luz, Cor e Efeitos, sincronização em lote e presets de cinema."
        },
        None,
    );

    let tabs = row![
        ui::btn(
            if is_en {
                "🎨  Pro Developer (Live)"
            } else {
                "🎨  Revelador Profissional (Ao Vivo)"
            },
            Btn::Tab(state.mode == Mode::Editor),
            Some(Msg::ModeChanged(Mode::Editor)),
        ),
        ui::btn(
            if is_en {
                "⚡  Batch Sync (Folder)"
            } else {
                "⚡  Sincronizar em Lote (Pasta)"
            },
            Btn::Tab(state.mode == Mode::Batch),
            Some(Msg::ModeChanged(Mode::Batch)),
        ),
        ui::btn(
            if is_en {
                "📐  Resize & Formats"
            } else {
                "📐  Redimensionar & Formatos"
            },
            Btn::Tab(state.mode == Mode::ConvertResize),
            Some(Msg::ModeChanged(Mode::ConvertResize)),
        ),
    ]
    .spacing(10)
    .align_items(Alignment::Center);

    let io_card = io_section(state, env);

    let body = match state.mode {
        Mode::Editor => photo_editor_view(state, env),
        Mode::Batch => batch_view(state, env),
        Mode::ConvertResize => convert_resize_view(state, env),
    };

    ui::page(env, vec![header, tabs.into(), io_card, body])
}

fn io_section<'a>(state: &'a State, env: &Env) -> Element<'a, Msg> {
    let c = env.c();
    let in_field = ui::field(
        env,
        "Foto ou imagem de entrada",
        "Formatos aceitos: PNG, JPG/JPEG, WebP, BMP, GIF, TIFF e ICO.",
        ui::path_field(
            env,
            "Escolha uma foto do seu computador…",
            &state.input,
            Msg::InputChanged,
            "📂  Escolher foto…",
            Msg::BrowseInput,
        ),
    );

    let out_field = ui::field(
        env,
        "Arquivo ou pasta de saída (opcional)",
        "Se deixar em branco, a foto editada é salva ao lado da original sem sobrescrever (gravação atômica).",
        ui::path_field(
            env,
            "Automático (mesma pasta da foto original)",
            &state.output,
            Msg::OutputChanged,
            "💾  Salvar como…",
            Msg::BrowseOutput,
        ),
    );

    let fields: Element<'a, Msg> = if env.wide() {
        row![in_field, out_field].spacing(14).into()
    } else {
        column![in_field, out_field].spacing(10).into()
    };

    let mut col = column![fields].spacing(10);
    if let Some(meta_res) = &state.meta {
        match meta_res {
            Ok(m) => {
                col = col.push(ui::list_row(
                    row![
                        ui::badge(&m.format_label, c.primary),
                        ui::strong(format!("Original: {} × {} px", m.width, m.height), 14)
                            .width(Length::Fill),
                        ui::badge(format!("Peso: {}", human_size(m.size_bytes)), c.accent),
                    ]
                    .spacing(12)
                    .align_items(Alignment::Center),
                ));
            }
            Err(err) => {
                col = col.push(ui::info_banner(env, BannerKind::Error, "⚠", err));
            }
        }
    }

    ui::card(env, col)
}

// ---------------------------------------------------------------------------
// Revelador de Fotos ao Vivo
// ---------------------------------------------------------------------------

fn photo_editor_view<'a>(state: &'a State, env: &Env) -> Element<'a, Msg> {
    let c = env.c();
    let has_img = state.preview_handle.is_some() && !state.busy;

    // Canvas de visualização
    let canvas_box: Element<'a, Msg> = match &state.preview_handle {
        Some(handle) => container(
            iced_image::Image::new(handle.clone())
                .width(Length::Fill)
                .height(Length::Fixed(310.0))
                .content_fit(ContentFit::Contain),
        )
        .padding(10)
        .width(Length::Fill)
        .center_x()
        .center_y()
        .style(theme::list_row)
        .into(),
        None => container(ui::empty_state(
            env,
            "🖼",
            "Selecione uma foto acima para abrir no Revelador",
            "Histograma, luz, cor, nitidez, granulação e presets serão aplicados e vistos em tempo real.",
        ))
        .height(Length::Fixed(280.0))
        .width(Length::Fill)
        .center_x()
        .center_y()
        .style(theme::list_row)
        .into(),
    };

    let transform_bar = ui::actions(
        env,
        vec![
            act(
                "↺  Girar 90° Esq.",
                Btn::Secondary,
                has_img.then_some(Msg::RotateLeft),
            ),
            act(
                "↻  Girar 90° Dir.",
                Btn::Secondary,
                has_img.then_some(Msg::RotateRight),
            ),
            act(
                if state.edits.flip_h {
                    "↔  Espelhado H ✓"
                } else {
                    "↔  Espelhar H"
                },
                Btn::Secondary,
                has_img.then_some(Msg::FlipH),
            ),
            act(
                if state.edits.flip_v {
                    "↕  Espelhado V ✓"
                } else {
                    "↕  Espelhar V"
                },
                Btn::Secondary,
                has_img.then_some(Msg::FlipV),
            ),
            act(
                "🔄  Restaurar original",
                Btn::Ghost,
                has_img.then_some(Msg::ResetEdits),
            ),
        ],
    );

    // Histograma ao vivo
    let hist_box: Element<'a, Msg> = if let Some(h) = &state.hist_handle {
        container(
            column![
                row![
                    ui::strong("📊  Histograma RGB + Luminância", 12).width(Length::Fill),
                    ui::badge("Ao vivo", c.accent),
                ]
                .align_items(Alignment::Center),
                container(
                    iced_image::Image::new(h.clone())
                        .width(Length::Fill)
                        .height(Length::Fixed(60.0))
                )
                .width(Length::Fill)
                .style(theme::list_row),
            ]
            .spacing(4),
        )
        .padding(8)
        .style(theme::list_row)
        .into()
    } else {
        container(
            text("Carregue uma imagem para ver o histograma.")
                .size(12)
                .style(c.muted),
        )
        .padding(8)
        .into()
    };

    // Recortes
    let crop_btns: Vec<Element<'a, Msg>> = AspectCrop::ALL
        .iter()
        .map(|&cr| {
            let active = state.edits.crop == cr;
            button(
                container(text(cr.label()).size(12).font(SEMIBOLD))
                    .center_x()
                    .width(Length::Fill),
            )
            .padding([8, 10])
            .width(Length::Fill)
            .style(theme::btn(Btn::Tab(active)))
            .on_press_maybe(has_img.then_some(Msg::CropSelected(cr)))
            .into()
        })
        .collect();
    let crops_grid = ui::grid(crop_btns, env.columns(170.0, 3), 8);

    let save_btn = button(
        container(
            text("💾   Exportar Foto Revelada em Resolução Máxima")
                .size(15)
                .font(ui::BOLD),
        )
        .center_x()
        .width(Length::Fill),
    )
    .padding([14, 22])
    .width(Length::Fill)
    .style(theme::btn(Btn::Primary))
    .on_press_maybe(has_img.then_some(Msg::SaveEditedPhoto));

    let mut left_col = column![
        ui::section(
            env,
            "👁",
            "Monitor de Pré-visualização",
            "Renderização com iluminação, cor e filtros atualizados em tempo real.",
            column![canvas_box, transform_bar, hist_box].spacing(10),
        ),
        ui::section(
            env,
            "✂",
            "Enquadramento & Proporção",
            "Corte no formato exato para Instagram (1:1), YouTube (16:9), Reels/TikTok (9:16) ou 4:3.",
            crops_grid,
        ),
    ]
    .spacing(14);

    if state.busy {
        left_col = left_col.push(ui::busy(env, "Processando imagem em alta resolução"));
    }
    if let Some(res) = &state.result {
        let open = res.output.clone().map(Msg::OpenPath);
        left_col = left_col.push(ui::result_banner(env, res, open));
    }

    // Abas de ajustes: Luz / Cor / Efeitos / Presets
    let subtabs = row![
        ui::btn(
            "☀ Luz",
            Btn::Tab(state.editor_tab == EditorTab::Light),
            Some(Msg::EditorTabChanged(EditorTab::Light))
        ),
        ui::btn(
            "🌈 Cor",
            Btn::Tab(state.editor_tab == EditorTab::Color),
            Some(Msg::EditorTabChanged(EditorTab::Color))
        ),
        ui::btn(
            "✨ Efeitos",
            Btn::Tab(state.editor_tab == EditorTab::Effects),
            Some(Msg::EditorTabChanged(EditorTab::Effects))
        ),
        ui::btn(
            "🎬 Presets",
            Btn::Tab(state.editor_tab == EditorTab::Presets),
            Some(Msg::EditorTabChanged(EditorTab::Presets))
        ),
    ]
    .spacing(6);

    let adjustments_panel: Element<'a, Msg> = match state.editor_tab {
        EditorTab::Light => render_sliders_block(Adj::LIGHT, state, env),
        EditorTab::Color => {
            let filter_btns: Vec<Element<'a, Msg>> = PhotoFilter::ALL
                .iter()
                .map(|&f| {
                    let active = state.edits.filter == f;
                    button(
                        container(text(f.label()).size(12).font(SEMIBOLD))
                            .center_x()
                            .width(Length::Fill),
                    )
                    .padding([7, 10])
                    .width(Length::Fill)
                    .style(theme::btn(Btn::Tab(active)))
                    .on_press_maybe(has_img.then_some(Msg::FilterSelected(f)))
                    .into()
                })
                .collect();
            let fgrid = ui::grid(filter_btns, env.columns(130.0, 4), 6);
            column![
                render_sliders_block(Adj::COLOR, state, env),
                ui::strong("Filtros de Estilo Rápido", 13),
                fgrid,
            ]
            .spacing(10)
            .into()
        }
        EditorTab::Effects => render_sliders_block(Adj::EFFECTS, state, env),
        EditorTab::Presets => {
            let preset_btns: Vec<Element<'a, Msg>> = state
                .presets
                .iter()
                .map(|p| {
                    let edits = p.edits;
                    button(
                        container(
                            column![
                                ui::strong(&p.name, 13),
                                text("Clique para carregar").size(11).style(c.muted),
                            ]
                            .spacing(2)
                            .align_items(Alignment::Center),
                        )
                        .center_x()
                        .width(Length::Fill),
                    )
                    .padding([10, 12])
                    .width(Length::Fill)
                    .style(theme::btn(Btn::Secondary))
                    .on_press_maybe(has_img.then_some(Msg::PresetApplied(edits)))
                    .into()
                })
                .collect();
            column![
                ui::strong(
                    "Looks Fotográficos Prontos (Estilo Lightroom / PhotoCraft)",
                    13
                ),
                ui::grid(preset_btns, env.columns(150.0, 2), 8),
            ]
            .spacing(10)
            .into()
        }
    };

    let right_col = column![ui::section(
        env,
        "🎚",
        "Ajustes Não Destrutivos",
        "Mude as abas para ajustar parâmetros finos ou escolher estilos.",
        column![subtabs, adjustments_panel, save_btn].spacing(14),
    ),]
    .spacing(14);

    ui::two_col(env, left_col.into(), right_col.into())
}

fn render_sliders_block<'a>(adjs: &[Adj], state: &'a State, env: &Env) -> Element<'a, Msg> {
    let c = env.c();
    let has_img = state.preview_handle.is_some() && !state.busy;
    let mut col = column![].spacing(8);

    for &adj in adjs {
        let val = adj.get(&state.edits);
        let (lo, hi) = adj.range();
        let row_item = ui::list_row(
            column![
                row![
                    ui::strong(adj.label(), 12).width(Length::Fill),
                    ui::badge(
                        format!("{:+}", val),
                        if val == 0 { c.muted } else { c.accent }
                    ),
                ]
                .align_items(Alignment::Center),
                slider(lo..=hi, val, move |v| Msg::AdjChanged(adj, v)),
            ]
            .spacing(4),
        );
        col = col.push(row_item);
    }

    if !has_img {
        col = col.push(
            text("Selecione uma foto para habilitar os controles.")
                .size(12)
                .style(c.muted),
        );
    }

    col.into()
}

// ---------------------------------------------------------------------------
// Sincronizar em Lote (Estilo LightCraft / Lightroom)
// ---------------------------------------------------------------------------

fn batch_view<'a>(state: &'a State, env: &Env) -> Element<'a, Msg> {
    let has_folder = !state.batch_folder.trim().is_empty() && !state.busy;

    let folder_picker = ui::field(
        env,
        "Pasta com as imagens que você quer sincronizar",
        "Todas as fotos da pasta (PNG, JPG, WEBP, GIF, TIFF, BMP) receberão os ajustes ativos agora no Revelador.",
        ui::path_field(
            env,
            "Escolha uma pasta do computador…",
            &state.batch_folder,
            Msg::BatchFolderChanged,
            "📂  Escolher pasta…",
            Msg::BrowseBatchFolder,
        ),
    );

    let summary = ui::list_row(
        column![
            ui::strong("Ajustes ativos que serão aplicados nas fotos:", 13),
            text(format!(
                "Exposição: {:+} | Contraste: {:+} | Realces: {:+} | Sombras: {:+} | Temp: {:+} | Vibração: {:+} | Nitidez: {} | Vinheta: {:+}",
                state.edits.exposure,
                state.edits.contrast,
                state.edits.highlights,
                state.edits.shadows,
                state.edits.temperature,
                state.edits.vibrance,
                state.edits.sharpen,
                state.edits.vignette
            ))
            .size(12)
            .style(env.c().muted),
        ]
        .spacing(4),
    );

    let run_btn = button(
        container(
            text("⚡   Aplicar Edições em Todas as Fotos da Pasta")
                .size(15)
                .font(ui::BOLD),
        )
        .center_x()
        .width(Length::Fill),
    )
    .padding([14, 22])
    .width(Length::Fill)
    .style(theme::btn(Btn::Primary))
    .on_press_maybe(has_folder.then_some(Msg::RunBatch));

    let mut main_col = column![
        ui::section(
            env,
            "⚡",
            "Processador em Lote (Estilo LightCraft)",
            "Ajuste uma foto no Revelador para achar o tom perfeito, venha aqui e sincronize centenas de imagens de uma só vez para uma subpasta sem tocar nas originais.",
            column![folder_picker, summary, run_btn].spacing(14),
        )
    ]
    .spacing(14);

    if state.busy {
        main_col = main_col.push(ui::busy(env, "Processando fotos da pasta em lote"));
    }
    if let Some(res) = &state.result {
        let open = res.output.clone().map(Msg::OpenPath);
        main_col = main_col.push(ui::result_banner(env, res, open));
    }

    let side = ui::help(
        env,
        &[
            "Primeiro abra uma foto na aba Revelador e ajuste a luz e as cores até ficar ideal.",
            "Venha para esta aba, selecione a pasta inteira onde estão suas fotos.",
            "Clique no botão de aplicar: uma pasta Axolote_Editadas será criada com tudo salvo em alta qualidade.",
            "As fotos originais NUNCA são modificadas ou apagadas.",
        ],
        Some("Dica: você pode usar presets prontos de Cinema ou Retrô antes de rodar o lote!"),
    );

    ui::two_col(env, main_col.into(), side)
}

// ---------------------------------------------------------------------------
// Redimensionar, Converter e Comprimir
// ---------------------------------------------------------------------------

fn convert_resize_view<'a>(state: &'a State, env: &Env) -> Element<'a, Msg> {
    let c = env.c();
    let has_input = !state.input.trim().is_empty() && !state.busy;

    let presets = row![
        ui::muted(env, "Escala rápida:"),
        ui::btn(
            "25%",
            Btn::Secondary,
            has_input.then_some(Msg::ScalePreset(25))
        ),
        ui::btn(
            "50%",
            Btn::Secondary,
            has_input.then_some(Msg::ScalePreset(50))
        ),
        ui::btn(
            "75%",
            Btn::Secondary,
            has_input.then_some(Msg::ScalePreset(75))
        ),
        ui::btn(
            "100%",
            Btn::Secondary,
            has_input.then_some(Msg::ScalePreset(100))
        ),
    ]
    .spacing(8)
    .align_items(Alignment::Center);

    let dims_row: Element<'a, Msg> = {
        let w = ui::field(
            env,
            "Nova largura (px)",
            "",
            ui::input("Ex.: 1920", &state.width, Msg::WidthChanged),
        );
        let h = ui::field(
            env,
            "Nova altura (px)",
            "",
            ui::input("Ex.: 1080", &state.height, Msg::HeightChanged),
        );
        if env.narrow() {
            column![w, h].spacing(10).into()
        } else {
            row![w, h].spacing(12).into()
        }
    };

    let aspect_chk = checkbox(
        "Manter proporção original automaticamente",
        state.keep_aspect,
    )
    .on_toggle(Msg::KeepAspectToggled)
    .size(18)
    .text_size(13);

    let resize_card = ui::section(
        env,
        "📐",
        "Redimensionar Imagem",
        "Altere a largura e a altura em pixels com reamostragem Lanczos3 de alta nitidez.",
        column![
            presets,
            dims_row,
            aspect_chk,
            ui::actions(
                env,
                vec![act(
                    "📐  Redimensionar agora",
                    Btn::Primary,
                    has_input.then_some(Msg::ResizeRun)
                )]
            ),
        ]
        .spacing(12),
    );

    let fmt_picker = pick_list(OutFormat::ALL, Some(state.format), Msg::FormatChanged)
        .padding([10, 12])
        .width(Length::Fill);

    let convert_card = ui::section(
        env,
        "🔄",
        "Converter Formato",
        "Transforme a imagem em PNG, JPG, WebP, BMP ou ícone ICO do Windows.",
        column![
            ui::field(env, "Formato de saída desejado", "", fmt_picker),
            ui::actions(
                env,
                vec![act(
                    format!("🔄  Converter para {}", state.format.ext().to_uppercase()),
                    Btn::Primary,
                    has_input.then_some(Msg::ConvertRun),
                )],
            ),
        ]
        .spacing(12),
    );

    let q_slider = slider(10..=100, state.quality, Msg::QualityChanged);
    let q_hint = match state.quality {
        90..=100 => "Qualidade máxima",
        70..=89 => "Equilíbrio recomendado",
        45..=69 => "Compressão alta (WhatsApp/E-mail)",
        _ => "Compressão extrema",
    };

    let compress_card = ui::section(
        env,
        "✨",
        "Comprimir Foto (Reduzir MB/KB)",
        "Gera um arquivo JPEG otimizado ajustando o nível de qualidade.",
        column![
            row![
                ui::strong(format!("Qualidade: {}%", state.quality), 14).width(Length::Fill),
                ui::badge(q_hint, c.accent),
            ]
            .align_items(Alignment::Center),
            q_slider,
            ui::actions(
                env,
                vec![act(
                    "✨  Comprimir imagem agora",
                    Btn::Primary,
                    has_input.then_some(Msg::CompressRun)
                )]
            ),
        ]
        .spacing(12),
    );

    let mut main_col = column![resize_card, convert_card, compress_card].spacing(16);
    if state.busy {
        main_col = main_col.push(ui::busy(env, "Processando imagem"));
    }
    if let Some(res) = &state.result {
        let open = res.output.clone().map(Msg::OpenPath);
        main_col = main_col.push(ui::result_banner(env, res, open));
    }

    let side = ui::help(
        env,
        &[
            "Escolha a foto no cartão superior.",
            "Use os botões 25%, 50%, 75% para reduzir fotos pesadas de celular rapidamente.",
            "Converta qualquer imagem para ícone (.ico) de até 256x256 com proporção ajustada.",
        ],
        Some(
            "Para revelar exposição, sombras, nitidez e cores, use a aba 'Revelador Profissional'.",
        ),
    );

    ui::two_col(env, main_col.into(), side)
}
