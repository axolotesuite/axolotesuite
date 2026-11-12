//! Módulo de Vídeo: Editor Rápido de Vídeo + Conversor e Compressor CRF.

pub mod service;
pub mod view;

use std::path::{Path, PathBuf};

use iced::widget::image::Handle;
use iced::Command;

use crate::core::{paths, system, Ctx};
use crate::shared::types::OperationResult;
use service::{CanvasAspect, SpeedPreset, VideoEditConfig, VideoFormat, VideoLook};

const MODULE: &str = "Vídeo";
const VIDEO_EXTS: &[&str] = &["mp4", "mkv", "avi", "mov", "wmv", "webm", "flv", "m4v"];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    VideoEditor,
    ConvertCompress,
}

pub struct State {
    pub mode: Mode,
    pub ffmpeg_ok: bool,
    pub downloading_ffmpeg: bool,
    pub input: String,
    pub output: String,
    pub info_lines: Vec<String>,
    pub thumbnail: Option<Handle>,
    // Editor de Vídeo
    pub edit: VideoEditConfig,
    // Conversor / Compressor
    pub format: VideoFormat,
    pub crf: u8,
    pub busy: bool,
    pub result: Option<OperationResult>,
}

#[derive(Debug, Clone)]
pub enum Msg {
    ModeChanged(Mode),
    RecheckFfmpeg,
    DownloadFfmpeg,
    DownloadFfmpegDone(OperationResult),
    InputChanged(String),
    BrowseInput,
    OutputChanged(String),
    BrowseOutput,
    // Editor de Vídeo
    StartTimeChanged(String),
    EndTimeChanged(String),
    SpeedSelected(SpeedPreset),
    AspectSelected(CanvasAspect),
    LookSelected(VideoLook),
    RotateVideo,
    FlipVideoH,
    MuteToggled(bool),
    VolumeChanged(u16),
    RefreshPreviewFrame,
    ResetVideoEdits,
    ExportEditedVideo,
    // Conversor / Compressor
    FormatChanged(VideoFormat),
    CrfChanged(u8),
    ConvertRun,
    CompressRun,
    Done(u64, OperationResult),
    OpenPath(PathBuf),
}

impl State {
    pub fn new() -> Self {
        Self {
            mode: Mode::VideoEditor,
            ffmpeg_ok: system::find_ffmpeg().is_some(),
            downloading_ffmpeg: false,
            input: String::new(),
            output: String::new(),
            info_lines: Vec::new(),
            thumbnail: None,
            edit: VideoEditConfig::default(),
            format: VideoFormat::Mp4,
            crf: 24,
            busy: false,
            result: None,
        }
    }

    pub fn update(&mut self, msg: Msg, ctx: &mut Ctx) -> Command<Msg> {
        match msg {
            Msg::ModeChanged(m) => {
                self.mode = m;
                self.result = None;
            }
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
                if p.is_file() {
                    self.info_lines = service::inspect(&p);
                    self.thumbnail = service::extract_thumbnail(&p, &self.edit.start_time);
                } else {
                    self.info_lines.clear();
                    self.thumbnail = None;
                }
                self.input = s;
            }
            Msg::BrowseInput => {
                if let Some(p) = system::pick_file("Escolha um arquivo de vídeo", "Vídeos", VIDEO_EXTS) {
                    self.info_lines = service::inspect(&p);
                    self.edit = VideoEditConfig::default();
                    self.thumbnail = service::extract_thumbnail(&p, "");
                    self.input = p.display().to_string();
                    self.result = None;
                }
            }
            Msg::OutputChanged(s) => self.output = s,
            Msg::BrowseOutput => {
                let ext = self.format.ext();
                if let Some(p) = system::save_file("Onde salvar o vídeo", &format!("video_editado.{ext}"), "Vídeo", &[ext]) {
                    self.output = p.display().to_string();
                }
            }

            // -------------------- Editor estilo CapCut --------------------
            Msg::StartTimeChanged(s) => self.edit.start_time = s,
            Msg::EndTimeChanged(s) => self.edit.end_time = s,
            Msg::SpeedSelected(sp) => self.edit.speed = sp,
            Msg::AspectSelected(a) => self.edit.aspect = a,
            Msg::LookSelected(l) => self.edit.look = l,
            Msg::RotateVideo => self.edit.rotation = (self.edit.rotation + 90) % 360,
            Msg::FlipVideoH => self.edit.flip_h = !self.edit.flip_h,
            Msg::MuteToggled(b) => self.edit.mute_audio = b,
            Msg::VolumeChanged(v) => self.edit.volume_pct = v,
            Msg::RefreshPreviewFrame => {
                let p = PathBuf::from(self.input.trim());
                if p.is_file() {
                    self.thumbnail = service::extract_thumbnail(&p, &self.edit.start_time);
                }
            }
            Msg::ResetVideoEdits => {
                self.edit = VideoEditConfig::default();
            }
            Msg::ExportEditedVideo => {
                let input = PathBuf::from(self.input.trim());
                let out = self.resolve_output(&input, "editado", "mp4");
                let cfg = self.edit.clone();
                let id = ctx.start_job(MODULE, &format!("Exportar vídeo editado {}", crate::shared::ui::file_name(&input)));
                self.busy = true;
                self.result = None;
                return Command::perform(
                    system::run_blocking(move || service::render_edited_video(input, out, cfg)),
                    move |r| Msg::Done(id, r),
                );
            }

            // -------------------- Conversor / Compressor --------------------
            Msg::FormatChanged(f) => self.format = f,
            Msg::CrfChanged(v) => self.crf = v,

            Msg::ConvertRun => {
                let input = PathBuf::from(self.input.trim());
                let fmt = self.format;
                let out = self.resolve_output(&input, "convertido", fmt.ext());
                let id = ctx.start_job(MODULE, &format!("Converter vídeo para {}", fmt.ext().to_uppercase()));
                self.busy = true;
                self.result = None;
                return Command::perform(system::run_blocking(move || service::convert(input, out, fmt)), move |r| Msg::Done(id, r));
            }
            Msg::CompressRun => {
                let input = PathBuf::from(self.input.trim());
                let crf = self.crf;
                let out = self.resolve_output(&input, &format!("comprimido_crf{crf}"), "mp4");
                let id = ctx.start_job(MODULE, &format!("Comprimir vídeo (CRF {crf})"));
                self.busy = true;
                self.result = None;
                return Command::perform(system::run_blocking(move || service::compress(input, out, crf)), move |r| Msg::Done(id, r));
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
