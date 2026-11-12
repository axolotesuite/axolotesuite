//! Interface do Instalador Principal (Setup em 4 etapas + Download Automático de
//! Dependências como o FFmpeg) e Tela de Desinstalação do Axolote Suite.

use iced::widget::{checkbox, column, container, image as iced_image, progress_bar, row, Space};
use iced::{Alignment, Element, Length};

use crate::modules::installer::{Msg, State, WizardStep};
use crate::shared::brand;
use crate::shared::theme::{self, BannerKind, Btn};
use crate::shared::types::Env;
use crate::shared::ui::{self, act};

pub fn view<'a>(state: &'a State, mandatory: bool, env: &Env) -> Element<'a, Msg> {
    let c = env.c();
    let is_en = env.lang.is_en();
    let ffmpeg_ok = state.is_ffmpeg_ready();
    let is_installed = state.is_installed();

    let status_badges = row![
        if is_installed {
            ui::badge(
                if is_en {
                    "✓ Installed"
                } else {
                    "✓ App Instalado"
                },
                c.success,
            )
        } else {
            ui::badge(
                if is_en {
                    "Setup Required"
                } else {
                    "Instalação Obrigatória"
                },
                c.accent,
            )
        },
        if ffmpeg_ok {
            ui::badge(
                if is_en {
                    "✓ FFmpeg Ready"
                } else {
                    "✓ FFmpeg Instalado"
                },
                c.success,
            )
        } else {
            ui::badge(
                if is_en {
                    "⬇ Auto FFmpeg Pending"
                } else {
                    "⬇ FFmpeg Obrigatório Pendente"
                },
                c.warning,
            )
        },
    ]
    .spacing(8)
    .align_items(Alignment::Center);

    let header = ui::page_header(
        env,
        if is_installed { "🗑" } else { "💿" },
        if mandatory {
            if is_en {
                "Axolote Suite Setup & Dependencies"
            } else {
                "Instalador Obrigatório — Axolote Suite Setup"
            }
        } else if is_installed {
            if is_en {
                "Uninstall & Cleanup Workstation"
            } else {
                "Desinstalação & Limpeza do Axolote Suite"
            }
        } else {
            if is_en {
                "Installer & Dependencies (Setup)"
            } else {
                "Instalador & Central de Dependências (Setup)"
            }
        },
        if is_installed {
            if is_en {
                "The application is installed and active on your PC. You can cleanly remove shortcuts, dependencies, or purge everything."
            } else {
                "O aplicativo já está instalado no seu PC. Você pode remover atalhos, gerenciar dependências ou apagar tudo com 1 clique."
            }
        } else {
            if is_en {
                "Installs essentials on your PC and automatically configures required engines (including FFmpeg) before first use."
            } else {
                "Instala o essencial no seu PC e baixa automaticamente todas as dependências necessárias (incluindo o motor FFmpeg para Vídeo e Áudio) antes de iniciar."
            }
        },
        Some(status_badges.into()),
    );

    let mut mode_bar = row![];

    if !is_installed {
        mode_bar = mode_bar.push(ui::btn(
            if is_en {
                "🚀  Setup Wizard"
            } else {
                "🚀  Assistente de Instalação (Setup)"
            },
            Btn::Tab(!matches!(
                state.step,
                WizardStep::UninstallConfirm | WizardStep::Uninstalled
            )),
            Some(Msg::GoToStep(WizardStep::Welcome)),
        ));
    }

    mode_bar = mode_bar.push(ui::btn(
        if is_en {
            "💥  Purge Everything"
        } else {
            "💥  Apagar Tudo do Computador"
        },
        Btn::Tab(matches!(
            state.step,
            WizardStep::UninstallConfirm | WizardStep::Uninstalled
        )),
        Some(Msg::GoToStep(WizardStep::UninstallConfirm)),
    ));

    mode_bar = mode_bar.push(ui::fill());

    if !mandatory {
        mode_bar = mode_bar.push(ui::btn(
            if is_en {
                "🏠  Back to Tools →"
            } else {
                "🏠  Voltar para as Ferramentas →"
            },
            Btn::Ghost,
            Some(Msg::FinishAndGoHome),
        ));
    }

    let wizard_card = match state.step {
        WizardStep::Welcome => step_welcome(state, mandatory, env),
        WizardStep::Location => step_location(state, env),
        WizardStep::Installing => step_installing(state, env),
        WizardStep::Finished => step_finished(state, env),
        WizardStep::UninstallConfirm => step_uninstall(state, env),
        WizardStep::Uninstalled => step_uninstalled(state, env),
    };

    let centered = container(container(wizard_card).max_width(760).width(Length::Fill))
        .width(Length::Fill)
        .center_x();

    ui::page(
        env,
        vec![
            header,
            mode_bar.spacing(10).align_items(Alignment::Center).into(),
            centered.into(),
        ],
    )
}

