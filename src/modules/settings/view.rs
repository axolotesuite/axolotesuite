//! Interface do módulo de Configurações.

use iced::widget::{checkbox, column, pick_list, row, slider, text, toggler};
use iced::{Alignment, Element, Length};

use crate::core::{config::Settings, paths};
use crate::modules::settings::{Msg, State};
use crate::shared::i18n::{LanguageMode, TextKey};
use crate::shared::theme::Btn;
use crate::shared::types::Env;
use crate::shared::ui::{self, act};

pub fn view<'a>(state: &'a State, settings: &Settings, env: &Env) -> Element<'a, Msg> {
    let c = env.c();

    let header = ui::page_header(
        env,
        "⚙",
        env.t(TextKey::SettingsTitle),
        env.t(TextKey::SettingsDesc),
        None,
    );

    let lang_picker = pick_list(
        LanguageMode::ALL,
        Some(settings.language),
        Msg::LanguageChanged,
    )
    .padding([9, 12])
    .width(Length::Fill);

    let lang_field = ui::field(
        env,
        env.t(TextKey::LanguageLabel),
        env.t(TextKey::LanguageDesc),
        lang_picker,
    );

    let theme_toggle = toggler(
        Some(env.t(TextKey::DarkModeLabel).to_string()),
        settings.dark_mode,
        Msg::DarkModeChanged,
    )
    .text_size(14)
    .width(Length::Fill);

    let auto_open_chk = checkbox(
        env.t(TextKey::AutoOpenOutputLabel),
        settings.open_folder_after,
    )
    .on_toggle(Msg::OpenFolderAfterChanged)
    .size(18)
    .text_size(14);

    let hist_slider = slider(
        20..=500,
        settings.max_history as u32,
        Msg::MaxHistoryChanged,
    );

    let prefs_card = ui::section(
        env,
        "🎨",
        env.t(TextKey::AppearanceSection),
        if env.lang.is_en() {
            "Your preferences are stored automatically in the local config.json file."
        } else {
            "Suas escolhas são gravadas automaticamente no arquivo config.json local."
        },
        column![
            ui::list_row(lang_field),
            ui::list_row(theme_toggle),
            ui::list_row(auto_open_chk),
            ui::list_row(
                column![
                    row![
                        ui::strong(env.t(TextKey::HistoryLimitLabel), 14).width(Length::Fill),
                        ui::badge(format!("{} items", settings.max_history), c.primary),
                    ]
                    .align_items(Alignment::Center),
                    hist_slider,
                    ui::muted(env, env.t(TextKey::HistoryLimitDesc)),
                ]
                .spacing(8),
            ),
            ui::actions(
                env,
                vec![
                    act(
                        env.t(TextKey::OpenDataFolderBtn),
                        Btn::Secondary,
                        Some(Msg::OpenDataFolder)
                    ),
                    act(
                        env.t(TextKey::RestoreDefaultsBtn),
                        Btn::Ghost,
                        Some(Msg::ResetDefaults)
                    ),
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
        env.t(TextKey::PrivacySectionTitle),
        if env.lang.is_en() {
            "Your files and notes never leave your personal computer."
        } else {
            "Seus arquivos e notas nunca saem do seu computador."
        },
        column![
            text(env.t(TextKey::PrivacyNotice1)).size(13),
            text(env.t(TextKey::PrivacyNotice2)).size(13),
            text(env.t(TextKey::DataFolderLabel)).size(13),
            ui::list_row(ui::muted(env, data_dir)),
        ]
        .spacing(8),
    );

    ui::page(env, vec![header, ui::two_col(env, main_col.into(), side)])
}
