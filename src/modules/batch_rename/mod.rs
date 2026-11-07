//! Módulo de Renomeação em Lote.

pub mod service;
pub mod view;

use std::path::PathBuf;

use iced::Command;

use crate::core::{system, Ctx};
use crate::shared::types::OperationResult;
use service::{PreviewItem, Rule};

const MODULE: &str = "Renomear em Lote";

pub struct State {
    pub files: Vec<PathBuf>,
    pub rule: Rule,
    pub find_text: String,
    pub value_text: String,
    pub busy: bool,
    pub result: Option<OperationResult>,
}

#[derive(Debug, Clone)]
pub enum Msg {
    AddFiles,
    AddFolder,
    RemoveFile(usize),
    ClearFiles,
    RuleChanged(Rule),
    FindChanged(String),
    ValueChanged(String),
    Execute,
    Done(u64, (OperationResult, Vec<PathBuf>)),
    OpenPath(PathBuf),
}

impl State {
    pub fn new() -> Self {
        Self {
            files: Vec::new(),
            rule: Rule::Prefix,
            find_text: String::new(),
            value_text: String::new(),
            busy: false,
            result: None,
        }
    }

    pub fn preview_items(&self) -> Vec<PreviewItem> {
        service::preview(&self.files, self.rule, &self.find_text, &self.value_text)
    }

    pub fn update(&mut self, msg: Msg, ctx: &mut Ctx) -> Command<Msg> {
        match msg {
            Msg::AddFiles => {
                for p in system::pick_files("Selecione os arquivos para renomear", "", &[]) {
                    if !self.files.contains(&p) {
                        self.files.push(p);
                    }
                }
                self.result = None;
            }
            Msg::AddFolder => {
                if let Some(dir) = system::pick_folder("Selecione uma pasta para carregar todos os arquivos dela") {
                    for p in service::files_in_folder(&dir) {
                        if !self.files.contains(&p) {
                            self.files.push(p);
                        }
                    }
                    self.result = None;
                }
            }
            Msg::RemoveFile(idx) => {
                if idx < self.files.len() {
                    self.files.remove(idx);
                }
            }
            Msg::ClearFiles => {
                self.files.clear();
                self.result = None;
            }
            Msg::RuleChanged(r) => {
                self.rule = r;
                self.result = None;
            }
            Msg::FindChanged(s) => {
                self.find_text = s;
                self.result = None;
            }
            Msg::ValueChanged(s) => {
                self.value_text = s;
                self.result = None;
            }
            Msg::Execute => {
                let files = self.files.clone();
                let rule = self.rule;
                let find = self.find_text.clone();
                let val = self.value_text.clone();
                let id = ctx.start_job(MODULE, &format!("Renomear {} arquivo(s)", files.len()));
                self.busy = true;
                self.result = None;
                return Command::perform(
                    system::run_blocking(move || service::execute(files, rule, find, val)),
                    move |out| Msg::Done(id, out),
                );
            }
            Msg::Done(id, (res, updated)) => {
                ctx.finish_job(id, &res);
                self.busy = false;
                self.files = updated;
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
}

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}