fn step_indicator<'a>(current: u8, env: &Env) -> Element<'a, Msg> {
    let c = env.c();
    let steps = [
        (1, "Boas-vindas"),
        (2, "Local & Dependências"),
        (3, "Baixando & Instalando"),
        (4, "Concluído"),
    ];
    let mut r = row![].spacing(10).align_items(Alignment::Center);
    for (num, label) in steps {
        let color = if num == current {
            c.primary
        } else if num < current {
            c.success
        } else {
            c.muted
        };
        r = r.push(ui::badge(format!("{num}. {label}"), color));
    }
    r.into()
}

// ---------------------------------------------------------------------------
// Passo 1: Boas-vindas
// ---------------------------------------------------------------------------

fn step_welcome<'a>(state: &'a State, mandatory: bool, env: &Env) -> Element<'a, Msg> {
    let c = env.c();
    let ffmpeg_ok = state.is_ffmpeg_ready();

    let logo = iced_image::Image::new(brand::logo_banner())
        .width(Length::Fixed(112.0))
        .height(Length::Fixed(112.0));

    let dep_row = ui::list_row(
        row![
            ui::strong("🎬  Dependência Multimídia (FFmpeg):", 14).width(Length::Fill),
            if ffmpeg_ok {
                ui::badge("✓ Já instalado e ativo", c.success)
            } else {
                ui::badge("Será baixado automaticamente na instalação", c.accent)
            },
        ]
        .spacing(10)
        .align_items(Alignment::Center),
    );

    let mut content = column![
        step_indicator(1, env),
        Space::with_height(6),
        logo,
        ui::strong("Axolote Suite — Instalador & Setup Obrigatório", 24),
        ui::muted(
            env,
            "Bem-vindo ao instalador do Axolote Suite! Clique em 'Instalar Tudo e Baixar Dependências' abaixo para instalar o programa e baixar automaticamente o motor FFmpeg essencial."
        )
        .size(14),
        dep_row,
        ui::list_row(
            column![
                ui::strong("📦 O que este instalador baixa e configura automaticamente:", 14),
                ui::muted(env, "• Instala o executável principal otimizado (axolote_suite.exe) e o ícone oficial (.ico)"),
                ui::muted(env, "• Baixa e configura sozinho o FFmpeg (essencial para o Editor de Vídeo e Conversor de Áudio)"),
                ui::muted(env, "• Cria atalhos na Área de Trabalho e no Menu Iniciar do Windows"),
                ui::muted(env, "• Registra o Desinstalador oficial no Windows (sem exigir permissão de Administrador)"),
            ]
            .spacing(4),
        ),
    ]
    .spacing(14)
    .align_items(Alignment::Center);

    if state.ffmpeg_busy {
        content = content.push(ui::busy(
            env,
            "Baixando e extraindo o FFmpeg automaticamente",
        ));
    }
    if let Some(res) = &state.result {
        content = content.push(ui::result_banner(env, res, None));
    }

    let mut btns = vec![
        act(
            "🚀  Instalar Tudo e Baixar Dependências Agora",
            Btn::Primary,
            (!state.ffmpeg_busy).then_some(Msg::StartInstall),
        ),
        act(
            "📂  Escolher Pasta / Opções  ›",
            Btn::Secondary,
            (!state.ffmpeg_busy).then_some(Msg::GoToStep(WizardStep::Location)),
        ),
    ];
    if !mandatory {
        btns.push(act(
            "Voltar para o App",
            Btn::Ghost,
            Some(Msg::FinishAndGoHome),
        ));
    }

    content = content.push(ui::actions(env, btns));

    container(content)
        .padding(28)
        .width(Length::Fill)
        .style(theme::hero)
        .into()
}

// ---------------------------------------------------------------------------
// Passo 2: Escolha do Local, Atalhos e Download de Dependências
// ---------------------------------------------------------------------------

