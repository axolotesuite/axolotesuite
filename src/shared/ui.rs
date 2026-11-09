//! Componentes visuais reutilizáveis (cartões, campos, botões, grades).
//!
//! Todas as telas usam estes blocos para manter a interface consistente,
//! bonita e responsiva.

use iced::font::{Family, Weight};
use iced::widget::text::Shaping;
use iced::widget::{button, column, container, horizontal_space, row, scrollable, text, text_input, Space};
use iced::{Alignment, Color, Element, Font, Length};

use crate::shared::theme::{self, BannerKind, Btn};
use crate::shared::types::{Env, OperationResult};

pub const BOLD: Font = Font {
    family: Family::Name("Segoe UI Symbol"),
    weight: Weight::Bold,
    ..Font::DEFAULT
};
pub const SEMIBOLD: Font = Font {
    family: Family::Name("Segoe UI Symbol"),
    weight: Weight::Semibold,
    ..Font::DEFAULT
};

/// Texto em negrito.
pub fn strong<'a>(content: impl ToString, size: u16) -> iced::widget::Text<'a> {
    text(content).size(size).font(BOLD).shaping(Shaping::Advanced)
}

/// Texto secundário (cinza).
pub fn muted<'a>(env: &Env, content: impl ToString) -> iced::widget::Text<'a> {
    text(content).size(13).style(env.c().muted).shaping(Shaping::Advanced)
}

/// Texto colorido.
pub fn colored<'a>(content: impl ToString, size: u16, color: Color) -> iced::widget::Text<'a> {
    text(content).size(size).style(color).shaping(Shaping::Advanced)
}

/// Botão estilizado. Se `msg` for `None`, o botão aparece desativado.
pub fn btn<'a, M: Clone + 'a>(label: impl ToString, kind: Btn, msg: Option<M>) -> iced::widget::Button<'a, M> {
    let size = match kind {
        Btn::Key | Btn::KeyOp => 18,
        _ => 14,
    };
    button(
        container(text(label).size(size).font(SEMIBOLD).shaping(Shaping::Advanced))
            .center_x()
            .width(Length::Shrink),
    )
    .padding([10, 18])
    .style(theme::btn(kind))
    .on_press_maybe(msg)
}

/// Linha de botões de ação. Em telas estreitas os botões ficam empilhados
/// e ocupam toda a largura, facilitando o clique.
pub fn actions<'a, M: Clone + 'a>(env: &Env, items: Vec<(String, Btn, Option<M>)>) -> Element<'a, M> {
    if env.narrow() {
        let mut col = column![].spacing(8).width(Length::Fill);
        for (label, kind, msg) in items {
            col = col.push(
                button(container(text(label).size(14).font(SEMIBOLD).shaping(Shaping::Advanced)).center_x().width(Length::Fill))
                    .padding([11, 18])
                    .width(Length::Fill)
                    .style(theme::btn(kind))
                    .on_press_maybe(msg),
            );
        }
        col.into()
    } else {
        let mut r = row![].spacing(10).align_items(Alignment::Center);
        for (label, kind, msg) in items {
            r = r.push(btn(label, kind, msg));
        }
        r.into()
    }
}

/// Atalho para montar um item de `actions`.
pub fn act<M>(label: impl ToString, kind: Btn, msg: Option<M>) -> (String, Btn, Option<M>) {
    (label.to_string(), kind, msg)
}

/// Cartão com cabeçalho (ícone + título + subtítulo) e conteúdo.
pub fn section<'a, M: 'a>(
    env: &Env,
    icon: &str,
    title: &str,
    subtitle: &str,
    body: impl Into<Element<'a, M>>,
) -> Element<'a, M> {
    let c = env.c();
    let mut header = row![
        container(text(icon).size(18).shaping(Shaping::Advanced)).padding([6, 9]).style(theme::icon_tile(c.primary)),
        column![strong(title, 17), muted(env, subtitle)].spacing(2).width(Length::Fill),
    ]
    .spacing(12)
    .align_items(Alignment::Center);
    if subtitle.is_empty() {
        header = row![
            container(text(icon).size(18).shaping(Shaping::Advanced)).padding([6, 9]).style(theme::icon_tile(c.primary)),
            strong(title, 17),
        ]
        .spacing(12)
        .align_items(Alignment::Center);
    }
    container(column![header, body.into()].spacing(16))
        .padding(if env.narrow() { 16 } else { 22 })
        .width(Length::Fill)
        .style(theme::card)
        .into()
}

