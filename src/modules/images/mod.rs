//! Módulo de Imagens: Editor de Fotos com pré-visualização ao vivo,
//! redimensionamento, conversão de formato e compressão.

pub mod service;
pub mod view;

use std::path::{Path, PathBuf};

use iced::widget::image::Handle;
use iced::Command;
use image::DynamicImage;

use crate::core::{paths, system, Ctx};
use crate::shared::types::OperationResult;
use service::{AspectCrop, ImageMeta, OutFormat, PhotoEdits, PhotoFilter};

const MODULE: &str = "Imagens";
const IMG_EXTS: &[&str] = &["png", "jpg", "jpeg", "webp", "bmp", "gif", "ico", "tif", "tiff"];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Editor,
    ConvertResize,
}

pub struct State {
    pub mode: Mode,
    pub input: String,
    pub output: String,
    pub meta: Option<Result<ImageMeta, String>>,
    // Editor de Fotos ao vivo
    pub preview_base: Option<DynamicImage>,
    pub preview_handle: Option<Handle>,
    pub edits: PhotoEdits,
    // Redimensionar / Converter / Comprimir
    pub width: String,
    pub height: String,
    pub keep_aspect: bool,
    pub format: OutFormat,
    pub quality: u8,
    // Estado
    pub busy: bool,
    pub result: Option<OperationResult>,
}

#[derive(Debug, Clone)]
pub enum Msg {
    ModeChanged(Mode),
    InputChanged(String),
    BrowseInput,
    OutputChanged(String),
    BrowseOutput,
    // Editor de Fotos
    BrightnessChanged(i32),
    ContrastChanged(i32),
    SaturationChanged(i32),
    RotateLeft,
    RotateRight,
    FlipH,
    FlipV,
    FilterSelected(PhotoFilter),
    CropSelected(AspectCrop),
    ResetEdits,
    SaveEditedPhoto,
    // Redimensionar / Converter / Comprimir
    WidthChanged(String),
    HeightChanged(String),
    KeepAspectToggled(bool),
    ScalePreset(u32),
    FormatChanged(OutFormat),
    QualityChanged(u8),
    ResizeRun,
    ConvertRun,
    CompressRun,
    Done(u64, OperationResult),
    OpenPath(PathBuf),
}