fn step_location<'a>(state: &'a State, env: &Env) -> Element<'a, Msg> {
    let c = env.c();
    let dir_field = ui::field(
        env,
        "Escolha o local de instalação",
        "Onde o Axolote Suite e suas dependências serão instalados.",
        ui::path_field(
            env,
            "Ex.: C:\\Users\\voce\\AppData\\Local\\Programs\\Axolote Suite",
            &state.install_dir,
            Msg::InstallDirChanged,
            "📂  Procurar…",
            Msg::BrowseInstallDir,
        ),
    );

    let chk_ffmpeg = checkbox(
        "Baixar e instalar automaticamente o FFmpeg (essencial para o Editor de Vídeo e Áudio)",
        state.download_ffmpeg,
    )
    .on_toggle(Msg::ToggleDownloadFfmpeg)
    .size(18)
    .text_size(14);

    let chk_desktop = checkbox(
        "Criar atalho na Área de Trabalho (Desktop)",
        state.create_desktop_shortcut,
    )
    .on_toggle(Msg::ToggleDesktopShortcut)
    .size(18)
    .text_size(14);

    let chk_start = checkbox(
        "Criar pasta e atalho no Menu Iniciar do Windows",
        state.create_start_menu_shortcut,
    )
    .on_toggle(Msg::ToggleStartMenuShortcut)
    .size(18)
    .text_size(14);

    let nav_btns = ui::actions(
        env,
        vec![
            act(
                "‹  Voltar",
                Btn::Secondary,
                Some(Msg::GoToStep(WizardStep::Welcome)),
            ),
            act(
                "🚀  Instalar Tudo Agora",
                Btn::Primary,
                Some(Msg::StartInstall),
            ),
        ],
    );

    let content = column![
        step_indicator(2, env),
        ui::strong("Configurar Instalação e Dependências Essenciais", 22),
        dir_field,
        ui::list_row(column![chk_ffmpeg, chk_desktop, chk_start].spacing(10)),
        ui::badge(
            if state.is_ffmpeg_ready() {
                "✓ FFmpeg já detectado no sistema — instalação instantânea (~12 MB)"
            } else {
                "Inclui download automático do FFmpeg oficial + Axolote Suite"
            },
            c.accent,
        ),
        nav_btns,
    ]
    .spacing(16);

    ui::card(env, content)
}

// ---------------------------------------------------------------------------
// Passo 3: Copiando arquivos e baixando dependências
// ---------------------------------------------------------------------------

fn step_installing<'a>(state: &'a State, env: &Env) -> Element<'a, Msg> {
    let content = column![
        step_indicator(3, env),
        iced_image::Image::new(brand::logo_small())
            .width(Length::Fixed(64.0))
            .height(Length::Fixed(64.0)),
        ui::strong("Instalando o Axolote Suite e Dependências…", 22),
        ui::muted(env, &state.status_text),
        progress_bar(0.0..=100.0, state.progress).height(Length::Fixed(10.0)),
        ui::muted(env, "Aguarde alguns instantes — tudo está sendo configurado automaticamente em segundo plano."),
    ]
    .spacing(16)
    .align_items(Alignment::Center);

    ui::card(env, content)
}

// ---------------------------------------------------------------------------
// Passo 4: Concluído
// ---------------------------------------------------------------------------

fn step_finished<'a>(state: &'a State, env: &Env) -> Element<'a, Msg> {
    let c = env.c();
    let chk_launch = checkbox(
        "Abrir a Central de Ferramentas do Axolote Suite ao concluir",
        state.launch_after_finish,
    )
    .on_toggle(Msg::ToggleLaunchAfter)
    .size(18)
    .text_size(14);

    let mut col = column![
        step_indicator(4, env),
        row![
            iced_image::Image::new(brand::logo_small())
                .width(Length::Fixed(56.0))
                .height(Length::Fixed(56.0)),
            ui::badge("✅ Instalação Completa", c.success),
        ]
        .spacing(12)
        .align_items(Alignment::Center),
        ui::strong("O Axolote Suite foi instalado com sucesso!", 22),
        chk_launch,
    ]
    .spacing(14);

    if let Some(res) = &state.result {
        col = col.push(ui::result_banner(env, res, Some(Msg::OpenInstalledFolder)));
    }

    col = col.push(ui::actions(
        env,
        vec![
            act(
                "🚀  Concluir e Abrir Axolote Suite",
                Btn::Primary,
                Some(Msg::FinishAndGoHome),
            ),
            act(
                "📂  Abrir pasta instalada",
                Btn::Secondary,
                Some(Msg::OpenInstalledFolder),
            ),
        ],
    ));

    container(col)
        .padding(28)
        .width(Length::Fill)
        .style(theme::hero)
        .into()
}

// ---------------------------------------------------------------------------
// Tela de Desinstalação
// ---------------------------------------------------------------------------

