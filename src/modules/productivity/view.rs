//! Interface do módulo de Produtividade: Bloco de Notas, Tarefas, Calculadora e Organizador.

use iced::widget::{button, checkbox, column, container, row, text, text_editor, Space};
use iced::{Alignment, Element, Length};

use crate::modules::productivity::{Msg, State, Tab, TaskFilter};
use crate::shared::theme::{self, BannerKind, Btn};
use crate::shared::types::Env;
use crate::shared::ui::{self, act, SEMIBOLD};

pub fn view<'a>(state: &'a State, env: &Env) -> Element<'a, Msg> {
    let header = ui::page_header(
        env,
        "📋",
        "Central de Produtividade",
        "Bloco de notas com salvamento local, lista de tarefas, calculadora científica e organizador automático de pastas.",
        None,
    );

    let tabs = tabs_bar(state, env);

    let content = match state.tab {
        Tab::Notes => notes_view(state, env),
        Tab::Tasks => tasks_view(state, env),
        Tab::Calc => calc_view(state, env),
        Tab::Organizer => organizer_view(state, env),
    };

    ui::page(env, vec![header, tabs, content])
}

fn tabs_bar<'a>(state: &State, env: &Env) -> Element<'a, Msg> {
    let pending = state.pending_tasks();
    let tasks_label = if pending > 0 {
        format!("✅  Tarefas ({pending})")
    } else {
        "✅  Tarefas".to_string()
    };

    let items = vec![
        ("📝  Bloco de Notas".to_string(), Tab::Notes),
        (tasks_label, Tab::Tasks),
        ("🖩  Calculadora".to_string(), Tab::Calc),
        ("🗂  Organizar Pastas".to_string(), Tab::Organizer),
    ];

    if env.narrow() {
        let btns: Vec<Element<'a, Msg>> = items
            .into_iter()
            .map(|(label, tab)| {
                button(container(text(label).size(13).font(SEMIBOLD)).center_x().width(Length::Fill))
                    .padding([10, 14])
                    .width(Length::Fill)
                    .style(theme::btn(Btn::Tab(state.tab == tab)))
                    .on_press(Msg::TabSelected(tab))
                    .into()
            })
            .collect();
        ui::grid(btns, 2, 8)
    } else {
        let mut r = row![].spacing(10).align_items(Alignment::Center);
        for (label, tab) in items {
            r = r.push(ui::btn(label, Btn::Tab(state.tab == tab), Some(Msg::TabSelected(tab))));
        }
        r.into()
    }
}

// ---------------------------------------------------------------------------
// 1. Bloco de Notas
// ---------------------------------------------------------------------------

