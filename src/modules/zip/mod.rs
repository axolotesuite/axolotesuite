//! Módulo ZIP: compactar arquivos/pastas e extrair ou inspecionar arquivos `.zip`.

pub mod service;
pub mod view;

use std::path::{Path, PathBuf};

use iced::Command;

use crate::core::{paths, system, Ctx};
use crate::shared::types::OperationResult;
use service::ZipEntryInfo;

const MODULE: &str = "ZIP";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Extract,
    Create,
}

pub struct State {
    pub mode: Mode,
    // Extrair / inspecionar
    pub zip_path: String,
    pub dest_dir: String,
    pub entries: Option<Result<Vec<ZipEntryInfo>, String>>,
    // Criar ZIP
    pub sources: Vec<PathBuf>,
    pub output_zip: String,
    // Estado geral
    pub busy: bool,
    pub result: Option<OperationResult>,
}

#[derive(Debug, Clone)]
pub enum Msg {
    ModeChanged(Mode),
    // Extrair
    ZipPathChanged(String),
    BrowseZip,
    DestDirChanged(String),
    BrowseDest,
    InspectZip,
    ExtractRun,
    // Criar
    AddFiles,
    AddFolder,
    RemoveSource(usize),
    ClearSources,
    OutputZipChanged(String),
    BrowseOutputZip,
    CreateRun,
    // Retorno
    Done(u64, OperationResult),
    OpenPath(PathBuf),
}

impl State {
    pub fn new() -> Self {
        Self {
            mode: Mode::Extract,
            zip_path: String::new(),
            dest_dir: String::new(),
            entries: None,
            sources: Vec::new(),
            output_zip: String::new(),
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
            Msg::ZipPathChanged(s) => {
                self.zip_path = s;
                self.entries = None;
            }
            Msg::BrowseZip => {
                if let Some(p) = system::pick_file("Selecione um arquivo ZIP", "Arquivos ZIP (*.zip)", &["zip"]) {
                    // Sugere automaticamente uma pasta de destino ao lado do ZIP
                    if self.dest_dir.trim().is_empty() {
                        let parent = p.parent().unwrap_or_else(|| Path::new("."));
                        let stem = p.file_stem().map(|s| s.to_string_lossy().to_string()).unwrap_or_else(|| "Extraido".into());
                        self.dest_dir = parent.join(stem).display().to_string();
                    }
                    self.zip_path = p.display().to_string();
                    self.entries = Some(service::list_contents(&p));
                    self.result = None;
                }
            }
            Msg::DestDirChanged(s) => self.dest_dir = s,
            Msg::BrowseDest => {
                if let Some(d) = system::pick_folder("Escolha onde os arquivos serão extraídos") {
                    self.dest_dir = d.display().to_string();
                }
            }
            Msg::InspectZip => {
                let p = PathBuf::from(self.zip_path.trim());
                self.entries = Some(service::list_contents(&p));
            }
            Msg::ExtractRun => {
                let zip_p = PathBuf::from(self.zip_path.trim());
                let dest = if self.dest_dir.trim().is_empty() {
                    let parent = zip_p.parent().unwrap_or_else(|| Path::new("."));
                    let stem = zip_p.file_stem().map(|s| s.to_string_lossy().to_string()).unwrap_or_else(|| "Extraido".into());
                    parent.join(stem)
                } else {
                    PathBuf::from(self.dest_dir.trim())
                };
                self.dest_dir = dest.display().to_string();
                let id = ctx.start_job(MODULE, &format!("Extrair {}", crate::shared::ui::file_name(&zip_p)));
                self.busy = true;
                self.result = None;
                return Command::perform(system::run_blocking(move || service::extract(zip_p, dest)), move |r| Msg::Done(id, r));
            }

            // Criar ZIP
            Msg::AddFiles => {
                for p in system::pick_files("Selecione os arquivos para compactar", "", &[]) {
                    if !self.sources.contains(&p) {
                        self.suggest_output_from(&p);
                        self.sources.push(p);
                    }
                }
                self.result = None;
            }
            Msg::AddFolder => {
                if let Some(dir) = system::pick_folder("Selecione uma pasta para incluir no ZIP") {
                    if !self.sources.contains(&dir) {
                        self.suggest_output_from(&dir);
                        self.sources.push(dir);
                    }
                    self.result = None;
                }
            }
            Msg::RemoveSource(i) => {
                if i < self.sources.len() {
                    self.sources.remove(i);
                }
            }
            Msg::ClearSources => {
                self.sources.clear();
                self.result = None;
            }
            Msg::OutputZipChanged(s) => self.output_zip = s,
            Msg::BrowseOutputZip => {
                if let Some(p) = system::save_file("Onde salvar o novo arquivo ZIP", "compactado.zip", "Arquivo ZIP (*.zip)", &["zip"]) {
                    self.output_zip = p.display().to_string();
                }
            }
            Msg::CreateRun => {
                let mut out = PathBuf::from(self.output_zip.trim());
                if out.extension().is_none() {
                    out.set_extension("zip");
                }
                let sources = self.sources.clone();
                let id = ctx.start_job(MODULE, &format!("Criar {}", crate::shared::ui::file_name(&out)));
                self.busy = true;
                self.result = None;
                return Command::perform(system::run_blocking(move || service::create(out, sources)), move |r| Msg::Done(id, r));
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

    fn suggest_output_from(&mut self, first: &Path) {
        if self.output_zip.trim().is_empty() {
            let parent = first.parent().unwrap_or_else(|| Path::new("."));
            let stem = first.file_stem().map(|s| s.to_string_lossy().to_string()).unwrap_or_else(|| "arquivos".into());
            let candidate = paths::unique_path(parent.join(format!("{stem}.zip")));
            self.output_zip = candidate.display().to_string();
        }
    }
}

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}
