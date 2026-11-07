//! Configurações do usuário, salvas em JSON local.

use serde::{Deserialize, Serialize};

use crate::core::paths;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Settings {
    /// Tema escuro (true) ou claro (false).
    pub dark_mode: bool,
    /// Quantidade máxima de itens guardados no histórico.
    pub max_history: usize,
    /// Abrir a pasta do resultado automaticamente ao terminar uma operação.
    pub open_folder_after: bool,
    /// Indica se o usuário já concluiu o Setup inicial (Instalador + dependências).
    pub setup_completed: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            dark_mode: true,
            max_history: 200,
            open_folder_after: false,
            setup_completed: false,
        }
    }
}

impl Settings {
    pub fn load() -> Self {
        std::fs::read_to_string(paths::config_file())
            .ok()
            .and_then(|s| serde_json::from_str(&s).ok())
            .unwrap_or_default()
    }

    pub fn save(&self) {
        if let Ok(json) = serde_json::to_string_pretty(self) {
            let _ = std::fs::write(paths::config_file(), json);
        }
    }
}