fn step_uninstall<'a>(state: &'a State, env: &Env) -> Element<'a, Msg> {
    let c = env.c();
    let is_en = env.lang.is_en();

    let dir_field = ui::field(
        env,
        if is_en {
            "Installation directory"
        } else {
            "Local da instalação a remover"
        },
        if is_en {
            "Removes installed executable, FFmpeg, application icon, and shortcuts from Desktop and Start Menu."
        } else {
            "Remove o executável instalado, o FFmpeg local, o ícone e os atalhos da Área de Trabalho e do Menu Iniciar."
        },
        ui::path_field(
            env,
            if is_en {
                "Folder where Axolote Suite is installed"
            } else {
                "Pasta onde o Axolote Suite está instalado"
            },
            &state.install_dir,
            Msg::InstallDirChanged,
            if is_en {
                "📂  Browse…"
            } else {
                "📂  Procurar…"
            },
            Msg::BrowseInstallDir,
        ),
    );

    let chk_data = checkbox(
        if is_en {
            "Also delete my saved notes, tasks list, history, and personal configurations"
        } else {
            "Apagar também minhas notas salvas, lista de tarefas, histórico e configurações pessoais"
        },
        state.remove_user_data_on_uninstall,
    )
    .on_toggle(Msg::ToggleRemoveUserData)
    .size(18)
    .text_size(14);

    let btns = ui::actions(
        env,
        vec![
            act(
                if is_en {
                    "💥  DELETE EVERYTHING FROM PC"
                } else {
                    "💥  APAGAR TUDO DO COMPUTADOR"
                },
                Btn::Danger,
                (!state.purge_busy).then_some(Msg::PurgeEverything),
            ),
            act(
                if is_en {
                    "🗑  Normal Uninstall (Keep Notes)"
                } else {
                    "🗑  Desinstalação Normal (Manter Notas)"
                },
                Btn::Secondary,
                (!state.purge_busy).then_some(Msg::StartUninstall),
            ),
            act(
                if is_en {
                    "Cancel / Back"
                } else {
                    "Cancelar / Voltar"
                },
                Btn::Ghost,
                Some(Msg::FinishAndGoHome),
            ),
        ],
    );

    let mut content = column![
        row![
            iced_image::Image::new(brand::logo_small())
                .width(Length::Fixed(48.0))
                .height(Length::Fixed(48.0)),
            column![
                ui::strong(if is_en { "Uninstall & Cleanup" } else { "Desinstalar & Limpar Tudo" }, 22),
                ui::muted(env, if is_en {
                    "Choose below: remove app or click 'Apagar Tudo' to wipe all traces from your PC."
                } else {
                    "Você pode desinstalar normalmente ou clicar em 'Apagar Tudo' para remover 100% dos rastros do PC."
                }),
            ]
            .spacing(2),
        ]
        .spacing(14)
        .align_items(Alignment::Center),
        dir_field,
        container(chk_data)
            .padding(14)
            .width(Length::Fill)
            .style(theme::banner(BannerKind::Warning)),
        ui::colored(
            if is_en {
                "• 'APAGAR TUDO': Deletes shortcuts, registry entries, install directory, notes, tasks, and configs in %APPDATA%."
            } else {
                "• 'APAGAR TUDO': Deleta atalhos da Área de Trabalho, Menu Iniciar, Registro do Windows, pasta instalada e todos os dados em %APPDATA%."
            },
            12,
            c.muted,
        ),
    ]
    .spacing(16);

    if state.purge_busy {
        content = content.push(ui::busy(
            env,
            if is_en {
                "Purging everything from PC..."
            } else {
                "Apagando tudo do computador..."
            },
        ));
    }

    content = content.push(btns);

    ui::card(env, content)
}

fn step_uninstalled<'a>(state: &'a State, env: &Env) -> Element<'a, Msg> {
    let is_en = env.lang.is_en();
    let mut col = column![
        ui::strong(
            if is_en {
                "Cleanup Completed"
            } else {
                "Desinstalação / Limpeza Concluída"
            },
            22
        ),
        ui::muted(
            env,
            if is_en {
                "All requested files, shortcuts, and data have been removed from your PC."
            } else {
                "Todos os arquivos, atalhos e dados solicitados do Axolote Suite foram removidos."
            }
        ),
    ]
    .spacing(14);

    if let Some(res) = &state.result {
        col = col.push(ui::result_banner(env, res, None));
    }

    col = col.push(ui::actions(
        env,
        vec![act(
            if is_en {
                "Back to Setup Wizard"
            } else {
                "Voltar ao Assistente"
            },
            Btn::Secondary,
            Some(Msg::GoToStep(WizardStep::Welcome)),
        )],
    ));

    ui::card(env, col)
}
