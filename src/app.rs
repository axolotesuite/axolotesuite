//! Aplicação principal do Axolote Suite:
//! - Barra lateral com animação suave de expansão/recolhimento (60 FPS) e
//!   indicador Neon animado no item selecionado
//! - Calculadora como item direto do menu lateral (e também em Produtividade)
//! - Barra superior com busca rápida de ferramentas (`🔍 Pesquisar ferramenta…`)
//! - Painel de boas-vindas com o banner do mascote, Central de Ferramentas,
//!   Status do Sistema e Últimas Novidades
//! - Roteamento para todos os módulos, incluindo Editor de Fotos, Editor de Vídeo
//!   e Instalador / Desinstalador integrado como tela inicial de Setup.

use std::path::PathBuf;
use std::time::Instant;

use iced::widget::{
    button, column, container, image as iced_image, progress_bar, row, scrollable, text, Space,
};
use iced::{Alignment, Application, Color, Command, Element, Event, Length, Subscription, Theme};

use crate::core::Ctx;
use crate::modules::{
    audio, batch_rename, devtools, history, images, installer, pdf, productivity, queue,
    quick_convert, settings, video, zip,
};
use crate::shared::brand;
use crate::shared::theme::{self, Btn};
use crate::shared::types::Env;
use crate::shared::ui::{self, BOLD, SEMIBOLD};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Module {
    Home,
    QuickConvert,
    Productivity,
    Calculator,
    Pdf,
    Images,
    Video,
    Audio,
    BatchRename,
    Zip,
    DevTools,
    Queue,
    History,
    Settings,
    Installer,
}

impl Module {
    pub fn icon(&self) -> &'static str {
        match self {
            Module::Home => "🏠",
            Module::QuickConvert => "🔄",
            Module::Productivity => "📋",
            Module::Calculator => "🖩",
            Module::Pdf => "📄",
            Module::Images => "🖼",
            Module::Video => "🎬",
            Module::Audio => "🎵",
            Module::BatchRename => "📁",
            Module::Zip => "🗜",
            Module::DevTools => "🎨",
            Module::Queue => "📦",
            Module::History => "🕒",
            Module::Settings => "⚙",
            Module::Installer => "💿",
        }
    }

    pub fn title_in(&self, lang: crate::shared::i18n::Lang, is_installed: bool) -> &'static str {
        if lang.is_en() {
            match self {
                Module::Home => "Home",
                Module::QuickConvert => "Quick Convert",
                Module::Productivity => "Productivity",
                Module::Calculator => "Calculator",
                Module::Pdf => "PDF",
                Module::Images => "Photos & Images",
                Module::Video => "Videos",
                Module::Audio => "Audio",
                Module::BatchRename => "Batch Files",
                Module::Zip => "ZIP Archiver",
                Module::DevTools => "Colors & Passwords",
                Module::Queue => "Tasks Queue",
                Module::History => "History",
                Module::Settings => "Settings",
                Module::Installer => {
                    if is_installed {
                        "Apagar Tudo / Cleanup"
                    } else {
                        "Install / Setup"
                    }
                }
            }
        } else {
            match self {
                Module::Installer => {
                    if is_installed {
                        "Apagar Tudo / Limpar"
                    } else {
                        "Instalar / Setup"
                    }
                }
                _ => self.title(),
            }
        }
    }

    pub fn title(&self) -> &'static str {
        match self {
            Module::Home => "Início",
            Module::QuickConvert => "Conversor Fácil",
            Module::Productivity => "Produtividade",
            Module::Calculator => "Calculadora",
            Module::Pdf => "PDF",
            Module::Images => "Imagens & Foto",
            Module::Video => "Vídeos",
            Module::Audio => "Áudio",
            Module::BatchRename => "Arquivos em Lote",
            Module::Zip => "ZIP (Compactador)",
            Module::DevTools => "Cores & Senhas",
            Module::Queue => "Fila de Operações",
            Module::History => "Histórico",
            Module::Settings => "Configurações",
            Module::Installer => "Instalar / Setup",
        }
    }

    pub fn summary_in(&self, lang: crate::shared::i18n::Lang) -> &'static str {
        if lang.is_en() {
            match self {
                Module::Home => "Tools dashboard, fast search, and system overview.",
                Module::QuickConvert => {
                    "Convert images, text, documents (DOCX/PDF), videos, audio, and ZIP."
                }
                Module::Productivity => {
                    "Quick notepad, tasks checklist, and scientific calculator."
                }
                Module::Calculator => {
                    "Scientific expression calculator with parentheses and history."
                }
                Module::Pdf => {
                    "Merge multiple PDFs, split pages and intervals, and inspect metadata."
                }
                Module::Images => {
                    "Live photo developer with RGB histogram, effects, and batch folder sync."
                }
                Module::Video => {
                    "Trim timeline clips, change speed, apply cinema looks, and CRF compression."
                }
                Module::Audio => {
                    "Convert audio formats, extract soundtracks from video, and adjust bitrate."
                }
                Module::BatchRename => {
                    "Bulk rename files with prefix, suffix, replace, and live preview."
                }
                Module::Zip => {
                    "Compress directories or extract archives safely with anti Zip-Slip shield."
                }
                Module::DevTools => {
                    "HEX/RGB/HSL/CMYK converter, photo palette picker, and strong password/UUID."
                }
                Module::Queue => "Monitor active background tasks without freezing the UI.",
                Module::History => "Complete audit log of operations performed on your PC.",
                Module::Settings => {
                    "Dark/light theme, auto-open output, language detection, and privacy."
                }
                Module::Installer => {
                    "Install essentials on PC + auto FFmpeg setup or uninstall cleanly."
                }
            }
        } else {
            self.summary()
        }
    }

    pub fn summary(&self) -> &'static str {
        match self {
            Module::Home => "Central de ferramentas, busca rápida e visão geral.",
            Module::QuickConvert => {
                "Converta imagens, textos, documentos (DOCX/PDF), vídeos, áudios e ZIP."
            }
            Module::Productivity => {
                "Bloco de notas (.txt), lista de tarefas e organizador de pastas."
            }
            Module::Calculator => {
                "Calculadora expressa com parênteses, potência, porcentagem e histórico."
            }
            Module::Pdf => "Junte, divida em páginas e consulte informações de PDFs.",
            Module::Images => {
                "Editor de fotos ao vivo (luz, filtros, corte), converter e comprimir."
            }
            Module::Video => {
                "Editor rápido de vídeo (corte, velocidade, 9:16, filtros) e conversor."
            }
            Module::Audio => "Converta, extraia trilha de vídeos e comprima áudios.",
            Module::BatchRename => "Renomeie arquivos em lote com regras e pré-visualização.",
            Module::Zip => "Compacte, descompacte e inspecione arquivos ZIP com segurança.",
            Module::DevTools => {
                "Estúdio de cores, paleta de fotos, gerador de senhas fortes e UUID."
            }
            Module::Queue => "Acompanhe as operações em segundo plano sem travar a tela.",
            Module::History => "Registro completo de atividades realizadas no aplicativo.",
            Module::Settings => "Tema escuro/claro, abertura automática de pastas e privacidade.",
            Module::Installer => "Instale o essencial no PC + FFmpeg automático ou desinstale.",
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    Navigate(Module),
    OpenQuickConvertShortcut,
    GlobalGestureTriggered(bool),
    QuickLaunchProductivity(productivity::Tab),
    SearchChanged(String),
    SearchSubmitted,
    ToggleSidebar,
    ToggleTheme,
    WindowResized(u32, u32),
    FileDropped(PathBuf),
    AnimationFrame(Instant),
    QuickConvert(quick_convert::Msg),
    Productivity(productivity::Msg),
    BatchRename(batch_rename::Msg),
    Zip(zip::Msg),
    Images(images::Msg),
    Pdf(pdf::Msg),
    Audio(audio::Msg),
    Video(video::Msg),
    DevTools(devtools::Msg),
    Queue(queue::Msg),
    History(history::Msg),
    Settings(settings::Msg),
    Installer(installer::Msg),
}