fn notes_view<'a>(state: &'a State, env: &Env) -> Element<'a, Msg> {
    let c = env.c();

    // Coluna de notas salvas + guia
    let mut list_col = column![row![
        ui::strong(format!("Suas notas ({})", state.notes.len()), 15).width(Length::Fill),
        ui::btn("➕  Nova nota", Btn::Secondary, Some(Msg::NoteNew)),
    ]
    .spacing(8)
    .align_items(Alignment::Center)]
    .spacing(8);

    if state.notes.is_empty() {
        list_col = list_col.push(ui::empty_state(
            env,
            "📝",
            "Nenhuma nota salva ainda",
            "Escreva ao lado e clique em \"Salvar nota\" para criar a primeira.",
        ));
    } else {
        for name in &state.notes {
            let active = state.selected_note.as_deref() == Some(name.as_str());
            let btn = button(
                row![
                    text(if active { "📄" } else { "🗒" }).size(15),
                    text(name).size(14).font(SEMIBOLD).width(Length::Fill),
                ]
                .spacing(10)
                .align_items(Alignment::Center),
            )
            .padding([10, 14])
            .width(Length::Fill)
            .style(theme::btn(Btn::Nav(active)))
            .on_press(Msg::NoteSelected(name.clone()));
            list_col = list_col.push(btn);
        }
    }

    list_col = list_col.push(Space::with_height(4));
    list_col = list_col.push(ui::btn("📂  Abrir pasta de notas no Explorer", Btn::Ghost, Some(Msg::OpenNotesFolder)));

    let side = column![
        ui::card(env, list_col),
        ui::help(
            env,
            &[
                "Digite um título curto para a sua nota (ex.: \" Ideias do projeto \").",
                "Escreva o conteúdo na caixa de texto com quantas linhas quiser.",
                "Clique em \"Salvar nota\" — ela fica gravada em um arquivo .txt no seu computador.",
            ],
            Some("Suas notas ficam salvas offline e podem ser abertas até no Bloco de Notas do Windows."),
        ),
    ]
    .spacing(16)
    .into();

    // Editor principal
    let status_badge = if state.note_dirty {
        ui::badge("● Alterações não salvas", c.warning)
    } else if state.selected_note.is_some() {
        ui::badge("✓ Salva", c.success)
    } else {
        ui::badge("Nova nota", c.accent)
    };

    let name_field = ui::field(
        env,
        "Título da nota",
        "Será usado como nome do arquivo .txt (caracteres especiais são ajustados automaticamente).",
        ui::input("Ex.: Reunião de segunda, Lista de compras, Ideias...", &state.note_name, Msg::NoteNameChanged),
    );

    let editor_box = text_editor(&state.editor)
        .on_action(Msg::NoteEdit)
        .height(Length::Fixed(270.0))
        .padding(14);

    let editor_field = ui::field(
        env,
        "Conteúdo da nota",
        "Suporta múltiplas linhas, seleção de texto, copiar (Ctrl+C) e colar (Ctrl+V).",
        editor_box,
    );

    let mut action_items = vec![
        act(
            "💾  Salvar nota",
            Btn::Primary,
            (!state.note_name.trim().is_empty()).then_some(Msg::NoteSave),
        ),
        act("📄  Limpar / Nova", Btn::Secondary, Some(Msg::NoteNew)),
    ];
    if state.selected_note.is_some() && !state.confirm_delete {
        action_items.push(act("🗑  Excluir", Btn::Danger, Some(Msg::NoteDelete)));
    }

    let mut main_col = column![
        row![ui::strong("Editor de Texto", 17).width(Length::Fill), status_badge].align_items(Alignment::Center),
        name_field,
        editor_field,
        ui::actions(env, action_items),
    ]
    .spacing(14);

    if state.confirm_delete {
        main_col = main_col.push(
            container(
                row![
                    text("⚠  Tem certeza que deseja excluir esta nota permanentemente?")
                        .size(13)
                        .style(c.danger)
                        .width(Length::Fill),
                    ui::btn("Sim, excluir", Btn::Danger, Some(Msg::NoteDeleteConfirm(true))),
                    ui::btn("Cancelar", Btn::Secondary, Some(Msg::NoteDeleteConfirm(false))),
                ]
                .spacing(10)
                .align_items(Alignment::Center),
            )
            .padding(14)
            .width(Length::Fill)
            .style(theme::banner(BannerKind::Error)),
        );
    }

    if let Some(res) = &state.note_result {
        let open = res.output.clone().map(Msg::OpenPath);
        main_col = main_col.push(ui::result_banner(env, res, open));
    }

    let main = ui::card(env, main_col);
    ui::two_col(env, main, side)
}

// ---------------------------------------------------------------------------
// 2. Tarefas
// ---------------------------------------------------------------------------

