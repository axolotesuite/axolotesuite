//! Interface do módulo de Fila de Operações.

use iced::widget::{column, progress_bar, row, text};
use iced::{Alignment, Element, Length};

use crate::core::jobs::{JobStatus, Jobs};
use crate::modules::queue::Msg;
use crate::shared::theme::Btn;
use crate::shared::types::Env;
use crate::shared::ui::{self, SEMIBOLD};

pub fn view<'a>(jobs: &'a Jobs, env: &Env) -> Element<'a, Msg> {
    let c = env.c();
    let all = jobs.all();
    let running = jobs.running();

    let status_badge = if running > 0 {
        ui::badge(format!("⏳ {running} em andamento"), c.accent)
    } else {
        ui::badge("✓ Fila livre", c.success)
    };

    let header = ui::page_header(
        env,
        "📦",
        "Fila de Operações",
        "Todas as tarefas pesadas (ZIP, Imagens, PDF, Áudio e Vídeo) rodam em threads separadas para nunca travar a janela.",
        Some(status_badge),
    );

    let top_row = row![
        ui::strong(
            format!("Operações registradas nesta sessão ({})", all.len()),
            15
        )
        .width(Length::Fill),
        ui::btn(
            "🗑  Limpar concluídas",
            Btn::Secondary,
            (all.len() > running).then_some(Msg::ClearFinished),
        ),
    ]
    .align_items(Alignment::Center);

    let mut list = column![].spacing(10);
    if all.is_empty() {
        list = list.push(ui::empty_state(
            env,
            "📦",
            "Nenhuma operação executada ainda",
            "Quando você converter uma imagem, criar um ZIP ou unir PDFs, o progresso e o tempo gasto aparecerão aqui.",
        ));
    } else {
        for job in all {
            let (icon, label, color, prog) = match job.status {
                JobStatus::Running => ("⏳", "Em andamento", c.accent, 55.0),
                JobStatus::Done => ("✅", "Concluída", c.success, 100.0),
                JobStatus::Failed => ("⚠", "Falhou", c.danger, 100.0),
            };
            let duration_txt = match job.elapsed {
                Some(d) if d.as_secs() > 0 => format!("{:.1}s", d.as_secs_f32()),
                Some(d) => format!("{} ms", d.as_millis()),
                None => "processando…".into(),
            };

            let item = ui::list_row(
                column![
                    row![
                        text(icon).size(16),
                        ui::badge(&job.module, c.primary),
                        text(&job.title).size(14).font(SEMIBOLD).width(Length::Fill),
                        ui::badge(label, color),
                        ui::muted(env, format!("{} • {}", job.started_at, duration_txt)),
                    ]
                    .spacing(10)
                    .align_items(Alignment::Center),
                    progress_bar(0.0..=100.0, prog).height(Length::Fixed(6.0)),
                    ui::muted(env, &job.message),
                ]
                .spacing(6),
            );
            list = list.push(item);
        }
    }

    let main = ui::section(
        env,
        "⚡",
        "Monitor de Execução Assíncrona",
        "Você pode disparar uma conversão pesada e continuar usando o bloco de notas ou a calculadora normalmente.",
        column![top_row, list].spacing(14),
    );

    ui::page(env, vec![header, main])
}
