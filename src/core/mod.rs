//! Núcleo da aplicação: configurações, histórico, fila de operações e
//! utilitários de sistema (pastas, diálogos, execução em segundo plano).

pub mod config;
pub mod history;
pub mod jobs;
pub mod paths;
pub mod system;

use crate::shared::types::OperationResult;

/// Contexto compartilhado que os módulos recebem ao processar mensagens.
pub struct Ctx {
    pub settings: config::Settings,
    pub history: history::History,
    pub jobs: jobs::Jobs,
}

impl Ctx {
    pub fn load() -> Self {
        let settings = config::Settings::load();
        let history = history::History::load(settings.max_history);
        Self { settings, history, jobs: jobs::Jobs::default() }
    }

    /// Registra o início de uma operação demorada na fila e devolve seu ID.
    pub fn start_job(&mut self, module: &str, title: &str) -> u64 {
        self.jobs.start(module, title)
    }

    /// Marca a operação como concluída e grava no histórico.
    pub fn finish_job(&mut self, id: u64, result: &OperationResult) {
        if let Some((module, title)) = self.jobs.finish(id, result) {
            self.history.add(&module, &title, &result.title, result.success);
        }
    }

    /// Grava uma operação rápida (sem fila) no histórico.
    pub fn log(&mut self, module: &str, action: &str, result: &OperationResult) {
        self.history.add(module, action, &result.title, result.success);
    }
}
