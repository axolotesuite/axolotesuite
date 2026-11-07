//! Operações de áudio via FFmpeg: converter formato, extrair áudio de vídeo,
//! comprimir por taxa de bits (bitrate) e consultar informações do arquivo.

use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};

use crate::core::system;
use crate::shared::types::OperationResult;
use crate::shared::ui::human_size;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AudioFormat {
    Mp3,
    Wav,
    Flac,
    Ogg,
    M4a,
}

impl AudioFormat {
    pub const ALL: &'static [AudioFormat] = &[
        AudioFormat::Mp3,
        AudioFormat::Wav,
        AudioFormat::Flac,
        AudioFormat::Ogg,
        AudioFormat::M4a,
    ];

    pub fn ext(&self) -> &'static str {
        match self {
            AudioFormat::Mp3 => "mp3",
            AudioFormat::Wav => "wav",
            AudioFormat::Flac => "flac",
            AudioFormat::Ogg => "ogg",
            AudioFormat::M4a => "m4a",
        }
    }
}

impl fmt::Display for AudioFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            AudioFormat::Mp3 => "MP3 (universal, compatível com tudo)",
            AudioFormat::Wav => "WAV (sem compressão, qualidade de estúdio)",
            AudioFormat::Flac => "FLAC (sem perdas, compactado)",
            AudioFormat::Ogg => "OGG Vorbis (leve e aberto)",
            AudioFormat::M4a => "M4A / AAC (ótimo para celulares)",
        };
        write!(f, "{s}")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Bitrate {
    K64,
    K96,
    K128,
    K192,
    K256,
    K320,
}

impl Bitrate {
    pub const ALL: &'static [Bitrate] = &[
        Bitrate::K64,
        Bitrate::K96,
        Bitrate::K128,
        Bitrate::K192,
        Bitrate::K256,
        Bitrate::K320,
    ];

    pub fn arg(&self) -> &'static str {
        match self {
            Bitrate::K64 => "64k",
            Bitrate::K96 => "96k",
            Bitrate::K128 => "128k",
            Bitrate::K192 => "192k",
            Bitrate::K256 => "256k",
            Bitrate::K320 => "320k",
        }
    }
}

impl fmt::Display for Bitrate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Bitrate::K64 => "64 kbps (voz / gravações longas, muito leve)",
            Bitrate::K96 => "96 kbps (podcasts, arquivo pequeno)",
            Bitrate::K128 => "128 kbps (padrão equilibrado)",
            Bitrate::K192 => "192 kbps (alta qualidade para música)",
            Bitrate::K256 => "256 kbps (qualidade excelente)",
            Bitrate::K320 => "320 kbps (qualidade máxima MP3)",
        };
        write!(f, "{s}")
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

pub fn convert(input: PathBuf, output: PathBuf, fmt: AudioFormat) -> OperationResult {
    let Some(ff) = system::find_ffmpeg() else {
        return OperationResult::err("FFmpeg não encontrado. Instale o FFmpeg para converter áudios.");
    };
    if !input.is_file() {
        return OperationResult::err("Arquivo de entrada não encontrado.");
    }
    let args = vec![
        "-i".to_string(),
        input.display().to_string(),
        "-vn".to_string(),
        output.display().to_string(),
    ];
    match system::run_ffmpeg(&ff, &args) {
        Ok(()) => {
            let sz = fs::metadata(&output).map(|m| m.len()).unwrap_or(0);
            OperationResult::ok(format!("Áudio convertido para {} ({})", fmt.ext().to_uppercase(), human_size(sz)))
                .with_detail(format!("Salvo em: {}", output.display()))
                .with_output(output)
        }
        Err(e) => OperationResult::err("Não foi possível converter o áudio.").with_detail(e),
    }
}

pub fn extract_from_video(video: PathBuf, output: PathBuf, fmt: AudioFormat) -> OperationResult {
    let Some(ff) = system::find_ffmpeg() else {
        return OperationResult::err("FFmpeg não encontrado. Instale o FFmpeg para extrair áudio de vídeos.");
    };
    if !video.is_file() {
        return OperationResult::err("Arquivo de vídeo não encontrado.");
    }
    let args = vec![
        "-i".to_string(),
        video.display().to_string(),
        "-vn".to_string(),
        output.display().to_string(),
    ];
    match system::run_ffmpeg(&ff, &args) {
        Ok(()) => {
            let sz = fs::metadata(&output).map(|m| m.len()).unwrap_or(0);
            OperationResult::ok(format!(
                "Faixa de áudio extraída como {} ({})",
                fmt.ext().to_uppercase(),
                human_size(sz)
            ))
            .with_detail(format!("Salvo em: {}", output.display()))
            .with_output(output)
        }
        Err(e) => OperationResult::err("Não foi possível extrair o áudio deste vídeo.").with_detail(e),
    }
}

pub fn compress(input: PathBuf, output: PathBuf, bitrate: Bitrate) -> OperationResult {
    let Some(ff) = system::find_ffmpeg() else {
        return OperationResult::err("FFmpeg não encontrado. Instale o FFmpeg para comprimir áudios.");
    };
    if !input.is_file() {
        return OperationResult::err("Arquivo de áudio não encontrado.");
    }
    let orig = fs::metadata(&input).map(|m| m.len()).unwrap_or(0);
    let args = vec![
        "-i".to_string(),
        input.display().to_string(),
        "-vn".to_string(),
        "-b:a".to_string(),
        bitrate.arg().to_string(),
        output.display().to_string(),
    ];
    match system::run_ffmpeg(&ff, &args) {
        Ok(()) => {
            let new_sz = fs::metadata(&output).map(|m| m.len()).unwrap_or(0);
            OperationResult::ok(format!(
                "Áudio comprimido ({}) — {} → {}",
                bitrate.arg(),
                human_size(orig),
                human_size(new_sz)
            ))
            .with_detail(format!("Salvo em: {}", output.display()))
            .with_output(output)
        }
        Err(e) => OperationResult::err("Não foi possível comprimir o áudio.").with_detail(e),
    }
}
