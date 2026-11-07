//! Módulo de Configurações Globais.

pub mod view;

use iced::Command;

use crate::core::{paths, system, Ctx};
use crate::shared::types::OperationResult;

#[derive(Default)]
pub struct State {
    pub saved_banner: Option<OperationResult>,
}

#[derive(Debug, Clone)]
pub enum Msg {
    DarkModeChanged(bool),
    OpenFolderAfterChanged(bool),
    MaxHistoryChanged(u32),
    ResetDefaults,
    OpenDataFolder,
}

impl State {
    pub fn update(&mut self, msg: Msg, ctx: &mut Ctx) -> Command<Msg> {
        match msg {
            Msg::DarkModeChanged(dark) => {
                ctx.settings.dark_mode = dark;
                ctx.settings.save();
                self.saved_banner = Some(OperationResult::ok(if dark {
                    "Tema escuro ativado e salvo."
                } else {
                    "Tema claro ativado e salvo."
                }));
            }
            Msg::OpenFolderAfterChanged(v) => {
                ctx.settings.open_folder_after = v;
                ctx.settings.save();
                self.saved_banner = Some(OperationResult::ok("Preferência salva automaticamente."));
            }
            Msg::MaxHistoryChanged(v) => {
                ctx.settings.max_history = v as usize;
                ctx.settings.save();
                ctx.history.set_max(v as usize);
                self.saved_banner = Some(OperationResult::ok(format!("Limite do histórico ajustado para {v} itens.")));
            }
            Msg::ResetDefaults => {
                ctx.settings = crate::core::config::Settings::default();
                ctx.settings.save();
                ctx.history.set_max(ctx.settings.max_history);
                self.saved_banner = Some(OperationResult::ok("Configurações restauradas para o padrão original."));
            }
            Msg::OpenDataFolder => {
                system::open_path(&paths::data_dir());
            }
        }
        Command::none()
    }
}