/// Cartão simples, sem cabeçalho.
pub fn card<'a, M: 'a>(env: &Env, body: impl Into<Element<'a, M>>) -> Element<'a, M> {
    container(body)
        .padding(if env.narrow() { 16 } else { 22 })
        .width(Length::Fill)
        .style(theme::card)
        .into()
}

/// Cartão estilo Bento Grid (superfície Cyber-Fluent com borda refinada).
pub fn bento_card<'a, M: 'a>(env: &Env, body: impl Into<Element<'a, M>>) -> Element<'a, M> {
    container(body)
        .padding(if env.narrow() { 16 } else { 20 })
        .width(Length::Fill)
        .style(theme::bento_card)
        .into()
}

/// Cartão Bento Hero com realce cibernético roxo e ciano.
pub fn bento_hero<'a, M: 'a>(env: &Env, body: impl Into<Element<'a, M>>) -> Element<'a, M> {
    container(body)
        .padding(if env.narrow() { 18 } else { 24 })
        .width(Length::Fill)
        .style(theme::bento_hero)
        .into()
}

/// Caixa explicativa "Como usar" com passos numerados e uma dica opcional.
pub fn help<'a, M: 'a>(env: &Env, steps: &[&str], tip: Option<&str>) -> Element<'a, M> {
    let c = env.c();
    let mut col = column![row![text("💡").size(16).shaping(Shaping::Advanced), strong("Como usar", 15)].spacing(8).align_items(Alignment::Center)]
        .spacing(10);
    for (i, step) in steps.iter().enumerate() {
        col = col.push(
            row![
                container(colored(i + 1, 12, Color::WHITE).font(BOLD))
                    .width(Length::Fixed(22.0))
                    .height(Length::Fixed(22.0))
                    .center_x()
                    .center_y()
                    .style(theme::badge(c.accent)),
                text(*step).size(13).shaping(Shaping::Advanced).width(Length::Fill),
            ]
            .spacing(10)
            .align_items(Alignment::Start),
        );
    }
    if let Some(tip) = tip {
        col = col.push(
            container(text(format!("Dica: {tip}")).size(12).style(c.muted).shaping(Shaping::Advanced))
                .padding([8, 0, 0, 0]),
        );
    }
    container(col).padding(18).width(Length::Fill).style(theme::help_box).into()
}

/// Rótulo + controle + dica abaixo.
pub fn field<'a, M: 'a>(env: &Env, label: &str, hint: &str, control: impl Into<Element<'a, M>>) -> Element<'a, M> {
    let mut col = column![text(label).size(13).font(SEMIBOLD).shaping(Shaping::Advanced), control.into()].spacing(6);
    if !hint.is_empty() {
        col = col.push(muted(env, hint).size(12));
    }
    col.width(Length::Fill).into()
}

/// Campo de texto estilizado.
pub fn input<'a, M: Clone + 'a>(
    placeholder: &str,
    value: &str,
    on_change: impl Fn(String) -> M + 'a,
) -> iced::widget::TextInput<'a, M> {
    text_input(placeholder, value)
        .on_input(on_change)
        .padding([10, 12])
        .size(14)
        .style(theme::input())
}

/// Campo de caminho com botão "Procurar…". Fica empilhado em telas estreitas.
pub fn path_field<'a, M: Clone + 'a>(
    env: &Env,
    placeholder: &str,
    value: &str,
    on_change: impl Fn(String) -> M + 'a,
    browse_label: &str,
    on_browse: M,
) -> Element<'a, M> {
    let inp = input(placeholder, value, on_change).width(Length::Fill);
    if env.narrow() {
        column![
            inp,
            button(container(text(browse_label).size(14).font(SEMIBOLD).shaping(Shaping::Advanced)).center_x().width(Length::Fill))
                .padding([10, 16])
                .width(Length::Fill)
                .style(theme::btn(Btn::Secondary))
                .on_press(on_browse)
        ]
        .spacing(8)
        .into()
    } else {
        row![inp, btn(browse_label, Btn::Secondary, Some(on_browse))]
            .spacing(8)
            .align_items(Alignment::Center)
            .into()
    }
}

/// Grade responsiva: distribui os itens em `cols` colunas de largura igual.
pub fn grid<'a, M: 'a>(items: Vec<Element<'a, M>>, cols: usize, spacing: u16) -> Element<'a, M> {
    let cols = cols.max(1);
    let mut out = column![].spacing(spacing).width(Length::Fill);
    let mut iter = items.into_iter().peekable();
    while iter.peek().is_some() {
        let mut r = row![].spacing(spacing).width(Length::Fill);
        for _ in 0..cols {
            match iter.next() {
                Some(item) => r = r.push(container(item).width(Length::FillPortion(1))),
                None => r = r.push(Space::with_width(Length::FillPortion(1))),
            }
        }
        out = out.push(r);
    }
    out.into()
}

/// Duas colunas lado a lado em telas largas; uma embaixo da outra nas demais.
/// Em telas estreitas a coluna `side` (ajuda) aparece primeiro.
pub fn two_col<'a, M: 'a>(env: &Env, main: Element<'a, M>, side: Element<'a, M>) -> Element<'a, M> {
    if env.wide() {
        row![
            container(main).width(Length::FillPortion(3)),
            container(side).width(Length::FillPortion(2)),
        ]
        .spacing(20)
        .width(Length::Fill)
        .into()
    } else {
        column![side, main].spacing(16).width(Length::Fill).into()
    }
}

/// Etiqueta arredondada.
pub fn badge<'a, M: 'a>(label: impl ToString, color: Color) -> Element<'a, M> {
    container(text(label).size(11).font(SEMIBOLD).style(color).shaping(Shaping::Advanced))
        .padding([3, 10])
        .style(theme::badge(color))
        .into()
}

