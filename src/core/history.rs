//! Histórico de atividades (salvo em JSON local).

use serde::{Deserialize, Serialize};

use crate::core::paths;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryEntry {
    pub module: String,
    pub action: String,
    pub details: String,
    /// Data/hora local já formatada (dd/mm/aaaa hh:mm:ss).
    pub timestamp: String,
    pub success: bool,
}

pub struct History {
    entries: Vec<HistoryEntry>,
    max: usize,
}

impl History {
    pub fn load(max: usize) -> Self {
        let entries = std::fs::read_to_string(paths::history_file())
            .ok()
            .and_then(|s| serde_json::from_str::<Vec<HistoryEntry>>(&s).ok())
            .unwrap_or_default();
        let mut h = Self {
            entries,
            max: max.max(10),
        };
        h.entries.truncate(h.max);
        h
    }

    pub fn entries(&self) -> &[HistoryEntry] {
        &self.entries
    }

    pub fn add(&mut self, module: &str, action: &str, details: &str, success: bool) {
        self.entries.insert(
            0,
            HistoryEntry {
                module: module.to_string(),
                action: action.to_string(),
                details: details.to_string(),
                timestamp: chrono::Local::now().format("%d/%m/%Y %H:%M:%S").to_string(),
                success,
            },
        );
        self.entries.truncate(self.max);
        self.save();
    }

    pub fn set_max(&mut self, max: usize) {
        self.max = max.max(10);
        self.entries.truncate(self.max);
        self.save();
    }

    pub fn clear(&mut self) {
        self.entries.clear();
        self.save();
    }

    fn save(&self) {
        if let Ok(json) = serde_json::to_string_pretty(&self.entries) {
            let _ = std::fs::write(paths::history_file(), json);
        }
    }

    #[cfg(test)]
    pub fn empty_for_test(max: usize) -> Self {
        Self {
            entries: Vec::new(),
            max: max.max(10),
        }
    }

    #[cfg(test)]
    pub fn add_in_memory(&mut self, module: &str, action: &str, details: &str, success: bool) {
        self.entries.insert(
            0,
            HistoryEntry {
                module: module.to_string(),
                action: action.to_string(),
                details: details.to_string(),
                timestamp: "11/11/2026 22:00:00".to_string(),
                success,
            },
        );
        self.entries.truncate(self.max);
    }
}
