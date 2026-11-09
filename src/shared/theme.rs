//! Tema visual do Axolote Suite.
//!
//! Identidade visual: fundo escuro (ou claro), destaques em roxo (#7c3aed) e
//! azul neon (#3b82f6). Aqui ficam a paleta de cores e as folhas de estilo
//! (botões, campos, cartões) usadas por toda a interface.

use iced::widget::{button, container, text_input};
use iced::{theme, Background, Border, Color, Shadow, Theme, Vector};

/// Paleta de cores usada pelas telas.
#[derive(Debug, Clone, Copy)]
pub struct Colors {
    pub bg: Color,
    pub sidebar: Color,
    pub surface: Color,
    pub surface2: Color,
    pub border: Color,
    pub text: Color,
    pub muted: Color,
    pub primary: Color,
    pub primary_hover: Color,
    pub primary_soft: Color,
    pub accent: Color,
    pub success: Color,
    pub danger: Color,
    pub warning: Color,
}

pub fn rgb(hex: u32) -> Color {
    Color::from_rgb8(((hex >> 16) & 0xff) as u8, ((hex >> 8) & 0xff) as u8, (hex & 0xff) as u8)
}

pub fn with_alpha(c: Color, a: f32) -> Color {
    Color { a, ..c }
}

pub fn dark() -> Colors {
    Colors {
        bg: rgb(0x13131a),
        sidebar: rgb(0x13131f),
        surface: rgb(0x13131f),
        surface2: rgb(0x1f1f26),
        border: rgb(0x242438),
        text: rgb(0xf8fafc),
        muted: rgb(0x8b93a1),
        primary: rgb(0x7c3aed),
        primary_hover: rgb(0x8b5cf6),
        primary_soft: with_alpha(rgb(0x7c3aed), 0.16),
        accent: rgb(0x4cd7f6),
        success: rgb(0x10b981),
        danger: rgb(0xef4444),
        warning: rgb(0xf59e0b),
    }
}

pub fn light() -> Colors {
    Colors {
        bg: rgb(0xf4f4f9),
        sidebar: rgb(0xffffff),
        surface: rgb(0xffffff),
        surface2: rgb(0xefeff7),
        border: rgb(0xdcdce9),
        text: rgb(0x1a1a2e),
        muted: rgb(0x5e5e78),
        primary: rgb(0x7c3aed),
        primary_hover: rgb(0x6d28d9),
        primary_soft: with_alpha(rgb(0x7c3aed), 0.10),
        accent: rgb(0x2563eb),
        success: rgb(0x16a34a),
        danger: rgb(0xdc2626),
        warning: rgb(0xd97706),
    }
}

/// Descobre se o tema atual é escuro observando a cor de fundo.
pub fn is_dark(theme: &Theme) -> bool {
    let b = theme.palette().background;
    (b.r + b.g + b.b) / 3.0 < 0.5
}

pub fn colors(theme: &Theme) -> Colors {
    if is_dark(theme) {
        dark()
    } else {
        light()
    }
}

/// Cria o tema do Iced com as cores da marca.
pub fn app_theme(dark_mode: bool) -> Theme {
    let c = if dark_mode { dark() } else { light() };
    Theme::custom(
        if dark_mode { "Axolote Escuro" } else { "Axolote Claro" }.to_string(),
        theme::Palette {
            background: c.bg,
            text: c.text,
            primary: c.primary,
            success: c.success,
            danger: c.danger,
        },
    )
}

// ---------------------------------------------------------------------------
// Containers
// ---------------------------------------------------------------------------

fn radius(r: f32) -> iced::border::Radius {
    r.into()
}

/// Cartão padrão (superfície com borda suave e sombra).
pub fn card(theme: &Theme) -> container::Appearance {
    let c = colors(theme);
    container::Appearance {
        text_color: None,
        background: Some(Background::Color(c.surface)),
        border: Border { color: c.border, width: 1.0, radius: radius(12.0) },
        shadow: Shadow {
            color: Color { a: if is_dark(theme) { 0.40 } else { 0.06 }, ..Color::BLACK },
            offset: Vector::new(0.0, 3.0),
            blur_radius: 12.0,
        },
    }
}

