//! Interface do módulo de Configurações.

use iced::widget::{checkbox, column, row, slider, text, toggler};
use iced::{Alignment, Element, Length};

use crate::core::{config::Settings, paths};
use crate::modules::settings::{Msg, State};
use crate::shared::theme::Btn;
use crate::shared::types::Env;
use crate::shared::ui::{self, act};

pub fn view<'a>(state: &'a State, settings: &Settings, env: &Env) -> Element<'a, Msg> {
    let c = env.c();

    let header = ui::page_header(
        env,
        "⚙",
        "Configurações",
        "Personalize a aparência e o comportamento do Axolote Suite. Tudo é salvo instantaneamente no seu computador.",
        None,
    );

    let theme_toggle = toggler(
        Some("Usar tema escuro (identidade visual roxo & azul neon)".to_string()),
        settings.dark_mode,
        Msg::DarkModeChanged,
    )
    .text_size(14)
    .width(Length::Fill);

    let auto_open_chk = checkbox(
        "Abrir automaticamente a pasta de saída no Explorer quando uma operação terminar",
        settings.open_folder_after,
    )
    .on_toggle(Msg::OpenFolderAfterChanged)
    .size(18)
    .text_size(14);

    let hist_slider = slider(20..=500, settings.max_history as u32, Msg::MaxHistoryChanged);

    let prefs_card = ui::section(
        env,
        "🎨",
        "Aparência e Comportamento",
        "Suas escolhas são gravadas automaticamente no arquivo config.json local.",
        column![
            ui::list_row(theme_toggle),
            ui::list_row(auto_open_chk),
            ui::list_row(
                column![
                    row![
                        ui::strong("Limite de registros no Histórico", 14).width(Length::Fill),
                        ui::badge(format!("{} itens", settings.max_history), c.primary),
                    ]
                    .align_items(Alignment::Center),
                    hist_slider,
                    ui::muted(env, "Define quantas operações recentes são mantidas no arquivo historico.json."),
                ]
                .spacing(8),
            ),
            ui::actions(
                env,
                vec![
                    act("📂  Abrir pasta de dados do Axolote", Btn::Secondary, Some(Msg::OpenDataFolder)),
                    act("🔄  Restaurar padrões", Btn::Ghost, Some(Msg::ResetDefaults)),
                ],
            ),
        ]
        .spacing(14),
    );

    let mut main_col = column![prefs_card].spacing(16);
    if let Some(res) = &state.saved_banner {
        main_col = main_col.push(ui::result_banner(env, res, None));
    }

    let data_dir = paths::data_dir().display().to_string();
    let side = ui::section(
        env,
        "🔒",
        "100% Offline e Privado",
        "Seus arquivos e notas nunca saem do seu computador.",
        column![
            text("• Nenhuma conta ou login necessário.").size(13),
            text("• Zero telemetria, zero anúncios e zero conexão com servidores.").size(13),
            text("• Pasta onde ficam suas configurações, notas e tarefas:").size(13),
            ui::list_row(ui::muted(env, data_dir)),
        ]
        .spacing(8),
    );

    ui::page(env, vec![header, ui::two_col(env, main_col.into(), side)])
}
