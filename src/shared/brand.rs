//! Logos e ícones embutidos no binário do Axolote Suite.

use iced::widget::image::Handle;

pub const LOGO_SMALL_PNG: &[u8] = include_bytes!("../../assets/logo_small.png");
pub const LOGO_BANNER_PNG: &[u8] = include_bytes!("../../assets/logo_banner.png");

pub fn logo_small() -> Handle {
    Handle::from_memory(LOGO_SMALL_PNG)
}

pub fn logo_banner() -> Handle {
    Handle::from_memory(LOGO_BANNER_PNG)
}

pub fn window_icon() -> Option<iced::window::Icon> {
    let img = image::load_from_memory(LOGO_SMALL_PNG).ok()?.into_rgba8();
    let (w, h) = img.dimensions();
    iced::window::icon::from_rgba(img.into_raw(), w, h).ok()
}
