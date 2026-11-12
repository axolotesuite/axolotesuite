//! Estado e mensagens do módulo "Conversor Fácil (1-Clique)".

pub mod service;
pub mod view;

use std::path::PathBuf;

use iced::Command;

use crate::core::{system, Ctx};
use crate::shared::types::OperationResult;

pub use service::WheelCategory;

pub struct State {
    pub category: WheelCategory,
    pub selected_idx: usize,
    pub files: Vec<PathBuf>,
    pub source_ext: String,
    pub ffmpeg_ok: bool,
    pub custom_out_dir: String,
    pub instant_on_slice_click: bool,
    pub strip_metadata: bool,
    pub busy: bool,
    pub result: Option<OperationResult>,
}

#[derive(Debug, Clone)]
pub enum Msg {
    SetCategory(WheelCategory),
    SelectSlice(usize),
    AddFiles,
    ClearFiles,
    RemoveFile(usize),
    OutDirChanged(String),
    BrowseOutDir,
    ToggleInstantClick(bool),
    ToggleStripMetadata(bool),
    ConvertNow,
    Done(u64, OperationResult),
    OpenResult,
}

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}

impl State {
    pub fn new() -> Self {
        let ffmpeg_ok = system::find_ffmpeg().is_some();
        Self {
            category: WheelCategory::Image,
            selected_idx: 0,
            files: Vec::new(),
            source_ext: "PNG".into(),
            ffmpeg_ok,
            custom_out_dir: String::new(),
            instant_on_slice_click: false,
            strip_metadata: true,
            busy: false,
            result: None,
        }
    }

    /// Retorna a lista dinâmica de formatos reais disponíveis para a extensão atual.
    pub fn available_targets(&self) -> Vec<&'static str> {
        service::targets_for_extension(&self.source_ext, self.ffmpeg_ok)
    }

    pub fn current_target_format(&self) -> &'static str {
        let fmts = self.available_targets();
        if fmts.is_empty() {
            "ZIP"
        } else {
            fmts[self.selected_idx % fmts.len()]
        }
    }

    pub fn refresh_targets(&mut self) {
        self.ffmpeg_ok = system::find_ffmpeg().is_some();
        let targets = self.available_targets();
        if !targets.is_empty() {
            self.selected_idx %= targets.len();
        }
    }

    /// Recebe arquivos arrastados do Windows Explorer e atualiza dinamicamente os botões de destino.
    pub fn push_dropped_file(&mut self, path: PathBuf, _dark_mode: bool) {
        if path.is_file() {
            if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                let upper = ext.to_ascii_uppercase();
                self.category = WheelCategory::from_extension(&upper);
                self.source_ext = upper;
            } else {
                self.category = WheelCategory::Archive;
                self.source_ext = "ARQ".into();
            }
            if !self.files.contains(&path) {
                self.files.push(path);
            }
            self.selected_idx = 0;
            self.refresh_targets();
        }
    }

    pub fn update(&mut self, msg: Msg, ctx: &mut Ctx) -> Command<Msg> {
        match msg {
            Msg::SetCategory(cat) => {
                self.category = cat;
                self.source_ext = cat.default_source_ext().into();
                self.selected_idx = 0;
                self.refresh_targets();
            }
            Msg::SelectSlice(idx) => {
                let len = self.available_targets().len().max(1);
                self.selected_idx = idx % len;
                self.refresh_targets();
                if self.instant_on_slice_click && !self.files.is_empty() && !self.busy {
                    return self.start_conversion(ctx);
                }
            }
            Msg::AddFiles => {
                let picked = system::pick_files(
                    "Selecione os arquivos para converter",
                    "Todos os arquivos suportados",
                    &["*"],
                );
                if !picked.is_empty() {
                    if let Some(ext) = picked[0].extension().and_then(|e| e.to_str()) {
                        let upper = ext.to_ascii_uppercase();
                        self.category = WheelCategory::from_extension(&upper);
                        self.source_ext = upper;
                    }
                    for p in picked {
                        if !self.files.contains(&p) {
                            self.files.push(p);
                        }
                    }
                    self.selected_idx = 0;
                    self.refresh_targets();
                }
            }
            Msg::ClearFiles => {
                self.files.clear();
                self.result = None;
                self.refresh_targets();
            }
            Msg::RemoveFile(i) => {
                if i < self.files.len() {
                    self.files.remove(i);
                    if let Some(first) = self.files.first() {
                        if let Some(ext) = first.extension().and_then(|e| e.to_str()) {
                            let upper = ext.to_ascii_uppercase();
                            self.category = WheelCategory::from_extension(&upper);
                            self.source_ext = upper;
                        }
                    }
                    self.refresh_targets();
                }
            }
            Msg::OutDirChanged(s) => self.custom_out_dir = s,
            Msg::BrowseOutDir => {
                if let Some(d) = system::pick_folder(
                    "Escolha a pasta de saída (ou deixe em branco para salvar ao lado do original)",
                ) {
                    self.custom_out_dir = d.display().to_string();
                }
            }
            Msg::ToggleInstantClick(b) => self.instant_on_slice_click = b,
            Msg::ToggleStripMetadata(b) => self.strip_metadata = b,
            Msg::ConvertNow => {
                if !self.busy {
                    return self.start_conversion(ctx);
                }
            }
            Msg::Done(job_id, res) => {
                self.busy = false;
                ctx.finish_job(job_id, &res);
                self.result = Some(res);
            }
            Msg::OpenResult => {
                if let Some(p) = self.result.as_ref().and_then(|r| r.output.as_ref()) {
                    system::open_path(p);
                }
            }
        }
        Command::none()
    }

    fn start_conversion(&mut self, ctx: &mut Ctx) -> Command<Msg> {
        let target = self.current_target_format().to_string();
        let files = self.files.clone();
        let cat = self.category;
        let strip = self.strip_metadata;
        let out_dir = if self.custom_out_dir.trim().is_empty() {
            None
        } else {
            Some(PathBuf::from(self.custom_out_dir.trim()))
        };
        let id = ctx.jobs.start(
            "Conversor Fácil",
            &format!("Converter {} arquivo(s) → {target}", files.len()),
        );
        self.busy = true;
        self.result = None;
        Command::perform(
            system::run_blocking(move || {
                service::execute_wheel_conversion(files, cat, target, out_dir, strip)
            }),
            move |r| Msg::Done(id, r),
        )
    }
}
