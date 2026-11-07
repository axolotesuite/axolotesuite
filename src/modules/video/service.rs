//! Operações de vídeo e Editor de Vídeo via FFmpeg:
//! - Corte na linha do tempo (Início / Fim)
//! - Velocidade de reprodução (0.5x, 1.0x, 1.25x, 1.5x, 2.0x) com ajuste de áudio (`atempo`)
//! - Formato / Proporção (Original, 16:9 YouTube, 9:16 TikTok/Reels/Shorts, 1:1 Quadrado)
//! - Rotação (0°, 90° Dir, 90° Esq, 180°) e Espelhamento Horizontal
//! - Filtros de cor e ajuste de Volume / Muda (Mute)
//! - Extração de miniatura (frame) para o monitor de vídeo
//! - Conversão de formato (MP4/MKV/AVI/MOV/WebM) e Compressão CRF

use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};

use iced::widget::image::Handle;

use crate::core::system;
use crate::shared::types::OperationResult;
use crate::shared::ui::human_size;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VideoFormat {
    Mp4,
    Mkv,
    Avi,
    Mov,
    Webm,
}

impl VideoFormat {
    pub const ALL: &'static [VideoFormat] = &[
        VideoFormat::Mp4,
        VideoFormat::Mkv,
        VideoFormat::Avi,
        VideoFormat::Mov,
        VideoFormat::Webm,
    ];

    pub fn ext(&self) -> &'static str {
        match self {
            VideoFormat::Mp4 => "mp4",
            VideoFormat::Mkv => "mkv",
            VideoFormat::Avi => "avi",
            VideoFormat::Mov => "mov",
            VideoFormat::Webm => "webm",
        }
    }
}

impl fmt::Display for VideoFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            VideoFormat::Mp4 => "MP4 (H.264 + AAC — abre em qualquer TV, celular e navegador)",
            VideoFormat::Mkv => "MKV (Matroska — ótimo para filmes e múltiplas faixas)",
            VideoFormat::Avi => "AVI (formato clássico Windows)",
            VideoFormat::Mov => "MOV (QuickTime / editores de vídeo)",
            VideoFormat::Webm => "WebM (leve para sites e navegadores)",
        };
        write!(f, "{s}")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpeedPreset {
    Slow05,
    Normal10,
    Fast125,
    Fast15,
    Fast20,
}

