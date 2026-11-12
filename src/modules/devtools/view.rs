//! Interface visual do módulo "Estúdio de Cores & Gerador de Senhas".

use iced::widget::{checkbox, column, image as iced_image, row, slider, Space};
use iced::{Alignment, Color, Element, Length};

use crate::modules::devtools::{service, Msg, State, Tab};
use crate::shared::theme::{BannerKind, Btn};
use crate::shared::types::Env;
use crate::shared::ui;

pub fn view<'a>(state: &'a State, env: &Env) -> Element<'a, Msg> {
    let header = ui::page_header(
        env,
        "🎨",
        "Estúdio de Cores & Gerador de Senhas",
        "Ferramentas essenciais offline: Paleta e Harmonia de Cores, Extrator de Cores de Fotos, Gerador de Senhas com Entropia e UUID.",
        None,
    );

    let tabs = row![
        ui::btn(
            "🎨  Cores & Paleta de Foto",
            Btn::Tab(state.tab == Tab::Colors),
            Some(Msg::SelectTab(Tab::Colors))
        ),
        ui::btn(
            "🔒  Senhas Fortes & UUID",
            Btn::Tab(state.tab == Tab::Passwords),
            Some(Msg::SelectTab(Tab::Passwords))
        ),
    ]
    .spacing(10);

    let body = match state.tab {
        Tab::Colors => colors_view(state, env),
        Tab::Passwords => passwords_view(state, env),
    };

    let mut children = vec![header, tabs.into()];
    if let Some(fb) = &state.clipboard_feedback {
        children.push(ui::info_banner(env, BannerKind::Success, "✓", fb));
    }
    children.push(body);

    ui::page(env, children)
}

// ---------------------------------------------------------------------------
// Aba 1: Estúdio de Cores & Extrator de Paleta de Imagem
// ---------------------------------------------------------------------------

