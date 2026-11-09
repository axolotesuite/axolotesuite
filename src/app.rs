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

use iced::widget::{button, column, container, image as iced_image, progress_bar, row, scrollable, text, Space};
use iced::{Alignment, Application, Command, Element, Event, Length, Subscription, Theme};

use crate::core::Ctx;
use crate::modules::{
    audio, batch_rename, devtools, history, images, installer, pdf, productivity, queue, quick_convert, settings,
    video, zip,
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
                Module::Installer => if is_installed { "Apagar Tudo / Cleanup" } else { "Install / Setup" },
            }
        } else {
            match self {
                Module::Installer => if is_installed { "Apagar Tudo / Limpar" } else { "Instalar / Setup" },
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
                Module::QuickConvert => "Convert images, text, documents (DOCX/PDF), videos, audio, and ZIP.",
                Module::Productivity => "Quick notepad, tasks checklist, and scientific calculator.",
                Module::Calculator => "Scientific expression calculator with parentheses and history.",
                Module::Pdf => "Merge multiple PDFs, split pages and intervals, and inspect metadata.",
                Module::Images => "Live photo developer with RGB histogram, effects, and batch folder sync.",
                Module::Video => "Trim timeline clips, change speed, apply cinema looks, and CRF compression.",
                Module::Audio => "Convert audio formats, extract soundtracks from video, and adjust bitrate.",
                Module::BatchRename => "Bulk rename files with prefix, suffix, replace, and live preview.",
                Module::Zip => "Compress directories or extract archives safely with anti Zip-Slip shield.",
                Module::DevTools => "HEX/RGB/HSL/CMYK converter, photo palette picker, and strong password/UUID.",
                Module::Queue => "Monitor active background tasks without freezing the UI.",
                Module::History => "Complete audit log of operations performed on your PC.",
                Module::Settings => "Dark/light theme, auto-open output, language detection, and privacy.",
                Module::Installer => "Install essentials on PC + auto FFmpeg setup or uninstall cleanly.",
            }
        } else {
            self.summary()
        }
    }

    pub fn summary(&self) -> &'static str {
        match self {
            Module::Home => "Central de ferramentas, busca rápida e visão geral.",
            Module::QuickConvert => "Converta imagens, textos, documentos (DOCX/PDF), vídeos, áudios e ZIP.",
            Module::Productivity => "Bloco de notas (.txt), lista de tarefas e organizador de pastas.",
            Module::Calculator => "Calculadora expressa com parênteses, potência, porcentagem e histórico.",
            Module::Pdf => "Junte, divida em páginas e consulte informações de PDFs.",
            Module::Images => "Editor de fotos ao vivo (luz, filtros, corte), converter e comprimir.",
            Module::Video => "Editor rápido de vídeo (corte, velocidade, 9:16, filtros) e conversor.",
            Module::Audio => "Converta, extraia trilha de vídeos e comprima áudios.",
            Module::BatchRename => "Renomeie arquivos em lote com regras e pré-visualização.",
            Module::Zip => "Compacte, descompacte e inspecione arquivos ZIP com segurança.",
            Module::DevTools => "Estúdio de cores, paleta de fotos, gerador de senhas fortes e UUID.",
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
        let running_outside_installed_dir = std::env::current_exe()
            .ok()
            .and_then(|p| p.parent().map(|d| d.to_path_buf()))
            .map(|exe_dir| {
                let inst_dir = std::path::PathBuf::from(installer_state.install_dir.trim());
                !exe_dir.to_string_lossy().eq_ignore_ascii_case(&inst_dir.to_string_lossy())
            })
            .unwrap_or(true);

        // Sempre que abrir o .exe fora da pasta instalada, ou se faltar instalar o app/FFmpeg,
        // entra direto na Tela Obrigatória do Instalador para baixar tudo!
        let needs_mandatory_setup = !ctx.settings.setup_completed
            || !installer_state.is_installed()
            || !installer_state.is_ffmpeg_ready()
            || running_outside_installed_dir;

        let (mut initial_module, installer_step, mandatory_setup) = if args.iter().any(|a| a == "--uninstall") {
            (Module::Installer, installer::WizardStep::UninstallConfirm, false)
        } else if args.iter().any(|a| a == "--install" || a == "--setup") || needs_mandatory_setup {
            (Module::Installer, installer::WizardStep::Welcome, true)
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
        format!("Axolote Suite v1.3 — {}", self.active.title())
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

        let gesture_sub = crate::core::system::global_gesture_subscription()
            .map(Message::GlobalGestureTriggered);

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
                    self.quick_convert.push_dropped_file(path, self.ctx.settings.dark_mode);
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

                let target_t = if self.is_sidebar_collapsed() { 0.0 } else { 1.0 };
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
            Message::QuickConvert(m) => self.quick_convert.update(m, &mut self.ctx).map(Message::QuickConvert),
            Message::Productivity(m) => self.productivity.update(m, &mut self.ctx).map(Message::Productivity),
            Message::BatchRename(m) => self.batch_rename.update(m, &mut self.ctx).map(Message::BatchRename),
            Message::Zip(m) => self.zip.update(m, &mut self.ctx).map(Message::Zip),
            Message::Images(m) => self.images.update(m, &mut self.ctx).map(Message::Images),
            Message::Pdf(m) => self.pdf.update(m, &mut self.ctx).map(Message::Pdf),
            Message::Audio(m) => self.audio.update(m, &mut self.ctx).map(Message::Audio),
            Message::Video(m) => self.video.update(m, &mut self.ctx).map(Message::Video),
            Message::DevTools(m) => self.devtools.update(m, &mut self.ctx).map(Message::DevTools),
            Message::Queue(m) => self.queue.update(m, &mut self.ctx).map(Message::Queue),
            Message::History(m) => self.history.update(m, &mut self.ctx).map(Message::History),
            Message::Settings(m) => self.settings.update(m, &mut self.ctx).map(Message::Settings),
            Message::Installer(m) => {
                let go_home = matches!(m, installer::Msg::FinishAndGoHome);
                let refresh_ff = matches!(
                    m,
                    installer::Msg::InstallFinished(_) | installer::Msg::DownloadFfmpegFinished(_)
                );
                let cmd = self.installer.update(m, &mut self.ctx).map(Message::Installer);
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
            let content = installer::view::view(&self.installer, true, &env).map(Message::Installer);
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
            Module::QuickConvert => quick_convert::view::view(&self.quick_convert, &env).map(Message::QuickConvert),
            Module::Productivity => productivity::view::view(&self.productivity, &env).map(Message::Productivity),
            Module::Calculator => self.calculator_standalone_view(&env),
            Module::BatchRename => batch_rename::view::view(&self.batch_rename, &env).map(Message::BatchRename),
            Module::Zip => zip::view::view(&self.zip, &env).map(Message::Zip),
            Module::Images => images::view::view(&self.images, &env).map(Message::Images),
            Module::Pdf => pdf::view::view(&self.pdf, &env).map(Message::Pdf),
            Module::Audio => audio::view::view(&self.audio, &env).map(Message::Audio),
            Module::Video => video::view::view(&self.video, &env).map(Message::Video),
            Module::DevTools => devtools::view::view(&self.devtools, &env).map(Message::DevTools),
            Module::Queue => queue::view::view(&self.ctx.jobs, &env).map(Message::Queue),
            Module::History => history::view::view(&self.history, &self.ctx.history, &env).map(Message::History),
            Module::Settings => settings::view::view(&self.settings, &self.ctx.settings, &env).map(Message::Settings),
            Module::Installer => installer::view::view(&self.installer, false, &env).map(Message::Installer),
        };

        container(row![sidebar, container(content).width(Length::Fill).height(Length::Fill)])
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
        let target_t = if self.is_sidebar_collapsed() { 0.0 } else { 1.0 };
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
        let body = productivity::view::calc_view(&self.productivity, env).map(Message::Productivity);
        ui::page(env, vec![header, body])
    }

    fn sidebar_view(&self, collapsed: bool, width: f32, env: &Env) -> Element<'_, Message> {
        let c = env.c();
        let running = self.ctx.jobs.running();

        // Cabeçalho com a Logo Oficial do Axolote Suite
        let brand_header: Element<'_, Message> = if collapsed {
            column![
                button(
                    container(
                        iced_image::Image::new(brand::logo_small())
                            .width(Length::Fixed(38.0))
                            .height(Length::Fixed(38.0)),
                    )
                    .center_x()
                    .width(Length::Fill),
                )
                .padding(4)
                .style(theme::btn(Btn::Ghost))
                .on_press(Message::Navigate(Module::Home)),
                button(container(text("»").size(16)).center_x().width(Length::Fill))
                    .padding([4, 8])
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
                        iced_image::Image::new(brand::logo_small())
                            .width(Length::Fixed(40.0))
                            .height(Length::Fixed(40.0)),
                        column![
                            row![
                                text("Axolote ").size(16).font(BOLD),
                                ui::colored("Suite", 16, c.accent).font(BOLD),
                            ],
                            text("v1.3.0 • 100% Offline").size(11).style(c.muted),
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
            if self.installer.is_installed() {
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
                        "SYSTEM & CLEANUP",
                        &[Module::Queue, Module::History, Module::Settings, Module::Installer],
                    ),
                ]
            } else {
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
                        "SYSTEM & SETUP",
                        &[Module::Queue, Module::History, Module::Settings, Module::Installer],
                    ),
                ]
            }
        } else {
            if self.installer.is_installed() {
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
                        "SISTEMA & LIMPEZA",
                        &[Module::Queue, Module::History, Module::Settings, Module::Installer],
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
                        "SISTEMA & SETUP",
                        &[Module::Queue, Module::History, Module::Settings, Module::Installer],
                    ),
                ]
            }
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
                let pill_h = if is_active { 16.0 + 8.0 * self.nav_pulse } else { 0.0 };
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
                    let indent = if is_active { 4.0 * (1.0 - self.nav_pulse * 0.5) } else { 0.0 };
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
            if self.ctx.settings.dark_mode { "☀" } else { "🌙" }
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

        // 1. Barra superior: Título "Início" / "Home" + Barra de pesquisa
        let top_bar: Element<'a, Message> = {
            let search_ph = if env.lang.is_en() {
                "🔍  Search tool (e.g. PDF, video, photo, notes, calculator, ZIP, setup)..."
            } else {
                "🔍  Pesquisar ferramenta (ex.: PDF, vídeo, foto, nota, calculadora, ZIP, instalar)..."
            };
            let search = ui::input(search_ph, &self.home_search, Message::SearchChanged)
                .width(if env.narrow() { Length::Fill } else { Length::Fixed(420.0) });

            let setup_label = if env.lang.is_en() { "💿  Installer / Setup" } else { "💿  Instalador / Setup" };
            let home_title = if env.lang.is_en() { "Home" } else { "Início" };

            if env.narrow() {
                column![ui::strong(home_title, 24), search].spacing(10).into()
            } else {
                row![
                    ui::strong(home_title, 26).width(Length::Fill),
                    search,
                    ui::btn(setup_label, Btn::Secondary, Some(Message::Navigate(Module::Installer))),
                ]
                .spacing(12)
                .align_items(Alignment::Center)
                .into()
            }
        };

        // 2. Banner de Boas-vindas com a Logo do Axolote à direita
        let hero_title_size = if env.narrow() { 22 } else { 28 };
        let badge_text = if env.lang.is_en() { "✨ v1.3.0 • 100% Offline & Native" } else { "✨ v1.3.0 • 100% Offline & Nativo" };
        let welcome_prefix = if env.lang.is_en() { "Hello! 👋  Welcome to Axolote " } else { "Olá! 👋  Bem-vindo ao Axolote " };
        let hero_desc = if env.lang.is_en() {
            "A complete local workstation with Universal Quick Convert, Colors & Passwords, Productivity, Calculator, Live Photo Developer, Video Studio, PDF, and file tools."
        } else {
            "Uma central completa com Conversor Fácil universal, Cores & Senhas, produtividade, calculadora, edição de fotos ao vivo, editor de vídeo, PDF e arquivos."
        };

        let hero = container(
            row![
                column![
                    ui::badge(badge_text, c.primary),
                    row![
                        ui::strong(welcome_prefix, hero_title_size),
                        ui::colored("Suite", hero_title_size, c.accent).font(BOLD),
                    ]
                    .align_items(Alignment::Center),
                    ui::muted(env, hero_desc).size(14),
                ]
                .spacing(8)
                .width(Length::Fill),
                iced_image::Image::new(brand::logo_banner())
                    .width(Length::Fixed(if env.narrow() { 72.0 } else { 112.0 }))
                    .height(Length::Fixed(if env.narrow() { 72.0 } else { 112.0 })),
            ]
            .spacing(18)
            .align_items(Alignment::Center),
        )
        .padding(if env.narrow() { 18 } else { 24 })
        .width(Length::Fill)
        .style(theme::hero);

        // 3. Central de Ferramentas
        let cards_data: Vec<(&str, &str, &str, Message)> = if env.lang.is_en() {
            vec![
                (
                    "🔄",
                    "Quick Convert",
                    "Convert images, text, documents (DOCX/PDF), videos, audio, and ZIP.",
                    Message::Navigate(Module::QuickConvert),
                ),
                (
                    "🎨",
                    "Colors & Passwords",
                    "Color studio, photo palette extractor, strong passwords, and UUID.",
                    Message::Navigate(Module::DevTools),
                ),
                (
                    "📝",
                    "Notepad",
                    "Create, edit, and save plain text .txt notes locally.",
                    Message::QuickLaunchProductivity(productivity::Tab::Notes),
                ),
                (
                    "✅",
                    "Tasks",
                    "Manage your daily tasks and checklist with filters.",
                    Message::QuickLaunchProductivity(productivity::Tab::Tasks),
                ),
                (
                    "🖩",
                    "Calculator",
                    "Perform fast arithmetic and advanced scientific expressions.",
                    Message::Navigate(Module::Calculator),
                ),
                (
                    "📄",
                    "PDF",
                    "Merge multiple PDFs, split pages, and inspect document metadata.",
                    Message::Navigate(Module::Pdf),
                ),
                (
                    "🖼",
                    "Photos & Developer",
                    "Live photo developer, RGB histogram, presets, and batch folder sync.",
                    Message::Navigate(Module::Images),
                ),
                (
                    "🎬",
                    "Videos",
                    "Timeline clip trimming, speed control, format convert, and CRF compress.",
                    Message::Navigate(Module::Video),
                ),
                (
                    "🎵",
                    "Audio",
                    "Convert between audio formats, extract soundtracks, and compress.",
                    Message::Navigate(Module::Audio),
                ),
                (
                    "🗜",
                    "ZIP Archiver",
                    "Safely compress files/folders or extract archives with anti Zip-Slip.",
                    Message::Navigate(Module::Zip),
                ),
                (
                    "📁",
                    "Batch Files",
                    "Bulk rename files with instant preview and folder organizer.",
                    Message::Navigate(Module::BatchRename),
                ),
            ]
        } else {
            vec![
                (
                    "🔄",
                    "Conversor Fácil",
                    "Converta imagens, textos, documentos (DOCX/PDF), vídeos, áudios e ZIP.",
                    Message::Navigate(Module::QuickConvert),
                ),
                (
                    "🎨",
                    "Cores & Senhas",
                    "Estúdio de cores, paleta de fotos, senhas fortes e UUID.",
                    Message::Navigate(Module::DevTools),
                ),
                (
                    "📝",
                    "Bloco de Notas",
                    "Crie, edite e salve suas notas .txt localmente.",
                    Message::QuickLaunchProductivity(productivity::Tab::Notes),
                ),
                (
                    "✅",
                    "Tarefas",
                    "Gerencie suas atividades diárias com filtros.",
                    Message::QuickLaunchProductivity(productivity::Tab::Tasks),
                ),
                (
                    "🖩",
                    "Calculadora",
                    "Realize cálculos rápidos e expressões avançadas.",
                    Message::Navigate(Module::Calculator),
                ),
                (
                    "📄",
                    "PDF",
                    "Junte, divida em páginas e consulte PDFs.",
                    Message::Navigate(Module::Pdf),
                ),
                (
                    "🖼",
                    "Imagens & Revelador",
                    "Revelador de fotos ao vivo, histograma RGB, presets e lote.",
                    Message::Navigate(Module::Images),
                ),
                (
                    "🎬",
                    "Vídeos",
                    "Corte na linha do tempo, velocidade, formatos e compressão.",
                    Message::Navigate(Module::Video),
                ),
                (
                    "🎵",
                    "Áudio",
                    "Converta, extraia de vídeos e comprima áudios.",
                    Message::Navigate(Module::Audio),
                ),
                (
                    "🗜",
                    "ZIP (Compactador)",
                    "Compacte, descompacte e gerencie arquivos ZIP.",
                    Message::Navigate(Module::Zip),
                ),
                (
                    "📁",
                    "Arquivos",
                    "Renomeie em lote com prévia e organize pastas.",
                    Message::Navigate(Module::BatchRename),
                ),
            ]
        };

        let q = self.home_search.trim().to_lowercase();
        let filtered_cards: Vec<Element<'a, Message>> = cards_data
            .into_iter()
            .filter(|(_, title, desc, _)| {
                q.is_empty() || title.to_lowercase().contains(&q) || desc.to_lowercase().contains(&q)
            })
            .map(|(icon, title, desc, msg)| {
                button(
                    row![
                        ui::icon_tile(icon, c.primary, 22),
                        column![
                            ui::strong(title, 15),
                            ui::muted(env, desc).size(12),
                        ]
                        .spacing(3)
                        .width(Length::Fill),
                    ]
                    .spacing(12)
                    .align_items(Alignment::Center),
                )
                .padding(16)
                .width(Length::Fill)
                .style(theme::btn(Btn::Tile))
                .on_press(msg)
                .into()
            })
            .collect();

        let tools_title = if env.lang.is_en() { "Tools Dashboard" } else { "Central de Ferramentas" };
        let tools_col = column![
            ui::strong(tools_title, 18),
            ui::grid(filtered_cards, env.columns(250.0, 3), 12),
        ]
        .spacing(12)
        .width(Length::Fill);

        // 4. Painel lateral direito do Início (Status do Sistema + Últimas Novidades)
        let status_title = if env.lang.is_en() { "System Status" } else { "Status do Sistema" };
        let ready_badge = if env.lang.is_en() { "100% Ready" } else { "100% Pronto" };
        let notes_tasks_text = if env.lang.is_en() {
            format!("✔  {} note(s) saved • {} pending task(s)", self.productivity.notes.len(), self.productivity.pending_tasks())
        } else {
            format!("✔  {} nota(s) salva(s) • {} tarefa(s) pendente(s)", self.productivity.notes.len(), self.productivity.pending_tasks())
        };
        let p1 = if env.lang.is_en() { "✔  Live Photo Developer with RGB histogram active" } else { "✔  Revelador de Fotos com Histograma ao Vivo ativo" };
        let p2 = if env.lang.is_en() { "✔  Video Studio with timeline clip trimming" } else { "✔  Editor de Vídeo com Linha do Tempo integrado" };
        let p3 = if env.lang.is_en() { "✔  Complete PDF Tools (Merge & Split)" } else { "✔  Ferramentas de PDF completas (Unir & Dividir)" };
        let p4 = if env.lang.is_en() { "✔  ZIP Archiver with anti Zip-Slip protection" } else { "✔  Compactador ZIP com proteção anti Zip-Slip" };
        let ffmpeg_text = if self.video.ffmpeg_ok {
            if env.lang.is_en() { "✔  FFmpeg engine ready" } else { "✔  Motor FFmpeg detectado e pronto" }
        } else {
            if env.lang.is_en() { "⬇  Auto FFmpeg setup ready in installer" } else { "⬇  FFmpeg automático disponível no Instalador" }
        };

        let status_card = ui::card(
            env,
            column![
                row![
                    ui::strong(status_title, 16).width(Length::Fill),
                    ui::badge(ready_badge, c.success),
                ]
                .align_items(Alignment::Center),
                progress_bar(0.0..=100.0, 100.0).height(Length::Fixed(8.0)),
                ui::muted(env, notes_tasks_text),
                ui::muted(env, p1),
                ui::muted(env, p2),
                ui::muted(env, p3),
                ui::muted(env, p4),
                ui::muted(env, ffmpeg_text),
            ]
            .spacing(8),
        );

        let news_title = if env.lang.is_en() { "Latest Updates" } else { "Últimas Novidades" };
        let n1 = if env.lang.is_en() { "✔  Universal Quick Convert with atomic safe writes" } else { "✔  Conversor Fácil universal (Imagens, Docs, Vídeo, Áudio e ZIP)" };
        let n2 = if env.lang.is_en() { "✔  LightCraft-inspired Photo Developer & Batch Sync" } else { "✔  Revelador de Fotos com Histograma e Lote estilo LightCraft" };
        let n3 = if env.lang.is_en() { "✔  Multi-language support with automatic OS detection" } else { "✔  Suporte multi-idioma com detecção automática do sistema" };
        let n4 = if env.lang.is_en() { "✔  Colors studio, photo palette, passwords & UUID" } else { "✔  Estúdio de Cores, Paleta de Fotos, Senhas Fortes e UUID" };
        let setup_btn_label = if env.lang.is_en() { "💿  Open Installer / Uninstaller" } else { "💿  Abrir Instalador / Desinstalador" };

        let news_card = ui::card(
            env,
            column![
                row![
                    ui::strong(news_title, 16).width(Length::Fill),
                    ui::badge("v1.3.0", c.accent),
                ]
                .align_items(Alignment::Center),
                ui::muted(env, n1),
                ui::muted(env, n2),
                ui::muted(env, n3),
                ui::muted(env, n4),
                ui::btn(
                    setup_btn_label,
                    Btn::Secondary,
                    Some(Message::Navigate(Module::Installer)),
                ),
            ]
            .spacing(8),
        );

        let right_panel = column![status_card, news_card].spacing(14).width(Length::Fill);

        let body: Element<'a, Message> = if env.wide() {
            row![
                container(tools_col).width(Length::FillPortion(7)),
                container(right_panel).width(Length::FillPortion(3)),
            ]
            .spacing(18)
            .into()
        } else {
            column![tools_col, right_panel].spacing(18).into()
        };

        ui::page(env, vec![top_bar, hero.into(), body])
    }
}
