//! Interface do módulo de Vídeo: Editor Rápido de Vídeo + Conversor/Compressor.

use iced::widget::{button, checkbox, column, container, image as iced_image, pick_list, row, slider, text};
use iced::{Alignment, ContentFit, Element, Length};

use crate::modules::video::service::{CanvasAspect, SpeedPreset, VideoFormat, VideoLook};
use crate::modules::video::{Mode, Msg, State};
use crate::shared::theme::{self, Btn};
use crate::shared::types::Env;
use crate::shared::ui::{self, act, SEMIBOLD};

pub fn view<'a>(state: &'a State, env: &Env) -> Element<'a, Msg> {
    let c = env.c();

    let status_badge = if state.ffmpeg_ok {
        ui::badge("✓ Motor FFmpeg ativo", c.success)
    } else {
        ui::badge("⬇ FFmpeg pendente", c.warning)
    };

    let header = ui::page_header(
        env,
        "🎬",
        "Estúdio & Editor de Vídeo",
        "Corte trechos na linha do tempo, mude a velocidade (0.5× a 2×), ajuste para TikTok/Reels/YouTube, aplique filtros ou converta formatos.",
        Some(status_badge),
    );

    let tabs = row![
        ui::btn(
            "✂  Editor de Vídeo (Linha do Tempo)",
            Btn::Tab(state.mode == Mode::VideoEditor),
            Some(Msg::ModeChanged(Mode::VideoEditor)),
        ),
        ui::btn(
            "🔄  Converter Formato & Comprimir (CRF)",
            Btn::Tab(state.mode == Mode::ConvertCompress),
            Some(Msg::ModeChanged(Mode::ConvertCompress)),
        ),
    ]
    .spacing(10)
    .align_items(Alignment::Center);

    let mut items = vec![header, tabs.into()];

    if !state.ffmpeg_ok {
        let mut ff_col = column![
            text("O motor multimídia FFmpeg ainda não foi detectado. Clique no botão abaixo para o Axolote Suite baixar e configurar tudo sozinho:").size(13),
            ui::actions(
                env,
                vec![
                    act(
                        "⬇  Baixar e Ativar FFmpeg Automaticamente",
                        Btn::Primary,
                        (!state.downloading_ffmpeg).then_some(Msg::DownloadFfmpeg),
                    ),
                    act("🔄  Verificar novamente", Btn::Secondary, Some(Msg::RecheckFfmpeg)),
                ],
            ),
        ]
        .spacing(10);
        if state.downloading_ffmpeg {
            ff_col = ff_col.push(ui::busy(env, "Baixando e instalando o FFmpeg automaticamente"));
        }
        items.push(ui::section(
            env,
            "⚙",
            "Dependência de Vídeo (FFmpeg Automático)",
            "Necessário para cortar, editar, converter e comprimir vídeos.",
            ff_col,
        ));
    }

    items.push(io_card(state, env));

    let body = match state.mode {
        Mode::VideoEditor => video_editor_view(state, env),
        Mode::ConvertCompress => convert_compress_view(state, env),
    };
    items.push(body);

    ui::page(env, items)
}

fn io_card<'a>(state: &'a State, env: &Env) -> Element<'a, Msg> {
    let in_field = ui::field(
        env,
        "Clipe de vídeo de entrada",
        "Formatos aceitos: MP4, MKV, AVI, MOV, WMV, WebM, FLV e M4V.",
        ui::path_field(
            env,
            "Escolha um vídeo do seu computador…",
            &state.input,
            Msg::InputChanged,
            "📂  Escolher vídeo…",
            Msg::BrowseInput,
        ),
    );

    let out_field = ui::field(
        env,
        "Arquivo de saída (opcional)",
        "Se deixar em branco, o vídeo editado será salvo ao lado do original com sufixo automático.",
        ui::path_field(
            env,
            "Automático (mesma pasta do vídeo original)",
            &state.output,
            Msg::OutputChanged,
            "💾  Salvar como…",
            Msg::BrowseOutput,
        ),
    );

    let row_or_col: Element<'a, Msg> = if env.wide() {
        row![in_field, out_field].spacing(14).into()
    } else {
        column![in_field, out_field].spacing(10).into()
    };

    let mut col = column![row_or_col].spacing(10);
    if !state.info_lines.is_empty() {
        let summary = state.info_lines.join("   •   ");
        col = col.push(ui::list_row(ui::muted(env, summary)));
    }
    ui::card(env, col)
}

// ---------------------------------------------------------------------------
// Aba 1: Editor Rápido de Vídeo
// ---------------------------------------------------------------------------