/// Ícone grande dentro de um quadrado colorido.
pub fn icon_tile<'a, M: 'a>(icon: &str, color: Color, size: u16) -> Element<'a, M> {
    container(text(icon).size(size).shaping(Shaping::Advanced))
        .padding([size / 3, size / 2 - 2])
        .style(theme::icon_tile(color))
        .into()
}

/// Cabeçalho de página.
pub fn page_header<'a, M: 'a>(env: &Env, icon: &str, title: &str, desc: &str, extra: Option<Element<'a, M>>) -> Element<'a, M> {
    let c = env.c();
    let title_size = if env.narrow() { 22 } else { 28 };
    let mut r = row![
        icon_tile(icon, c.primary, if env.narrow() { 22 } else { 28 }),
        column![strong(title, title_size), muted(env, desc).size(14)].spacing(4).width(Length::Fill),
    ]
    .spacing(16)
    .align_items(Alignment::Center);
    if let Some(extra) = extra {
        if !env.narrow() {
            r = r.push(extra);
        }
    }
    r.into()
}

/// Página rolável, centralizada e com largura máxima confortável.
pub fn page<'a, M: 'a>(env: &Env, children: Vec<Element<'a, M>>) -> Element<'a, M> {
    let mut col = column![].spacing(18).width(Length::Fill).max_width(1240);
    for child in children {
        col = col.push(child);
    }
    scrollable(
        container(col)
            .padding(env.page_padding())
            .width(Length::Fill)
            .center_x(),
    )
    .height(Length::Fill)
    .width(Length::Fill)
    .into()
}

/// Faixa com o resultado de uma operação (sucesso ou erro).
pub fn result_banner<'a, M: Clone + 'a>(env: &Env, res: &OperationResult, open: Option<M>) -> Element<'a, M> {
    let c = env.c();
    let (kind, icon, color) = if res.success {
        (BannerKind::Success, "✅", c.success)
    } else {
        (BannerKind::Error, "⚠", c.danger)
    };
    let mut col = column![row![
        text(icon).size(18).shaping(Shaping::Advanced),
        text(&res.title).size(15).font(SEMIBOLD).style(color).shaping(Shaping::Advanced).width(Length::Fill),
    ]
    .spacing(10)
    .align_items(Alignment::Center)]
    .spacing(6);

    let max_lines = 40;
    for line in res.details.iter().take(max_lines) {
        col = col.push(text(line).size(12).style(c.muted).shaping(Shaping::Advanced));
    }
    if res.details.len() > max_lines {
        col = col.push(text(format!("… e mais {} linha(s)", res.details.len() - max_lines)).size(12).style(c.muted).shaping(Shaping::Advanced));
    }
    if let (Some(msg), Some(_)) = (open, res.output.as_ref()) {
        col = col.push(Space::with_height(4));
        col = col.push(btn("📂  Abrir no Explorer", Btn::Secondary, Some(msg)));
    }
    container(col).padding(16).width(Length::Fill).style(theme::banner(kind)).into()
}

/// Faixa informativa simples.
pub fn info_banner<'a, M: 'a>(env: &Env, kind: BannerKind, icon: &str, message: &str) -> Element<'a, M> {
    let c = env.c();
    let color = match kind {
        BannerKind::Success => c.success,
        BannerKind::Error => c.danger,
        BannerKind::Info => c.accent,
        BannerKind::Warning => c.warning,
    };
    container(
        row![text(icon).size(16).shaping(Shaping::Advanced), text(message).size(13).style(color).shaping(Shaping::Advanced).width(Length::Fill)]
            .spacing(10)
            .align_items(Alignment::Center),
    )
    .padding([12, 16])
    .width(Length::Fill)
    .style(theme::banner(kind))
    .into()
}

