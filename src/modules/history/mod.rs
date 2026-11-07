//! Módulo de Histórico de Atividades.

pub mod view;

use iced::Command;

use crate::core::Ctx;

#[derive(Default)]
pub struct State {
    pub search: String,
    pub only_errors: bool,
    pub confirm_clear: bool,
}

#[derive(Debug, Clone)]
pub enum Msg {
    SearchChanged(String),
    ToggleOnlyErrors(bool),
    AskClear,
    ConfirmClear(bool),
}

impl State {
    pub fn update(&mut self, msg: Msg, ctx: &mut Ctx) -> Command<Msg> {
        match msg {
            Msg::SearchChanged(s) => self.search = s,
            Msg::ToggleOnlyErrors(b) => self.only_errors = b,
            Msg::AskClear => self.confirm_clear = true,
            Msg::ConfirmClear(yes) => {
                self.confirm_clear = false;
                if yes {
                    ctx.history.clear();
                }
            }
        }
        Command::none()
    }
}
