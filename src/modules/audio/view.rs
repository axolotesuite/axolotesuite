//! Interface do módulo de Áudio.

use iced::widget::{column, pick_list, text};
use iced::{Element, Length};

use crate::modules::audio::service::{AudioFormat, Bitrate};
use crate::modules::audio::{Msg, State};
use crate::shared::theme::Btn;
use crate::shared::types::Env;
use crate::shared::ui::{self, act};

pub fn view<'a>(state: &'a State, env: &Env) -> Element<'a, Msg> {
    let c = env.c();
    let can_run = state.ffmpeg_ok && !state.input.trim().is_empty() && !state.busy;

    let status_badge = if state.ffmpeg_ok {
        ui::badge("✓ FFmpeg ativo", c.success)
    } else {
        ui::badge("⚠ FFmpeg não detectado", c.warning)
    };

    let header = ui::page_header(
        env,
        "🎵",
        "Estúdio de Áudio",
        "Converta músicas e gravações (MP3, WAV, FLAC, OGG, M4A), extraia o áudio de vídeos e reduza o tamanho dos arquivos.",
        Some(status_badge),
    );

    let mut page_items = vec![header];

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
            ff_col = ff_col.push(ui::busy(
                env,
                "Baixando e instalando o FFmpeg automaticamente",
            ));
        }
        page_items.push(ui::section(
            env,
            "⚙",
            "Dependência de Áudio (FFmpeg Automático)",
            "Necessário para converter e comprimir áudios.",
            ff_col,
        ));
    }

    let in_field = ui::field(
        env,
        "Arquivo de áudio (ou vídeo para extrair a música)",
        "Aceita MP3, WAV, FLAC, OGG, M4A, AAC, MP4, MKV, AVI, MOV e WebM.",
        ui::path_field(
            env,
            "Escolha um arquivo de áudio ou vídeo…",
            &state.input,
            Msg::InputChanged,
            "📂  Escolher arquivo…",
            Msg::BrowseInput,
        ),
    );

    let out_field = ui::field(
        env,
        "Arquivo de saída (opcional)",
        "Se deixar em branco, o novo áudio será salvo automaticamente na mesma pasta do arquivo original.",
        ui::path_field(
            env,
            "Automático (mesma pasta do arquivo original)",
            &state.output,
            Msg::OutputChanged,
            "💾  Salvar como…",
            Msg::BrowseOutput,
        ),
    );

    let mut io_col = column![in_field, out_field].spacing(12);
    if !state.info_lines.is_empty() {
        let mut info = column![ui::strong("Informações detectadas no arquivo:", 14)].spacing(4);
        for line in &state.info_lines {
            info = info.push(ui::muted(env, format!("• {line}")));
        }
        io_col = io_col.push(ui::list_row(info));
    }

    let io_card = ui::section(
        env,
        "📂",
        "1. Selecionar Arquivo",
        "Escolha o áudio ou vídeo de origem.",
        io_col,
    );

    // Converter / Extrair
    let fmt_picker = pick_list(AudioFormat::ALL, Some(state.format), Msg::FormatChanged)
        .padding([10, 12])
        .width(Length::Fill);

    let conv_card = ui::section(
        env,
        "🔄",
        "Converter Formato ou Extrair Áudio de Vídeo",
        "Transforme para outro formato ou tire apenas a trilha sonora de um vídeo MP4/MKV.",
        column![
            ui::field(env, "Formato de saída", "", fmt_picker),
            ui::actions(
                env,
                vec![
                    act(
                        format!("🔄  Converter para {}", state.format.ext().to_uppercase()),
                        Btn::Primary,
                        can_run.then_some(Msg::ConvertRun),
                    ),
                    act(
                        format!("🎬  Extrair áudio de vídeo (.{})", state.format.ext()),
                        Btn::Secondary,
                        can_run.then_some(Msg::ExtractRun),
                    ),
                ],
            ),
        ]
        .spacing(12),
    );

    // Comprimir
    let br_picker = pick_list(Bitrate::ALL, Some(state.bitrate), Msg::BitrateChanged)
        .padding([10, 12])
        .width(Length::Fill);

    let comp_card = ui::section(
        env,
        "✨",
        "Comprimir Áudio (Diminuir Tamanho em MB)",
        "Reduz o tamanho do arquivo ajustando a taxa de bits (bitrate) em MP3.",
        column![
            ui::field(
                env,
                "Taxa de bits (qualidade × tamanho)",
                "Quanto menor o valor em kbps, menor fica o arquivo final.",
                br_picker
            ),
            ui::actions(
                env,
                vec![act(
                    format!("✨  Comprimir áudio ({})", state.bitrate.arg()),
                    Btn::Primary,
                    can_run.then_some(Msg::CompressRun),
                )],
            ),
        ]
        .spacing(12),
    );

    let mut main_col = column![io_card, conv_card, comp_card].spacing(16);

    if state.busy {
        main_col = main_col.push(ui::busy(env, "Processando áudio com FFmpeg"));
    }
    if let Some(res) = &state.result {
        let open = res.output.clone().map(Msg::OpenPath);
        main_col = main_col.push(ui::result_banner(env, res, open));
    }

    let side = ui::help(
        env,
        &[
            "Clique em \"Escolher arquivo…\" e selecione um áudio (ou um vídeo MP4 se quiser extrair a música dele).",
            "Para mudar o formato (ex.: WAV para MP3), escolha o formato e clique em \"Converter\".",
            "Para deixar uma gravação mais leve para enviar no WhatsApp ou e-mail, use a seção \"Comprimir Áudio\".",
        ],
        Some("O processamento roda em segundo plano — a janela nunca congela mesmo em arquivos grandes."),
    );

    page_items.push(ui::two_col(env, main_col.into(), side));
    ui::page(env, page_items)
}