pub struct AxoloteSuite {
    pub ctx: Ctx,
    pub active: Module,
    pub mandatory_setup: bool,
    pub home_search: String,
    pub window_width: f32,
    pub window_height: f32,
    pub sidebar_collapsed: Option<bool>,
    // Estado de animação fluida do menu lateral (0.0 = recolhido, 1.0 = expandido)
    pub sidebar_open_t: f32,
    // Animação de pulso na seleção de item do menu (1.0 -> 0.0)
    pub nav_pulse: f32,
    pub last_tick: Option<Instant>,
    // Estados persistentes de cada módulo
    pub quick_convert: quick_convert::State,
    pub productivity: productivity::State,
    pub batch_rename: batch_rename::State,
    pub zip: zip::State,
    pub images: images::State,
    pub pdf: pdf::State,
    pub audio: audio::State,
    pub video: video::State,
    pub devtools: devtools::State,
    pub queue: queue::State,
    pub history: history::State,
    pub settings: settings::State,
    pub installer: installer::State,
}

impl Application for AxoloteSuite {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Self::Message>) {
        let ctx = Ctx::load();
        let args: Vec<String> = std::env::args().collect();

        let installer_state = installer::State::new(installer::WizardStep::Welcome);
        let _running_outside_installed_dir = std::env::current_exe()
            .ok()
            .and_then(|p| p.parent().map(|d| d.to_path_buf()))
            .map(|exe_dir| {
                let inst_dir = std::path::PathBuf::from(installer_state.install_dir.trim());
                !exe_dir
                    .to_string_lossy()
                    .eq_ignore_ascii_case(&inst_dir.to_string_lossy())
            })
            .unwrap_or(true);

        let (mut initial_module, installer_step, mandatory_setup) =
            if args.iter().any(|a| a == "--uninstall") {
                (
                    Module::Installer,
                    installer::WizardStep::UninstallConfirm,
                    false,
                )
            } else if args.iter().any(|a| a == "--install" || a == "--setup") {
                (Module::Installer, installer::WizardStep::Welcome, false)
            } else {
                (Module::Home, installer::WizardStep::Welcome, false)
            };

        let mut qc_state = quick_convert::State::new();
        for arg in args.iter().skip(1) {
            if !arg.starts_with("--") {
                let p = PathBuf::from(arg);
                if p.is_file() {
                    qc_state.push_dropped_file(p, ctx.settings.dark_mode);
                    if !mandatory_setup {
                        initial_module = Module::QuickConvert;
                    }
                }
            }
        }

        (
            Self {
                ctx,
                active: initial_module,
                mandatory_setup,
                home_search: String::new(),
                window_width: 1380.0,
                window_height: 880.0,
                sidebar_collapsed: None,
                sidebar_open_t: 1.0,
                nav_pulse: 1.0,
                last_tick: None,
                quick_convert: qc_state,
                productivity: productivity::State::new(),
                batch_rename: batch_rename::State::new(),
                zip: zip::State::new(),
                images: images::State::new(),
                pdf: pdf::State::new(),
                audio: audio::State::new(),
                video: video::State::new(),
                devtools: devtools::State::new(),
                queue: queue::State,
                history: history::State::default(),
                settings: settings::State::default(),
                installer: installer::State::new(installer_step),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        format!(
            "Axolote Suite v{} — {}",
            env!("CARGO_PKG_VERSION"),
            self.active.title()
        )
    }

    fn theme(&self) -> Theme {
        theme::app_theme(self.ctx.settings.dark_mode)
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        let win_sub = iced::event::listen_with(|event, _status| match event {
            Event::Window(_, iced::window::Event::Resized { width, height }) => {
                Some(Message::WindowResized(width, height))
            }
            Event::Window(_, iced::window::Event::FileDropped(path)) => {
                Some(Message::FileDropped(path))
            }
            Event::Keyboard(iced::keyboard::Event::KeyPressed {
                key: iced::keyboard::Key::Character(ref c),
                modifiers,
                ..
            }) if modifiers.control() && modifiers.alt() && c.eq_ignore_ascii_case("n") => {
                Some(Message::OpenQuickConvertShortcut)
            }
            _ => None,
        });

        let gesture_sub =
            crate::core::system::global_gesture_subscription().map(Message::GlobalGestureTriggered);

        if self.is_animating() {
            Subscription::batch([
                win_sub,
                gesture_sub,
                iced::window::frames().map(Message::AnimationFrame),
            ])
        } else {
            Subscription::batch([win_sub, gesture_sub])
        }
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::Navigate(m) => {
                if m == Module::Calculator {
                    self.productivity.tab = productivity::Tab::Calc;
                }
                self.active = m;
                self.home_search.clear();
                self.nav_pulse = 1.0;
                self.last_tick = Some(Instant::now());
                Command::none()
            }
            Message::OpenQuickConvertShortcut => {
                if !self.mandatory_setup {
                    self.active = if self.active == Module::QuickConvert {
                        Module::Home
                    } else {
                        Module::QuickConvert
                    };
                    self.nav_pulse = 1.0;
                    self.last_tick = Some(Instant::now());
                }
                Command::none()
            }
            Message::GlobalGestureTriggered(is_drag) => {
                if !self.mandatory_setup {
                    if is_drag {
                        self.active = Module::QuickConvert;
                    } else {
                        self.active = if self.active == Module::QuickConvert {
                            Module::Home
                        } else {
                            Module::QuickConvert
                        };
                    }
                    self.nav_pulse = 1.0;
                    self.last_tick = Some(Instant::now());
                }
                Command::none()
            }
            Message::QuickLaunchProductivity(tab) => {
                self.productivity.tab = tab;
                self.active = if tab == productivity::Tab::Calc {
                    Module::Calculator
                } else {
                    Module::Productivity
                };
                self.nav_pulse = 1.0;
                self.last_tick = Some(Instant::now());
                Command::none()
            }
            Message::SearchChanged(q) => {
                self.home_search = q;
                Command::none()
            }
            Message::SearchSubmitted => {
                let q = self.home_search.trim();
                if !q.is_empty() {
                    let matches = self.search_matches(q);
                    if let Some((first_mod, _, _)) = matches.first() {
                        let m = *first_mod;
                        if m == Module::Calculator {
                            self.productivity.tab = productivity::Tab::Calc;
                        }
                        self.active = m;
                        self.home_search.clear();
                        self.nav_pulse = 1.0;
                        self.last_tick = Some(Instant::now());
                    }
                }
                Command::none()
            }
            Message::ToggleSidebar => {
                let current = self.is_sidebar_collapsed();
                self.sidebar_collapsed = Some(!current);
                self.last_tick = Some(Instant::now());
                Command::none()
            }
            Message::ToggleTheme => {
                self.ctx.settings.dark_mode = !self.ctx.settings.dark_mode;
                self.ctx.settings.save();
                Command::none()
            }
            Message::WindowResized(w, h) => {
                self.window_width = (w as f32).max(360.0);
                self.window_height = (h as f32).max(320.0);
                self.last_tick = Some(Instant::now());
                Command::none()
            }
            Message::FileDropped(path) => {
                if !self.mandatory_setup {
                    self.quick_convert
                        .push_dropped_file(path, self.ctx.settings.dark_mode);
                    self.active = Module::QuickConvert;
                    self.nav_pulse = 1.0;
                    self.last_tick = Some(Instant::now());
                }
                Command::none()
            }
            Message::AnimationFrame(now) => {
                let dt = self
                    .last_tick
                    .map(|prev| now.saturating_duration_since(prev).as_secs_f32().min(0.05))
                    .unwrap_or(0.016);
                self.last_tick = Some(now);

                let target_t = if self.is_sidebar_collapsed() {
                    0.0
                } else {
                    1.0
                };
                let diff = target_t - self.sidebar_open_t;
                if diff.abs() < 0.01 {
                    self.sidebar_open_t = target_t;
                } else {
                    // Interpolação exponencial suave (ease-out)
                    self.sidebar_open_t += diff * (dt * 14.0).min(1.0);
                }

                if self.nav_pulse > 0.0 {
                    self.nav_pulse = (self.nav_pulse - dt * 3.2).max(0.0);
                }
                if !self.is_animating() {
                    self.last_tick = None;
                }
                Command::none()
            }
            Message::QuickConvert(m) => self
                .quick_convert
                .update(m, &mut self.ctx)
                .map(Message::QuickConvert),
            Message::Productivity(m) => self
                .productivity
                .update(m, &mut self.ctx)
                .map(Message::Productivity),
            Message::BatchRename(m) => self
                .batch_rename
                .update(m, &mut self.ctx)
                .map(Message::BatchRename),
            Message::Zip(m) => self.zip.update(m, &mut self.ctx).map(Message::Zip),
            Message::Images(m) => self.images.update(m, &mut self.ctx).map(Message::Images),
            Message::Pdf(m) => self.pdf.update(m, &mut self.ctx).map(Message::Pdf),
            Message::Audio(m) => self.audio.update(m, &mut self.ctx).map(Message::Audio),
            Message::Video(m) => self.video.update(m, &mut self.ctx).map(Message::Video),
            Message::DevTools(m) => self
                .devtools
                .update(m, &mut self.ctx)
                .map(Message::DevTools),
            Message::Queue(m) => self.queue.update(m, &mut self.ctx).map(Message::Queue),
            Message::History(m) => self.history.update(m, &mut self.ctx).map(Message::History),
            Message::Settings(m) => self
                .settings
                .update(m, &mut self.ctx)
                .map(Message::Settings),
            Message::Installer(m) => {
                let go_home = matches!(m, installer::Msg::FinishAndGoHome);
                let refresh_ff = matches!(
                    m,
                    installer::Msg::InstallFinished(_) | installer::Msg::DownloadFfmpegFinished(_)
                );
                let cmd = self
                    .installer
                    .update(m, &mut self.ctx)
                    .map(Message::Installer);
                if refresh_ff {
                    let ok = crate::core::system::find_ffmpeg().is_some();
                    self.video.ffmpeg_ok = ok;
                    self.audio.ffmpeg_ok = ok;
                }
                if go_home {
                    self.mandatory_setup = false;
                    self.active = Module::Home;
                    self.nav_pulse = 1.0;
                    self.last_tick = Some(Instant::now());
                }
                cmd
            }
        }
    }