impl State {
    pub fn new() -> Self {
        Self {
            mode: Mode::Editor,
            input: String::new(),
            output: String::new(),
            meta: None,
            preview_base: None,
            preview_handle: None,
            edits: PhotoEdits::default(),
            width: "1280".into(),
            height: "720".into(),
            keep_aspect: true,
            format: OutFormat::Png,
            quality: 80,
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
            Msg::InputChanged(s) => {
                let p = PathBuf::from(s.trim());
                if p.is_file() {
                    self.load_image(&p);
                } else {
                    self.meta = None;
                    self.preview_base = None;
                    self.preview_handle = None;
                }
                self.input = s;
            }
            Msg::BrowseInput => {
                if let Some(p) = system::pick_file("Escolha uma foto ou imagem", "Imagens", IMG_EXTS) {
                    self.load_image(&p);
                    self.input = p.display().to_string();
                    self.result = None;
                }
            }
            Msg::OutputChanged(s) => self.output = s,
            Msg::BrowseOutput => {
                if let Some(p) = system::save_file("Onde salvar a imagem processada", "foto_editada.png", "Imagens", IMG_EXTS) {
                    self.output = p.display().to_string();
                }
            }

            // -------------------- Editor de Fotos ao vivo --------------------
            Msg::BrightnessChanged(v) => {
                self.edits.brightness = v;
                self.refresh_preview();
            }
            Msg::ContrastChanged(v) => {
                self.edits.contrast = v;
                self.refresh_preview();
            }
            Msg::SaturationChanged(v) => {
                self.edits.saturation = v;
                self.refresh_preview();
            }
            Msg::RotateLeft => {
                self.edits.rotation = (self.edits.rotation + 270) % 360;
                self.refresh_preview();
            }
            Msg::RotateRight => {
                self.edits.rotation = (self.edits.rotation + 90) % 360;
                self.refresh_preview();
            }
            Msg::FlipH => {
                self.edits.flip_h = !self.edits.flip_h;
                self.refresh_preview();
            }
            Msg::FlipV => {
                self.edits.flip_v = !self.edits.flip_v;
                self.refresh_preview();
            }
            Msg::FilterSelected(f) => {
                self.edits.filter = f;
                self.refresh_preview();
            }
            Msg::CropSelected(c) => {
                self.edits.crop = c;
                self.refresh_preview();
            }
            Msg::ResetEdits => {
                self.edits = PhotoEdits::default();
                self.refresh_preview();
            }
            Msg::SaveEditedPhoto => {
                let input = PathBuf::from(self.input.trim());
                let ext = input.extension().and_then(|e| e.to_str()).unwrap_or("png").to_string();
                let out = self.resolve_output(&input, "editada", &ext);
                let edits = self.edits;
                let id = ctx.start_job(MODULE, &format!("Salvar foto editada {}", crate::shared::ui::file_name(&input)));
                self.busy = true;
                self.result = None;
                return Command::perform(
                    system::run_blocking(move || service::save_edited_photo(input, out, edits)),
                    move |r| Msg::Done(id, r),
                );
            }

            // ----------- Redimensionar / Converter / Comprimir -----------
            Msg::WidthChanged(w) => {
                if self.keep_aspect {
                    if let (Ok(new_w), Some(Ok(m))) = (w.trim().parse::<u32>(), &self.meta) {
                        if m.width > 0 && new_w > 0 {
                            let h = ((new_w as f64) * (m.height as f64) / (m.width as f64)).round() as u32;
                            self.height = h.max(1).to_string();
                        }
                    }
                }
                self.width = w;
            }
            Msg::HeightChanged(h) => {
                if self.keep_aspect {
                    if let (Ok(new_h), Some(Ok(m))) = (h.trim().parse::<u32>(), &self.meta) {
                        if m.height > 0 && new_h > 0 {
                            let w = ((new_h as f64) * (m.width as f64) / (m.height as f64)).round() as u32;
                            self.width = w.max(1).to_string();
                        }
                    }
                }
                self.height = h;
            }
            Msg::KeepAspectToggled(b) => self.keep_aspect = b,
            Msg::ScalePreset(pct) => {
                if let Some(Ok(m)) = &self.meta {
                    let w = ((m.width as u64 * pct as u64) / 100).max(1) as u32;
                    let h = ((m.height as u64 * pct as u64) / 100).max(1) as u32;
                    self.width = w.to_string();
                    self.height = h.to_string();
                }
            }
            Msg::FormatChanged(f) => self.format = f,
            Msg::QualityChanged(q) => self.quality = q,

            Msg::ResizeRun => {
                let input = PathBuf::from(self.input.trim());
                let w = self.width.trim().parse::<u32>().unwrap_or(0);
                let h = self.height.trim().parse::<u32>().unwrap_or(0);
                let ext = input.extension().and_then(|e| e.to_str()).unwrap_or("png").to_string();
                let out = self.resolve_output(&input, &format!("{w}x{h}"), &ext);
                let keep = self.keep_aspect;
                let id = ctx.start_job(MODULE, &format!("Redimensionar {}", crate::shared::ui::file_name(&input)));
                self.busy = true;
                self.result = None;
                return Command::perform(system::run_blocking(move || service::resize(input, out, w, h, keep)), move |r| Msg::Done(id, r));
            }
            Msg::ConvertRun => {
                let input = PathBuf::from(self.input.trim());
                let fmt = self.format;
                let out = self.resolve_output(&input, "convertida", fmt.ext());
                let id = ctx.start_job(MODULE, &format!("Converter para {}", fmt.ext().to_uppercase()));
                self.busy = true;
                self.result = None;
                return Command::perform(system::run_blocking(move || service::convert(input, out, fmt)), move |r| Msg::Done(id, r));
            }
            Msg::CompressRun => {
                let input = PathBuf::from(self.input.trim());
                let q = self.quality;
                let out = self.resolve_output(&input, &format!("comprimida_q{q}"), "jpg");
                let id = ctx.start_job(MODULE, &format!("Comprimir imagem ({q}%)"));
                self.busy = true;
                self.result = None;
                return Command::perform(system::run_blocking(move || service::compress(input, out, q)), move |r| Msg::Done(id, r));
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

    fn load_image(&mut self, p: &Path) {
        let info = service::inspect(p);
        if let Ok(m) = &info {
            self.width = m.width.to_string();
            self.height = m.height.to_string();
        }
        self.meta = Some(info);
        self.edits = PhotoEdits::default();
        self.preview_base = service::load_preview_base(p);
        self.refresh_preview();
    }

    fn refresh_preview(&mut self) {
        if let Some(base) = &self.preview_base {
            self.preview_handle = Some(service::build_preview_handle(base, &self.edits));
        }
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
            p
        }
    }
}

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}
