//! Módulo de Fila de Operações em segundo plano.

pub mod view;

use iced::Command;

use crate::core::Ctx;

#[derive(Default)]
pub struct State;

#[derive(Debug, Clone)]
pub enum Msg {
    ClearFinished,
}

impl State {
    pub fn update(&mut self, msg: Msg, ctx: &mut Ctx) -> Command<Msg> {
        match msg {
            Msg::ClearFinished => ctx.jobs.clear_finished(),
        }
        Command::none()
    }
}