fn tasks_view<'a>(state: &'a State, env: &Env) -> Element<'a, Msg> {
    let c = env.c();
    let total = state.tasks.len();
    let done_count = state.tasks.iter().filter(|t| t.done).count();
    let pending_count = total.saturating_sub(done_count);

    let add_row: Element<'a, Msg> = {
        let inp = ui::input("Digite uma nova tarefa (ex.: Enviar relatório até sexta)...", &state.new_task, Msg::TaskInput)
            .on_submit(Msg::TaskAdd)
            .width(Length::Fill);
        let can_add = !state.new_task.trim().is_empty();
        if env.narrow() {
            column![
                inp,
                button(container(text("➕  Adicionar tarefa").size(14).font(SEMIBOLD)).center_x().width(Length::Fill))
                    .padding([11, 18])
                    .width(Length::Fill)
                    .style(theme::btn(Btn::Primary))
                    .on_press_maybe(can_add.then_some(Msg::TaskAdd)),
            ]
            .spacing(8)
            .into()
        } else {
            row![inp, ui::btn("➕  Adicionar", Btn::Primary, can_add.then_some(Msg::TaskAdd))]
                .spacing(10)
                .align_items(Alignment::Center)
                .into()
        }
    };

    let filters = row![
        ui::btn(
            format!("Todas ({total})"),
            Btn::Tab(state.task_filter == TaskFilter::All),
            Some(Msg::TaskFilterChanged(TaskFilter::All)),
        ),
        ui::btn(
            format!("Pendentes ({pending_count})"),
            Btn::Tab(state.task_filter == TaskFilter::Pending),
            Some(Msg::TaskFilterChanged(TaskFilter::Pending)),
        ),
        ui::btn(
            format!("Concluídas ({done_count})"),
            Btn::Tab(state.task_filter == TaskFilter::Done),
            Some(Msg::TaskFilterChanged(TaskFilter::Done)),
        ),
        ui::fill(),
        ui::btn(
            "🗑  Limpar concluídas",
            Btn::Ghost,
            (done_count > 0).then_some(Msg::TasksClearDone),
        ),
    ]
    .spacing(8)
    .align_items(Alignment::Center);

    let filtered: Vec<_> = state
        .tasks
        .iter()
        .filter(|t| match state.task_filter {
            TaskFilter::All => true,
            TaskFilter::Pending => !t.done,
            TaskFilter::Done => t.done,
        })
        .collect();

    let mut list = column![].spacing(8);
    if filtered.is_empty() {
        list = list.push(ui::empty_state(
            env,
            "✅",
            "Nenhuma tarefa por aqui",
            "Adicione uma tarefa no campo acima e pressione Enter ou clique em \"Adicionar\".",
        ));
    } else {
        for task in filtered {
            let id = task.id;
            let chk = checkbox(&task.title, task.done)
                .on_toggle(move |checked| Msg::TaskToggle(id, checked))
                .size(18)
                .text_size(14);
            let item = ui::list_row(
                row![
                    container(chk).width(Length::Fill),
                    ui::badge(&task.created, if task.done { c.success } else { c.muted }),
                    ui::btn("🗑", Btn::Danger, Some(Msg::TaskDelete(id))),
                ]
                .spacing(10)
                .align_items(Alignment::Center),
            );
            list = list.push(item);
        }
    }

    let main = ui::section(
        env,
        "✅",
        "Lista de Tarefas",
        "Organize seu dia. As tarefas são salvas automaticamente a cada alteração.",
        column![add_row, filters, list].spacing(14),
    );

    let stats_card = ui::section(
        env,
        "📊",
        "Resumo",
        "Acompanhe seu progresso diário.",
        column![
            ui::list_row(row![text("⏳  Pendentes").size(14).width(Length::Fill), ui::strong(pending_count, 16)]),
            ui::list_row(row![text("✅  Concluídas").size(14).width(Length::Fill), ui::strong(done_count, 16)]),
            ui::list_row(row![text("📋  Total").size(14).width(Length::Fill), ui::strong(total, 16)]),
        ]
        .spacing(8),
    );

    let side = column![
        stats_card,
        ui::help(
            env,
            &[
                "Digite o que precisa fazer e pressione Enter (ou clique em \"Adicionar\").",
                "Clique na caixinha ao lado da tarefa para marcá-la como concluída.",
                "Use os filtros \"Pendentes\" e \"Concluídas\" para focar no que falta.",
            ],
            Some("Tudo fica salvo automaticamente no seu computador mesmo após fechar o programa."),
        ),
    ]
    .spacing(16)
    .into();

    ui::two_col(env, main, side)
}

// ---------------------------------------------------------------------------
// 3. Calculadora
// ---------------------------------------------------------------------------