fn video_editor_view<'a>(state: &'a State, env: &Env) -> Element<'a, Msg> {
    let c = env.c();
    let can_run = state.ffmpeg_ok && !state.input.trim().is_empty() && !state.busy;

    // Monitor do Clipe + Resumo de efeitos ativos
    let monitor: Element<'a, Msg> = match &state.thumbnail {
        Some(handle) => container(
            iced_image::Image::new(handle.clone())
                .width(Length::Fill)
                .height(Length::Fixed(240.0))
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
            "🎞",
            "Monitor do Clipe",
            "Selecione um vídeo acima para carregar o quadro de pré-visualização e a linha do tempo.",
        ))
        .height(Length::Fixed(220.0))
        .width(Length::Fill)
        .center_x()
        .center_y()
        .style(theme::list_row)
        .into(),
    };

    let active_badges = row![
        ui::badge(state.edit.aspect.label(), c.primary),
        ui::badge(state.edit.speed.label(), c.accent),
        ui::badge(state.edit.look.label(), c.success),
        ui::badge(format!("Rotação: {}°", state.edit.rotation), c.muted),
    ]
    .spacing(8)
    .align_items(Alignment::Center);

    let transform_row = ui::actions(
        env,
        vec![
            act("↻  Girar 90°", Btn::Secondary, can_run.then_some(Msg::RotateVideo)),
            act(
                if state.edit.flip_h { "↔  Espelhado ✓" } else { "↔  Espelhar Horizontal" },
                Btn::Secondary,
                can_run.then_some(Msg::FlipVideoH),
            ),
            act("🖼  Atualizar Quadro", Btn::Secondary, can_run.then_some(Msg::RefreshPreviewFrame)),
            act("🔄  Zerar Edições", Btn::Ghost, Some(Msg::ResetVideoEdits)),
        ],
    );

    // Linha do Tempo (Corte Início / Fim)
    let trim_row: Element<'a, Msg> = {
        let start = ui::field(
            env,
            "⏱ Cortar Início (De)",
            "Deixe vazio para começar do início (00:00:00).",
            ui::input("Ex.: 00:00:05", &state.edit.start_time, Msg::StartTimeChanged),
        );
        let end = ui::field(
            env,
            "⏱ Cortar Fim (Até)",
            "Deixe vazio para ir até o final do clipe.",
            ui::input("Ex.: 00:00:45", &state.edit.end_time, Msg::EndTimeChanged),
        );
        if env.narrow() {
            column![start, end].spacing(10).into()
        } else {
            row![start, end].spacing(12).into()
        }
    };

    // Velocidade
    let speed_btns: Vec<Element<'a, Msg>> = SpeedPreset::ALL
        .iter()
        .map(|&sp| {
            let active = state.edit.speed == sp;
            button(container(text(sp.label()).size(12).font(SEMIBOLD)).center_x().width(Length::Fill))
                .padding([9, 10])
                .width(Length::Fill)
                .style(theme::btn(Btn::Tab(active)))
                .on_press(Msg::SpeedSelected(sp))
                .into()
        })
        .collect();
    let speed_grid = ui::grid(speed_btns, env.columns(145.0, 3), 8);

    // Proporção da Tela (Canvas)
    let aspect_btns: Vec<Element<'a, Msg>> = CanvasAspect::ALL
        .iter()
        .map(|&a| {
            let active = state.edit.aspect == a;
            button(container(text(a.label()).size(12).font(SEMIBOLD)).center_x().width(Length::Fill))
                .padding([9, 10])
                .width(Length::Fill)
                .style(theme::btn(Btn::Tab(active)))
                .on_press(Msg::AspectSelected(a))
                .into()
        })
        .collect();
    let aspect_grid = ui::grid(aspect_btns, env.columns(180.0, 2), 8);

    // Filtros de Cor (Look)
    let look_btns: Vec<Element<'a, Msg>> = VideoLook::ALL
        .iter()
        .map(|&l| {
            let active = state.edit.look == l;
            button(container(text(l.label()).size(12).font(SEMIBOLD)).center_x().width(Length::Fill))
                .padding([9, 10])
                .width(Length::Fill)
                .style(theme::btn(Btn::Tab(active)))
                .on_press(Msg::LookSelected(l))
                .into()
        })
        .collect();
    let look_grid = ui::grid(look_btns, env.columns(130.0, 3), 8);

    // Áudio e Volume
    let audio_box = ui::list_row(
        column![
            checkbox("🔇 Remover todo o áudio do vídeo (Deixar Mudo)", state.edit.mute_audio)
                .on_toggle(Msg::MuteToggled)
                .size(18)
                .text_size(13),
            row![
                ui::strong(format!("🔊 Volume da faixa de áudio: {}%", state.edit.volume_pct), 13).width(Length::Fill),
                ui::badge(if state.edit.volume_pct > 100 { "Amplificado" } else { "Normal" }, c.accent),
            ]
            .align_items(Alignment::Center),
            slider(0..=200, state.edit.volume_pct, Msg::VolumeChanged),
        ]
        .spacing(8),
    );

    let export_btn = button(
        container(text("🎬   Renderizar e Exportar Vídeo Editado (MP4 H.264)").size(15).font(ui::BOLD))
            .center_x()
            .width(Length::Fill),
    )
    .padding([14, 22])
    .width(Length::Fill)
    .style(theme::btn(Btn::Primary))
    .on_press_maybe(can_run.then_some(Msg::ExportEditedVideo));

    let mut left_col = column![
        ui::section(
            env,
            "🖥",
            "Monitor & Linha do Tempo",
            "Defina o ponto inicial/final do corte e gire ou espelhe o clipe.",
            column![monitor, active_badges, transform_row, trim_row].spacing(12),
        ),
        ui::section(
            env,
            "⚡",
            "Velocidade de Reprodução (Slow-Motion / Fast-Forward)",
            "Ajusta automaticamente a imagem e o tom do áudio.",
            speed_grid,
        ),
    ]
    .spacing(16);

    if state.busy {
        left_col = left_col.push(ui::busy(env, "Renderizando vídeo editado com FFmpeg"));
    }
    if let Some(res) = &state.result {
        let open = res.output.clone().map(Msg::OpenPath);
        left_col = left_col.push(ui::result_banner(env, res, open));
    }

    let right_col = column![
        ui::section(
            env,
            "📱",
            "Formato da Tela (YouTube / TikTok / Reels / Feed)",
            "Adapta o vídeo com barras suaves sem distorcer a imagem.",
            aspect_grid,
        ),
        ui::section(
            env,
            "🎨",
            "Estilo Visual & Áudio",
            "Aplique filtros de cor de cinema, silencie ou aumente o volume até 200%.",
            column![look_grid, audio_box, export_btn].spacing(14),
        ),
    ]
    .spacing(16);

    ui::two_col(env, left_col.into(), right_col.into())
}

