//! Estado e mensagens do módulo "Estúdio de Cores & Gerador de Senhas".

pub mod service;
pub mod view;

use iced::widget::image as iced_image;
use iced::Command;

use crate::core::{system, Ctx};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tab {
    Colors,
    Passwords,
}

pub struct State {
    pub tab: Tab,
    pub clipboard_feedback: Option<String>,

    // 1. Estúdio de Cores & Extrator de Paleta
    pub color_hex: String,
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub color_swatch: iced_image::Handle,
    pub palette_source_path: String,
    pub extracted_palette: Vec<(String, (u8, u8, u8), u32)>,

    // 2. Gerador de Senhas & UUIDs
    pub pwd_len: u8,
    pub pwd_upper: bool,
    pub pwd_lower: bool,
    pub pwd_digits: bool,
    pub pwd_symbols: bool,
    pub pwd_avoid_ambiguous: bool,
    pub generated_passwords: Vec<String>,
    pub pwd_entropy: f32,
    pub pwd_rating: &'static str,
    pub generated_uuids: Vec<String>,
}

#[derive(Debug, Clone)]
pub enum Msg {
    SelectTab(Tab),
    CopyText(String),

    // Cores
    HexChanged(String),
    RChanged(f32),
    GChanged(f32),
    BChanged(f32),
    SelectRgb(u8, u8, u8),
    PickImageForPalette,

    // Senhas & UUID
    PwdLenChanged(f32),
    TogglePwdUpper(bool),
    TogglePwdLower(bool),
    TogglePwdDigits(bool),
    TogglePwdSymbols(bool),
    TogglePwdAmbiguous(bool),
    RegeneratePasswords,
    RegenerateUuids,
}

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}

impl State {
    pub fn new() -> Self {
        let r = 124;
        let g = 58;
        let b = 237;
        let swatch = iced_image::Handle::from_pixels(110, 110, service::render_color_swatch(r, g, b, 110, 110));
        let (pwds, entropy, rating) = service::generate_passwords(20, true, true, true, true, true, 5);
        let uuids = service::generate_uuids(5);

        Self {
            tab: Tab::Colors,
            clipboard_feedback: None,
            color_hex: "#7C3AED".into(),
            r,
            g,
            b,
            color_swatch: swatch,
            palette_source_path: String::new(),
            extracted_palette: Vec::new(),
            pwd_len: 20,
            pwd_upper: true,
            pwd_lower: true,
            pwd_digits: true,
            pwd_symbols: true,
            pwd_avoid_ambiguous: true,
            generated_passwords: pwds,
            pwd_entropy: entropy,
            pwd_rating: rating,
            generated_uuids: uuids,
        }
    }

    fn refresh_color_swatch(&mut self) {
        self.color_swatch = iced_image::Handle::from_pixels(
            110,
            110,
            service::render_color_swatch(self.r, self.g, self.b, 110, 110),
        );
    }

    fn refresh_passwords(&mut self) {
        let (pwds, entropy, rating) = service::generate_passwords(
            self.pwd_len as usize,
            self.pwd_upper,
            self.pwd_lower,
            self.pwd_digits,
            self.pwd_symbols,
            self.pwd_avoid_ambiguous,
            5,
        );
        self.generated_passwords = pwds;
        self.pwd_entropy = entropy;
        self.pwd_rating = rating;
    }

    pub fn update(&mut self, msg: Msg, _ctx: &mut Ctx) -> Command<Msg> {
        match msg {
            Msg::SelectTab(t) => {
                self.tab = t;
                self.clipboard_feedback = None;
            }
            Msg::CopyText(s) => {
                if service::copy_to_clipboard(&s) {
                    self.clipboard_feedback = Some(format!("✓ Copiado para a área de transferência: {s}"));
                }
            }
            Msg::HexChanged(hex) => {
                if let Some((r, g, b)) = service::parse_hex_color(&hex) {
                    self.r = r;
                    self.g = g;
                    self.b = b;
                    self.refresh_color_swatch();
                }
                self.color_hex = hex;
            }
            Msg::RChanged(v) => {
                self.r = v.round().clamp(0.0, 255.0) as u8;
                self.color_hex = service::rgb_to_hex(self.r, self.g, self.b);
                self.refresh_color_swatch();
            }
            Msg::GChanged(v) => {
                self.g = v.round().clamp(0.0, 255.0) as u8;
                self.color_hex = service::rgb_to_hex(self.r, self.g, self.b);
                self.refresh_color_swatch();
            }
            Msg::BChanged(v) => {
                self.b = v.round().clamp(0.0, 255.0) as u8;
                self.color_hex = service::rgb_to_hex(self.r, self.g, self.b);
                self.refresh_color_swatch();
            }
            Msg::SelectRgb(r, g, b) => {
                self.r = r;
                self.g = g;
                self.b = b;
                self.color_hex = service::rgb_to_hex(r, g, b);
                self.refresh_color_swatch();
            }
            Msg::PickImageForPalette => {
                if let Some(p) = system::pick_file(
                    "Escolha uma imagem para extrair a paleta de cores dominantes",
                    "Imagens",
                    &["png", "jpg", "jpeg", "webp", "bmp", "gif"],
                ) {
                    self.palette_source_path = p.display().to_string();
                    if let Ok(pal) = service::extract_image_palette(&p) {
                        if let Some((_, (r, g, b), _)) = pal.first() {
                            self.r = *r;
                            self.g = *g;
                            self.b = *b;
                            self.color_hex = service::rgb_to_hex(*r, *g, *b);
                            self.refresh_color_swatch();
                        }
                        self.extracted_palette = pal;
                    }
                }
            }
            Msg::PwdLenChanged(v) => {
                self.pwd_len = v.round().clamp(6.0, 64.0) as u8;
                self.refresh_passwords();
            }
            Msg::TogglePwdUpper(b) => {
                self.pwd_upper = b;
                self.refresh_passwords();
            }
            Msg::TogglePwdLower(b) => {
                self.pwd_lower = b;
                self.refresh_passwords();
            }
            Msg::TogglePwdDigits(b) => {
                self.pwd_digits = b;
                self.refresh_passwords();
            }
            Msg::TogglePwdSymbols(b) => {
                self.pwd_symbols = b;
                self.refresh_passwords();
            }
            Msg::TogglePwdAmbiguous(b) => {
                self.pwd_avoid_ambiguous = b;
                self.refresh_passwords();
            }
            Msg::RegeneratePasswords => self.refresh_passwords(),
            Msg::RegenerateUuids => {
                self.generated_uuids = service::generate_uuids(5);
            }
        }
        Command::none()
    }
}
