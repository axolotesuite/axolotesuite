//! Módulo de Áudio: converter formato, extrair áudio de vídeo e comprimir.

pub mod service;
pub mod view;

use std::path::{Path, PathBuf};

use iced::Command;

use crate::core::{paths, system, Ctx};
use crate::shared::types::OperationResult;
use service::{AudioFormat, Bitrate};

const MODULE: &str = "Áudio";
const AUDIO_VIDEO_EXTS: &[&str] = &[
    "mp3", "wav", "flac", "ogg", "m4a", "aac", "wma", "opus", "mp4", "mkv", "avi", "mov", "webm",
];

pub struct State {
    pub ffmpeg_ok: bool,
    pub downloading_ffmpeg: bool,
    pub input: String,
    pub output: String,
    pub info_lines: Vec<String>,
    pub format: AudioFormat,
    pub bitrate: Bitrate,
    pub busy: bool,
    pub result: Option<OperationResult>,
}

#[derive(Debug, Clone)]
pub enum Msg {
    RecheckFfmpeg,
    DownloadFfmpeg,
    DownloadFfmpegDone(OperationResult),
    InputChanged(String),
    BrowseInput,
    OutputChanged(String),
    BrowseOutput,
    FormatChanged(AudioFormat),
    BitrateChanged(Bitrate),
    ConvertRun,
    ExtractRun,
    CompressRun,
    Done(u64, OperationResult),
    OpenPath(PathBuf),
}

impl State {
    pub fn new() -> Self {
        Self {
            ffmpeg_ok: system::find_ffmpeg().is_some(),
            downloading_ffmpeg: false,
            input: String::new(),
            output: String::new(),
            info_lines: Vec::new(),
            format: AudioFormat::Mp3,
            bitrate: Bitrate::K128,
            busy: false,
            result: None,
        }
    }

    pub fn update(&mut self, msg: Msg, ctx: &mut Ctx) -> Command<Msg> {
        match msg {
            Msg::RecheckFfmpeg => {
                self.ffmpeg_ok = system::find_ffmpeg().is_some();
            }
            Msg::DownloadFfmpeg => {
                self.downloading_ffmpeg = true;
                self.result = None;
                let bin = paths::bin_dir();
                return Command::perform(
                    system::run_blocking(move || match system::ensure_ffmpeg_installed(&bin) {
                        Ok(p) => OperationResult::ok("FFmpeg baixado e ativado com sucesso!")
                            .with_detail(format!("Instalado em: {}", p.display())),
                        Err(e) => OperationResult::err(e),
                    }),
                    Msg::DownloadFfmpegDone,
                );
            }
            Msg::DownloadFfmpegDone(res) => {
                self.downloading_ffmpeg = false;
                self.ffmpeg_ok = system::find_ffmpeg().is_some();
                ctx.log(MODULE, "Baixar FFmpeg automaticamente", &res);
                self.result = Some(res);
            }
            Msg::InputChanged(s) => {
                let p = PathBuf::from(s.trim());
                self.info_lines = if p.is_file() {
                    service::inspect(&p)
                } else {
                    Vec::new()
                };
                self.input = s;
            }
            Msg::BrowseInput => {
                if let Some(p) = system::pick_file(
                    "Escolha um arquivo de áudio ou vídeo",
                    "Áudio e Vídeo",
                    AUDIO_VIDEO_EXTS,
                ) {
                    self.info_lines = service::inspect(&p);
                    self.input = p.display().to_string();
                    self.result = None;
                }
            }
            Msg::OutputChanged(s) => self.output = s,
            Msg::BrowseOutput => {
                let ext = self.format.ext();
                if let Some(p) = system::save_file(
                    "Onde salvar o áudio",
                    &format!("audio_convertido.{ext}"),
                    "Áudio",
                    &[ext],
                ) {
                    self.output = p.display().to_string();
                }
            }
            Msg::FormatChanged(f) => self.format = f,
            Msg::BitrateChanged(b) => self.bitrate = b,

            Msg::ConvertRun => {
                let input = PathBuf::from(self.input.trim());
                let fmt = self.format;
                let out = self.resolve_output(&input, "convertido", fmt.ext());
                let id = ctx.start_job(
                    MODULE,
                    &format!("Converter áudio para {}", fmt.ext().to_uppercase()),
                );
                self.busy = true;
                self.result = None;
                return Command::perform(
                    system::run_blocking(move || service::convert(input, out, fmt)),
                    move |r| Msg::Done(id, r),
                );
            }
            Msg::ExtractRun => {
                let input = PathBuf::from(self.input.trim());
                let fmt = self.format;
                let out = self.resolve_output(&input, "audio_extraido", fmt.ext());
                let id = ctx.start_job(
                    MODULE,
                    &format!("Extrair áudio ({})", fmt.ext().to_uppercase()),
                );
                self.busy = true;
                self.result = None;
                return Command::perform(
                    system::run_blocking(move || service::extract_from_video(input, out, fmt)),
                    move |r| Msg::Done(id, r),
                );
            }
            Msg::CompressRun => {
                let input = PathBuf::from(self.input.trim());
                let br = self.bitrate;
                let out = self.resolve_output(&input, &format!("comprimido_{}", br.arg()), "mp3");
                let id = ctx.start_job(MODULE, &format!("Comprimir áudio ({})", br.arg()));
                self.busy = true;
                self.result = None;
                return Command::perform(
                    system::run_blocking(move || service::compress(input, out, br)),
                    move |r| Msg::Done(id, r),
                );
            }
            Msg::Done(id, res) => {
                ctx.finish_job(id, &res);
                self.busy = false;
                if res.success && ctx.settings.open_folder_after {
                    if let Some(p) = &res.output {
                        system::open_path(p);
                    }
                }
                self.result = Some(res);
            }
            Msg::OpenPath(p) => system::open_path(&p),
        }
        Command::none()
    }

    fn resolve_output(&self, input: &Path, suffix: &str, ext: &str) -> PathBuf {
        let trimmed = self.output.trim();
        if trimmed.is_empty() {
            paths::output_path(input, None, suffix, ext)
        } else {
            let mut p = PathBuf::from(trimmed);
            if p.is_dir() {
                return paths::output_path(input, Some(&p), suffix, ext);
            }
            p.set_extension(ext);
            if p == input {
                paths::output_path(input, None, suffix, ext)
            } else {
                paths::unique_path(p)
            }
        }
    }
}

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}