pub fn calc_view<'a>(state: &'a State, env: &Env) -> Element<'a, Msg> {
    let c = env.c();

    let expr_input = ui::input("Ex.: (125,50 + 40) * 1,10 ou 2^10", &state.calc_expr, Msg::CalcInput)
        .on_submit(Msg::CalcEval)
        .size(18)
        .padding([12, 14]);

    let result_box: Element<'a, Msg> = match &state.calc_result {
        Some(Ok(val)) => container(
            row![
                text("Resultado:").size(14).style(c.muted),
                ui::fill(),
                ui::colored(format!("= {val}"), 26, c.success).font(ui::BOLD),
            ]
            .align_items(Alignment::Center),
        )
        .padding([14, 18])
        .width(Length::Fill)
        .style(theme::banner(BannerKind::Success))
        .into(),
        Some(Err(err)) => ui::info_banner(env, BannerKind::Error, "⚠", err),
        None => container(
            row![
                text("Digite ou use o teclado abaixo e clique em \"=\" para calcular.").size(13).style(c.muted),
            ]
            .align_items(Alignment::Center),
        )
        .padding([14, 18])
        .width(Length::Fill)
        .style(theme::list_row)
        .into(),
    };

    // Teclado numérico
    let keys: &[(&str, Btn, Msg)] = &[
        ("C", Btn::Danger, Msg::CalcClear),
        ("(", Btn::KeyOp, Msg::CalcKey("(")),
        (")", Btn::KeyOp, Msg::CalcKey(")")),
        ("⌫", Btn::Secondary, Msg::CalcBackspace),
        ("7", Btn::Key, Msg::CalcKey("7")),
        ("8", Btn::Key, Msg::CalcKey("8")),
        ("9", Btn::Key, Msg::CalcKey("9")),
        ("÷", Btn::KeyOp, Msg::CalcKey(" / ")),
        ("4", Btn::Key, Msg::CalcKey("4")),
        ("5", Btn::Key, Msg::CalcKey("5")),
        ("6", Btn::Key, Msg::CalcKey("6")),
        ("×", Btn::KeyOp, Msg::CalcKey(" * ")),
        ("1", Btn::Key, Msg::CalcKey("1")),
        ("2", Btn::Key, Msg::CalcKey("2")),
        ("3", Btn::Key, Msg::CalcKey("3")),
        ("−", Btn::KeyOp, Msg::CalcKey(" - ")),
        ("0", Btn::Key, Msg::CalcKey("0")),
        (",", Btn::Key, Msg::CalcKey(",")),
        ("^", Btn::KeyOp, Msg::CalcKey("^")),
        ("+", Btn::KeyOp, Msg::CalcKey(" + ")),
    ];

    let key_widgets: Vec<Element<'a, Msg>> = keys
        .iter()
        .map(|(label, kind, msg)| {
            button(container(text(*label).size(18).font(SEMIBOLD)).center_x().width(Length::Fill))
                .padding([12, 0])
                .width(Length::Fill)
                .style(theme::btn(*kind))
                .on_press(msg.clone())
                .into()
        })
        .collect();

    let keypad = ui::grid(key_widgets, 4, 8);

    let eq_btn = button(container(text("=   Calcular agora").size(16).font(ui::BOLD)).center_x().width(Length::Fill))
        .padding([13, 20])
        .width(Length::Fill)
        .style(theme::btn(Btn::Primary))
        .on_press(Msg::CalcEval);

    let main = ui::section(
        env,
        "🖩",
        "Calculadora Expressa",
        "Suporta parênteses, potência (^), resto (%), decimais com vírgula ou ponto e precedência matemática real.",
        column![expr_input, result_box, keypad, eq_btn].spacing(12),
    );

    // Histórico de cálculos
    let mut hist_col = column![].spacing(8);
    if state.calc_history.is_empty() {
        hist_col = hist_col.push(ui::muted(env, "Seus cálculos recentes aparecerão aqui. Clique em um para reutilizar."));
    } else {
        for (expr, res) in &state.calc_history {
            hist_col = hist_col.push(
                button(
                    row![
                        text(expr).size(13).style(c.muted).width(Length::Fill),
                        ui::strong(format!("= {res}"), 14),
                    ]
                    .spacing(8)
                    .align_items(Alignment::Center),
                )
                .padding([8, 12])
                .width(Length::Fill)
                .style(theme::btn(Btn::Secondary))
                .on_press(Msg::CalcUseHistory(res.clone())),
            );
        }
    }

    let side = column![
        ui::section(env, "🕒", "Últimos Cálculos", "Clique em um resultado para usá-lo na conta atual.", hist_col),
        ui::help(
            env,
            &[
                "Digite a conta ou clique nos botões da calculadora.",
                "Use parênteses ( ) para agrupar operações e ^ para potências (ex.: 2^8 = 256).",
                "Pressione Enter ou clique em \"Calcular agora\" para ver o resultado.",
            ],
            Some("Você pode usar tanto vírgula quanto ponto para separar casas decimais."),
        ),
    ]
    .spacing(16)
    .into();

    ui::two_col(env, main, side)
}

