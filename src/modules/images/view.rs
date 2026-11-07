//! Interface do módulo de Imagens: Editor de Fotos com Preview ao Vivo +
//! Conversor, Redimensionador e Compressor.

use iced::widget::{button, checkbox, column, container, image as iced_image, pick_list, row, slider, text};
use iced::{Alignment, ContentFit, Element, Length};

use crate::modules::images::service::{AspectCrop, OutFormat, PhotoFilter};
use crate::modules::images::{Mode, Msg, State};
use crate::shared::theme::{self, BannerKind, Btn};
use crate::shared::types::Env;
use crate::shared::ui::{self, act, human_size, SEMIBOLD};

pub fn view<'a>(state: &'a State, env: &Env) -> Element<'a, Msg> {
    let header = ui::page_header(
        env,
        "🖼",
        "Estúdio & Editor de Fotos",
        "Edite fotos com pré-visualização ao vivo (filtros, luz, cor, rotação e corte), redimensione, converta formato e comprima.",
        None,
    );

    let tabs = row![
        ui::btn(
            "🎨  Editor de Fotos (Preview ao Vivo)",
            Btn::Tab(state.mode == Mode::Editor),
            Some(Msg::ModeChanged(Mode::Editor)),
        ),
        ui::btn(
            "📐  Redimensionar, Converter & Comprimir",
            Btn::Tab(state.mode == Mode::ConvertResize),
            Some(Msg::ModeChanged(Mode::ConvertResize)),
        ),
    ]
    .spacing(10)
    .align_items(Alignment::Center);

    let io_card = io_section(state, env);

    let body = match state.mode {
        Mode::Editor => photo_editor_view(state, env),
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
        "Se deixar em branco, a foto editada é salva ao lado da original com sufixo automático (nunca apaga sua original).",
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
                        ui::strong(format!("Original: {} × {} px", m.width, m.height), 14).width(Length::Fill),
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
// Aba 1: Editor de Fotos com Preview ao Vivo
// ---------------------------------------------------------------------------

fn photo_editor_view<'a>(state: &'a State, env: &Env) -> Element<'a, Msg> {
    let c = env.c();
    let has_img = state.preview_handle.is_some() && !state.busy;

    // Painel do Canvas (Pré-visualização ao vivo)
    let canvas_box: Element<'a, Msg> = match &state.preview_handle {
        Some(handle) => container(
            iced_image::Image::new(handle.clone())
                .width(Length::Fill)
                .height(Length::Fixed(310.0))
                .content_fit(ContentFit::Contain),
        )
        .padding(12)
        .width(Length::Fill)
        .center_x()
        .center_y()
        .style(theme::list_row)
        .into(),
        None => container(ui::empty_state(
            env,
            "🖼",
            "Selecione uma foto acima para abrir no Editor",
            "Você verá a imagem aqui e todas as mudanças de brilho, contraste, filtros e cortes acontecerão ao vivo.",
        ))
        .height(Length::Fixed(280.0))
        .width(Length::Fill)
        .center_x()
        .center_y()
        .style(theme::list_row)
        .into(),
    };

    // Botões de rotação e espelhamento abaixo do preview
    let transform_bar = ui::actions(
        env,
        vec![
            act("↺  Girar 90° Esq.", Btn::Secondary, has_img.then_some(Msg::RotateLeft)),
            act("↻  Girar 90° Dir.", Btn::Secondary, has_img.then_some(Msg::RotateRight)),
            act(
                if state.edits.flip_h { "↔  Espelhado H ✓" } else { "↔  Espelhar H" },
                Btn::Secondary,
                has_img.then_some(Msg::FlipH),
            ),
            act(
                if state.edits.flip_v { "↕  Espelhado V ✓" } else { "↕  Espelhar V" },
                Btn::Secondary,
                has_img.then_some(Msg::FlipV),
            ),
            act("🔄  Restaurar original", Btn::Ghost, has_img.then_some(Msg::ResetEdits)),
        ],
    );

    // Filtros rápidos em pílulas
    let filter_btns: Vec<Element<'a, Msg>> = PhotoFilter::ALL
        .iter()
        .map(|&f| {
            let active = state.edits.filter == f;
            button(container(text(f.label()).size(13).font(SEMIBOLD)).center_x().width(Length::Fill))
                .padding([9, 12])
                .width(Length::Fill)
                .style(theme::btn(Btn::Tab(active)))
                .on_press_maybe(has_img.then_some(Msg::FilterSelected(f)))
                .into()
        })
        .collect();
    let filters_grid = ui::grid(filter_btns, env.columns(145.0, 4), 8);

    // Recorte por proporção
    let crop_btns: Vec<Element<'a, Msg>> = AspectCrop::ALL
        .iter()
        .map(|&cr| {
            let active = state.edits.crop == cr;
            button(container(text(cr.label()).size(12).font(SEMIBOLD)).center_x().width(Length::Fill))
                .padding([8, 10])
                .width(Length::Fill)
                .style(theme::btn(Btn::Tab(active)))
                .on_press_maybe(has_img.then_some(Msg::CropSelected(cr)))
                .into()
        })
        .collect();
    let crops_grid = ui::grid(crop_btns, env.columns(170.0, 3), 8);

    // Sliders de luz e cor
    let sliders_col = column![
        ui::list_row(
            column![
                row![
                    ui::strong("☀  Brilho (Exposição)", 13).width(Length::Fill),
                    ui::badge(format!("{:+}", state.edits.brightness), c.accent),
                ],
                slider(-100..=100, state.edits.brightness, Msg::BrightnessChanged),
            ]
            .spacing(6),
        ),
        ui::list_row(
            column![
                row![
                    ui::strong("◐  Contraste", 13).width(Length::Fill),
                    ui::badge(format!("{:+}", state.edits.contrast), c.primary),
                ],
                slider(-100..=100, state.edits.contrast, Msg::ContrastChanged),
            ]
            .spacing(6),
        ),
        ui::list_row(
            column![
                row![
                    ui::strong("🌈  Saturação (Intensidade das Cores)", 13).width(Length::Fill),
                    ui::badge(format!("{:+}", state.edits.saturation), c.success),
                ],
                slider(-100..=100, state.edits.saturation, Msg::SaturationChanged),
            ]
            .spacing(6),
        ),
    ]
    .spacing(10);

    let save_btn = button(
        container(text("💾   Exportar Foto Editada em Resolução Máxima").size(15).font(ui::BOLD))
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
            "Monitor de Pré-visualização ao Vivo",
            "Tudo o que você ajusta ao lado aparece imediatamente aqui.",
            column![canvas_box, transform_bar].spacing(12),
        ),
        ui::section(
            env,
            "✂",
            "Enquadramento & Recorte Automático",
            "Corte no formato exato para Instagram (1:1), YouTube (16:9), Reels/TikTok (9:16) ou Foto 4:3.",
            crops_grid,
        ),
    ]
    .spacing(16);

    if state.busy {
        left_col = left_col.push(ui::busy(env, "Renderizando foto em alta resolução"));
    }
    if let Some(res) = &state.result {
        let open = res.output.clone().map(Msg::OpenPath);
        left_col = left_col.push(ui::result_banner(env, res, open));
    }

    let right_col = column![
        ui::section(
            env,
            "🎨",
            "Filtros Instantâneos",
            "Aplique estilos fotográficos com um clique.",
            filters_grid,
        ),
        ui::section(
            env,
            "🎚",
            "Luz e Cor",
            "Ajuste brilho, contraste e saturação com precisão.",
            column![sliders_col, save_btn].spacing(14),
        ),
    ]
    .spacing(16);

    ui::two_col(env, left_col.into(), right_col.into())
}

