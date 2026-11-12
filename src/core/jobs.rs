//! Fila de operações: acompanha as tarefas demoradas que rodam em segundo
//! plano (conversões, compactação, PDFs...) sem travar a interface.

use std::time::{Duration, Instant};

use crate::shared::types::OperationResult;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JobStatus {
    Running,
    Done,
    Failed,
}

#[derive(Debug, Clone)]
pub struct Job {
    pub id: u64,
    pub module: String,
    pub title: String,
    pub status: JobStatus,
    pub started: Instant,
    pub elapsed: Option<Duration>,
    pub message: String,
    pub started_at: String,
}

#[derive(Default)]
pub struct Jobs {
    jobs: Vec<Job>,
    next_id: u64,
}

impl Jobs {
    pub fn start(&mut self, module: &str, title: &str) -> u64 {
        self.next_id += 1;
        let id = self.next_id;
        self.jobs.insert(
            0,
            Job {
                id,
                module: module.to_string(),
                title: title.to_string(),
                status: JobStatus::Running,
                started: Instant::now(),
                elapsed: None,
                message: "Em andamento…".into(),
                started_at: chrono::Local::now().format("%H:%M:%S").to_string(),
            },
        );
        id
    }

    /// Conclui a operação; devolve (módulo, título) para registrar no histórico.
    pub fn finish(&mut self, id: u64, result: &OperationResult) -> Option<(String, String)> {
        let job = self.jobs.iter_mut().find(|j| j.id == id)?;
        job.status = if result.success {
            JobStatus::Done
        } else {
            JobStatus::Failed
        };
        job.elapsed = Some(job.started.elapsed());
        job.message = result.title.clone();
        Some((job.module.clone(), job.title.clone()))
    }

    pub fn all(&self) -> &[Job] {
        &self.jobs
    }

    pub fn running(&self) -> usize {
        self.jobs
            .iter()
            .filter(|j| j.status == JobStatus::Running)
            .count()
    }

    pub fn clear_finished(&mut self) {
        self.jobs.retain(|j| j.status == JobStatus::Running);
    }
}