/// Cartão de destaque estilo Bento Grid (superfície Cyber-Fluent).
pub fn bento_card(theme: &Theme) -> container::Appearance {
    let c = colors(theme);
    container::Appearance {
        text_color: None,
        background: Some(Background::Color(c.surface)),
        border: Border {
            color: c.border,
            width: 1.0,
            radius: radius(12.0),
        },
        shadow: Shadow {
            color: Color { a: if is_dark(theme) { 0.45 } else { 0.08 }, ..Color::BLACK },
            offset: Vector::new(0.0, 4.0),
            blur_radius: 16.0,
        },
    }
}

/// Caixa interna para cards (fundo ultra escuro `#0E0E15` do Stitch).
pub fn inner_box(theme: &Theme) -> container::Appearance {
    let c = colors(theme);
    container::Appearance {
        text_color: None,
        background: Some(Background::Color(if is_dark(theme) { rgb(0x0e0e15) } else { rgb(0xf1f1f8) })),
        border: Border {
            color: with_alpha(c.border, 0.70),
            width: 1.0,
            radius: radius(8.0),
        },
        shadow: Shadow::default(),
    }
}

/// Área interativa de Drag & Drop do Stitch (fundo `#0E0E15` com borda sutil).
pub fn dropzone_box(theme: &Theme) -> container::Appearance {
    let c = colors(theme);
    container::Appearance {
        text_color: None,
        background: Some(Background::Color(if is_dark(theme) { rgb(0x0e0e15) } else { rgb(0xf5f5fb) })),
        border: Border {
            color: with_alpha(c.primary, if is_dark(theme) { 0.35 } else { 0.20 }),
            width: 1.0,
            radius: radius(12.0),
        },
        shadow: Shadow {
            color: with_alpha(c.primary, if is_dark(theme) { 0.15 } else { 0.05 }),
            offset: Vector::new(0.0, 2.0),
            blur_radius: 8.0,
        },
    }
}

/// Cartão Bento Hero com gradiente/realce cibernético roxo e ciano.
pub fn bento_hero(theme: &Theme) -> container::Appearance {
    let c = colors(theme);
    container::Appearance {
        text_color: None,
        background: Some(Background::Color(if is_dark(theme) {
            with_alpha(c.surface, 0.95)
        } else {
            c.surface
        })),
        border: Border {
            color: with_alpha(c.primary, if is_dark(theme) { 0.60 } else { 0.40 }),
            width: 1.5,
            radius: radius(14.0),
        },
        shadow: Shadow {
            color: with_alpha(c.primary, if is_dark(theme) { 0.35 } else { 0.12 }),
            offset: Vector::new(0.0, 4.0),
            blur_radius: 20.0,
        },
    }
}

/// Container com brilho sutil para acomodar o Logo Oficial do Axolote.
pub fn logo_container(theme: &Theme) -> container::Appearance {
    let c = colors(theme);
    container::Appearance {
        text_color: None,
        background: Some(Background::Color(with_alpha(c.primary_soft, 0.40))),
        border: Border {
            color: with_alpha(c.primary, 0.35),
            width: 1.0,
            radius: radius(12.0),
        },
        shadow: Shadow {
            color: with_alpha(c.primary, if is_dark(theme) { 0.30 } else { 0.10 }),
            offset: Vector::new(0.0, 2.0),
            blur_radius: 10.0,
        },
    }
}

/// Cartão de destaque (gradiente simulado com roxo suave).
pub fn hero(theme: &Theme) -> container::Appearance {
    let c = colors(theme);
    container::Appearance {
        text_color: None,
        background: Some(Background::Color(c.primary_soft)),
        border: Border { color: with_alpha(c.primary, 0.45), width: 1.0, radius: radius(16.0) },
        shadow: Shadow::default(),
    }
}

/// Caixa de ajuda ("Como usar").
pub fn help_box(theme: &Theme) -> container::Appearance {
    let c = colors(theme);
    container::Appearance {
        text_color: None,
        background: Some(Background::Color(with_alpha(c.accent, 0.08))),
        border: Border { color: with_alpha(c.accent, 0.35), width: 1.0, radius: radius(14.0) },
        shadow: Shadow::default(),
    }
}