    fn view(&self) -> Element<'_, Self::Message> {
        // Enquanto o setup obrigatório não foi concluído, mostra APENAS a tela do instalador em tela cheia!
        if self.mandatory_setup {
            let env = Env {
                width: self.window_width.max(320.0),
                dark: self.ctx.settings.dark_mode,
                lang: self.ctx.settings.resolved_lang(),
            };
            let content =
                installer::view::view(&self.installer, true, &env).map(Message::Installer);
            return container(content)
                .width(Length::Fill)
                .height(Length::Fill)
                .style(theme::app_bg)
                .into();
        }

        // Largura interpolada continuamente entre 78px e 258px
        let sidebar_w = 78.0 + (258.0 - 78.0) * self.sidebar_open_t;
        let show_labels = self.sidebar_open_t > 0.55;
        let env = Env {
            width: (self.window_width - sidebar_w).max(320.0),
            dark: self.ctx.settings.dark_mode,
            lang: self.ctx.settings.resolved_lang(),
        };

        let sidebar = self.sidebar_view(!show_labels, sidebar_w, &env);
        let content = match self.active {
            Module::Home => self.home_view(&env),
            Module::QuickConvert => {
                quick_convert::view::view(&self.quick_convert, &env).map(Message::QuickConvert)
            }
            Module::Productivity => {
                productivity::view::view(&self.productivity, &env).map(Message::Productivity)
            }
            Module::Calculator => self.calculator_standalone_view(&env),
            Module::BatchRename => {
                batch_rename::view::view(&self.batch_rename, &env).map(Message::BatchRename)
            }
            Module::Zip => zip::view::view(&self.zip, &env).map(Message::Zip),
            Module::Images => images::view::view(&self.images, &env).map(Message::Images),
            Module::Pdf => pdf::view::view(&self.pdf, &env).map(Message::Pdf),
            Module::Audio => audio::view::view(&self.audio, &env).map(Message::Audio),
            Module::Video => video::view::view(&self.video, &env).map(Message::Video),
            Module::DevTools => devtools::view::view(&self.devtools, &env).map(Message::DevTools),
            Module::Queue => queue::view::view(&self.ctx.jobs, &env).map(Message::Queue),
            Module::History => {
                history::view::view(&self.history, &self.ctx.history, &env).map(Message::History)
            }
            Module::Settings => settings::view::view(&self.settings, &self.ctx.settings, &env)
                .map(Message::Settings),
            Module::Installer => {
                installer::view::view(&self.installer, false, &env).map(Message::Installer)
            }
        };

        container(row![
            sidebar,
            container(content).width(Length::Fill).height(Length::Fill)
        ])
        .width(Length::Fill)
        .height(Length::Fill)
        .style(theme::app_bg)
        .into()
    }
}

impl AxoloteSuite {
    fn is_sidebar_collapsed(&self) -> bool {
        self.sidebar_collapsed.unwrap_or(self.window_width < 940.0)
    }

    fn is_animating(&self) -> bool {
        let target_t = if self.is_sidebar_collapsed() {
            0.0
        } else {
            1.0
        };
        (self.sidebar_open_t - target_t).abs() > 0.005 || self.nav_pulse > 0.005
    }