impl SpeedPreset {
    pub const ALL: &'static [SpeedPreset] = &[
        SpeedPreset::Slow05,
        SpeedPreset::Normal10,
        SpeedPreset::Fast125,
        SpeedPreset::Fast15,
        SpeedPreset::Fast20,
    ];

    pub fn label(&self) -> &'static str {
        match self {
            SpeedPreset::Slow05 => "0.5× (Câmera Lenta)",
            SpeedPreset::Normal10 => "1.0× (Normal)",
            SpeedPreset::Fast125 => "1.25× (Dinâmico)",
            SpeedPreset::Fast15 => "1.5× (Rápido)",
            SpeedPreset::Fast20 => "2.0× (Time-lapse)",
        }
    }

    pub fn factor(&self) -> f32 {
        match self {
            SpeedPreset::Slow05 => 0.5,
            SpeedPreset::Normal10 => 1.0,
            SpeedPreset::Fast125 => 1.25,
            SpeedPreset::Fast15 => 1.5,
            SpeedPreset::Fast20 => 2.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CanvasAspect {
    Original,
    Youtube16x9,
    Reels9x16,
    Square1x1,
}

impl CanvasAspect {
    pub const ALL: &'static [CanvasAspect] = &[
        CanvasAspect::Original,
        CanvasAspect::Youtube16x9,
        CanvasAspect::Reels9x16,
        CanvasAspect::Square1x1,
    ];

    pub fn label(&self) -> &'static str {
        match self {
            CanvasAspect::Original => "Original",
            CanvasAspect::Youtube16x9 => "16:9 YouTube (1920×1080)",
            CanvasAspect::Reels9x16 => "9:16 TikTok/Reels (1080×1920)",
            CanvasAspect::Square1x1 => "1:1 Feed (1080×1080)",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VideoLook {
    Original,
    Vivid,
    BlackAndWhite,
    Vintage,
    CinemaContrast,
}

impl VideoLook {
    pub const ALL: &'static [VideoLook] = &[
        VideoLook::Original,
        VideoLook::Vivid,
        VideoLook::BlackAndWhite,
        VideoLook::Vintage,
        VideoLook::CinemaContrast,
    ];

    pub fn label(&self) -> &'static str {
        match self {
            VideoLook::Original => "✨ Original",
            VideoLook::Vivid => "🌈 Vibrante",
            VideoLook::BlackAndWhite => "🖤 P&B",
            VideoLook::Vintage => "🎞 Retrô",
            VideoLook::CinemaContrast => "🎬 Cinema",
        }
    }
}

#[derive(Debug, Clone)]
pub struct VideoEditConfig {
    pub start_time: String, // ex.: "00:00:00" ou ""
    pub end_time: String,   // ex.: "00:00:30" ou ""
    pub speed: SpeedPreset,
    pub aspect: CanvasAspect,
    pub look: VideoLook,
    pub rotation: u16, // 0, 90, 180, 270
    pub flip_h: bool,
    pub mute_audio: bool,
    pub volume_pct: u16, // 0..=200
}

impl Default for VideoEditConfig {
    fn default() -> Self {
        Self {
            start_time: String::new(),
            end_time: String::new(),
            speed: SpeedPreset::Normal10,
            aspect: CanvasAspect::Original,
            look: VideoLook::Original,
            rotation: 0,
            flip_h: false,
            mute_audio: false,
            volume_pct: 100,
        }
    }
}

pub fn inspect(path: &Path) -> Vec<String> {
    let mut lines = Vec::new();
    if let Ok(m) = fs::metadata(path) {
        lines.push(format!("Tamanho no disco: {}", human_size(m.len())));
    }
    if let Some(ff) = system::find_ffmpeg() {
        lines.extend(system::probe_media(&ff, path));
    }
    lines
}

/// Extrai um quadro (frame) do vídeo usando o FFmpeg para mostrar no Monitor do Editor.
pub fn extract_thumbnail(path: &Path, at_time: &str) -> Option<Handle> {
    let ff = system::find_ffmpeg()?;
    if !path.is_file() {
        return None;
    }
    let tmp = std::env::temp_dir().join("axolote_video_preview.jpg");
    let seek = if at_time.trim().is_empty() { "00:00:01" } else { at_time.trim() };
    let args = vec![
        "-ss".to_string(),
        seek.to_string(),
        "-i".to_string(),
        path.display().to_string(),
        "-frames:v".to_string(),
        "1".to_string(),
        "-vf".to_string(),
        "scale=480:-1".to_string(),
        tmp.display().to_string(),
    ];
    if system::run_ffmpeg(&ff, &args).is_err() {
        // Tenta no segundo 0 se o vídeo for muito curto
        let fallback = vec![
            "-i".to_string(),
            path.display().to_string(),
            "-frames:v".to_string(),
            "1".to_string(),
            "-vf".to_string(),
            "scale=480:-1".to_string(),
            tmp.display().to_string(),
        ];
        system::run_ffmpeg(&ff, &fallback).ok()?;
    }
    let bytes = fs::read(&tmp).ok()?;
    let _ = fs::remove_file(&tmp);
    Some(Handle::from_memory(bytes))
}

/// Renderiza o vídeo aplicando os cortes, velocidade, proporção, filtros e áudio do Editor de Vídeo.
pub fn render_edited_video(input: PathBuf, output: PathBuf, cfg: VideoEditConfig) -> OperationResult {
    let Some(ff) = system::find_ffmpeg() else {
        return OperationResult::err("FFmpeg não encontrado. Instale o FFmpeg para editar e exportar vídeos.");
    };
    if !input.is_file() {
        return OperationResult::err("Arquivo de vídeo não encontrado.");
    }

    let mut args: Vec<String> = Vec::new();
    if !cfg.start_time.trim().is_empty() {
        args.push("-ss".into());
        args.push(cfg.start_time.trim().to_string());
    }
    if !cfg.end_time.trim().is_empty() {
        args.push("-to".into());
        args.push(cfg.end_time.trim().to_string());
    }
    args.push("-i".into());
    args.push(input.display().to_string());

    // Monta a cadeia de filtros de vídeo (-vf)
    let mut vfilters: Vec<String> = Vec::new();

    match cfg.rotation % 360 {
        90 => vfilters.push("transpose=1".into()),
        180 => vfilters.push("transpose=1,transpose=1".into()),
        270 => vfilters.push("transpose=2".into()),
        _ => {}
    }
    if cfg.flip_h {
        vfilters.push("hflip".into());
    }

    match cfg.aspect {
        CanvasAspect::Original => {}
        CanvasAspect::Youtube16x9 => {
            vfilters.push("scale=1920:1080:force_original_aspect_ratio=decrease,pad=1920:1080:(ow-iw)/2:(oh-ih)/2:black".into());
        }
        CanvasAspect::Reels9x16 => {
            vfilters.push("scale=1080:1920:force_original_aspect_ratio=decrease,pad=1080:1920:(ow-iw)/2:(oh-ih)/2:black".into());
        }
        CanvasAspect::Square1x1 => {
            vfilters.push("scale=1080:1080:force_original_aspect_ratio=decrease,pad=1080:1080:(ow-iw)/2:(oh-ih)/2:black".into());
        }
    }

    match cfg.look {
        VideoLook::Original => {}
        VideoLook::Vivid => vfilters.push("eq=saturation=1.35:contrast=1.08".into()),
        VideoLook::BlackAndWhite => vfilters.push("hue=s=0".into()),
        VideoLook::Vintage => vfilters.push("colorchannelmixer=.393:.769:.189:0:.349:.686:.168:0:.272:.534:.131".into()),
        VideoLook::CinemaContrast => vfilters.push("eq=contrast=1.22:brightness=-0.03:saturation=0.9".into()),
    }

    let spd = cfg.speed.factor();
    if (spd - 1.0).abs() > 0.01 {
        vfilters.push(format!("setpts={:.4}*PTS", 1.0 / spd));
    }

    if !vfilters.is_empty() {
        args.push("-vf".into());
        args.push(vfilters.join(","));
    }

    // Áudio
    if cfg.mute_audio {
        args.push("-an".into());
    } else {
        let mut afilters: Vec<String> = Vec::new();
        if (spd - 1.0).abs() > 0.01 {
            afilters.push(format!("atempo={spd:.2}"));
        }
        if cfg.volume_pct != 100 {
            afilters.push(format!("volume={:.2}", cfg.volume_pct as f32 / 100.0));
        }
        if !afilters.is_empty() {
            args.push("-af".into());
            args.push(afilters.join(","));
        }
        args.push("-c:a".into());
        args.push("aac".into());
    }

    args.push("-c:v".into());
    args.push("libx264".into());
    args.push("-preset".into());
    args.push("fast".into());
    args.push("-pix_fmt".into());
    args.push("yuv420p".into());
    args.push(output.display().to_string());

    match system::run_ffmpeg(&ff, &args) {
        Ok(()) => {
            let sz = fs::metadata(&output).map(|m| m.len()).unwrap_or(0);
            OperationResult::ok(format!("Vídeo editado e exportado com sucesso ({})", human_size(sz)))
                .with_detail(format!("Salvo em: {}", output.display()))
                .with_output(output)
        }
        Err(e) => OperationResult::err("Não foi possível exportar o vídeo editado.").with_detail(e),
    }
}

pub fn convert(input: PathBuf, output: PathBuf, fmt: VideoFormat) -> OperationResult {
    let Some(ff) = system::find_ffmpeg() else {
        return OperationResult::err("FFmpeg não encontrado. Instale o FFmpeg para converter vídeos.");
    };
    if !input.is_file() {
        return OperationResult::err("Arquivo de vídeo não encontrado.");
    }
    let args: Vec<String> = match fmt {
        VideoFormat::Webm => vec![
            "-i".into(),
            input.display().to_string(),
            "-c:v".into(),
            "libvpx-vp9".into(),
            "-crf".into(),
            "30".into(),
            "-b:v".into(),
            "0".into(),
            "-c:a".into(),
            "libopus".into(),
            output.display().to_string(),
        ],
        _ => vec![
            "-i".into(),
            input.display().to_string(),
            "-c:v".into(),
            "libx264".into(),
            "-preset".into(),
            "fast".into(),
            "-c:a".into(),
            "aac".into(),
            output.display().to_string(),
        ],
    };

    match system::run_ffmpeg(&ff, &args) {
        Ok(()) => {
            let sz = fs::metadata(&output).map(|m| m.len()).unwrap_or(0);
            OperationResult::ok(format!("Vídeo convertido para {} ({})", fmt.ext().to_uppercase(), human_size(sz)))
                .with_detail(format!("Salvo em: {}", output.display()))
                .with_output(output)
        }
        Err(e) => OperationResult::err("Não foi possível converter o vídeo.").with_detail(e),
    }
}

pub fn compress(input: PathBuf, output: PathBuf, crf: u8) -> OperationResult {
    let Some(ff) = system::find_ffmpeg() else {
        return OperationResult::err("FFmpeg não encontrado. Instale o FFmpeg para comprimir vídeos.");
    };
    if !input.is_file() {
        return OperationResult::err("Arquivo de vídeo não encontrado.");
    }
    let orig = fs::metadata(&input).map(|m| m.len()).unwrap_or(0);
    let args = vec![
        "-i".to_string(),
        input.display().to_string(),
        "-c:v".to_string(),
        "libx264".to_string(),
        "-preset".to_string(),
        "fast".to_string(),
        "-crf".to_string(),
        crf.clamp(18, 36).to_string(),
        "-c:a".to_string(),
        "aac".to_string(),
        "-b:a".to_string(),
        "128k".to_string(),
        output.display().to_string(),
    ];
    match system::run_ffmpeg(&ff, &args) {
        Ok(()) => {
            let new_sz = fs::metadata(&output).map(|m| m.len()).unwrap_or(0);
            OperationResult::ok(format!(
                "Vídeo comprimido (CRF {crf}) — {} → {}",
                human_size(orig),
                human_size(new_sz)
            ))
            .with_detail(format!("Salvo em: {}", output.display()))
            .with_output(output)
        }
        Err(e) => OperationResult::err("Não foi possível comprimir o vídeo.").with_detail(e),
    }
}