pub fn sidebar(theme: &Theme) -> container::Appearance {
    let c = colors(theme);
    container::Appearance {
        text_color: None,
        background: Some(Background::Color(c.sidebar)),
        border: Border { color: c.border, width: 1.0, radius: radius(0.0) },
        shadow: Shadow::default(),
    }
}

pub fn app_bg(theme: &Theme) -> container::Appearance {
    let c = colors(theme);
    container::Appearance {
        text_color: Some(c.text),
        background: Some(Background::Color(c.bg)),
        ..Default::default()
    }
}

/// Linha de lista (ex.: item do histórico, prévia de renomeação).
pub fn list_row(theme: &Theme) -> container::Appearance {
    let c = colors(theme);
    container::Appearance {
        text_color: None,
        background: Some(Background::Color(c.surface2)),
        border: Border { color: Color::TRANSPARENT, width: 0.0, radius: radius(10.0) },
        shadow: Shadow::default(),
    }
}

/// Faixa de resultado (sucesso / erro / informação).
pub fn banner(kind: BannerKind) -> impl Fn(&Theme) -> container::Appearance {
    move |theme: &Theme| {
        let c = colors(theme);
        let base = match kind {
            BannerKind::Success => c.success,
            BannerKind::Error => c.danger,
            BannerKind::Info => c.accent,
            BannerKind::Warning => c.warning,
        };
        container::Appearance {
            text_color: None,
            background: Some(Background::Color(with_alpha(base, 0.12))),
            border: Border { color: with_alpha(base, 0.55), width: 1.0, radius: radius(12.0) },
            shadow: Shadow::default(),
        }
    }
}

/// Pequena etiqueta colorida.
pub fn badge(color: Color) -> impl Fn(&Theme) -> container::Appearance {
    move |_theme: &Theme| container::Appearance {
        text_color: Some(color),
        background: Some(Background::Color(with_alpha(color, 0.15))),
        border: Border { color: with_alpha(color, 0.4), width: 1.0, radius: radius(999.0) },
        shadow: Shadow::default(),
    }
}

/// Ícone dentro de um quadrado arredondado colorido.
pub fn icon_tile(color: Color) -> impl Fn(&Theme) -> container::Appearance {
    move |_theme: &Theme| container::Appearance {
        text_color: None,
        background: Some(Background::Color(with_alpha(color, 0.16))),
        border: Border { color: Color::TRANSPARENT, width: 0.0, radius: radius(12.0) },
        shadow: Shadow::default(),
    }
}

/// Bloco com cor sólida e raio de borda suave.
pub fn solid_box(color: Color, radius_px: f32) -> impl Fn(&Theme) -> container::Appearance {
    move |_theme: &Theme| container::Appearance {
        text_color: None,
        background: Some(Background::Color(color)),
        border: Border { color: Color::TRANSPARENT, width: 0.0, radius: radius(radius_px) },
        shadow: Shadow::default(),
    }
}

