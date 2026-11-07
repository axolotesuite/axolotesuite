//! Interface do módulo de Histórico de Atividades.

use iced::widget::{checkbox, column, container, row, text};
use iced::{Alignment, Element, Length};

use crate::core::history::History;
use crate::modules::history::{Msg, State};
use crate::shared::theme::{self, BannerKind, Btn};
use crate::shared::types::Env;
use crate::shared::ui::{self, SEMIBOLD};

pub fn view<'a>(state: &'a State, history: &'a History, env: &Env) -> Element<'a, Msg> {
    let c = env.c();
    let entries = history.entries();

    let header = ui::page_header(
        env,
        "🕒",
        "Histórico de Atividades",
        "Registro completo de todas as operações executadas nos módulos, com data, hora e resultado.",
        Some(ui::badge(format!("{} registro(s)", entries.len()), c.primary)),
    );

    let search_bar: Element<'a, Msg> = {
        let inp = ui::input("Filtrar por módulo, ação ou detalhe…", &state.search, Msg::SearchChanged).width(Length::Fill);
        let chk = checkbox("Mostrar apenas falhas", state.only_errors)
            .on_toggle(Msg::ToggleOnlyErrors)
            .size(18)
            .text_size(13);
        let clear_btn = ui::btn("🗑  Limpar histórico", Btn::Danger, (!entries.is_empty()).then_some(Msg::AskClear));
        if env.narrow() {
            column![inp, chk, clear_btn].spacing(8).into()
        } else {
            row![inp, chk, clear_btn].spacing(12).align_items(Alignment::Center).into()
        }
    };

    let q = state.search.trim().to_lowercase();
    let filtered: Vec<_> = entries
        .iter()
        .filter(|e| !state.only_errors || !e.success)
        .filter(|e| {
            q.is_empty()
                || e.module.to_lowercase().contains(&q)
                || e.action.to_lowercase().contains(&q)
                || e.details.to_lowercase().contains(&q)
        })
        .collect();

    let mut list = column![].spacing(8);
    if filtered.is_empty() {
        list = list.push(ui::empty_state(
            env,
            "🕒",
            "Nenhum registro encontrado",
            "Conforme você usa as ferramentas do Axolote Suite, cada operação aparece registrada aqui.",
        ));
    } else {
        for item in filtered.iter().take(100) {
            let (icon, col) = if item.success { ("✅", c.success) } else { ("⚠", c.danger) };
            let row_content: Element<'a, Msg> = if env.narrow() {
                column![
                    row![
                        text(icon).size(15),
                        ui::badge(&item.module, c.primary),
                        ui::fill(),
                        ui::muted(env, &item.timestamp),
                    ]
                    .spacing(8)
                    .align_items(Alignment::Center),
                    text(&item.action).size(14).font(SEMIBOLD),
                    ui::colored(&item.details, 13, col),
                ]
                .spacing(4)
                .into()
            } else {
                row![
                    text(icon).size(16),
                    ui::badge(&item.module, c.primary),
                    column![
                        text(&item.action).size(14).font(SEMIBOLD),
                        ui::colored(&item.details, 12, col),
                    ]
                    .spacing(2)
                    .width(Length::Fill),
                    ui::muted(env, &item.timestamp),
                ]
                .spacing(12)
                .align_items(Alignment::Center)
                .into()
            };
            list = list.push(ui::list_row(row_content));
        }
    }

    let mut body = column![search_bar].spacing(14);
    if state.confirm_clear {
        body = body.push(
            container(
                row![
                    text("⚠  Deseja apagar todo o histórico de atividades?")
                        .size(13)
                        .style(c.danger)
                        .width(Length::Fill),
                    ui::btn("Sim, limpar tudo", Btn::Danger, Some(Msg::ConfirmClear(true))),
                    ui::btn("Cancelar", Btn::Secondary, Some(Msg::ConfirmClear(false))),
                ]
                .spacing(10)
                .align_items(Alignment::Center),
            )
            .padding(14)
            .width(Length::Fill)
            .style(theme::banner(BannerKind::Error)),
        );
    }
    body = body.push(list);

    let main = ui::section(
        env,
        "📜",
        "Linha do Tempo",
        "Salvo localmente no seu computador (sem telemetria e sem nuvem).",
        body,
    );

    ui::page(env, vec![header, main])
}
