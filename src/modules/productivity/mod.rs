//! Módulo de Produtividade: bloco de notas, tarefas, calculadora e
//! organizador de arquivos.

pub mod service;
pub mod view;

use std::path::PathBuf;

use iced::widget::text_editor;
use iced::Command;

use crate::core::{paths, system, Ctx};
use crate::shared::types::OperationResult;
use service::Task;

const MODULE: &str = "Produtividade";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tab {
    Notes,
    Tasks,
    Calc,
    Organizer,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskFilter {
    All,
    Pending,
    Done,
}

pub struct State {
    pub tab: Tab,
    // Notas
    pub notes_dir: PathBuf,
    pub notes: Vec<String>,
    pub selected_note: Option<String>,
    pub note_name: String,
    pub editor: text_editor::Content,
    pub note_dirty: bool,
    pub confirm_delete: bool,
    pub note_result: Option<OperationResult>,
    // Tarefas
    pub tasks: Vec<Task>,
    pub new_task: String,
    pub task_filter: TaskFilter,
    // Calculadora
    pub calc_expr: String,
    pub calc_result: Option<Result<String, String>>,
    pub calc_history: Vec<(String, String)>,
    // Organizador
    pub org_dir: String,
    pub org_preview: Option<Result<Vec<(String, usize)>, String>>,
    pub org_busy: bool,
    pub org_result: Option<OperationResult>,
}

#[derive(Debug, Clone)]
pub enum Msg {
    TabSelected(Tab),
    // Notas
    NoteSelected(String),
    NoteNew,
    NoteNameChanged(String),
    NoteEdit(text_editor::Action),
    NoteSave,
    NoteDelete,
    NoteDeleteConfirm(bool),
    OpenNotesFolder,
    // Tarefas
    TaskInput(String),
    TaskAdd,
    TaskToggle(u64, bool),
    TaskDelete(u64),
    TaskFilterChanged(TaskFilter),
    TasksClearDone,
    // Calculadora
    CalcInput(String),
    CalcKey(&'static str),
    CalcEval,
    CalcClear,
    CalcBackspace,
    CalcUseHistory(String),
    // Organizador
    OrgDirChanged(String),
    OrgBrowse,
    OrgPreview,
    OrgRun,
    OrgDone(u64, OperationResult),
    OpenPath(PathBuf),
}

impl State {
    pub fn new() -> Self {
        let notes_dir = paths::notes_dir();
        let notes = service::list_notes(&notes_dir);
        Self {
            tab: Tab::Notes,
            notes_dir,
            notes,
            selected_note: None,
            note_name: String::new(),
            editor: text_editor::Content::new(),
            note_dirty: false,
            confirm_delete: false,
            note_result: None,
            tasks: service::load_tasks(),
            new_task: String::new(),
            task_filter: TaskFilter::All,
            calc_expr: String::new(),
            calc_result: None,
            calc_history: Vec::new(),
            org_dir: String::new(),
            org_preview: None,
            org_busy: false,
            org_result: None,
        }
    }

    pub fn pending_tasks(&self) -> usize {
        self.tasks.iter().filter(|t| !t.done).count()
    }

    pub fn update(&mut self, msg: Msg, ctx: &mut Ctx) -> Command<Msg> {
        match msg {
            Msg::TabSelected(tab) => self.tab = tab,

            // ----------------------------- Notas -----------------------------
            Msg::NoteSelected(name) => match service::read_note(&self.notes_dir, &name) {
                Ok(content) => {
                    self.editor = text_editor::Content::with_text(&content);
                    self.note_name = name.clone();
                    self.selected_note = Some(name);
                    self.note_dirty = false;
                    self.confirm_delete = false;
                    self.note_result = None;
                }
                Err(e) => {
                    self.note_result = Some(OperationResult::err(format!(
                        "Não foi possível abrir a nota: {e}"
                    )))
                }
            },
            Msg::NoteNew => {
                self.editor = text_editor::Content::new();
                self.note_name = String::new();
                self.selected_note = None;
                self.note_dirty = false;
                self.confirm_delete = false;
                self.note_result = None;
            }
            Msg::NoteNameChanged(name) => {
                self.note_name = name;
                self.note_dirty = true;
            }
            Msg::NoteEdit(action) => {
                if action.is_edit() {
                    self.note_dirty = true;
                }
                self.editor.perform(action);
            }
            Msg::NoteSave => {
                let content = self.editor.text();
                let content = content.strip_suffix('\n').unwrap_or(&content).to_string();
                let res = service::save_note(&self.notes_dir, &self.note_name, &content);
                if res.success {
                    let clean = service::sanitize_name(&self.note_name);
                    // Renomear: se o nome mudou, remove o arquivo antigo.
                    if let Some(old) = &self.selected_note {
                        if *old != clean {
                            let _ = std::fs::remove_file(self.notes_dir.join(format!("{old}.txt")));
                        }
                    }
                    self.note_name = clean.clone();
                    self.selected_note = Some(clean);
                    self.note_dirty = false;
                    self.notes = service::list_notes(&self.notes_dir);
                    ctx.log(MODULE, "Salvar nota", &res);
                }
                self.note_result = Some(res);
            }
            Msg::NoteDelete => self.confirm_delete = true,
            Msg::NoteDeleteConfirm(yes) => {
                self.confirm_delete = false;
                if yes {
                    if let Some(name) = self.selected_note.take() {
                        let res = service::delete_note(&self.notes_dir, &name);
                        ctx.log(MODULE, "Excluir nota", &res);
                        self.note_result = Some(res);
                        self.notes = service::list_notes(&self.notes_dir);
                        self.editor = text_editor::Content::new();
                        self.note_name.clear();
                        self.note_dirty = false;
                    }
                }
            }
            Msg::OpenNotesFolder => system::open_path(&self.notes_dir),

            // ---------------------------- Tarefas ----------------------------
            Msg::TaskInput(s) => self.new_task = s,
            Msg::TaskAdd => {
                let title = self.new_task.trim().to_string();
                if !title.is_empty() {
                    let id = self.tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1;
                    self.tasks.insert(
                        0,
                        Task {
                            id,
                            title,
                            done: false,
                            created: chrono::Local::now().format("%d/%m %H:%M").to_string(),
                        },
                    );
                    self.new_task.clear();
                    service::save_tasks(&self.tasks);
                }
            }
            Msg::TaskToggle(id, done) => {
                if let Some(t) = self.tasks.iter_mut().find(|t| t.id == id) {
                    t.done = done;
                }
                service::save_tasks(&self.tasks);
            }
            Msg::TaskDelete(id) => {
                self.tasks.retain(|t| t.id != id);
                service::save_tasks(&self.tasks);
            }
            Msg::TaskFilterChanged(f) => self.task_filter = f,
            Msg::TasksClearDone => {
                self.tasks.retain(|t| !t.done);
                service::save_tasks(&self.tasks);
            }

            // -------------------------- Calculadora --------------------------
            Msg::CalcInput(s) => {
                self.calc_expr = s;
                self.calc_result = None;
            }
            Msg::CalcKey(k) => {
                self.calc_expr.push_str(k);
                self.calc_result = None;
            }
            Msg::CalcBackspace => {
                self.calc_expr.pop();
                self.calc_result = None;
            }
            Msg::CalcClear => {
                self.calc_expr.clear();
                self.calc_result = None;
            }
            Msg::CalcEval => match service::evaluate(&self.calc_expr) {
                Ok(v) => {
                    let formatted = service::format_number(v);
                    self.calc_history
                        .insert(0, (self.calc_expr.clone(), formatted.clone()));
                    self.calc_history.truncate(12);
                    self.calc_result = Some(Ok(formatted));
                }
                Err(e) => self.calc_result = Some(Err(e)),
            },
            Msg::CalcUseHistory(v) => {
                self.calc_expr = v;
                self.calc_result = None;
            }

            // -------------------------- Organizador --------------------------
            Msg::OrgDirChanged(s) => {
                self.org_dir = s;
                self.org_preview = None;
            }
            Msg::OrgBrowse => {
                if let Some(dir) = system::pick_folder("Escolha a pasta que deseja organizar") {
                    self.org_dir = dir.display().to_string();
                    return self.update(Msg::OrgPreview, ctx);
                }
            }
            Msg::OrgPreview => {
                self.org_result = None;
                self.org_preview = Some(
                    service::organize_preview(std::path::Path::new(self.org_dir.trim()))
                        .map(|m| m.into_iter().map(|(k, v)| (k.to_string(), v)).collect()),
                );
            }
            Msg::OrgRun => {
                let dir = PathBuf::from(self.org_dir.trim());
                let id = ctx.start_job(
                    MODULE,
                    &format!("Organizar pasta {}", crate::shared::ui::file_name(&dir)),
                );
                self.org_busy = true;
                self.org_result = None;
                return Command::perform(
                    system::run_blocking(move || service::organize(dir)),
                    move |r| Msg::OrgDone(id, r),
                );
            }
            Msg::OrgDone(id, res) => {
                ctx.finish_job(id, &res);
                self.org_busy = false;
                self.org_preview = None;
                if res.success && ctx.settings.open_folder_after {
                    if let Some(p) = &res.output {
                        system::open_path(p);
                    }
                }
                self.org_result = Some(res);
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