// ---------------------------------------------------------------------------
// Aba 2: Redimensionar, Converter e Comprimir
// ---------------------------------------------------------------------------

fn convert_resize_view<'a>(state: &'a State, env: &Env) -> Element<'a, Msg> {
    let c = env.c();
    let has_input = !state.input.trim().is_empty() && !state.busy;

    let presets = row![
        ui::muted(env, "Escala rápida:"),
        ui::btn("25%", Btn::Secondary, has_input.then_some(Msg::ScalePreset(25))),
        ui::btn("50%", Btn::Secondary, has_input.then_some(Msg::ScalePreset(50))),
        ui::btn("75%", Btn::Secondary, has_input.then_some(Msg::ScalePreset(75))),
        ui::btn("100%", Btn::Secondary, has_input.then_some(Msg::ScalePreset(100))),
    ]
    .spacing(8)
    .align_items(Alignment::Center);

    let dims_row: Element<'a, Msg> = {
        let w = ui::field(env, "Nova largura (px)", "", ui::input("Ex.: 1920", &state.width, Msg::WidthChanged));
        let h = ui::field(env, "Nova altura (px)", "", ui::input("Ex.: 1080", &state.height, Msg::HeightChanged));
        if env.narrow() {
            column![w, h].spacing(10).into()
        } else {
            row![w, h].spacing(12).into()
        }
    };

    let aspect_chk = checkbox("Manter proporção original automaticamente", state.keep_aspect)
        .on_toggle(Msg::KeepAspectToggled)
        .size(18)
        .text_size(13);

    let resize_card = ui::section(
        env,
        "📐",
        "Redimensionar Imagem",
        "Altere a largura e a altura em pixels com reamostragem Lanczos3.",
        column![
            presets,
            dims_row,
            aspect_chk,
            ui::actions(env, vec![act("📐  Redimensionar agora", Btn::Primary, has_input.then_some(Msg::ResizeRun))]),
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
            ui::actions(env, vec![act("✨  Comprimir imagem agora", Btn::Primary, has_input.then_some(Msg::CompressRun))]),
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
            "Use os botões 25%, 50%, 75% para reduzir fotos grandes de celular em um clique.",
            "Converta qualquer PNG ou JPG para ícone do Windows (.ico) automaticamente.",
        ],
        Some("Para ajustar luz, cor, filtros e recortar para Instagram/Reels, use a aba \"Editor de Fotos\"."),
    );

    ui::two_col(env, main_col.into(), side)
}