    fn calculator_standalone_view<'a>(&'a self, env: &Env) -> Element<'a, Message> {
        let header = ui::page_header(
            env,
            "🖩",
            "Calculadora Científica Expressa",
            "Faça contas rápidas ou expressões completas com parênteses, potência (^), porcentagem (%) e histórico clicável.",
            None,
        );
        let body =
            productivity::view::calc_view(&self.productivity, env).map(Message::Productivity);
        ui::page(env, vec![header, body])
    }

    fn sidebar_view(&self, collapsed: bool, width: f32, env: &Env) -> Element<'_, Message> {
        let c = env.c();
        let running = self.ctx.jobs.running();

        // Cabeçalho com a Logo Oficial do Axolote Suite estilizada com brilho Cyber-Fluent
        let brand_header: Element<'_, Message> = if collapsed {
            column![
                button(
                    container(
                        iced_image::Image::new(brand::logo_small())
                            .width(Length::Fixed(34.0))
                            .height(Length::Fixed(34.0)),
                    )
                    .padding(3)
                    .style(theme::logo_container)
                    .center_x(),
                )
                .padding(2)
                .style(theme::btn(Btn::Ghost))
                .on_press(Message::Navigate(Module::Home)),
                button(container(text("»").size(15)).center_x().width(Length::Fill))
                    .padding([3, 6])
                    .style(theme::btn(Btn::Ghost))
                    .on_press(Message::ToggleSidebar),
            ]
            .spacing(4)
            .align_items(Alignment::Center)
            .into()
        } else {
            row![
                button(
                    row![
                        container(
                            iced_image::Image::new(brand::logo_small())
                                .width(Length::Fixed(36.0))
                                .height(Length::Fixed(36.0)),
                        )
                        .padding(3)
                        .style(theme::logo_container),
                        column![
                            row![
                                text("Axolote ").size(16).font(BOLD),
                                ui::colored("Suite", 16, c.accent).font(BOLD),
                            ],
                            text(concat!("v", env!("CARGO_PKG_VERSION"), " • 100% Offline"))
                                .size(11)
                                .style(c.muted),
                        ]
                        .spacing(1),
                    ]
                    .spacing(10)
                    .align_items(Alignment::Center),
                )
                .padding([4, 6])
                .style(theme::btn(Btn::Ghost))
                .on_press(Message::Navigate(Module::Home)),
                ui::fill(),
                button(text("«").size(16))
                    .padding([6, 10])
                    .style(theme::btn(Btn::Ghost))
                    .on_press(Message::ToggleSidebar),
            ]
            .align_items(Alignment::Center)
            .into()
        };

        let groups: &[(&str, &[Module])] = if env.lang.is_en() {
            &[
                (
                    "TOOLS WORKSTATION",
                    &[
                        Module::Home,
                        Module::QuickConvert,
                        Module::Productivity,
                        Module::Calculator,
                        Module::Pdf,
                        Module::Images,
                        Module::Video,
                        Module::Audio,
                        Module::BatchRename,
                        Module::Zip,
                        Module::DevTools,
                    ],
                ),
                (
                    "SYSTEM",
                    &[Module::Queue, Module::History, Module::Settings],
                ),
            ]
        } else {
            &[
                (
                    "CENTRAL DE FERRAMENTAS",
                    &[
                        Module::Home,
                        Module::QuickConvert,
                        Module::Productivity,
                        Module::Calculator,
                        Module::Pdf,
                        Module::Images,
                        Module::Video,
                        Module::Audio,
                        Module::BatchRename,
                        Module::Zip,
                        Module::DevTools,
                    ],
                ),
                (
                    "SISTEMA",
                    &[Module::Queue, Module::History, Module::Settings],
                ),
            ]
        };

        let mut nav = column![].spacing(5).width(Length::Fill);

        for (group_title, mods) in groups {
            if !collapsed {
                nav = nav.push(
                    container(text(*group_title).size(10).font(BOLD).style(c.muted))
                        .padding([10, 10, 2, 10]),
                );
            } else {
                nav = nav.push(Space::with_height(8));
            }
            for &m in *mods {
                let is_active = self.active == m;
                // Altura da barra indicadora Neon animada quando o item é selecionado
                let pill_h = if is_active {
                    16.0 + 8.0 * self.nav_pulse
                } else {
                    0.0
                };
                let pill_ind: Element<'_, Message> = if is_active {
                    container(Space::new(Length::Fixed(4.0), Length::Fixed(pill_h)))
                        .style(theme::active_pill(self.nav_pulse))
                        .into()
                } else {
                    Space::with_width(Length::Fixed(4.0)).into()
                };

                let item_content: Element<'_, Message> = if collapsed {
                    row![
                        pill_ind,
                        container(text(m.icon()).size(if is_active { 21 } else { 19 }))
                            .center_x()
                            .width(Length::Fill),
                    ]
                    .align_items(Alignment::Center)
                    .into()
                } else {
                    // Deslocamento suave do texto ao selecionar (efeito slide)
                    let indent = if is_active {
                        4.0 * (1.0 - self.nav_pulse * 0.5)
                    } else {
                        0.0
                    };
                    let mut r = row![
                        pill_ind,
                        Space::with_width(Length::Fixed(indent)),
                        text(m.icon()).size(if is_active { 18 } else { 16 }),
                        text(m.title_in(env.lang, self.installer.is_installed()))
                            .size(13)
                            .font(if is_active { BOLD } else { SEMIBOLD })
                            .width(Length::Fill),
                    ]
                    .spacing(9)
                    .align_items(Alignment::Center);

                    if m == Module::Queue && running > 0 {
                        r = r.push(ui::badge(running, c.accent));
                    } else if m == Module::Productivity && self.productivity.pending_tasks() > 0 {
                        r = r.push(ui::badge(self.productivity.pending_tasks(), c.primary));
                    } else if is_active {
                        r = r.push(ui::colored("›", 16, c.accent).font(BOLD));
                    }
                    r.into()
                };

                nav = nav.push(
                    button(item_content)
                        .padding(if collapsed { [10, 6] } else { [9, 10] })
                        .width(Length::Fill)
                        .style(theme::btn(Btn::Nav(is_active)))
                        .on_press(Message::Navigate(m)),
                );
            }
        }

        let theme_label = if collapsed {
            if self.ctx.settings.dark_mode {
                "☀"
            } else {
                "🌙"
            }
        } else if self.ctx.settings.dark_mode {
            "☀  Tema Claro"
        } else {
            "🌙  Tema Escuro"
        };

        let footer = button(
            container(text(theme_label).size(12).font(SEMIBOLD))
                .center_x()
                .width(Length::Fill),
        )
        .padding([10, 10])
        .width(Length::Fill)
        .style(theme::btn(Btn::Secondary))
        .on_press(Message::ToggleTheme);

        let col = column![
            brand_header,
            Space::with_height(6),
            scrollable(nav).height(Length::Fill),
            footer,
        ]
        .spacing(8)
        .padding(if collapsed { 10 } else { 14 })
        .width(Length::Fixed(width))
        .height(Length::Fill);

        container(col)
            .width(Length::Fixed(width))
            .height(Length::Fill)
            .style(theme::sidebar)
            .into()
    }

    fn home_view<'a>(&'a self, env: &Env) -> Element<'a, Message> {
        let c = env.c();

        // 1. Barra Superior Cyber-Fluent (Idêntica ao Header do Stitch, 100% Responsiva)
        let top_bar: Element<'a, Message> = {
            let search_ph = if env.lang.is_en() {
                "Search tools, files or actions... (Enter to open)"
            } else {
                "Buscar ferramenta, arquivo ou ação... (Enter para abrir)"
            };
            let search_input = ui::input(search_ph, &self.home_search, Message::SearchChanged)
                .on_submit(Message::SearchSubmitted)
                .width(if env.width < 960.0 {
                    Length::Fill
                } else {
                    Length::Fixed(400.0)
                });

            let search: Element<'a, Message> = if !self.home_search.is_empty() {
                row![
                    search_input,
                    ui::btn("✕", Btn::Ghost, Some(Message::SearchChanged(String::new()))),
                ]
                .spacing(4)
                .align_items(Alignment::Center)
                .into()
            } else {
                search_input.into()
            };

            let brand_tag = row![
                container(
                    iced_image::Image::new(brand::logo_small())
                        .width(Length::Fixed(28.0))
                        .height(Length::Fixed(28.0)),
                )
                .padding(2)
                .style(theme::logo_container),
                ui::strong("Axolote", 18),
                container(text("v1.4.0").size(10).font(BOLD).style(c.primary))
                    .padding([2, 6])
                    .style(theme::solid_box(Color::from_rgb8(42, 41, 49), 4.0)),
                ui::btn("🛡️ LGPD Local", Btn::Secondary, None),
            ]
            .spacing(8)
            .align_items(Alignment::Center);

            let telemetry_pill = container(
                row![
                    container(Space::new(Length::Fixed(7.0), Length::Fixed(7.0)))
                        .style(theme::solid_box(c.success, 999.0)),
                    ui::colored("GPU Accel", 11, c.accent).font(BOLD),
                    text("•").size(11).style(c.muted),
                    text("38 MB RAM").size(11).style(c.text),
                    text("•").size(11).style(c.muted),
                    text("Offline").size(11).style(c.muted),
                ]
                .spacing(6)
                .align_items(Alignment::Center),
            )
            .padding([5, 12])
            .style(theme::inner_box);

            if env.width < 960.0 {
                column![
                    row![brand_tag, ui::fill(), telemetry_pill]
                        .spacing(8)
                        .align_items(Alignment::Center),
                    search,
                ]
                .spacing(10)
                .into()
            } else {
                row![
                    brand_tag,
                    Space::with_width(Length::Fixed(12.0)),
                    search,
                    ui::fill(),
                    telemetry_pill,
                ]
                .spacing(12)
                .align_items(Alignment::Center)
                .into()
            }
        };

        // Bento Card Principal (Col Span 2): Conversão & Despacho Rápido com Drag & Drop
        let hero_card = ui::bento_card(
            env,
            column![
                // Cabeçalho
                row![
                    row![
                        container(text("⇄").size(18).font(BOLD).style(c.accent))
                            .padding([6, 10])
                            .style(theme::solid_box(Color::from_rgb8(31, 31, 38), 8.0)),
                        column![
                            ui::strong("Conversão & Despacho Rápido", 17),
                            ui::muted(env, "Processamento atômico sem upload, bufferizado em memória RAM").size(12),
                        ]
                        .spacing(2),
                    ]
                    .spacing(10)
                    .align_items(Alignment::Center),
                    ui::fill(),
                    container(text("Rayon SIMD").size(10).font(BOLD).style(c.accent))
                        .padding([3, 8])
                        .style(theme::inner_box),
                ]
                .align_items(Alignment::Center),
                Space::with_height(10),
                // Área Central Interativa de Drag & Drop
                button(
                    container(
                        column![
                            container(text("☁ ⬆").size(24).style(c.primary))
                                .padding([12, 16])
                                .style(theme::solid_box(Color::from_rgb8(31, 25, 48), 999.0)),
                            ui::strong("Solte arquivos aqui para processamento instantâneo", 15),
                            ui::muted(
                                env,
                                "Vídeos MP4/MKV, Imagens RAW/AVIF, Áudios FLAC/WAV ou documentos PDF criptografados",
                            )
                            .size(12),
                            Space::with_height(8),
                            // Quick Action Buttons
                            row![
                                ui::btn("📹  Comprimir H.265", Btn::Secondary, Some(Message::Navigate(Module::Video))),
                                ui::btn("🖼  Converter WebP/AVIF", Btn::Secondary, Some(Message::Navigate(Module::QuickConvert))),
                                ui::btn("🎧  Remover Chiado", Btn::Secondary, Some(Message::Navigate(Module::Audio))),
                            ]
                            .spacing(8),
                        ]
                        .spacing(6)
                        .align_items(Alignment::Center)
                        .width(Length::Fill),
                    )
                    .padding([22, 16])
                    .width(Length::Fill)
                    .style(theme::dropzone_box),
                )
                .style(theme::btn(Btn::Ghost))
                .on_press(Message::Navigate(Module::QuickConvert)),
                Space::with_height(8),
                // Rodapé do Card
                row![
                    row![
                        container(Space::new(Length::Fixed(6.0), Length::Fixed(6.0)))
                            .style(theme::solid_box(c.success, 999.0)),
                        ui::muted(env, "Motor Rust image & FFmpeg integrado").size(11),
                    ]
                    .spacing(6)
                    .align_items(Alignment::Center),
                    ui::fill(),
                    ui::btn("Configurar perfis de exportação ➔", Btn::Ghost, Some(Message::Navigate(Module::QuickConvert))),
                ]
                .align_items(Alignment::Center),
            ]
            .spacing(6),
        );

        // 4. Card 2: Revelador RAW & Foto Rápida
        let bento_raw = ui::bento_card(
            env,
            column![
                row![
                    row![
                        container(text("📷").size(16).style(c.primary))
                            .padding([5, 8])
                            .style(theme::inner_box),
                        ui::strong("Revelador RAW", 15),
                    ]
                    .spacing(8)
                    .align_items(Alignment::Center),
                    ui::fill(),
                    container(text("16-bit float").size(10).style(c.muted))
                        .padding([2, 6])
                        .style(theme::inner_box),
                ]
                .align_items(Alignment::Center),
                Space::with_height(6),
                // Mini Live Image Preview com Imagem do Axolote + Overlay da Onda Espectral
                container(column![row![
                    iced_image::Image::new(brand::logo_banner())
                        .width(Length::Fixed(80.0))
                        .height(Length::Fixed(70.0)),
                    column![
                        row![
                            ui::colored("RGB Spectral Wave", 10, c.accent).font(BOLD),
                            ui::fill(),
                            ui::colored("ISO 100 • 1/250s", 10, c.success).font(BOLD),
                        ]
                        .align_items(Alignment::Center),
                        Space::with_height(4),
                        // Onda espectral simulada
                        row![
                            container(Space::new(Length::Fixed(32.0), Length::Fixed(4.0)))
                                .style(theme::solid_box(Color::from_rgb8(239, 68, 68), 2.0)),
                            container(Space::new(Length::Fixed(48.0), Length::Fixed(4.0)))
                                .style(theme::solid_box(Color::from_rgb8(16, 185, 129), 2.0)),
                            container(Space::new(Length::Fixed(40.0), Length::Fixed(4.0)))
                                .style(theme::solid_box(Color::from_rgb8(59, 130, 246), 2.0)),
                        ]
                        .spacing(4),
                        Space::with_height(4),
                        ui::muted(env, "Histograma dinâmico ativado").size(10),
                    ]
                    .spacing(2)
                    .width(Length::Fill),
                ]
                .spacing(10)
                .align_items(Alignment::Center),],)
                .padding(8)
                .style(theme::inner_box),
                Space::with_height(6),
                ui::muted(env, "LUT INSTANTÂNEA").size(10),
                row![
                    button(
                        container(text("Neon Teal").size(11))
                            .center_x()
                            .width(Length::Fill)
                    )
                    .padding([6, 6])
                    .width(Length::FillPortion(1))
                    .style(theme::btn(Btn::Secondary))
                    .on_press(Message::Navigate(Module::Images)),
                    button(
                        container(text("Cyberpunk").size(11).font(BOLD))
                            .center_x()
                            .width(Length::Fill)
                    )
                    .padding([6, 6])
                    .width(Length::FillPortion(1))
                    .style(theme::btn(Btn::Primary))
                    .on_press(Message::Navigate(Module::Images)),
                    button(
                        container(text("Moody 35mm").size(11))
                            .center_x()
                            .width(Length::Fill)
                    )
                    .padding([6, 6])
                    .width(Length::FillPortion(1))
                    .style(theme::btn(Btn::Secondary))
                    .on_press(Message::Navigate(Module::Images)),
                ]
                .spacing(6)
                .width(Length::Fill),
            ]
            .spacing(4),
        );

        // 5. Card 3: Fila de Render & Hardware Monitor
        let bento_render = ui::bento_card(
            env,
            column![
                row![
                    row![
                        container(text("⚡").size(16).style(c.success))
                            .padding([5, 8])
                            .style(theme::inner_box),
                        ui::strong("Fila de Render", 15),
                    ]
                    .spacing(8)
                    .align_items(Alignment::Center),
                    ui::fill(),
                    container(text("8 Threads").size(10).font(BOLD).style(c.success))
                        .padding([2, 6])
                        .style(theme::inner_box),
                ]
                .align_items(Alignment::Center),
                Space::with_height(6),
                // Task 1
                container(
                    column![
                        row![
                            ui::strong("projeto_vfx_4k_crf18.mp4", 12).width(Length::Fill),
                            ui::colored("78%", 11, c.accent).font(BOLD),
                        ],
                        progress_bar(0.0..=100.0, 78.0).height(Length::Fixed(6.0)),
                        row![
                            text("NVENC AV1").size(10).style(c.muted),
                            ui::fill(),
                            text("ETA: 4s").size(10).style(c.muted),
                        ],
                    ]
                    .spacing(3),
                )
                .padding(8)
                .style(theme::inner_box),
                Space::with_height(4),
                // Task 2
                container(
                    column![
                        row![
                            ui::strong("backup_raw_session.zip", 12).width(Length::Fill),
                            ui::colored("42%", 11, c.success).font(BOLD),
                        ],
                        progress_bar(0.0..=100.0, 42.0).height(Length::Fixed(6.0)),
                        row![
                            text("Zstandard -19").size(10).style(c.muted),
                            ui::fill(),
                            text("ETA: 11s").size(10).style(c.muted),
                        ],
                    ]
                    .spacing(3),
                )
                .padding(8)
                .style(theme::inner_box),
                Space::with_height(4),
                row![
                    text("CPU Rayon: 24% load").size(11).style(c.muted),
                    ui::fill(),
                    ui::btn(
                        "⏸ Pausar Fila",
                        Btn::Ghost,
                        Some(Message::Navigate(Module::Queue))
                    ),
                ]
                .align_items(Alignment::Center),
            ]
            .spacing(4),
        );

        // 6. Card 4: Segurança & Hash (128 bits)
        let bento_sec = ui::bento_card(
            env,
            column![
                row![
                    row![
                        container(text("🔑").size(16).style(c.primary))
                            .padding([5, 8])
                            .style(theme::inner_box),
                        ui::strong("Segurança & Hash", 15),
                    ]
                    .spacing(8)
                    .align_items(Alignment::Center),
                    ui::fill(),
                    container(text("128 bits").size(10).font(BOLD).style(c.success))
                        .padding([2, 6])
                        .style(theme::inner_box),
                ]
                .align_items(Alignment::Center),
                Space::with_height(6),
                container(
                    column![
                        row![
                            text("Senha Criptográfica").size(10).style(c.muted),
                            ui::fill(),
                            ui::colored("Alta Entropia", 10, c.success).font(BOLD),
                        ],
                        row![
                            ui::strong("k9#Px!8Lm@90$vWq", 13).width(Length::Fill),
                            ui::btn(
                                "Copiar",
                                Btn::Secondary,
                                Some(Message::Navigate(Module::DevTools))
                            ),
                        ]
                        .align_items(Alignment::Center),
                    ]
                    .spacing(4),
                )
                .padding(8)
                .style(theme::inner_box),
                Space::with_height(6),
                row![
                    row![
                        container(Space::new(Length::Fixed(14.0), Length::Fixed(14.0)))
                            .style(theme::solid_box(Color::from_rgb8(124, 58, 237), 999.0)),
                        container(Space::new(Length::Fixed(14.0), Length::Fixed(14.0)))
                            .style(theme::solid_box(Color::from_rgb8(76, 215, 246), 999.0)),
                        container(Space::new(Length::Fixed(14.0), Length::Fixed(14.0)))
                            .style(theme::solid_box(Color::from_rgb8(16, 185, 129), 999.0)),
                        text("#7C3AED").size(11).style(c.muted),
                    ]
                    .spacing(6)
                    .align_items(Alignment::Center),
                    ui::fill(),
                    ui::btn(
                        "Pipeta ➔",
                        Btn::Ghost,
                        Some(Message::Navigate(Module::DevTools))
                    ),
                ]
                .align_items(Alignment::Center),
            ]
            .spacing(4),
        );

        // 7. Card 5: PDF & Docs
        let bento_pdf = ui::bento_card(
            env,
            column![
                row![
                    row![
                        container(text("📄").size(16).style(c.accent))
                            .padding([5, 8])
                            .style(theme::inner_box),
                        ui::strong("PDF & Docs", 15),
                    ]
                    .spacing(8)
                    .align_items(Alignment::Center),
                    ui::fill(),
                    container(text("Zero Telemetria").size(10).style(c.muted))
                        .padding([2, 6])
                        .style(theme::inner_box),
                ]
                .align_items(Alignment::Center),
                Space::with_height(6),
                row![
                    button(
                        container(
                            column![
                                text("⤹").size(20).style(c.accent),
                                ui::strong("Mesclar", 12),
                                text("PDFs em lote").size(10).style(c.muted),
                            ]
                            .spacing(2)
                            .align_items(Alignment::Center),
                        )
                        .padding([12, 10])
                        .width(Length::Fill)
                        .style(theme::inner_box),
                    )
                    .style(theme::btn(Btn::Ghost))
                    .on_press(Message::Navigate(Module::Pdf)),
                    button(
                        container(
                            column![
                                text("🗜").size(20).style(c.success),
                                ui::strong("Comprimir", 12),
                                text("Motor Nativo lopdf").size(10).style(c.muted),
                            ]
                            .spacing(2)
                            .align_items(Alignment::Center),
                        )
                        .padding([12, 10])
                        .width(Length::Fill)
                        .style(theme::inner_box),
                    )
                    .style(theme::btn(Btn::Ghost))
                    .on_press(Message::Navigate(Module::Pdf)),
                ]
                .spacing(8),
                Space::with_height(6),
                row![
                    text("Desbloqueio de senha").size(11).style(c.muted),
                    ui::fill(),
                    ui::btn(
                        "Abrir Central ➔",
                        Btn::Ghost,
                        Some(Message::Navigate(Module::Pdf))
                    ),
                ]
                .align_items(Alignment::Center),
            ]
            .spacing(4),
        );

        // 8. Card 6 (Col Span 2): Atalhos Globais do Sistema & Auditoria
        let shortcuts_ribbon: Element<'a, Message> = if env.narrow() {
            column![
                row![
                    button(
                        container(
                            row![
                                text("Paleta").size(11).style(c.muted),
                                ui::fill(),
                                container(text("Ctrl+K").size(10).font(BOLD).style(c.primary))
                                    .padding([2, 6])
                                    .style(theme::solid_box(Color::from_rgb8(31, 31, 38), 4.0)),
                            ]
                            .align_items(Alignment::Center),
                        )
                        .padding([6, 10])
                        .width(Length::Fill)
                        .style(theme::inner_box),
                    )
                    .width(Length::FillPortion(1))
                    .style(theme::btn(Btn::Ghost))
                    .on_press(Message::Navigate(Module::DevTools)),
                    button(
                        container(
                            row![
                                text("Conversor").size(11).style(c.muted),
                                ui::fill(),
                                container(text("Ctrl+D").size(10).font(BOLD).style(c.accent))
                                    .padding([2, 6])
                                    .style(theme::solid_box(Color::from_rgb8(31, 31, 38), 4.0)),
                            ]
                            .align_items(Alignment::Center),
                        )
                        .padding([6, 10])
                        .width(Length::Fill)
                        .style(theme::inner_box),
                    )
                    .width(Length::FillPortion(1))
                    .style(theme::btn(Btn::Ghost))
                    .on_press(Message::Navigate(Module::QuickConvert)),
                ]
                .spacing(8),
                row![
                    button(
                        container(
                            row![
                                text("Revelador").size(11).style(c.muted),
                                ui::fill(),
                                container(text("Ctrl+P").size(10).font(BOLD).style(c.primary))
                                    .padding([2, 6])
                                    .style(theme::solid_box(Color::from_rgb8(31, 31, 38), 4.0)),
                            ]
                            .align_items(Alignment::Center),
                        )
                        .padding([6, 10])
                        .width(Length::Fill)
                        .style(theme::inner_box),
                    )
                    .width(Length::FillPortion(1))
                    .style(theme::btn(Btn::Ghost))
                    .on_press(Message::Navigate(Module::Images)),
                    button(
                        container(
                            row![
                                text("Fila Tasks").size(11).style(c.muted),
                                ui::fill(),
                                container(text("Ctrl+B").size(10).font(BOLD).style(c.success))
                                    .padding([2, 6])
                                    .style(theme::solid_box(Color::from_rgb8(31, 31, 38), 4.0)),
                            ]
                            .align_items(Alignment::Center),
                        )
                        .padding([6, 10])
                        .width(Length::Fill)
                        .style(theme::inner_box),
                    )
                    .width(Length::FillPortion(1))
                    .style(theme::btn(Btn::Ghost))
                    .on_press(Message::Navigate(Module::Queue)),
                ]
                .spacing(8),
            ]
            .spacing(8)
            .width(Length::Fill)
            .into()
        } else {
            row![
                button(
                    container(
                        row![
                            text("Paleta").size(11).style(c.muted),
                            ui::fill(),
                            container(text("Ctrl+K").size(10).font(BOLD).style(c.primary))
                                .padding([2, 6])
                                .style(theme::solid_box(Color::from_rgb8(31, 31, 38), 4.0)),
                        ]
                        .align_items(Alignment::Center),
                    )
                    .padding([6, 10])
                    .width(Length::Fill)
                    .style(theme::inner_box),
                )
                .width(Length::FillPortion(1))
                .style(theme::btn(Btn::Ghost))
                .on_press(Message::Navigate(Module::DevTools)),
                button(
                    container(
                        row![
                            text("Conversor").size(11).style(c.muted),
                            ui::fill(),
                            container(text("Ctrl+D").size(10).font(BOLD).style(c.accent))
                                .padding([2, 6])
                                .style(theme::solid_box(Color::from_rgb8(31, 31, 38), 4.0)),
                        ]
                        .align_items(Alignment::Center),
                    )
                    .padding([6, 10])
                    .width(Length::Fill)
                    .style(theme::inner_box),
                )
                .width(Length::FillPortion(1))
                .style(theme::btn(Btn::Ghost))
                .on_press(Message::Navigate(Module::QuickConvert)),
                button(
                    container(
                        row![
                            text("Revelador").size(11).style(c.muted),
                            ui::fill(),
                            container(text("Ctrl+P").size(10).font(BOLD).style(c.primary))
                                .padding([2, 6])
                                .style(theme::solid_box(Color::from_rgb8(31, 31, 38), 4.0)),
                        ]
                        .align_items(Alignment::Center),
                    )
                    .padding([6, 10])
                    .width(Length::Fill)
                    .style(theme::inner_box),
                )
                .width(Length::FillPortion(1))
                .style(theme::btn(Btn::Ghost))
                .on_press(Message::Navigate(Module::Images)),
                button(
                    container(
                        row![
                            text("Fila Tasks").size(11).style(c.muted),
                            ui::fill(),
                            container(text("Ctrl+B").size(10).font(BOLD).style(c.success))
                                .padding([2, 6])
                                .style(theme::solid_box(Color::from_rgb8(31, 31, 38), 4.0)),
                        ]
                        .align_items(Alignment::Center),
                    )
                    .padding([6, 10])
                    .width(Length::Fill)
                    .style(theme::inner_box),
                )
                .width(Length::FillPortion(1))
                .style(theme::btn(Btn::Ghost))
                .on_press(Message::Navigate(Module::Queue)),
            ]
            .spacing(8)
            .width(Length::Fill)
            .into()
        };

        let bento_shortcuts = ui::bento_card(
            env,
            column![
                row![
                    row![
                        container(text("⌨").size(16).style(c.primary))
                            .padding([5, 8])
                            .style(theme::inner_box),
                        ui::strong("Atalhos Globais do Sistema", 15),
                    ]
                    .spacing(8)
                    .align_items(Alignment::Center),
                    ui::fill(),
                    container(text("Win32 Hook Ativo").size(10).style(c.muted))
                        .padding([2, 6])
                        .style(theme::inner_box),
                ]
                .align_items(Alignment::Center),
                Space::with_height(6),
                column![
                    shortcuts_ribbon,
                    Space::with_height(6),
                    // Feed de Auditoria Recente
                    row![
                        text("Últimas Ações Concluídas").size(11).style(c.muted),
                        ui::fill(),
                        ui::btn(
                            "Ver log completo",
                            Btn::Ghost,
                            Some(Message::Navigate(Module::History))
                        ),
                    ]
                    .align_items(Alignment::Center),
                    container(
                        row![
                            container(Space::new(Length::Fixed(6.0), Length::Fixed(6.0)))
                                .style(theme::solid_box(c.success, 999.0)),
                            ui::strong("DSC_4091_RAW.cr3 → webp (lossless)", 11)
                                .width(Length::Fill),
                            text("12ms • 3.2MB").size(10).style(c.success),
                        ]
                        .spacing(8)
                        .align_items(Alignment::Center),
                    )
                    .padding([6, 10])
                    .style(theme::inner_box),
                    container(
                        row![
                            container(Space::new(Length::Fixed(6.0), Length::Fixed(6.0)))
                                .style(theme::solid_box(c.success, 999.0)),
                            ui::strong("relatorio_financeiro_q3.pdf → otimizado", 11)
                                .width(Length::Fill),
                            text("45ms • -64% tam").size(10).style(c.success),
                        ]
                        .spacing(8)
                        .align_items(Alignment::Center),
                    )
                    .padding([6, 10])
                    .style(theme::inner_box),
                ]
                .spacing(4),
            ]
            .spacing(4),
        );

        // Grid Bento Responsivo Inteligente
        let bento_grid: Element<'a, Message> = if env.is_desktop_wide() {
            // Desktop Amplo (>= 1180px): Layout completo de 4 colunas como no Stitch
            column![
                // Linha 1: Hero (Col Span 2) + Revelador RAW (1 Col) + Fila de Render (1 Col)
                row![
                    container(hero_card).width(Length::FillPortion(2)),
                    container(bento_raw).width(Length::FillPortion(1)),
                    container(bento_render).width(Length::FillPortion(1)),
                ]
                .spacing(14),
                // Linha 2: Segurança & Hash (1 Col) + PDF & Docs (1 Col) + Atalhos Globais (Col Span 2)
                row![
                    container(bento_sec).width(Length::FillPortion(1)),
                    container(bento_pdf).width(Length::FillPortion(1)),
                    container(bento_shortcuts).width(Length::FillPortion(2)),
                ]
                .spacing(14),
            ]
            .spacing(14)
            .width(Length::Fill)
            .into()
        } else if env.is_medium() {
            // Telas Médias (680px - 1180px): Layout organizado em 2 colunas perfeitamente equilibradas
            column![
                // Hero ocupa largura total
                hero_card,
                // Linha 2: Revelador RAW (50%) + Fila de Render (50%)
                row![
                    container(bento_raw).width(Length::FillPortion(1)),
                    container(bento_render).width(Length::FillPortion(1)),
                ]
                .spacing(14),
                // Linha 3: Segurança (50%) + PDF & Docs (50%)
                row![
                    container(bento_sec).width(Length::FillPortion(1)),
                    container(bento_pdf).width(Length::FillPortion(1)),
                ]
                .spacing(14),
                // Atalhos ocupa largura total
                bento_shortcuts,
            ]
            .spacing(14)
            .width(Length::Fill)
            .into()
        } else {
            // Telas Pequenas (< 680px): Coluna única limpa
            column![
                hero_card,
                bento_raw,
                bento_render,
                bento_sec,
                bento_pdf,
                bento_shortcuts,
            ]
            .spacing(14)
            .width(Length::Fill)
            .into()
        };

        if !self.home_search.trim().is_empty() {
            let search_results = self.search_results_view(env, self.home_search.trim());
            return ui::page(env, vec![top_bar, search_results]);
        }

        ui::page(env, vec![top_bar, bento_grid])
    }

    fn search_matches(&self, query: &str) -> Vec<(Module, &'static str, &'static str)> {
        let q = normalize_search(query);
        let all_tools: &[(Module, &[&str], &str, &str)] = &[
            (
                Module::QuickConvert,
                &["converter", "conversor", "webp", "png", "jpg", "jpeg", "avif", "mp4", "mkv", "mp3", "wav", "flac", "pdf", "docx", "txt", "csv", "quick convert", "transcode", "formato", "formatos", "drag", "drop"],
                "Conversão e transcodificação rápida de imagem, vídeo, áudio e texto.",
                "Quick file conversion for images, videos, audio, and documents.",
            ),
            (
                Module::Images,
                &["imagem", "imagens", "foto", "fotos", "raw", "editor", "exposicao", "brilho", "contraste", "crop", "cortar", "filtro", "temperatura", "vinheta", "develop", "revelador", "presets", "fotografia"],
                "Ajuste fino de exposição, contraste, balanço de branco, crop e presets fotográficos.",
                "RAW developer, photo adjustments, cropping, color grading, and presets.",
            ),
            (
                Module::Video,
                &["video", "vídeo", "cortar", "trim", "comprimir", "compressao", "nvenc", "h264", "h265", "hevc", "av1", "gif", "fps", "resolucao", "transcoder", "render", "hardware"],
                "Corte preciso sem perda (smart streamcopy) e render com aceleração GPU.",
                "Fast video trimming, lossy/lossless streamcopy, and GPU encoding.",
            ),
            (
                Module::Audio,
                &["audio", "áudio", "som", "musica", "música", "mp3", "flac", "wav", "volume", "normalizar", "chiado", "ruido", "fade", "equalizador", "ebu r128", "estudio"],
                "Normalização de volume (EBU R128), remoção de ruídos e conversão de áudio.",
                "Audio normalization, noise reduction, and bitrate conversion.",
            ),
            (
                Module::Pdf,
                &["pdf", "documento", "juntar", "mesclar", "merge", "comprimir", "compressao", "dividir", "split", "proteger", "senha", "desbloquear", "lopdf", "extrair", "docs"],
                "Mesclar múltiplos PDFs, compactação atômica nativa e proteção por senha.",
                "Merge PDFs, compress documents offline, and manage encryption.",
            ),
            (
                Module::BatchRename,
                &["renomear", "rename", "lote", "batch", "arquivos", "prefixo", "sufixo", "numeracao", "substituir", "regex", "maiusculas", "minusculas", "pastas"],
                "Renomeie coleções inteiras com regras automáticas, numeração e substituição.",
                "Bulk file renamer with prefixes, suffixes, numbering, and patterns.",
            ),
            (
                Module::Zip,
                &["zip", "compactar", "descompactar", "extrair", "extract", "compressao", "arquivo", "backup", "zstandard", "deflate", "zip-slip", "tar", "descompactador"],
                "Compactação segura com Zstandard e extração protegida contra zip-slip.",
                "Fast ZIP compression with Zstandard and safe extraction sandbox.",
            ),
            (
                Module::Productivity,
                &["produtividade", "notas", "bloco", "anotacoes", "anotações", "tarefas", "checklist", "todo", "to-do", "lembretes", "notepad", "texto", "markdown"],
                "Bloco de anotações persistente e lista de tarefas rápidas.",
                "Quick persistent notepad and actionable task checklist.",
            ),
            (
                Module::Calculator,
                &["calculadora", "calc", "calcular", "conta", "matematica", "cientifica", "científica", "hex", "binario", "programador", "historico", "pi", "seno", "cosseno", "porcentagem"],
                "Cálculos científicos, funções trigonométricas, conversão de bases e histórico.",
                "Scientific calculator with unit base conversions and evaluation history.",
            ),
            (
                Module::DevTools,
                &["devtools", "dev", "desenvolvedor", "hash", "sha256", "md5", "sha1", "base64", "json", "formatar", "uuid", "senha", "gerador", "pipeta", "cores", "hex", "rgb", "seguranca", "criptografia"],
                "Gerador de senhas seguras, gerador de UUID, hash de arquivos, base64 e pipeta de cores.",
                "Hash generator (SHA/MD5), UUIDs, password entropy, Base64, and color picker.",
            ),
            (
                Module::Queue,
                &["fila", "queue", "tarefas", "processos", "background", "progresso", "pausar", "cancelar", "jobs", "concluidos", "ativos"],
                "Monitoramento e fila de processamento assíncrono em segundo plano.",
                "Background task processing monitor and priority queue.",
            ),
            (
                Module::History,
                &["historico", "histórico", "history", "auditoria", "atividades", "logs", "registro", "operacoes", "exportar"],
                "Registro detalhado de operações executadas para conformidade e conferência.",
                "Complete audit log of file transformations and operations.",
            ),
            (
                Module::Settings,
                &["configuracoes", "configurações", "settings", "tema", "escuro", "claro", "dark", "light", "idioma", "linguagem", "pasta", "destino", "privacidade", "hardware", "preferencias"],
                "Tema visual, idioma do sistema, preferências de pasta de saída e hardware.",
                "Theme toggling, interface language, output directories, and hardware flags.",
            ),
        ];

        let mut matches = Vec::new();
        for &(m, tags, pt, en) in all_tools {
            let title = normalize_search(m.title());
            let title_en = normalize_search(m.title_in(crate::shared::i18n::Lang::En, false));
            let matches_tags = tags.iter().any(|t| {
                let nt = normalize_search(t);
                nt.contains(&q) || q.contains(&nt)
            });
            if title.contains(&q)
                || title_en.contains(&q)
                || matches_tags
                || normalize_search(pt).contains(&q)
                || normalize_search(en).contains(&q)
            {
                matches.push((m, pt, en));
            }
        }
        matches
    }

    fn search_results_view<'a>(&'a self, env: &Env, query: &str) -> Element<'a, Message> {
        let c = env.c();
        let matches = self.search_matches(query);

        let header = row![
            column![
                row![
                    ui::strong(
                        if env.lang.is_en() {
                            "Search Results"
                        } else {
                            "Resultados da Busca"
                        },
                        18
                    ),
                    container(
                        text(format!(
                            "{} {}",
                            matches.len(),
                            if env.lang.is_en() {
                                "found"
                            } else {
                                "ferramenta(s)"
                            }
                        ))
                        .size(11)
                        .font(BOLD)
                        .style(c.primary)
                    )
                    .padding([2, 8])
                    .style(theme::solid_box(Color::from_rgb8(31, 25, 48), 6.0)),
                ]
                .spacing(10)
                .align_items(Alignment::Center),
                ui::muted(
                    env,
                    if env.lang.is_en() {
                        format!("Showing tools matching \"{}\"", query)
                    } else {
                        format!(
                            "Exibindo ferramentas correspondentes ao termo \"{}\"",
                            query
                        )
                    }
                )
                .size(12),
            ]
            .spacing(4),
            ui::fill(),
            ui::btn(
                if env.lang.is_en() {
                    "✕ Clear Search"
                } else {
                    "✕ Limpar Busca"
                },
                Btn::Secondary,
                Some(Message::SearchChanged(String::new()))
            ),
        ]
        .align_items(Alignment::Center);

        if matches.is_empty() {
            let empty_box = container(
                column![
                    text("🔍").size(36),
                    Space::with_height(6),
                    ui::strong(
                        if env.lang.is_en() {
                            "No tools found"
                        } else {
                            "Nenhuma ferramenta encontrada"
                        },
                        16
                    ),
                    ui::muted(
                        env,
                        if env.lang.is_en() {
                            format!("No matching tools for \"{}\". Try broader terms:", query)
                        } else {
                            format!(
                                "Nenhuma ferramenta encontrada para \"{}\". Tente termos como:",
                                query
                            )
                        }
                    )
                    .size(13),
                    Space::with_height(10),
                    row![
                        ui::btn(
                            "🔄 Conversor",
                            Btn::Secondary,
                            Some(Message::Navigate(Module::QuickConvert))
                        ),
                        ui::btn(
                            "🖼️ Fotos & RAW",
                            Btn::Secondary,
                            Some(Message::Navigate(Module::Images))
                        ),
                        ui::btn(
                            "🎬 Vídeo",
                            Btn::Secondary,
                            Some(Message::Navigate(Module::Video))
                        ),
                        ui::btn(
                            "📄 PDF",
                            Btn::Secondary,
                            Some(Message::Navigate(Module::Pdf))
                        ),
                        ui::btn(
                            "🖩 Calculadora",
                            Btn::Secondary,
                            Some(Message::Navigate(Module::Calculator))
                        ),
                        ui::btn(
                            "🎨 DevTools",
                            Btn::Secondary,
                            Some(Message::Navigate(Module::DevTools))
                        ),
                    ]
                    .spacing(8)
                    .align_items(Alignment::Center),
                ]
                .spacing(6)
                .align_items(Alignment::Center)
                .width(Length::Fill),
            )
            .padding([36, 24])
            .style(theme::bento_card);

            column![header, Space::with_height(14), empty_box]
                .spacing(8)
                .into()
        } else {
            let mut list = column![].spacing(10).width(Length::Fill);

            for (module, desc_pt, desc_en) in matches {
                let desc = if env.lang.is_en() { desc_en } else { desc_pt };
                let title = module.title_in(env.lang, false);
                let icon = module.icon();

                let item_card = button(
                    container(
                        row![
                            container(text(icon).size(24))
                                .padding([10, 14])
                                .style(theme::solid_box(Color::from_rgb8(31, 31, 38), 8.0)),
                            column![
                                row![
                                    ui::strong(title, 16),
                                    ui::fill(),
                                    container(
                                        text(if env.lang.is_en() {
                                            "Open Tool ➔"
                                        } else {
                                            "Abrir Ferramenta ➔"
                                        })
                                        .size(12)
                                        .font(BOLD)
                                        .style(c.primary)
                                    )
                                    .padding([4, 10])
                                    .style(theme::inner_box),
                                ]
                                .align_items(Alignment::Center),
                                ui::muted(env, desc).size(12),
                            ]
                            .spacing(4)
                            .width(Length::Fill),
                        ]
                        .spacing(14)
                        .align_items(Alignment::Center),
                    )
                    .padding([12, 16])
                    .width(Length::Fill)
                    .style(theme::bento_card),
                )
                .style(theme::btn(Btn::Ghost))
                .width(Length::Fill)
                .on_press(Message::Navigate(module));

                list = list.push(item_card);
            }

            column![header, Space::with_height(14), list]
                .spacing(8)
                .into()
        }
    }
}

fn normalize_search(s: &str) -> String {
    s.to_lowercase()
        .chars()
        .map(|ch| match ch {
            'á' | 'à' | 'ã' | 'â' | 'ä' => 'a',
            'é' | 'è' | 'ê' | 'ë' => 'e',
            'í' | 'ì' | 'î' | 'ï' => 'i',
            'ó' | 'ò' | 'õ' | 'ô' | 'ö' => 'o',
            'ú' | 'ù' | 'û' | 'ü' => 'u',
            'ç' => 'c',
            other => other,
        })
        .collect()
}
