//! Módulo PDF: unir múltiplos PDFs, dividir em páginas e inspecionar documento.

pub mod service;
pub mod view;

use std::path::{Path, PathBuf};

use iced::Command;

use crate::core::{paths, system, Ctx};
use crate::shared::types::OperationResult;
use service::PdfMeta;

const MODULE: &str = "PDF";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Merge,
    Split,
}

pub struct State {
    pub mode: Mode,
    // Unir PDFs
    pub merge_files: Vec<(PathBuf, Option<usize>)>,
    pub merge_output: String,
    // Dividir / inspecionar PDF
    pub split_input: String,
    pub split_meta: Option<Result<PdfMeta, String>>,
    pub split_out_dir: String,
    pub page_from: String,
    pub page_to: String,
    // Geral
    pub busy: bool,
    pub result: Option<OperationResult>,
}

#[derive(Debug, Clone)]
pub enum Msg {
    ModeChanged(Mode),
    // Unir
    AddMergeFiles,
    MoveUp(usize),
    MoveDown(usize),
    RemoveMergeFile(usize),
    ClearMerge,
    MergeOutputChanged(String),
    BrowseMergeOutput,
    MergeRun,
    // Dividir
    SplitInputChanged(String),
    BrowseSplitInput,
    SplitOutDirChanged(String),
    BrowseSplitOutDir,
    PageFromChanged(String),
    PageToChanged(String),
    SplitRun,
    // Retorno
    Done(u64, OperationResult),
    OpenPath(PathBuf),
}

impl State {
    pub fn new() -> Self {
        Self {
            mode: Mode::Merge,
            merge_files: Vec::new(),
            merge_output: String::new(),
            split_input: String::new(),
            split_meta: None,
            split_out_dir: String::new(),
            page_from: String::new(),
            page_to: String::new(),
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
            Msg::AddMergeFiles => {
                for p in system::pick_files(
                    "Selecione os arquivos PDF para unir",
                    "Documentos PDF (*.pdf)",
                    &["pdf"],
                ) {
                    if !self.merge_files.iter().any(|(existing, _)| existing == &p) {
                        if self.merge_output.trim().is_empty() {
                            let parent = p.parent().unwrap_or_else(|| Path::new("."));
                            let out = paths::unique_path(parent.join("documento_unido.pdf"));
                            self.merge_output = out.display().to_string();
                        }
                        let pages = service::inspect(&p).ok().map(|m| m.pages);
                        self.merge_files.push((p, pages));
                    }
                }
                self.result = None;
            }
            Msg::MoveUp(i) => {
                if i > 0 && i < self.merge_files.len() {
                    self.merge_files.swap(i - 1, i);
                }
            }
            Msg::MoveDown(i) => {
                if i + 1 < self.merge_files.len() {
                    self.merge_files.swap(i, i + 1);
                }
            }
            Msg::RemoveMergeFile(i) => {
                if i < self.merge_files.len() {
                    self.merge_files.remove(i);
                }
            }
            Msg::ClearMerge => {
                self.merge_files.clear();
                self.result = None;
            }
            Msg::MergeOutputChanged(s) => self.merge_output = s,
            Msg::BrowseMergeOutput => {
                if let Some(p) = system::save_file(
                    "Onde salvar o PDF unido",
                    "documento_unido.pdf",
                    "PDF (*.pdf)",
                    &["pdf"],
                ) {
                    self.merge_output = p.display().to_string();
                }
            }
            Msg::MergeRun => {
                let inputs: Vec<PathBuf> =
                    self.merge_files.iter().map(|(p, _)| p.clone()).collect();
                let mut out = PathBuf::from(self.merge_output.trim());
                if out.extension().is_none() {
                    out.set_extension("pdf");
                }
                let id = ctx.start_job(MODULE, &format!("Unir {} arquivos PDF", inputs.len()));
                self.busy = true;
                self.result = None;
                return Command::perform(
                    system::run_blocking(move || service::merge(out, inputs)),
                    move |r| Msg::Done(id, r),
                );
            }

            // Dividir / inspecionar
            Msg::SplitInputChanged(s) => {
                let p = PathBuf::from(s.trim());
                if p.is_file() {
                    self.load_split_meta(&p);
                } else {
                    self.split_meta = None;
                }
                self.split_input = s;
            }
            Msg::BrowseSplitInput => {
                if let Some(p) =
                    system::pick_file("Escolha um arquivo PDF", "PDF (*.pdf)", &["pdf"])
                {
                    self.load_split_meta(&p);
                    if self.split_out_dir.trim().is_empty() {
                        let parent = p.parent().unwrap_or_else(|| Path::new("."));
                        let stem = p
                            .file_stem()
                            .map(|s| s.to_string_lossy().to_string())
                            .unwrap_or_else(|| "paginas".into());
                        self.split_out_dir =
                            parent.join(format!("{stem}_paginas")).display().to_string();
                    }
                    self.split_input = p.display().to_string();
                    self.result = None;
                }
            }
            Msg::SplitOutDirChanged(s) => self.split_out_dir = s,
            Msg::BrowseSplitOutDir => {
                if let Some(d) = system::pick_folder("Escolha onde salvar as páginas extraídas") {
                    self.split_out_dir = d.display().to_string();
                }
            }
            Msg::PageFromChanged(s) => self.page_from = s,
            Msg::PageToChanged(s) => self.page_to = s,
            Msg::SplitRun => {
                let pdf = PathBuf::from(self.split_input.trim());
                let out_dir = if self.split_out_dir.trim().is_empty() {
                    let parent = pdf.parent().unwrap_or_else(|| Path::new("."));
                    let stem = pdf
                        .file_stem()
                        .map(|s| s.to_string_lossy().to_string())
                        .unwrap_or_else(|| "paginas".into());
                    parent.join(format!("{stem}_paginas"))
                } else {
                    PathBuf::from(self.split_out_dir.trim())
                };
                self.split_out_dir = out_dir.display().to_string();
                let range = match (
                    self.page_from.trim().parse::<u32>(),
                    self.page_to.trim().parse::<u32>(),
                ) {
                    (Ok(f), Ok(t)) => Some((f, t)),
                    _ => None,
                };
                let id = ctx.start_job(
                    MODULE,
                    &format!("Dividir {}", crate::shared::ui::file_name(&pdf)),
                );
                self.busy = true;
                self.result = None;
                return Command::perform(
                    system::run_blocking(move || service::split(pdf, out_dir, range)),
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

    fn load_split_meta(&mut self, p: &Path) {
        let meta = service::inspect(p);
        if let Ok(m) = &meta {
            self.page_from = "1".into();
            self.page_to = m.pages.max(1).to_string();
        }
        self.split_meta = Some(meta);
    }
}

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}