/// Indicador de operação em andamento.
pub fn busy<'a, M: 'a>(env: &Env, message: &str) -> Element<'a, M> {
    info_banner(env, BannerKind::Info, "⏳", &format!("{message} — você pode continuar usando o programa enquanto isso."))
}

/// Estado vazio (nenhum item).
pub fn empty_state<'a, M: 'a>(env: &Env, icon: &str, title: &str, desc: &str) -> Element<'a, M> {
    container(
        column![text(icon).size(36).shaping(Shaping::Advanced), strong(title, 15), muted(env, desc)]
            .spacing(6)
            .align_items(Alignment::Center),
    )
    .padding(24)
    .width(Length::Fill)
    .center_x()
    .into()
}

/// Linha de lista com fundo suave.
pub fn list_row<'a, M: 'a>(content: impl Into<Element<'a, M>>) -> Element<'a, M> {
    container(content).padding([10, 14]).width(Length::Fill).style(theme::list_row).into()
}

/// Espaço flexível horizontal.
pub fn fill<'a, M: 'a>() -> Element<'a, M> {
    horizontal_space().into()
}

/// Formata um tamanho em bytes de forma legível.
pub fn human_size(bytes: u64) -> String {
    const UNITS: [&str; 5] = ["B", "KB", "MB", "GB", "TB"];
    let mut value = bytes as f64;
    let mut unit = 0;
    while value >= 1024.0 && unit < UNITS.len() - 1 {
        value /= 1024.0;
        unit += 1;
    }
    if unit == 0 {
        format!("{bytes} B")
    } else {
        format!("{:.1} {}", value, UNITS[unit]).replace('.', ",")
    }
}

/// Mostra apenas o nome do arquivo de um caminho.
pub fn file_name(path: &std::path::Path) -> String {
    path.file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| path.to_string_lossy().to_string())
}