// ---------------------------------------------------------------------------
// Aba 2: Converter Formato e Comprimir (CRF)
// ---------------------------------------------------------------------------

fn convert_compress_view<'a>(state: &'a State, env: &Env) -> Element<'a, Msg> {
    let c = env.c();
    let can_run = state.ffmpeg_ok && !state.input.trim().is_empty() && !state.busy;

    let fmt_picker = pick_list(VideoFormat::ALL, Some(state.format), Msg::FormatChanged)
        .padding([10, 12])
        .width(Length::Fill);

    let conv_card = ui::section(
        env,
        "🔄",
        "Converter Formato de Vídeo",
        "Reencoda em H.264 + AAC (ou VP9 para WebM) para máxima compatibilidade.",
        column![
            ui::field(env, "Formato de saída", "", fmt_picker),
            ui::actions(
                env,
                vec![act(
                    format!("🔄  Converter vídeo para {}", state.format.ext().to_uppercase()),
                    Btn::Primary,
                    can_run.then_some(Msg::ConvertRun),
                )],
            ),
        ]
        .spacing(12),
    );

    let crf_slider = slider(18..=36, state.crf, Msg::CrfChanged);
    let crf_hint = match state.crf {
        18..=21 => "Qualidade quase sem perdas",
        22..=26 => "Equilíbrio ideal (ótimo visual com grande redução de MB)",
        27..=31 => "Arquivo bem leve (WhatsApp / Discord)",
        _ => "Compressão máxima",
    };

    let comp_card = ui::section(
        env,
        "✨",
        "Comprimir Vídeo (Fator de Qualidade CRF)",
        "Valores menores de CRF significam maior qualidade; valores maiores deixam o arquivo bem menor.",
        column![
            row![
                ui::strong(format!("CRF atual: {}", state.crf), 14).width(Length::Fill),
                ui::badge(crf_hint, c.accent),
            ]
            .align_items(Alignment::Center),
            crf_slider,
            ui::actions(
                env,
                vec![act(
                    format!("✨  Comprimir vídeo em MP4 (CRF {})", state.crf),
                    Btn::Primary,
                    can_run.then_some(Msg::CompressRun),
                )],
            ),
        ]
        .spacing(12),
    );

    let mut main_col = column![conv_card, comp_card].spacing(16);
    if state.busy {
        main_col = main_col.push(ui::busy(env, "Codificando vídeo com FFmpeg"));
    }
    if let Some(res) = &state.result {
        let open = res.output.clone().map(Msg::OpenPath);
        main_col = main_col.push(ui::result_banner(env, res, open));
    }

    let side = ui::help(
        env,
        &[
            "Escolha o vídeo no cartão superior.",
            "Para mudar o formato (ex.: MKV para MP4), escolha o formato e clique em \"Converter\".",
            "Para reduzir o tamanho em MB sem cortar o vídeo, ajuste o CRF (24 é o ponto ideal) e clique em \"Comprimir\".",
        ],
        Some("Para cortar trechos, acelerar ou formatar para Reels/TikTok, use a aba \"Editor de Vídeo (Linha do Tempo)\"."),
    );

    ui::two_col(env, main_col.into(), side)
}