/// Barra Neon vertical animada para o item ativo da barra lateral.
pub fn active_pill(pulse: f32) -> impl Fn(&Theme) -> container::Appearance {
    move |theme: &Theme| {
        let c = colors(theme);
        container::Appearance {
            text_color: None,
            background: Some(Background::Color(c.accent)),
            border: Border { color: c.primary_hover, width: 0.0, radius: radius(999.0) },
            shadow: Shadow {
                color: with_alpha(c.accent, 0.55 + 0.35 * pulse),
                offset: Vector::new(0.0, 0.0),
                blur_radius: 6.0 + 4.0 * pulse,
            },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BannerKind {
    Success,
    Error,
    Info,
    Warning,
}

// ---------------------------------------------------------------------------
// Botões
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Btn {
    /// Ação principal (roxo).
    Primary,
    /// Ação secundária (contorno).
    Secondary,
    /// Ação destrutiva (vermelho).
    Danger,
    /// Botão discreto, sem fundo.
    Ghost,
    /// Item da barra lateral.
    Nav(bool),
    /// Aba (selecionada ou não).
    Tab(bool),
    /// Cartão clicável do painel inicial.
    Tile,
    /// Tecla da calculadora.
    Key,
    /// Tecla de operador da calculadora.
    KeyOp,
}

pub struct ButtonStyle(pub Btn);

impl button::StyleSheet for ButtonStyle {
    type Style = Theme;

    fn active(&self, theme: &Theme) -> button::Appearance {
        let c = colors(theme);
        let r = radius(10.0);
        match self.0 {
            Btn::Primary => button::Appearance {
                background: Some(Background::Color(c.primary)),
                text_color: Color::WHITE,
                border: Border { color: c.primary, width: 1.0, radius: r },
                shadow: Shadow {
                    color: with_alpha(c.primary, 0.35),
                    offset: Vector::new(0.0, 2.0),
                    blur_radius: 8.0,
                },
                shadow_offset: Vector::default(),
            },
            Btn::Secondary => button::Appearance {
                background: Some(Background::Color(c.surface2)),
                text_color: c.text,
                border: Border { color: c.border, width: 1.0, radius: r },
                ..Default::default()
            },
            Btn::Danger => button::Appearance {
                background: Some(Background::Color(with_alpha(c.danger, 0.12))),
                text_color: c.danger,
                border: Border { color: with_alpha(c.danger, 0.5), width: 1.0, radius: r },
                ..Default::default()
            },
            Btn::Ghost => button::Appearance {
                background: None,
                text_color: c.muted,
                border: Border { color: Color::TRANSPARENT, width: 0.0, radius: r },
                ..Default::default()
            },
            Btn::Nav(active) => button::Appearance {
                background: if active {
                    Some(Background::Color(c.primary))
                } else {
                    None
                },
                text_color: if active { Color::WHITE } else { c.muted },
                border: Border {
                    color: if active { c.primary } else { Color::TRANSPARENT },
                    width: 1.0,
                    radius: radius(6.0),
                },
                shadow: if active {
                    Shadow {
                        color: with_alpha(c.primary, 0.45),
                        offset: Vector::new(0.0, 2.0),
                        blur_radius: 12.0,
                    }
                } else {
                    Shadow::default()
                },
                shadow_offset: Vector::default(),
            },
            Btn::Tab(active) => button::Appearance {
                background: Some(Background::Color(if active { c.primary } else { c.surface2 })),
                text_color: if active { Color::WHITE } else { c.muted },
                border: Border {
                    color: if active { c.primary } else { c.border },
                    width: 1.0,
                    radius: radius(999.0),
                },
                ..Default::default()
            },
            Btn::Tile => button::Appearance {
                background: Some(Background::Color(c.surface)),
                text_color: c.text,
                border: Border { color: c.border, width: 1.0, radius: radius(14.0) },
                shadow: Shadow {
                    color: Color { a: if is_dark(theme) { 0.3 } else { 0.05 }, ..Color::BLACK },
                    offset: Vector::new(0.0, 2.0),
                    blur_radius: 8.0,
                },
                ..Default::default()
            },
            Btn::Key => button::Appearance {
                background: Some(Background::Color(c.surface2)),
                text_color: c.text,
                border: Border { color: c.border, width: 1.0, radius: r },
                ..Default::default()
            },
            Btn::KeyOp => button::Appearance {
                background: Some(Background::Color(c.primary_soft)),
                text_color: c.primary_hover,
                border: Border { color: with_alpha(c.primary, 0.4), width: 1.0, radius: r },
                ..Default::default()
            },
        }
    }

    fn hovered(&self, theme: &Theme) -> button::Appearance {
        let c = colors(theme);
        let base = self.active(theme);
        match self.0 {
            Btn::Primary => button::Appearance {
                background: Some(Background::Color(c.primary_hover)),
                border: Border { color: c.primary_hover, ..base.border },
                ..base
            },
            Btn::Secondary | Btn::Key => button::Appearance {
                border: Border { color: c.primary, ..base.border },
                ..base
            },
            Btn::Danger => button::Appearance {
                background: Some(Background::Color(with_alpha(c.danger, 0.22))),
                ..base
            },
            Btn::Ghost => button::Appearance {
                background: Some(Background::Color(c.surface2)),
                text_color: c.text,
                ..base
            },
            Btn::Nav(active) => button::Appearance {
                background: Some(Background::Color(if active {
                    with_alpha(c.primary, 0.38)
                } else {
                    with_alpha(c.primary, 0.14)
                })),
                text_color: Color::WHITE,
                border: Border {
                    color: if active { c.accent } else { with_alpha(c.primary, 0.55) },
                    width: 1.5,
                    ..base.border
                },
                shadow: Shadow {
                    color: with_alpha(c.primary, if active { 0.55 } else { 0.28 }),
                    offset: Vector::new(2.0, 2.0),
                    blur_radius: 10.0,
                },
                shadow_offset: Vector::new(3.0, 0.0),
            },
            Btn::Tab(active) => button::Appearance {
                text_color: if active { Color::WHITE } else { c.text },
                border: Border { color: c.primary, ..base.border },
                ..base
            },
            Btn::Tile => button::Appearance {
                border: Border { color: c.primary, ..base.border },
                shadow: Shadow {
                    color: with_alpha(c.primary, 0.25),
                    offset: Vector::new(0.0, 4.0),
                    blur_radius: 14.0,
                },
                ..base
            },
            Btn::KeyOp => button::Appearance {
                background: Some(Background::Color(with_alpha(c.primary, 0.3))),
                ..base
            },
        }
    }

    fn pressed(&self, theme: &Theme) -> button::Appearance {
        let base = self.hovered(theme);
        button::Appearance { shadow: Shadow::default(), ..base }
    }

    fn disabled(&self, theme: &Theme) -> button::Appearance {
        let c = colors(theme);
        let base = self.active(theme);
        button::Appearance {
            background: base.background.map(|bg| match bg {
                Background::Color(col) => Background::Color(with_alpha(col, col.a * 0.4)),
                other => other,
            }),
            text_color: with_alpha(base.text_color, 0.45),
            border: Border { color: with_alpha(c.border, 0.5), ..base.border },
            shadow: Shadow::default(),
            ..base
        }
    }
}

pub fn btn(kind: Btn) -> theme::Button {
    theme::Button::custom(ButtonStyle(kind))
}

// ---------------------------------------------------------------------------
// Campos de texto
// ---------------------------------------------------------------------------

pub struct InputStyle;

impl text_input::StyleSheet for InputStyle {
    type Style = Theme;

    fn active(&self, theme: &Theme) -> text_input::Appearance {
        let c = colors(theme);
        text_input::Appearance {
            background: Background::Color(c.surface2),
            border: Border { color: c.border, width: 1.0, radius: radius(10.0) },
            icon_color: c.muted,
        }
    }

    fn focused(&self, theme: &Theme) -> text_input::Appearance {
        let c = colors(theme);
        text_input::Appearance {
            border: Border { color: c.primary, width: 1.5, radius: radius(10.0) },
            ..self.active(theme)
        }
    }

    fn hovered(&self, theme: &Theme) -> text_input::Appearance {
        let c = colors(theme);
        text_input::Appearance {
            border: Border { color: with_alpha(c.primary, 0.6), width: 1.0, radius: radius(10.0) },
            ..self.active(theme)
        }
    }

    fn placeholder_color(&self, theme: &Theme) -> Color {
        with_alpha(colors(theme).muted, 0.8)
    }

    fn value_color(&self, theme: &Theme) -> Color {
        colors(theme).text
    }

    fn disabled_color(&self, theme: &Theme) -> Color {
        colors(theme).muted
    }

    fn selection_color(&self, theme: &Theme) -> Color {
        with_alpha(colors(theme).primary, 0.4)
    }

    fn disabled(&self, theme: &Theme) -> text_input::Appearance {
        let c = colors(theme);
        text_input::Appearance {
            background: Background::Color(with_alpha(c.surface2, 0.5)),
            ..self.active(theme)
        }
    }
}

pub fn input() -> theme::TextInput {
    theme::TextInput::Custom(Box::new(InputStyle))
}