fn colors_view<'a>(state: &'a State, env: &Env) -> Element<'a, Msg> {
    let (h, s, l) = service::rgb_to_hsl(state.r, state.g, state.b);
    let (cy, mg, yl, k) = service::rgb_to_cmyk(state.r, state.g, state.b);
    let rgb_str = format!("rgb({}, {}, {})", state.r, state.g, state.b);
    let hsl_str = format!("hsl({:.0}°, {:.0}%, {:.0}%)", h, s, l);
    let cmyk_str = format!("cmyk({cy}%, {mg}%, {yl}%, {k}%)");

    let swatch_row = row![
        iced_image::Image::new(state.color_swatch.clone())
            .width(Length::Fixed(100.0))
            .height(Length::Fixed(100.0)),
        column![
            ui::strong(&state.color_hex, 22),
            ui::muted(env, &rgb_str),
            ui::muted(env, &hsl_str),
            ui::muted(env, &cmyk_str),
        ]
        .spacing(4)
        .width(Length::Fill),
    ]
    .spacing(16)
    .align_items(Alignment::Center);

    let hex_field = ui::field(
        env,
        "Código HEX",
        "Digite qualquer cor hexadecimal (#RRGGBB) ou ajuste os canais R, G, B abaixo.",
        row![
            ui::input("#7C3AED", &state.color_hex, Msg::HexChanged).width(Length::Fill),
            ui::btn(
                "📋 Copiar HEX",
                Btn::Secondary,
                Some(Msg::CopyText(state.color_hex.clone()))
            ),
            ui::btn(
                "📋 Copiar RGB",
                Btn::Secondary,
                Some(Msg::CopyText(rgb_str))
            ),
        ]
        .spacing(8)
        .align_items(Alignment::Center),
    );

    let sliders = column![
        row![
            ui::strong(format!("Vermelho (R): {}", state.r), 13).width(Length::Fixed(140.0)),
            slider(0.0..=255.0, state.r as f32, Msg::RChanged).width(Length::Fill),
        ]
        .spacing(10)
        .align_items(Alignment::Center),
        row![
            ui::strong(format!("Verde (G): {}", state.g), 13).width(Length::Fixed(140.0)),
            slider(0.0..=255.0, state.g as f32, Msg::GChanged).width(Length::Fill),
        ]
        .spacing(10)
        .align_items(Alignment::Center),
        row![
            ui::strong(format!("Azul (B): {}", state.b), 13).width(Length::Fixed(140.0)),
            slider(0.0..=255.0, state.b as f32, Msg::BChanged).width(Length::Fill),
        ]
        .spacing(10)
        .align_items(Alignment::Center),
    ]
    .spacing(10);

    let harmonies = service::color_harmonies(state.r, state.g, state.b);
    let mut harm_col = column![ui::strong(
        "Harmonias Cromáticas (clique para selecionar)",
        14
    )]
    .spacing(6);
    for (name, hex, (hr, hg, hb)) in harmonies {
        let chip_color = Color::from_rgb8(hr, hg, hb);
        harm_col = harm_col.push(ui::list_row(
            row![
                ui::badge("●", chip_color),
                ui::strong(name, 13).width(Length::Fill),
                ui::badge(hex.clone(), chip_color),
                ui::btn("Usar", Btn::Secondary, Some(Msg::SelectRgb(hr, hg, hb))),
                ui::btn("📋", Btn::Ghost, Some(Msg::CopyText(hex))),
            ]
            .spacing(8)
            .align_items(Alignment::Center),
        ));
    }

    let left = ui::section(
        env,
        "🎨",
        "Conversor & Harmonia de Cores ao Vivo",
        "Converte instantaneamente entre HEX, RGB, HSL e CMYK.",
        column![
            swatch_row,
            hex_field,
            sliders,
            Space::with_height(4),
            harm_col
        ]
        .spacing(14),
    );

    let mut pal_col = column![
        ui::muted(
            env,
            "Escolha qualquer foto ou logo do seu PC para extrair automaticamente as 6 cores dominantes:"
        ),
        ui::btn(
            "🖼  Escolher Imagem e Extrair Paleta Dominante…",
            Btn::Primary,
            Some(Msg::PickImageForPalette),
        ),
    ]
    .spacing(10);

    if !state.palette_source_path.is_empty() {
        pal_col = pal_col.push(ui::muted(
            env,
            format!("Imagem: {}", state.palette_source_path),
        ));
    }
    if state.extracted_palette.is_empty() {
        pal_col = pal_col.push(ui::empty_state(
            env,
            "🌈",
            "Nenhuma imagem analisada ainda",
            "Clique no botão acima para descobrir os códigos HEX exatos das cores de qualquer foto ou arte.",
        ));
    } else {
        for (hex, (pr, pg, pb), pct) in &state.extracted_palette {
            let col = Color::from_rgb8(*pr, *pg, *pb);
            pal_col = pal_col.push(ui::list_row(
                row![
                    ui::badge(format!("  {hex}  "), col),
                    ui::strong(format!("RGB({pr}, {pg}, {pb})"), 13).width(Length::Fill),
                    ui::badge(format!("{pct}% da foto"), env.c().accent),
                    ui::btn(
                        "Inspecionar",
                        Btn::Secondary,
                        Some(Msg::SelectRgb(*pr, *pg, *pb))
                    ),
                    ui::btn(
                        "📋 Copiar",
                        Btn::Secondary,
                        Some(Msg::CopyText(hex.clone()))
                    ),
                ]
                .spacing(8)
                .align_items(Alignment::Center),
            ));
        }
    }

    let right = column![
        ui::section(
            env,
            "🌈",
            "Extrator de Paleta de Cores de Imagem",
            "Analisa os pixels da imagem e encontra a paleta principal.",
            pal_col,
        ),
        ui::help(
            env,
            &[
                "Digite um código HEX (ex.: #7C3AED) ou arraste os controles R, G, B.",
                "Veja automaticamente as cores Complementares, Análogas e Tríades para seus designs.",
                "Ou clique em 'Escolher Imagem' para extrair as 6 cores dominantes de qualquer foto!",
            ],
            None,
        ),
    ]
    .spacing(16);

    ui::two_col(env, left, right.into())
}

