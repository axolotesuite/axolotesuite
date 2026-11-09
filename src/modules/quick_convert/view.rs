//! Interface visual limpa em botões do módulo "Conversor Fácil".

use iced::widget::{checkbox, column, container, row, Space};
use iced::{Alignment, Element, Length};

use crate::modules::quick_convert::{Msg, State, WheelCategory};
use crate::shared::theme::{self, BannerKind, Btn};
use crate::shared::types::Env;
use crate::shared::ui::{self, act};

pub fn view<'a>(state: &'a State, env: &Env) -> Element<'a, Msg> {
    let c = env.c();
    let is_en = env.lang.is_en();
    let header = ui::page_header(
        env,
        "🔄",
        if is_en { "Universal Quick Convert" } else { "Conversor Fácil — Tudo em 1 Lugar" },
        if is_en {
            "Select files (or drag & drop from File Explorer), click the target format button, and convert instantly without overwriting original files!"
        } else {
            "Escolha seus arquivos (ou arraste do Windows Explorer), clique no botão do formato desejado e converta na hora preservando o original!"
        },
        Some(
            row![
                ui::badge(
                    format!("{}: {}", if is_en { "Source" } else { "Origem" }, if state.source_ext.is_empty() { "AUTO" } else { &state.source_ext }),
                    c.primary
                ),
                ui::badge(
                    format!("{} {}", state.files.len(), if is_en { "file(s)" } else { "arquivo(s)" }),
                    if state.files.is_empty() { c.muted } else { c.success },
                ),
            ]
            .spacing(8)
            .into(),
        ),
    );

    let cat_tabs = row![
        ui::btn(
            WheelCategory::Image.label(),
            Btn::Tab(state.category == WheelCategory::Image),
            Some(Msg::SetCategory(WheelCategory::Image)),
        ),
        ui::btn(
            WheelCategory::Document.label(),
            Btn::Tab(state.category == WheelCategory::Document),
            Some(Msg::SetCategory(WheelCategory::Document)),
        ),
        ui::btn(
            WheelCategory::Video.label(),
            Btn::Tab(state.category == WheelCategory::Video),
            Some(Msg::SetCategory(WheelCategory::Video)),
        ),
        ui::btn(
            WheelCategory::Audio.label(),
            Btn::Tab(state.category == WheelCategory::Audio),
            Some(Msg::SetCategory(WheelCategory::Audio)),
        ),
        ui::btn(
            WheelCategory::Archive.label(),
            Btn::Tab(state.category == WheelCategory::Archive),
            Some(Msg::SetCategory(WheelCategory::Archive)),
        ),
    ]
    .spacing(8);

    let formats = state.available_targets();
    let target_fmt = state.current_target_format();

    let mut fmt_cards: Vec<Element<'a, Msg>> = Vec::with_capacity(formats.len());
    for (idx, fmt) in formats.iter().enumerate() {
        let active = (state.selected_idx % formats.len().max(1)) == idx;
        let label = if active {
            format!("✔  Converter para {fmt}")
        } else {
            format!("➜  {fmt}")
        };
        fmt_cards.push(
            ui::btn(
                label,
                if active { Btn::Primary } else { Btn::Secondary },
                Some(Msg::SelectSlice(idx)),
            )
            .width(Length::Fill)
            .into(),
        );
    }

    let formats_grid = ui::grid(fmt_cards, env.columns(150.0, 4), 10);

    let chk_instant = checkbox(
        "⚡ Converter imediatamente assim que apertar no botão do formato acima",
        state.instant_on_slice_click,
    )
    .on_toggle(Msg::ToggleInstantClick)
    .size(18)
    .text_size(13);

    let mut left_col = column![
        ui::list_row(
            row![
                ui::badge(if is_en { "SOURCE" } else { "ORIGEM" }, c.primary),
                ui::strong(
                    if state.source_ext.is_empty() { "AUTO" } else { &state.source_ext },
                    16
                ),
                ui::muted(env, "   ───►   "),
                ui::badge(if is_en { "TARGET" } else { "DESTINO SELECIONADO" }, c.accent),
                ui::strong(target_fmt, 16),
            ]
            .spacing(8)
            .align_items(Alignment::Center),
        ),
        ui::strong(if is_en { "Click your desired target format:" } else { "Aperte no formato de destino desejado:" }, 14),
        formats_grid,
        ui::list_row(chk_instant),
    ]
    .spacing(14);

    if state.category == WheelCategory::Image {
        let chk_strip = checkbox(
            if is_en {
                "🛡 Privacy: Strip hidden metadata (EXIF / GPS / Camera) when converting photos"
            } else {
                "🛡 Privacidade: Limpar metadados ocultos (EXIF / GPS / Câmera) ao converter imagens"
            },
            state.strip_metadata,
        )
        .on_toggle(Msg::ToggleStripMetadata)
        .size(18)
        .text_size(13);
        left_col = left_col.push(ui::list_row(chk_strip));
    }

    let convert_btn = ui::actions(
        env,
        vec![
            act(
                if is_en { format!("🚀  Convert to {target_fmt} Now") } else { format!("🚀  Converter para {target_fmt} Agora") },
                Btn::Primary,
                (!state.busy && !state.files.is_empty()).then_some(Msg::ConvertNow),
            ),
            act(if is_en { "➕  Choose File(s)…" } else { "➕  Escolher Arquivo(s)…" }, Btn::Secondary, (!state.busy).then_some(Msg::AddFiles)),
            act(
                if is_en { "🗑  Clear List" } else { "🗑  Limpar Lista" },
                Btn::Ghost,
                (!state.busy && !state.files.is_empty()).then_some(Msg::ClearFiles),
            ),
        ],
    );
    left_col = left_col.push(convert_btn);

    if state.busy {
        left_col = left_col.push(ui::busy(env, "Convertendo arquivos com preservação do original"));
    }
    if let Some(res) = &state.result {
        left_col = left_col.push(ui::result_banner(env, res, Some(Msg::OpenResult)));
    }

    let left_card = ui::section(
        env,
        "🔄",
        "Escolha o Formato de Saída",
        "Apenas conversões reais suportadas para o seu arquivo são exibidas abaixo (ZIP está sempre disponível).",
        left_col,
    );

    let mut files_list = column![].spacing(6);
    if state.files.is_empty() {
        files_list = files_list.push(ui::empty_state(
            env,
            "📂",
            "Nenhum arquivo selecionado ainda",
            "Clique em 'Escolher Arquivo(s)' ou simplesmente arraste qualquer arquivo do Windows Explorer para esta janela!",
        ));
    } else {
        for (idx, f) in state.files.iter().enumerate() {
            let ext = f
                .extension()
                .and_then(|e| e.to_str())
                .unwrap_or("ARQ")
                .to_ascii_uppercase();
            files_list = files_list.push(ui::list_row(
                row![
                    ui::badge(format!("#{}", idx + 1), c.primary),
                    ui::badge(ext, c.accent),
                    ui::strong(ui::file_name(f), 13).width(Length::Fill),
                    ui::btn("✖", Btn::Danger, Some(Msg::RemoveFile(idx))),
                ]
                .spacing(8)
                .align_items(Alignment::Center),
            ));
        }
    }

    let out_dir_field = ui::field(
        env,
        "Local de saída (Preservação Automática)",
        "Por padrão (em branco), grava ao lado do arquivo original e usa sufixos (1), (2) para nunca sobrescrever.",
        ui::path_field(
            env,
            "Ao lado do arquivo original — sem sobrescrita silenciosa (1), (2)...",
            &state.custom_out_dir,
            Msg::OutDirChanged,
            "📂  Outra Pasta…",
            Msg::BrowseOutDir,
        ),
    );

    let right_col = column![
        ui::section(
            env,
            "📥",
            "Arquivo(s) Selecionado(s) (Somente Leitura)",
            "O arquivo original nunca é movido, alterado ou apagado.",
            column![files_list, Space::with_height(4), out_dir_field].spacing(12),
        ),
        ui::help(
            env,
            &[
                "Imagens (PNG, JPG, WEBP, GIF, BMP, TIFF, ICO) → demais formatos de imagem + PDF + ZIP.",
                "Texto & Documentos: TXT ↔ MD ↔ HTML ↔ DOCX ↔ PDF + ZIP.",
                "Vídeos (MP4, MKV, WEBM, AVI, MOV...) → MP4, WEBM, MKV, GIF animado, MP3 + ZIP.",
                "Áudios (MP3, WAV, FLAC, OGG, AAC, M4A) → demais formatos de áudio + ZIP.",
                "ZIP → EXTRAIR pasta com proteção anti Zip-Slip (e qualquer outro arquivo oferece ZIP).",
            ],
            Some("Os arquivos convertidos são salvos ao lado do original com sufixos (1), (2) para nunca sobrescrever nada!"),
        ),
    ]
    .spacing(16);

    let drag_banner = container(
        row![
            ui::badge("ARRASTE E SOLTE", c.accent),
            ui::muted(
                env,
                "Dica rápida: Você pode arrastar qualquer arquivo do Windows Explorer para a janela (ou usar o atalho Ctrl + Alt + N)!"
            ),
        ]
        .spacing(10)
        .align_items(Alignment::Center),
    )
    .padding([10, 16])
    .width(Length::Fill)
    .style(theme::banner(BannerKind::Info));

    ui::page(
        env,
        vec![
            header,
            cat_tabs.into(),
            drag_banner.into(),
            ui::two_col(env, left_card, right_col.into()),
        ],
    )
}
