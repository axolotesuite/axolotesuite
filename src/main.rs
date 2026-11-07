#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
//! Ponto de entrada do Axolote Suite.

use iced::{Application, Font, Pixels, Settings};

fn main() -> iced::Result {
    axolote_suite::AxoloteSuite::run(Settings {
        window: iced::window::Settings {
            size: iced::Size::new(1380.0, 880.0),
            min_size: Some(iced::Size::new(560.0, 480.0)),
            icon: axolote_suite::shared::brand::window_icon(),
            ..Default::default()
        },
        default_font: Font::with_name("Segoe UI Symbol"),
        default_text_size: Pixels(14.0),
        antialiasing: true,
        ..Settings::default()
    })
}