// ---------------------------------------------------------------------------
// Aba 2: Gerador de Senhas Fortes, Entropia & UUID v4
// ---------------------------------------------------------------------------

fn passwords_view<'a>(state: &'a State, env: &Env) -> Element<'a, Msg> {
    let c = env.c();
    let rating_color = if state.pwd_entropy < 50.0 {
        c.warning
    } else {
        c.success
    };

    let controls = column![
        row![
            ui::strong(
                format!("Comprimento da senha: {} caracteres", state.pwd_len),
                14
            )
            .width(Length::Fill),
            ui::badge(
                format!(
                    "Entropia: {:.1} bits • {}",
                    state.pwd_entropy, state.pwd_rating
                ),
                rating_color
            ),
        ]
        .spacing(10)
        .align_items(Alignment::Center),
        slider(6.0..=64.0, state.pwd_len as f32, Msg::PwdLenChanged),
        ui::list_row(
            column![
                checkbox("Letras Maiúsculas (A–Z)", state.pwd_upper)
                    .on_toggle(Msg::TogglePwdUpper)
                    .size(18)
                    .text_size(13),
                checkbox("Letras Minúsculas (a–z)", state.pwd_lower)
                    .on_toggle(Msg::TogglePwdLower)
                    .size(18)
                    .text_size(13),
                checkbox("Números (0–9)", state.pwd_digits)
                    .on_toggle(Msg::TogglePwdDigits)
                    .size(18)
                    .text_size(13),
                checkbox("Símbolos Especiais (!@#$%^&*...)", state.pwd_symbols)
                    .on_toggle(Msg::TogglePwdSymbols)
                    .size(18)
                    .text_size(13),
                checkbox(
                    "Evitar caracteres parecidos (0, O, 1, l, I)",
                    state.pwd_avoid_ambiguous
                )
                .on_toggle(Msg::TogglePwdAmbiguous)
                .size(18)
                .text_size(13),
            ]
            .spacing(8),
        ),
        ui::btn(
            "🔄  Gerar Novas Senhas Seguras",
            Btn::Primary,
            Some(Msg::RegeneratePasswords)
        ),
    ]
    .spacing(12);

    let mut pwd_list = column![ui::strong("Senhas Geradas (clique em Copiar):", 14)].spacing(6);
    for pwd in &state.generated_passwords {
        pwd_list = pwd_list.push(ui::list_row(
            row![
                ui::strong(pwd, 14).width(Length::Fill),
                ui::btn(
                    "📋 Copiar",
                    Btn::Secondary,
                    Some(Msg::CopyText(pwd.clone()))
                ),
            ]
            .spacing(10)
            .align_items(Alignment::Center),
        ));
    }

    let left = ui::section(
        env,
        "🔒",
        "Gerador de Senhas com Medidor de Entropia",
        "Geração 100% local e offline — nenhuma senha sai do seu computador.",
        column![controls, Space::with_height(4), pwd_list].spacing(14),
    );

    let mut uuid_list = column![ui::btn(
        "🔄  Gerar Novos UUIDs v4 (RFC 4122)",
        Btn::Secondary,
        Some(Msg::RegenerateUuids)
    ),]
    .spacing(8);
    for u in &state.generated_uuids {
        uuid_list = uuid_list.push(ui::list_row(
            row![
                ui::strong(u, 13).width(Length::Fill),
                ui::btn("📋 Copiar", Btn::Secondary, Some(Msg::CopyText(u.clone()))),
            ]
            .spacing(8)
            .align_items(Alignment::Center),
        ));
    }

    let right = column![
        ui::section(
            env,
            "🆔",
            "Gerador de Identificadores Únicos (UUID v4)",
            "Ideal para bancos de dados, chaves de API, testes e projetos.",
            uuid_list,
        ),
        ui::help(
            env,
            &[
                "Ajuste o tamanho da senha (recomendado: 16 caracteres ou mais para >100 bits de entropia).",
                "Clique em 'Copiar' ao lado de qualquer senha ou UUID para enviá-lo direto ao seu Ctrl+V.",
            ],
            None,
        ),
    ]
    .spacing(16);

    ui::two_col(env, left, right.into())
}
