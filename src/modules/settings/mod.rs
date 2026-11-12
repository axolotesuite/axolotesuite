//! Módulo de Configurações Globais.

pub mod view;

use iced::Command;

use crate::core::{paths, system, Ctx};
use crate::shared::i18n::LanguageMode;
use crate::shared::types::OperationResult;

#[derive(Default)]
pub struct State {
    pub saved_banner: Option<OperationResult>,
}

#[derive(Debug, Clone)]
pub enum Msg {
    DarkModeChanged(bool),
    LanguageChanged(LanguageMode),
    OpenFolderAfterChanged(bool),
    MaxHistoryChanged(u32),
    ResetDefaults,
    OpenDataFolder,
}

impl State {
    pub fn update(&mut self, msg: Msg, ctx: &mut Ctx) -> Command<Msg> {
        let is_en = ctx.settings.resolved_lang().is_en();
        match msg {
            Msg::DarkModeChanged(dark) => {
                ctx.settings.dark_mode = dark;
                ctx.settings.save();
                self.saved_banner = Some(OperationResult::ok(if dark {
                    if is_en {
                        "Dark theme activated and saved."
                    } else {
                        "Tema escuro ativado e salvo."
                    }
                } else {
                    if is_en {
                        "Light theme activated and saved."
                    } else {
                        "Tema claro ativado e salvo."
                    }
                }));
            }
            Msg::LanguageChanged(lang) => {
                ctx.settings.language = lang;
                ctx.settings.save();
                let is_en_now = ctx.settings.resolved_lang().is_en();
                self.saved_banner = Some(OperationResult::ok(if is_en_now {
                    "Language updated and saved."
                } else {
                    "Idioma atualizado e salvo com sucesso."
                }));
            }
            Msg::OpenFolderAfterChanged(v) => {
                ctx.settings.open_folder_after = v;
                ctx.settings.save();
                self.saved_banner = Some(OperationResult::ok(if is_en {
                    "Preference saved automatically."
                } else {
                    "Preferência salva automaticamente."
                }));
            }
            Msg::MaxHistoryChanged(v) => {
                ctx.settings.max_history = v as usize;
                ctx.settings.save();
                ctx.history.set_max(v as usize);
                self.saved_banner = Some(OperationResult::ok(if is_en {
                    format!("History limit adjusted to {v} items.")
                } else {
                    format!("Limite do histórico ajustado para {v} itens.")
                }));
            }
            Msg::ResetDefaults => {
                ctx.settings = crate::core::config::Settings::default();
                ctx.settings.save();
                ctx.history.set_max(ctx.settings.max_history);
                self.saved_banner = Some(OperationResult::ok(if is_en {
                    "Settings restored to defaults."
                } else {
                    "Configurações restauradas para o padrão original."
                }));
            }
            Msg::OpenDataFolder => {
                system::open_path(&paths::data_dir());
            }
        }
        Command::none()
    }
}