// ---------------------------------------------------------------------------
// 4. Organizador de Pastas
// ---------------------------------------------------------------------------

fn organizer_view<'a>(state: &'a State, env: &Env) -> Element<'a, Msg> {
    let c = env.c();
    let has_dir = !state.org_dir.trim().is_empty();

    let dir_field = ui::field(
        env,
        "Pasta que você quer organizar",
        "Os arquivos soltos dentro desta pasta serão separados em subpastas (Imagens, PDFs, Documentos, Áudios, Vídeos, Compactados...).",
        ui::path_field(
            env,
            "Ex.: C:\\Users\\voce\\Downloads",
            &state.org_dir,
            Msg::OrgDirChanged,
            "📂  Escolher pasta…",
            Msg::OrgBrowse,
        ),
    );

    let btns = ui::actions(
        env,
        vec![
            act(
                "🔍  Analisar pasta (simulação)",
                Btn::Secondary,
                (has_dir && !state.org_busy).then_some(Msg::OrgPreview),
            ),
            act(
                "✨  Organizar arquivos agora",
                Btn::Primary,
                (has_dir && !state.org_busy).then_some(Msg::OrgRun),
            ),
        ],
    );

    let mut body = column![dir_field, btns].spacing(14);

    if state.org_busy {
        body = body.push(ui::busy(env, "Organizando arquivos em subpastas"));
    }

    if let Some(preview) = &state.org_preview {
        match preview {
            Ok(items) if items.is_empty() => {
                body = body.push(ui::info_banner(
                    env,
                    BannerKind::Info,
                    "ℹ",
                    "Nenhum arquivo solto foi encontrado nesta pasta (subpastas existentes não são alteradas).",
                ));
            }
            Ok(items) => {
                let total: usize = items.iter().map(|(_, n)| *n).sum();
                let mut rows = column![ui::strong(
                    format!("Prévia: {total} arquivo(s) serão organizados nas seguintes subpastas:"),
                    14
                )]
                .spacing(8);
                for (cat, count) in items {
                    rows = rows.push(ui::list_row(
                        row![
                            text(format!("📁  {cat}")).size(14).font(SEMIBOLD).width(Length::Fill),
                            ui::badge(format!("{count} arquivo(s)"), c.primary),
                        ]
                        .align_items(Alignment::Center),
                    ));
                }
                body = body.push(rows);
            }
            Err(err) => {
                body = body.push(ui::info_banner(env, BannerKind::Error, "⚠", err));
            }
        }
    }

    if let Some(res) = &state.org_result {
        let open = res.output.clone().map(Msg::OpenPath);
        body = body.push(ui::result_banner(env, res, open));
    }

    let main = ui::section(
        env,
        "🗂",
        "Organizador Inteligente de Arquivos",
        "Ideal para colocar ordem na pasta Downloads ou Área de Trabalho em um clique.",
        body,
    );

    let side = ui::help(
        env,
        &[
            "Clique em \"Escolher pasta…\" e selecione a pasta que está bagunçada (como Downloads).",
            "Veja a prévia de quantos arquivos irão para cada categoria (Imagens, PDFs, Documentos, Vídeos...).",
            "Clique em \"Organizar arquivos agora\" para criar as subpastas e mover tudo com segurança.",
        ],
        Some("Se já existir um arquivo com o mesmo nome na subpasta, o Axolote acrescenta \"(2)\" para nunca sobrescrever nada."),
    );

    ui::two_col(env, main, side)
}
