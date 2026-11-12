//! Aplicação principal do Axolote Suite:
//! - Barra lateral com animação suave de expansão/recolhimento (60 FPS) e
//!   indicador Neon animado no item selecionado
//! - Calculadora como item direto do menu lateral (e também em Produtividade)
//! - Barra superior com busca rápida de ferramentas
//! - Painel inicial (Home/Dashboard) com acesso direto aos módulos, fila de
//!   operações da sessão atual e histórico recente real
//! - Roteamento para todos os módulos, incluindo Editor de Fotos, Editor de Vídeo,
//!   Estúdio de Cores & Senhas e Central de PDF.

use std::path::PathBuf;
use std::time::Instant;

use iced::widget::text::Shaping;
use iced::widget::{button, column, container, image as iced_image, row, scrollable, text, Space};
use iced::{Alignment, Application, Command, Element, Event, Length, Subscription, Theme};

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
            Module::Calculator => "🔢",
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
                        "Uninstall / Cleanup"
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
                    "Quick notepad (.txt), task checklist, and folder organizer."
                }
                Module::Calculator => {
                    "Expression calculator with parentheses, power (^), modulo (%), and history."
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
    QuickLaunchQuickConvert(quick_convert::WheelCategory),
    QuickLaunchProductivity(productivity::Tab),
    QuickLaunchImages(images::Mode),
    QuickLaunchVideo(video::Mode),
    QuickLaunchPdf(pdf::Mode),
    QuickLaunchDevTools(devtools::Tab),
    QuickLaunchZip(zip::Mode),
    SearchChanged(String),
    SearchSubmitted,
    OpenSearchResult(Module),
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
        let lang = self.ctx.settings.resolved_lang();
        format!(
            "Axolote Suite v{} — {}",
            env!("CARGO_PKG_VERSION"),
            self.active.title_in(lang, self.installer.is_installed())
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
            Message::QuickLaunchQuickConvert(cat) => {
                self.quick_convert.category = cat;
                self.quick_convert.source_ext = cat.default_source_ext().into();
                self.quick_convert.selected_idx = 0;
                self.quick_convert.refresh_targets();
                self.active = Module::QuickConvert;
                self.home_search.clear();
                self.nav_pulse = 1.0;
                self.last_tick = Some(Instant::now());
                Command::none()
            }
            Message::QuickLaunchProductivity(tab) => {
                self.productivity.tab = tab;
                self.active = if tab == productivity::Tab::Calc {
                    Module::Calculator
                } else {
                    Module::Productivity
                };
                self.home_search.clear();
                self.nav_pulse = 1.0;
                self.last_tick = Some(Instant::now());
                Command::none()
            }
            Message::QuickLaunchImages(mode) => {
                self.images.mode = mode;
                self.active = Module::Images;
                self.home_search.clear();
                self.nav_pulse = 1.0;
                self.last_tick = Some(Instant::now());
                Command::none()
            }
            Message::QuickLaunchVideo(mode) => {
                self.video.mode = mode;
                self.active = Module::Video;
                self.home_search.clear();
                self.nav_pulse = 1.0;
                self.last_tick = Some(Instant::now());
                Command::none()
            }
            Message::QuickLaunchPdf(mode) => {
                self.pdf.mode = mode;
                self.active = Module::Pdf;
                self.home_search.clear();
                self.nav_pulse = 1.0;
                self.last_tick = Some(Instant::now());
                Command::none()
            }
            Message::QuickLaunchDevTools(tab) => {
                self.devtools.tab = tab;
                self.active = Module::DevTools;
                self.home_search.clear();
                self.nav_pulse = 1.0;
                self.last_tick = Some(Instant::now());
                Command::none()
            }
            Message::QuickLaunchZip(mode) => {
                self.zip.mode = mode;
                self.active = Module::Zip;
                self.home_search.clear();
                self.nav_pulse = 1.0;
                self.last_tick = Some(Instant::now());
                Command::none()
            }
            Message::SearchChanged(q) => {
                self.home_search = q;
                Command::none()
            }
            Message::SearchSubmitted => {
                let q = self.home_search.trim().to_string();
                if !q.is_empty() {
                    let matches = self.search_matches(&q);
                    if let Some((first_mod, _, _)) = matches.first() {
                        self.open_search_target(*first_mod, &q);
                    }
                }
                Command::none()
            }
            Message::OpenSearchResult(module) => {
                let q = self.home_search.trim().to_string();
                self.open_search_target(module, &q);
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
            Message::Audio(m) => {
                let refresh_ff = matches!(
                    m,
                    audio::Msg::DownloadFfmpegDone(_) | audio::Msg::RecheckFfmpeg
                );
                let cmd = self.audio.update(m, &mut self.ctx).map(Message::Audio);
                if refresh_ff {
                    self.sync_ffmpeg_status();
                }
                cmd
            }
            Message::Video(m) => {
                let refresh_ff = matches!(
                    m,
                    video::Msg::DownloadFfmpegDone(_) | video::Msg::RecheckFfmpeg
                );
                let cmd = self.video.update(m, &mut self.ctx).map(Message::Video);
                if refresh_ff {
                    self.sync_ffmpeg_status();
                }
                cmd
            }
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
                    self.sync_ffmpeg_status();
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
    fn sync_ffmpeg_status(&mut self) {
        let ok = crate::core::system::find_ffmpeg().is_some();
        self.video.ffmpeg_ok = ok;
        self.audio.ffmpeg_ok = ok;
        self.quick_convert.refresh_targets();
    }

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
        let is_en = env.lang.is_en();
        let header = ui::page_header(
            env,
            "🔢",
            if is_en {
                "Express Scientific Calculator"
            } else {
                "Calculadora Científica Expressa"
            },
            if is_en {
                "Evaluate quick math or full expressions with parentheses, power (^), modulo (%), and clickable history."
            } else {
                "Faça contas rápidas ou expressões completas com parênteses, potência (^), porcentagem (%) e histórico clicável."
            },
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
                        container(
                            text(m.icon())
                                .size(if is_active { 20 } else { 18 })
                                .shaping(Shaping::Advanced)
                        )
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
                        text(m.icon())
                            .size(if is_active { 17 } else { 15 })
                            .shaping(Shaping::Advanced),
                        text(m.title_in(env.lang, self.installer.is_installed()))
                            .size(13)
                            .font(if is_active { BOLD } else { SEMIBOLD })
                            .shaping(Shaping::Advanced)
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

        let theme_icon = if self.ctx.settings.dark_mode {
            "☀"
        } else {
            "🌙"
        };
        let footer_content: Element<'_, Message> = if collapsed {
            container(text(theme_icon).size(15).shaping(Shaping::Advanced))
                .center_x()
                .width(Length::Fill)
                .into()
        } else {
            let theme_text = if env.lang.is_en() {
                if self.ctx.settings.dark_mode {
                    "Light Theme"
                } else {
                    "Dark Theme"
                }
            } else if self.ctx.settings.dark_mode {
                "Tema Claro"
            } else {
                "Tema Escuro"
            };
            container(
                row![
                    text(theme_icon).size(14).shaping(Shaping::Advanced),
                    text(theme_text)
                        .size(12)
                        .font(SEMIBOLD)
                        .shaping(Shaping::Advanced),
                ]
                .spacing(8)
                .align_items(Alignment::Center),
            )
            .center_x()
            .width(Length::Fill)
            .into()
        };

        let footer = button(footer_content)
            .padding([10, 10])
            .width(Length::Fill)
            .style(theme::btn(Btn::Secondary))
            .on_press(Message::ToggleTheme);

        let col = column![
            brand_header,
            Space::with_height(6),
            scrollable(nav)
                .height(Length::Fill)
                .direction(scrollable::Direction::Vertical(
                    scrollable::Properties::new().width(4).scroller_width(4),
                )),
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
        let is_en = env.lang.is_en();
        let wide_grid = env.is_desktop_wide();
        let med_grid = env.is_medium();

        // 1. Barra Superior Responsiva
        let top_bar: Element<'a, Message> = {
            let search_ph = if is_en {
                "Search tools, actions or formats…"
            } else {
                "Buscar ferramenta, ação ou formato…"
            };
            let search_input = ui::input(search_ph, &self.home_search, Message::SearchChanged)
                .on_submit(Message::SearchSubmitted)
                .width(Length::Fill);

            let search_box: Element<'a, Message> = if !self.home_search.is_empty() {
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
                ui::strong("Axolote Suite", 17),
                container(
                    text(format!("v{}", env!("CARGO_PKG_VERSION")))
                        .size(10)
                        .font(BOLD)
                        .style(c.primary)
                )
                .padding([3, 7])
                .style(theme::solid_box(c.surface2, 6.0)),
            ]
            .spacing(8)
            .align_items(Alignment::Center);

            let running_now = self.ctx.jobs.running();
            let status_btn: Element<'a, Message> = if running_now > 0 {
                button(
                    row![
                        container(Space::new(Length::Fixed(7.0), Length::Fixed(7.0)))
                            .style(theme::solid_box(c.accent, 999.0)),
                        ui::colored(
                            if is_en {
                                format!("{running_now} task(s) running")
                            } else {
                                format!("{running_now} tarefa(s) ativa(s)")
                            },
                            11,
                            c.accent
                        )
                        .font(BOLD),
                    ]
                    .spacing(6)
                    .align_items(Alignment::Center),
                )
                .padding([6, 12])
                .style(theme::btn(Btn::Secondary))
                .on_press(Message::Navigate(Module::Queue))
                .into()
            } else {
                button(
                    row![
                        container(Space::new(Length::Fixed(7.0), Length::Fixed(7.0)))
                            .style(theme::solid_box(c.success, 999.0)),
                        ui::colored("100% Offline", 11, c.accent).font(BOLD),
                        text("•").size(11).style(c.muted),
                        text(if is_en {
                            "Local Processing"
                        } else {
                            "Processamento local"
                        })
                        .size(11)
                        .style(c.text),
                    ]
                    .spacing(6)
                    .align_items(Alignment::Center),
                )
                .padding([6, 12])
                .style(theme::btn(Btn::Secondary))
                .on_press(Message::Navigate(Module::Settings))
                .into()
            };

            if env.width < 900.0 {
                column![
                    row![brand_tag, ui::fill(), status_btn]
                        .spacing(8)
                        .align_items(Alignment::Center),
                    search_box,
                ]
                .spacing(10)
                .into()
            } else {
                let search_w = (env.width - 560.0).clamp(240.0, 460.0);
                row![
                    brand_tag,
                    container(search_box).width(Length::Fixed(search_w)),
                    ui::fill(),
                    status_btn,
                ]
                .spacing(12)
                .align_items(Alignment::Center)
                .into()
            }
        };

        let hero_h = wide_grid.then_some(280.0);
        let row1_pair_h = if wide_grid {
            Some(280.0)
        } else if med_grid {
            Some(266.0)
        } else {
            None
        };
        let row2_pair_h = if wide_grid {
            Some(252.0)
        } else if med_grid {
            Some(242.0)
        } else {
            None
        };
        let shortcuts_h = wide_grid.then_some(252.0);
        let row3_pair_h = (wide_grid || med_grid).then_some(164.0);
        let prod_h = wide_grid.then_some(164.0);

        // 2. Card 1 (Span 2): Conversão & Despacho Rápido
        let quick_actions: Element<'a, Message> = {
            let b1 = home_action_btn(
                if is_en {
                    "🖼  Images"
                } else {
                    "🖼  Imagens"
                },
                Btn::Secondary,
                Some(Message::QuickLaunchQuickConvert(
                    quick_convert::WheelCategory::Image,
                )),
            );
            let b2 = home_action_btn(
                if is_en {
                    "🎬  Compress"
                } else {
                    "🎬  Comprimir"
                },
                Btn::Secondary,
                Some(Message::QuickLaunchVideo(video::Mode::ConvertCompress)),
            );
            let b3 = home_action_btn(
                if is_en { "🎵  Audio" } else { "🎵  Áudio" },
                Btn::Secondary,
                Some(Message::Navigate(Module::Audio)),
            );
            if env.narrow() {
                column![b1, b2, b3].spacing(8).width(Length::Fill).into()
            } else {
                row![b1, b2, b3].spacing(8).width(Length::Fill).into()
            }
        };

        let hero_card = bento_panel(
            env,
            hero_h,
            column![
                row![
                    row![
                        container(text("🔄").size(17).shaping(Shaping::Advanced))
                            .padding([6, 9])
                            .style(theme::solid_box(c.surface2, 8.0)),
                        column![
                            ui::strong(
                                if is_en {
                                    "Quick Convert & Dispatch"
                                } else {
                                    "Conversão & Despacho Rápido"
                                },
                                16
                            ),
                            ui::muted(
                                env,
                                if is_en {
                                    "Local processing without uploading your files"
                                } else {
                                    "Processamento local, sem envio dos seus arquivos"
                                }
                            )
                            .size(12),
                        ]
                        .spacing(2),
                    ]
                    .spacing(10)
                    .align_items(Alignment::Center),
                    ui::fill(),
                    container(text("Local").size(10).font(BOLD).style(c.accent))
                        .padding([3, 8])
                        .style(theme::inner_box),
                ]
                .align_items(Alignment::Center),
                container(
                    column![
                        button(
                            column![
                                container(
                                    text("☁ ⬆")
                                        .size(20)
                                        .style(c.primary)
                                        .shaping(Shaping::Advanced)
                                )
                                .padding([8, 12])
                                .style(theme::solid_box(c.primary_soft, 999.0)),
                                ui::strong(
                                    if is_en {
                                        "Drop files anywhere or click to choose"
                                    } else {
                                        "Solte arquivos na janela ou clique para escolher"
                                    },
                                    14
                                ),
                                ui::muted(
                                    env,
                                    if is_en {
                                        "Images, Docs (PDF/DOCX/MD/HTML), Video, Audio & ZIP"
                                    } else {
                                        "Imagens, Docs (PDF/DOCX/MD/HTML), Vídeo, Áudio e ZIP"
                                    },
                                )
                                .size(11),
                            ]
                            .spacing(4)
                            .align_items(Alignment::Center)
                            .width(Length::Fill),
                        )
                        .padding([8, 10])
                        .width(Length::Fill)
                        .style(theme::btn(Btn::Ghost))
                        .on_press(Message::Navigate(Module::QuickConvert)),
                        quick_actions,
                    ]
                    .spacing(8)
                    .width(Length::Fill),
                )
                .padding([10, 12])
                .width(Length::Fill)
                .style(theme::dropzone_box),
                card_spacer(hero_h),
                row![
                    row![
                        container(Space::new(Length::Fixed(6.0), Length::Fixed(6.0))).style(
                            theme::solid_box(
                                if self.video.ffmpeg_ok {
                                    c.success
                                } else {
                                    c.warning
                                },
                                999.0
                            )
                        ),
                        ui::muted(
                            env,
                            if self.video.ffmpeg_ok {
                                if is_en {
                                    "FFmpeg ready for video & audio"
                                } else {
                                    "FFmpeg ativo para vídeo e áudio"
                                }
                            } else if is_en {
                                "FFmpeg auto-downloads on first media task"
                            } else {
                                "FFmpeg sob demanda ao converter mídia"
                            }
                        )
                        .size(11),
                    ]
                    .spacing(6)
                    .align_items(Alignment::Center),
                    ui::fill(),
                    home_link_btn(
                        if is_en {
                            "Open Converter →"
                        } else {
                            "Abrir Conversor →"
                        },
                        Some(Message::Navigate(Module::QuickConvert)),
                    ),
                ]
                .align_items(Alignment::Center),
            ]
            .spacing(8)
            .into(),
        );

        // 3. Card 2: Estúdio de Imagens
        let bento_images = bento_panel(
            env,
            row1_pair_h,
            column![
                row![
                    row![
                        container(text("🖼").size(16).shaping(Shaping::Advanced))
                            .padding([5, 8])
                            .style(theme::inner_box),
                        ui::strong(
                            if is_en {
                                "Photo Studio"
                            } else {
                                "Estúdio de Imagens"
                            },
                            15
                        ),
                    ]
                    .spacing(8)
                    .align_items(Alignment::Center),
                    ui::fill(),
                    container(text("Local").size(10).style(c.success))
                        .padding([2, 6])
                        .style(theme::inner_box),
                ]
                .align_items(Alignment::Center),
                ui::muted(
                    env,
                    if is_en {
                        "Live RGB histogram, light/color edits, crop and batch."
                    } else {
                        "Histograma RGB ao vivo, ajustes de luz/cor e lote."
                    }
                )
                .size(12),
                column![
                    home_action_btn(
                        if is_en {
                            "🎨  Live Photo Developer"
                        } else {
                            "🎨  Revelador ao Vivo"
                        },
                        Btn::Primary,
                        Some(Message::QuickLaunchImages(images::Mode::Editor)),
                    ),
                    home_action_btn(
                        if is_en {
                            "⚡  Batch Sync (Folder)"
                        } else {
                            "⚡  Sincronizar em Lote"
                        },
                        Btn::Secondary,
                        Some(Message::QuickLaunchImages(images::Mode::Batch)),
                    ),
                    home_action_btn(
                        if is_en {
                            "📐  Resize & Convert"
                        } else {
                            "📐  Redimensionar & Formatos"
                        },
                        Btn::Secondary,
                        Some(Message::QuickLaunchImages(images::Mode::ConvertResize)),
                    ),
                ]
                .spacing(6)
                .width(Length::Fill),
                card_spacer(row1_pair_h),
                row![
                    text("PNG • JPG • WebP • ICO").size(10).style(c.muted),
                    ui::fill(),
                    home_link_btn(
                        if is_en { "Open →" } else { "Abrir →" },
                        Some(Message::Navigate(Module::Images)),
                    ),
                ]
                .align_items(Alignment::Center),
            ]
            .spacing(8)
            .into(),
        );

        // 4. Card 3: Fila de Operações (dados reais da sessão)
        let jobs = self.ctx.jobs.all();
        let running_jobs = self.ctx.jobs.running();
        let finished_jobs = jobs.len().saturating_sub(running_jobs);
        let mut jobs_list = column![].spacing(6).width(Length::Fill);

        if jobs.is_empty() {
            jobs_list = jobs_list.push(
                container(
                    column![
                        ui::strong(
                            if is_en {
                                "Queue is idle"
                            } else {
                                "Nenhuma operação na sessão"
                            },
                            12
                        ),
                        ui::muted(
                            env,
                            if is_en {
                                "Background tasks from converter, video, audio, PDF and ZIP appear here."
                            } else {
                                "Tarefas do conversor, vídeo, áudio, PDF e ZIP aparecerão aqui em tempo real."
                            }
                        )
                        .size(11),
                    ]
                    .spacing(4),
                )
                .padding(12)
                .width(Length::Fill)
                .style(theme::inner_box),
            );
        } else {
            for job in jobs.iter().take(2) {
                let (status, status_color) = match job.status {
                    crate::core::jobs::JobStatus::Running => {
                        (if is_en { "Running" } else { "Em andamento" }, c.accent)
                    }
                    crate::core::jobs::JobStatus::Done => {
                        (if is_en { "Done" } else { "Concluída" }, c.success)
                    }
                    crate::core::jobs::JobStatus::Failed => {
                        (if is_en { "Failed" } else { "Falhou" }, c.danger)
                    }
                };
                let time_info = match job.elapsed {
                    Some(d) => format_duration_short(d),
                    None => job.started_at.clone(),
                };
                let detail = truncate_label(&format!("{} • {}", job.module, job.message), 48);
                let detail_color = if job.status == crate::core::jobs::JobStatus::Failed {
                    c.danger
                } else {
                    c.muted
                };

                jobs_list = jobs_list.push(
                    container(
                        column![
                            row![
                                ui::strong(truncate_label(&job.title, 26), 12).width(Length::Fill),
                                ui::badge(status, status_color),
                            ]
                            .spacing(6)
                            .align_items(Alignment::Center),
                            row![
                                text(detail)
                                    .size(10)
                                    .style(detail_color)
                                    .width(Length::Fill),
                                text(time_info).size(10).style(c.muted),
                            ]
                            .spacing(6)
                            .align_items(Alignment::Center),
                        ]
                        .spacing(3),
                    )
                    .padding([7, 10])
                    .width(Length::Fill)
                    .style(theme::inner_box),
                );
            }
            if jobs.len() > 2 {
                jobs_list = jobs_list.push(
                    ui::muted(
                        env,
                        if is_en {
                            format!("+{} more task(s) in queue", jobs.len() - 2)
                        } else {
                            format!("+{} operação(ões) na fila", jobs.len() - 2)
                        },
                    )
                    .size(10),
                );
            }
        }

        let queue_badge_text = if running_jobs > 0 {
            if is_en {
                format!("{running_jobs} active")
            } else {
                format!("{running_jobs} ativa(s)")
            }
        } else if !jobs.is_empty() {
            if is_en {
                format!("{} logged", jobs.len())
            } else {
                format!("{} na sessão", jobs.len())
            }
        } else if is_en {
            "Idle".to_string()
        } else {
            "Livre".to_string()
        };

        let bento_queue = bento_panel(
            env,
            row1_pair_h,
            column![
                row![
                    row![
                        container(text("⚡").size(16).shaping(Shaping::Advanced))
                            .padding([5, 8])
                            .style(theme::inner_box),
                        ui::strong(
                            if is_en {
                                "Tasks Queue"
                            } else {
                                "Fila de Operações"
                            },
                            15
                        ),
                    ]
                    .spacing(8)
                    .align_items(Alignment::Center),
                    ui::fill(),
                    ui::badge(
                        queue_badge_text,
                        if running_jobs == 0 {
                            c.success
                        } else {
                            c.accent
                        }
                    ),
                ]
                .align_items(Alignment::Center),
                jobs_list,
                card_spacer(row1_pair_h),
                row![
                    if finished_jobs > 0 {
                        home_link_btn(
                            if is_en { "Clear done" } else { "Limpar" },
                            Some(Message::Queue(queue::Msg::ClearFinished)),
                        )
                    } else {
                        ui::muted(
                            env,
                            if is_en {
                                "Current session"
                            } else {
                                "Sessão atual"
                            },
                        )
                        .size(10)
                        .into()
                    },
                    ui::fill(),
                    home_link_btn(
                        if is_en {
                            "Open queue →"
                        } else {
                            "Abrir fila →"
                        },
                        Some(Message::Navigate(Module::Queue)),
                    ),
                ]
                .align_items(Alignment::Center),
            ]
            .spacing(8)
            .into(),
        );

        // 5. Card 4: Cores & Senhas (DevTools)
        let bento_colors_pass = bento_panel(
            env,
            row2_pair_h,
            column![
                row![
                    row![
                        container(text("🎨").size(16).shaping(Shaping::Advanced))
                            .padding([5, 8])
                            .style(theme::inner_box),
                        ui::strong(
                            if is_en {
                                "Colors & Passwords"
                            } else {
                                "Cores & Senhas"
                            },
                            15
                        ),
                    ]
                    .spacing(8)
                    .align_items(Alignment::Center),
                    ui::fill(),
                    container(text("Local").size(10).font(BOLD).style(c.success))
                        .padding([2, 6])
                        .style(theme::inner_box),
                ]
                .align_items(Alignment::Center),
                ui::muted(
                    env,
                    if is_en {
                        "HEX/RGB/HSL/CMYK converter, photo palette, passwords & UUID."
                    } else {
                        "Conversor HEX/RGB/HSL/CMYK, paleta de fotos, senhas e UUID."
                    }
                )
                .size(12),
                column![
                    home_action_btn(
                        if is_en {
                            "🔐  Passwords & UUID v4"
                        } else {
                            "🔐  Senhas Fortes & UUID"
                        },
                        Btn::Primary,
                        Some(Message::QuickLaunchDevTools(devtools::Tab::Passwords)),
                    ),
                    home_action_btn(
                        if is_en {
                            "🎨  Colors & Palette"
                        } else {
                            "🎨  Cores & Paleta"
                        },
                        Btn::Secondary,
                        Some(Message::QuickLaunchDevTools(devtools::Tab::Colors)),
                    ),
                ]
                .spacing(6)
                .width(Length::Fill),
                card_spacer(row2_pair_h),
                row![
                    text(if is_en { "On demand" } else { "Sob demanda" })
                        .size(10)
                        .style(c.muted),
                    ui::fill(),
                    home_link_btn(
                        if is_en { "Open →" } else { "Abrir →" },
                        Some(Message::Navigate(Module::DevTools)),
                    ),
                ]
                .align_items(Alignment::Center),
            ]
            .spacing(8)
            .into(),
        );

        // 6. Card 5: PDF & Docs
        let bento_pdf = bento_panel(
            env,
            row2_pair_h,
            column![
                row![
                    row![
                        container(text("📄").size(16).shaping(Shaping::Advanced))
                            .padding([5, 8])
                            .style(theme::inner_box),
                        ui::strong("PDF & Docs", 15),
                    ]
                    .spacing(8)
                    .align_items(Alignment::Center),
                    ui::fill(),
                    container(text("Local").size(10).style(c.muted))
                        .padding([2, 6])
                        .style(theme::inner_box),
                ]
                .align_items(Alignment::Center),
                row![
                    button(
                        container(
                            column![
                                text("🔗").size(17).shaping(Shaping::Advanced),
                                ui::strong(if is_en { "Merge" } else { "Unir" }, 12),
                                text(if is_en { "Order PDFs" } else { "Juntar PDFs" })
                                    .size(10)
                                    .style(c.muted),
                            ]
                            .spacing(2)
                            .align_items(Alignment::Center),
                        )
                        .padding([9, 6])
                        .width(Length::Fill)
                        .style(theme::inner_box),
                    )
                    .width(Length::FillPortion(1))
                    .style(theme::btn(Btn::Ghost))
                    .on_press(Message::QuickLaunchPdf(pdf::Mode::Merge)),
                    button(
                        container(
                            column![
                                text("✂").size(17).shaping(Shaping::Advanced),
                                ui::strong(if is_en { "Split" } else { "Dividir" }, 12),
                                text(if is_en { "Page ranges" } else { "Páginas" })
                                    .size(10)
                                    .style(c.muted),
                            ]
                            .spacing(2)
                            .align_items(Alignment::Center),
                        )
                        .padding([9, 6])
                        .width(Length::Fill)
                        .style(theme::inner_box),
                    )
                    .width(Length::FillPortion(1))
                    .style(theme::btn(Btn::Ghost))
                    .on_press(Message::QuickLaunchPdf(pdf::Mode::Split)),
                ]
                .spacing(8)
                .width(Length::Fill),
                ui::muted(
                    env,
                    if is_en {
                        "Inspect pages and metadata without modifying originals."
                    } else {
                        "Inspecione páginas e metadados sem alterar os originais."
                    }
                )
                .size(11),
                card_spacer(row2_pair_h),
                row![
                    text(if is_en {
                        "Merge • Split"
                    } else {
                        "Unir • Dividir"
                    })
                    .size(10)
                    .style(c.muted),
                    ui::fill(),
                    home_link_btn(
                        if is_en {
                            "Open Hub →"
                        } else {
                            "Abrir Central →"
                        },
                        Some(Message::Navigate(Module::Pdf)),
                    ),
                ]
                .align_items(Alignment::Center),
            ]
            .spacing(8)
            .into(),
        );

        // 7. Card 6 (Span 2): Atalhos & Atividade Recente (Histórico real)
        let recent_activity: Element<'a, Message> = {
            let entries = self.ctx.history.entries();
            if entries.is_empty() {
                container(
                    ui::muted(
                        env,
                        if is_en {
                            "No activity recorded yet. Completed operations appear here."
                        } else {
                            "Nenhuma atividade registrada ainda. As operações concluídas aparecerão aqui."
                        },
                    )
                    .size(11),
                )
                .padding([10, 12])
                .width(Length::Fill)
                .style(theme::inner_box)
                .into()
            } else {
                let mut rows = column![].spacing(6).width(Length::Fill);
                for entry in entries.iter().take(2) {
                    let (status, status_color) = if entry.success {
                        (if is_en { "Done" } else { "Concluída" }, c.success)
                    } else {
                        (if is_en { "Failed" } else { "Falhou" }, c.danger)
                    };
                    let time_short = short_timestamp(&entry.timestamp);
                    let detail = if entry.details.trim().is_empty()
                        || entry.details.trim() == entry.action.trim()
                    {
                        entry.module.clone()
                    } else {
                        truncate_label(&format!("{} • {}", entry.module, entry.details), 52)
                    };
                    let detail_color = if entry.success { c.muted } else { c.danger };
                    rows = rows.push(
                        container(
                            column![
                                row![
                                    ui::strong(truncate_label(&entry.action, 40), 11)
                                        .width(Length::Fill),
                                    text(time_short).size(10).style(c.muted),
                                    ui::badge(status, status_color),
                                ]
                                .spacing(6)
                                .align_items(Alignment::Center),
                                text(detail).size(10).style(detail_color),
                            ]
                            .spacing(2),
                        )
                        .padding([6, 10])
                        .width(Length::Fill)
                        .style(theme::inner_box),
                    );
                }
                rows.into()
            }
        };

        let quick_tiles: Element<'a, Message> = {
            let t1 = quick_nav_tile(
                "🔄",
                if is_en { "Convert" } else { "Conversor" },
                Message::Navigate(Module::QuickConvert),
            );
            let t2 = quick_nav_tile(
                "🖼",
                if is_en { "Photos" } else { "Imagens" },
                Message::Navigate(Module::Images),
            );
            let t3 = quick_nav_tile("🔢", "Calc", Message::Navigate(Module::Calculator));
            let t4 = quick_nav_tile(
                "📦",
                if is_en { "Queue" } else { "Fila" },
                Message::Navigate(Module::Queue),
            );
            if env.narrow() {
                column![
                    row![t1, t2].spacing(8).width(Length::Fill),
                    row![t3, t4].spacing(8).width(Length::Fill),
                ]
                .spacing(8)
                .width(Length::Fill)
                .into()
            } else {
                row![t1, t2, t3, t4].spacing(8).width(Length::Fill).into()
            }
        };

        let bento_shortcuts = bento_panel(
            env,
            shortcuts_h,
            column![
                row![
                    row![
                        container(text("🕒").size(16).shaping(Shaping::Advanced))
                            .padding([5, 8])
                            .style(theme::inner_box),
                        ui::strong(
                            if is_en {
                                "Shortcuts & Recent Activity"
                            } else {
                                "Atalhos & Atividade Recente"
                            },
                            15
                        ),
                    ]
                    .spacing(8)
                    .align_items(Alignment::Center),
                    ui::fill(),
                    button(
                        text("Ctrl + Alt + N")
                            .size(10)
                            .font(BOLD)
                            .style(c.accent)
                            .shaping(Shaping::Advanced),
                    )
                    .padding([4, 8])
                    .style(theme::btn(Btn::Secondary))
                    .on_press(Message::OpenQuickConvertShortcut),
                ]
                .align_items(Alignment::Center),
                quick_tiles,
                row![
                    text(if is_en {
                        "Recent history"
                    } else {
                        "Histórico recente"
                    })
                    .size(11)
                    .style(c.muted),
                    ui::fill(),
                    home_link_btn(
                        if is_en {
                            "Full history →"
                        } else {
                            "Ver histórico completo →"
                        },
                        Some(Message::Navigate(Module::History)),
                    ),
                ]
                .align_items(Alignment::Center),
                recent_activity,
            ]
            .spacing(7)
            .into(),
        );

        // 8. Linha 3: Demais módulos reais da suíte (Vídeo/Áudio, ZIP/Lote, Produtividade/Calculadora)
        let bento_media = bento_panel(
            env,
            row3_pair_h,
            column![
                row![
                    row![
                        container(text("🎬").size(15).shaping(Shaping::Advanced))
                            .padding([4, 7])
                            .style(theme::inner_box),
                        ui::strong(
                            if is_en {
                                "Video & Audio"
                            } else {
                                "Vídeo & Áudio"
                            },
                            14
                        ),
                    ]
                    .spacing(8)
                    .align_items(Alignment::Center),
                    ui::fill(),
                    ui::badge(
                        if self.video.ffmpeg_ok {
                            if is_en {
                                "FFmpeg ready"
                            } else {
                                "FFmpeg ativo"
                            }
                        } else if is_en {
                            "FFmpeg on demand"
                        } else {
                            "FFmpeg pendente"
                        },
                        if self.video.ffmpeg_ok {
                            c.success
                        } else {
                            c.warning
                        },
                    ),
                ]
                .align_items(Alignment::Center),
                ui::muted(
                    env,
                    if is_en {
                        "Trim clips, 9:16/16:9 framing, CRF compression, and audio bitrate."
                    } else {
                        "Corte na linha do tempo, 9:16/16:9, compressão CRF e bitrate."
                    }
                )
                .size(11),
                card_spacer(row3_pair_h),
                row![
                    home_action_btn(
                        if is_en {
                            "🎬  Videos"
                        } else {
                            "🎬  Vídeos"
                        },
                        Btn::Secondary,
                        Some(Message::Navigate(Module::Video)),
                    ),
                    home_action_btn(
                        if is_en { "🎵  Audio" } else { "🎵  Áudio" },
                        Btn::Secondary,
                        Some(Message::Navigate(Module::Audio)),
                    ),
                ]
                .spacing(8)
                .width(Length::Fill),
            ]
            .spacing(8)
            .into(),
        );

        let bento_files = bento_panel(
            env,
            row3_pair_h,
            column![
                row![
                    row![
                        container(text("🗜").size(15).shaping(Shaping::Advanced))
                            .padding([4, 7])
                            .style(theme::inner_box),
                        ui::strong(
                            if is_en {
                                "ZIP & Batch Files"
                            } else {
                                "ZIP & Lote"
                            },
                            14
                        ),
                    ]
                    .spacing(8)
                    .align_items(Alignment::Center),
                    ui::fill(),
                    container(text("Anti Zip-Slip").size(10).style(c.success))
                        .padding([2, 6])
                        .style(theme::inner_box),
                ]
                .align_items(Alignment::Center),
                ui::muted(
                    env,
                    if is_en {
                        "Compress folders, extract ZIPs safely, and bulk rename files."
                    } else {
                        "Compacte pastas, extraia ZIPs e renomeie arquivos em lote."
                    }
                )
                .size(11),
                card_spacer(row3_pair_h),
                row![
                    home_action_btn(
                        "🗜  ZIP",
                        Btn::Secondary,
                        Some(Message::QuickLaunchZip(zip::Mode::Extract)),
                    ),
                    home_action_btn(
                        if is_en {
                            "📁  Batch Rename"
                        } else {
                            "📁  Renomear"
                        },
                        Btn::Secondary,
                        Some(Message::Navigate(Module::BatchRename)),
                    ),
                ]
                .spacing(8)
                .width(Length::Fill),
            ]
            .spacing(8)
            .into(),
        );

        let notes_count = self.productivity.notes.len();
        let pending_tasks = self.productivity.pending_tasks();
        let prod_badge = if env.width < 1120.0 && wide_grid {
            format!("{notes_count}N • {pending_tasks}T")
        } else if is_en {
            format!("{notes_count} notes • {pending_tasks} pend.")
        } else {
            format!("{notes_count} notas • {pending_tasks} pend.")
        };
        let bento_productivity = bento_panel(
            env,
            prod_h,
            column![
                row![
                    row![
                        container(text("📋").size(15).shaping(Shaping::Advanced))
                            .padding([4, 7])
                            .style(theme::inner_box),
                        ui::strong(
                            if is_en {
                                "Productivity & Calc"
                            } else {
                                "Produtividade & Calc"
                            },
                            14
                        ),
                    ]
                    .spacing(8)
                    .align_items(Alignment::Center),
                    ui::fill(),
                    ui::badge(prod_badge, c.primary),
                ]
                .align_items(Alignment::Center),
                card_spacer(prod_h),
                column![
                    row![
                        home_action_btn(
                            if is_en { "📝  Notes" } else { "📝  Notas" },
                            Btn::Secondary,
                            Some(Message::QuickLaunchProductivity(productivity::Tab::Notes)),
                        ),
                        home_action_btn(
                            if is_en { "☑  Tasks" } else { "☑  Tarefas" },
                            Btn::Secondary,
                            Some(Message::QuickLaunchProductivity(productivity::Tab::Tasks)),
                        ),
                    ]
                    .spacing(6)
                    .width(Length::Fill),
                    row![
                        home_action_btn(
                            if is_en {
                                "🔢  Calculator"
                            } else {
                                "🔢  Calculadora"
                            },
                            Btn::Secondary,
                            Some(Message::QuickLaunchProductivity(productivity::Tab::Calc)),
                        ),
                        home_action_btn(
                            if is_en {
                                "🗂  Organizer"
                            } else {
                                "🗂  Organizar"
                            },
                            Btn::Secondary,
                            Some(Message::QuickLaunchProductivity(
                                productivity::Tab::Organizer
                            )),
                        ),
                    ]
                    .spacing(6)
                    .width(Length::Fill),
                ]
                .spacing(6)
                .width(Length::Fill),
            ]
            .spacing(8)
            .into(),
        );

        // Grid Bento Responsivo
        let bento_grid: Element<'a, Message> = if wide_grid {
            column![
                row![
                    container(hero_card).width(Length::FillPortion(2)),
                    container(bento_images).width(Length::FillPortion(1)),
                    container(bento_queue).width(Length::FillPortion(1)),
                ]
                .spacing(12),
                row![
                    container(bento_colors_pass).width(Length::FillPortion(1)),
                    container(bento_pdf).width(Length::FillPortion(1)),
                    container(bento_shortcuts).width(Length::FillPortion(2)),
                ]
                .spacing(12),
                row![
                    container(bento_media).width(Length::FillPortion(1)),
                    container(bento_files).width(Length::FillPortion(1)),
                    container(bento_productivity).width(Length::FillPortion(1)),
                ]
                .spacing(12),
            ]
            .spacing(12)
            .width(Length::Fill)
            .into()
        } else if med_grid {
            column![
                hero_card,
                row![
                    container(bento_images).width(Length::FillPortion(1)),
                    container(bento_queue).width(Length::FillPortion(1)),
                ]
                .spacing(12),
                row![
                    container(bento_colors_pass).width(Length::FillPortion(1)),
                    container(bento_pdf).width(Length::FillPortion(1)),
                ]
                .spacing(12),
                bento_shortcuts,
                row![
                    container(bento_media).width(Length::FillPortion(1)),
                    container(bento_files).width(Length::FillPortion(1)),
                ]
                .spacing(12),
                bento_productivity,
            ]
            .spacing(12)
            .width(Length::Fill)
            .into()
        } else {
            column![
                hero_card,
                bento_images,
                bento_queue,
                bento_colors_pass,
                bento_pdf,
                bento_shortcuts,
                bento_media,
                bento_files,
                bento_productivity,
            ]
            .spacing(12)
            .width(Length::Fill)
            .into()
        };

        if !self.home_search.trim().is_empty() {
            let search_results = self.search_results_view(env, self.home_search.trim());
            return home_page(env, vec![top_bar, search_results]);
        }

        home_page(env, vec![top_bar, bento_grid])
    }

    fn open_search_target(&mut self, module: Module, raw_query: &str) {
        let q = normalize_search(raw_query);
        match module {
            Module::Calculator => {
                self.productivity.tab = productivity::Tab::Calc;
                self.active = Module::Calculator;
            }
            Module::Productivity => {
                if q.contains("tarefa")
                    || q.contains("task")
                    || q.contains("checklist")
                    || q.contains("todo")
                {
                    self.productivity.tab = productivity::Tab::Tasks;
                } else if q.contains("organiz") || q.contains("pasta") || q.contains("folder") {
                    self.productivity.tab = productivity::Tab::Organizer;
                } else {
                    self.productivity.tab = productivity::Tab::Notes;
                }
                self.active = Module::Productivity;
            }
            Module::DevTools => {
                if q.contains("senha")
                    || q.contains("password")
                    || q.contains("uuid")
                    || q.contains("entrop")
                {
                    self.devtools.tab = devtools::Tab::Passwords;
                } else if q.contains("cor")
                    || q.contains("color")
                    || q.contains("paleta")
                    || q.contains("palette")
                    || q.contains("hex")
                    || q.contains("rgb")
                    || q.contains("hsl")
                    || q.contains("cmyk")
                {
                    self.devtools.tab = devtools::Tab::Colors;
                }
                self.active = Module::DevTools;
            }
            Module::Pdf => {
                if q.contains("divid")
                    || q.contains("split")
                    || q.contains("pagina")
                    || q.contains("extrair")
                {
                    self.pdf.mode = pdf::Mode::Split;
                } else if q.contains("unir")
                    || q.contains("juntar")
                    || q.contains("mesclar")
                    || q.contains("merge")
                {
                    self.pdf.mode = pdf::Mode::Merge;
                }
                self.active = Module::Pdf;
            }
            Module::Images => {
                if q.contains("lote") || q.contains("batch") {
                    self.images.mode = images::Mode::Batch;
                } else if q.contains("redimensionar")
                    || q.contains("resize")
                    || q.contains("comprimir")
                    || q.contains("ico")
                {
                    self.images.mode = images::Mode::ConvertResize;
                } else {
                    self.images.mode = images::Mode::Editor;
                }
                self.active = Module::Images;
            }
            Module::Video => {
                if q.contains("comprimir")
                    || q.contains("compress")
                    || q.contains("converter")
                    || q.contains("convert")
                    || q.contains("crf")
                    || q.contains("formato")
                    || q.contains("format")
                    || q.contains("mp4")
                    || q.contains("mkv")
                    || q.contains("webm")
                    || q.contains("avi")
                    || q.contains("mov")
                {
                    self.video.mode = video::Mode::ConvertCompress;
                } else {
                    self.video.mode = video::Mode::VideoEditor;
                }
                self.active = Module::Video;
            }
            Module::Zip => {
                if q.contains("criar")
                    || q.contains("compact")
                    || q.contains("create")
                    || q.contains("compress")
                    || q.contains("comprimir")
                {
                    self.zip.mode = zip::Mode::Create;
                } else if q.contains("extrair") || q.contains("descompact") || q.contains("extract")
                {
                    self.zip.mode = zip::Mode::Extract;
                }
                self.active = Module::Zip;
            }
            other => {
                self.active = other;
            }
        }
        self.home_search.clear();
        self.nav_pulse = 1.0;
        self.last_tick = Some(Instant::now());
    }

    fn search_matches(&self, query: &str) -> Vec<(Module, &'static str, &'static str)> {
        let q = normalize_search(query);
        let terms: Vec<&str> = q.split_whitespace().collect();
        if terms.is_empty() {
            return Vec::new();
        }
        let all_tools: &[(Module, &[&str], &str, &str)] = &[
            (
                Module::QuickConvert,
                &["converter", "conversor", "webp", "png", "jpg", "jpeg", "bmp", "gif", "tiff", "ico", "mp4", "mkv", "webm", "avi", "mov", "mp3", "wav", "flac", "ogg", "aac", "m4a", "pdf", "docx", "html", "markdown", "md", "txt", "zip", "extrair", "quick convert", "formato", "formatos", "drag", "drop", "arrastar"],
                "Conversão rápida em 1 clique para imagens, documentos (TXT/MD/HTML/DOCX/PDF), vídeo, áudio e ZIP.",
                "Quick 1-click conversion for images, documents (TXT/MD/HTML/DOCX/PDF), video, audio, and ZIP.",
            ),
            (
                Module::Images,
                &["imagem", "imagens", "foto", "fotos", "editor", "exposicao", "brilho", "contraste", "realces", "sombras", "crop", "cortar", "recorte", "filtro", "temperatura", "vinheta", "granulado", "nitidez", "revelador", "presets", "lote", "redimensionar", "comprimir", "ico", "histograma"],
                "Revelador de fotos ao vivo com histograma RGB, ajustes de luz/cor, presets, lote, recorte e redimensionamento.",
                "Live photo developer with RGB histogram, light/color controls, presets, batch sync, crop, and resize.",
            ),
            (
                Module::Video,
                &["video", "vídeo", "cortar", "trim", "comprimir", "compressao", "crf", "h264", "mp4", "mkv", "avi", "mov", "webm", "velocidade", "speed", "tiktok", "reels", "shorts", "youtube", "rotacionar", "espelhar", "filtros", "ffmpeg"],
                "Corte na linha do tempo, controle de velocidade, formatos 16:9/9:16/1:1, filtros e compressão CRF.",
                "Timeline video trimming, speed control, 16:9/9:16/1:1 aspect ratios, filters, and CRF compression.",
            ),
            (
                Module::Audio,
                &["audio", "áudio", "som", "musica", "música", "mp3", "flac", "wav", "ogg", "m4a", "aac", "extrair", "extracao", "bitrate", "comprimir", "converter", "estudio"],
                "Conversão de áudio, extração de trilha sonora de vídeos e compressão por bitrate.",
                "Audio format conversion, soundtrack extraction from video, and bitrate compression.",
            ),
            (
                Module::Pdf,
                &["pdf", "documento", "documentos", "docs", "juntar", "unir", "mesclar", "merge", "dividir", "split", "extrair", "paginas", "páginas", "metadados", "lopdf"],
                "Unir múltiplos PDFs na ordem desejada, dividir páginas ou intervalos e inspecionar metadados.",
                "Merge multiple PDFs in custom order, split pages or intervals, and inspect document metadata.",
            ),
            (
                Module::BatchRename,
                &["renomear", "rename", "lote", "batch", "arquivos", "prefixo", "sufixo", "numeracao", "substituir", "maiusculas", "minusculas"],
                "Renomeie coleções de arquivos com prefixo, sufixo, numeração, substituição e pré-visualização.",
                "Bulk file renamer with prefixes, suffixes, numbering, text replace, and live preview.",
            ),
            (
                Module::Zip,
                &["zip", "compactar", "comprimir", "descompactar", "extrair", "extract", "compressao", "arquivo", "backup", "deflate", "zip-slip", "pastas"],
                "Compactação de arquivos/pastas em ZIP e extração protegida contra Zip-Slip.",
                "Create ZIP archives from files/folders and extract safely with Zip-Slip protection.",
            ),
            (
                Module::Productivity,
                &["produtividade", "notas", "bloco", "anotacoes", "anotações", "tarefas", "checklist", "todo", "to-do", "notepad", "texto", "organizador", "pastas"],
                "Bloco de notas (.txt), lista de tarefas persistente e organizador automático de pastas.",
                "Plain text notepad (.txt), persistent task checklist, and automatic folder organizer.",
            ),
            (
                Module::Calculator,
                &["calculadora", "calc", "calcular", "conta", "matematica", "matemática", "expressao", "parenteses", "potencia", "porcentagem", "resto", "historico"],
                "Calculadora de expressões com parênteses, potência (^), resto (%) e histórico clicável.",
                "Expression calculator with parentheses, power (^), modulo (%), and clickable history.",
            ),
            (
                Module::DevTools,
                &["cores", "paleta", "pipeta", "hex", "rgb", "hsl", "cmyk", "harmonia", "uuid", "senha", "senhas", "password", "entropia", "gerador", "devtools", "seguranca"],
                "Conversor HEX/RGB/HSL/CMYK, extrator de paleta de imagens, gerador de senhas fortes e UUID v4.",
                "HEX/RGB/HSL/CMYK color converter, image palette extractor, strong password generator, and UUID v4.",
            ),
            (
                Module::Queue,
                &["fila", "queue", "tarefas", "processos", "operacoes", "segundo plano", "background", "jobs", "concluidos", "ativos"],
                "Monitoramento em tempo real da fila de operações em segundo plano da sessão atual.",
                "Real-time monitor for background operations in the current session.",
            ),
            (
                Module::History,
                &["historico", "histórico", "history", "auditoria", "atividades", "registro", "operacoes", "falhas", "erros"],
                "Registro local completo de operações realizadas com filtro de busca e status.",
                "Complete local log of completed operations with search and error filtering.",
            ),
            (
                Module::Settings,
                &["configuracoes", "configurações", "settings", "tema", "escuro", "claro", "dark", "light", "idioma", "linguagem", "privacidade", "abrir pasta", "preferencias"],
                "Tema escuro/claro, idioma automático ou manual, abertura de pasta e privacidade local.",
                "Dark/light theme, automatic or manual language, output folder opening, and local privacy.",
            ),
            (
                Module::Installer,
                &["instalador", "instalar", "setup", "desinstalar", "uninstall", "apagar tudo", "limpar", "atalho", "dependencias", "ffmpeg"],
                "Instalar no PC, baixar motor FFmpeg automático, criar atalhos ou desinstalar por completo.",
                "Install on PC, download FFmpeg automatically, create shortcuts, or uninstall completely.",
            ),
        ];

        let mut scored: Vec<(i32, usize, (Module, &'static str, &'static str))> = Vec::new();
        for (idx, &(m, tags, pt, en)) in all_tools.iter().enumerate() {
            let title = normalize_search(m.title());
            let title_en = normalize_search(m.title_in(crate::shared::i18n::Lang::En, false));
            let pt_norm = normalize_search(pt);
            let en_norm = normalize_search(en);
            let norm_tags: Vec<String> = tags.iter().map(|t| normalize_search(t)).collect();

            let all_match = terms.iter().all(|w| {
                title.contains(w)
                    || title_en.contains(w)
                    || norm_tags.iter().any(|nt| nt.contains(w))
                    || pt_norm.contains(w)
                    || en_norm.contains(w)
            });

            if !all_match {
                continue;
            }

            let mut score = 0i32;
            if title == q || title_en == q {
                score += 1000;
            } else if title.starts_with(&q) || title_en.starts_with(&q) {
                score += 500;
            } else if title.contains(&q) || title_en.contains(&q) {
                score += 250;
            } else if terms
                .iter()
                .all(|w| title.contains(w) || title_en.contains(w))
            {
                score += 200;
            }

            if norm_tags.iter().any(|nt| nt == &q) {
                score += 140;
            }
            for w in &terms {
                if norm_tags.iter().any(|nt| nt == w) {
                    score += 60;
                } else if norm_tags.iter().any(|nt| nt.contains(w)) {
                    score += 30;
                }
                if pt_norm.contains(w) || en_norm.contains(w) {
                    score += 10;
                }
            }

            scored.push((score, idx, (m, pt, en)));
        }

        scored.sort_by(|a, b| b.0.cmp(&a.0).then_with(|| a.1.cmp(&b.1)));
        scored.into_iter().map(|(_, _, item)| item).collect()
    }

    fn search_results_view<'a>(&'a self, env: &Env, query: &str) -> Element<'a, Message> {
        let c = env.c();
        let is_en = env.lang.is_en();
        let matches = self.search_matches(query);

        let title_col = column![
            row![
                ui::strong(
                    if is_en {
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
                        if is_en { "found" } else { "ferramenta(s)" }
                    ))
                    .size(11)
                    .font(BOLD)
                    .style(c.primary)
                )
                .padding([2, 8])
                .style(theme::solid_box(c.primary_soft, 6.0)),
            ]
            .spacing(10)
            .align_items(Alignment::Center),
            ui::muted(
                env,
                if is_en {
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
        .spacing(4);

        let clear_btn = ui::btn(
            if is_en {
                "✕ Clear Search"
            } else {
                "✕ Limpar Busca"
            },
            Btn::Secondary,
            Some(Message::SearchChanged(String::new())),
        );

        let header: Element<'a, Message> = if env.narrow() {
            column![title_col, clear_btn].spacing(8).into()
        } else {
            row![title_col.width(Length::Fill), clear_btn]
                .spacing(10)
                .align_items(Alignment::Center)
                .into()
        };

        if matches.is_empty() {
            let suggestion_items: Vec<Element<'a, Message>> = vec![
                home_action_btn(
                    if is_en {
                        "🔄 Quick Convert"
                    } else {
                        "🔄 Conversor"
                    },
                    Btn::Secondary,
                    Some(Message::Navigate(Module::QuickConvert)),
                ),
                home_action_btn(
                    if is_en {
                        "🖼 Photos & Images"
                    } else {
                        "🖼 Imagens & Foto"
                    },
                    Btn::Secondary,
                    Some(Message::Navigate(Module::Images)),
                ),
                home_action_btn(
                    if is_en { "🎬 Video" } else { "🎬 Vídeo" },
                    Btn::Secondary,
                    Some(Message::Navigate(Module::Video)),
                ),
                home_action_btn(
                    "📄 PDF",
                    Btn::Secondary,
                    Some(Message::Navigate(Module::Pdf)),
                ),
                home_action_btn(
                    if is_en {
                        "🔢 Calculator"
                    } else {
                        "🔢 Calculadora"
                    },
                    Btn::Secondary,
                    Some(Message::Navigate(Module::Calculator)),
                ),
                home_action_btn(
                    if is_en {
                        "🎨 Colors & Passwords"
                    } else {
                        "🎨 Cores & Senhas"
                    },
                    Btn::Secondary,
                    Some(Message::Navigate(Module::DevTools)),
                ),
            ];
            let suggestions = ui::grid(suggestion_items, env.columns(160.0, 3), 8);

            let empty_box = container(
                column![
                    text("🔍").size(36).shaping(Shaping::Advanced),
                    Space::with_height(6),
                    ui::strong(
                        if is_en {
                            "No tools found"
                        } else {
                            "Nenhuma ferramenta encontrada"
                        },
                        16
                    ),
                    ui::muted(
                        env,
                        if is_en {
                            format!("No matching tools for \"{}\". Try one of the modules below:", query)
                        } else {
                            format!("Nenhuma ferramenta encontrada para \"{}\". Acesse um módulo abaixo:", query)
                        }
                    )
                    .size(13),
                    Space::with_height(10),
                    suggestions,
                ]
                .spacing(6)
                .align_items(Alignment::Center)
                .width(Length::Fill),
            )
            .padding([32, 24])
            .style(theme::bento_card);

            column![header, Space::with_height(14), empty_box]
                .spacing(8)
                .into()
        } else {
            let mut list = column![].spacing(10).width(Length::Fill);

            for (module, desc_pt, desc_en) in matches {
                let desc = if is_en { desc_en } else { desc_pt };
                let title = module.title_in(env.lang, self.installer.is_installed());
                let icon = module.icon();

                let item_card = button(
                    container(
                        row![
                            container(text(icon).size(24).shaping(Shaping::Advanced))
                                .padding([10, 14])
                                .style(theme::solid_box(c.surface2, 8.0)),
                            column![
                                row![
                                    ui::strong(title, 16),
                                    ui::fill(),
                                    container(
                                        text(if is_en {
                                            "Open Tool →"
                                        } else {
                                            "Abrir Ferramenta →"
                                        })
                                        .size(12)
                                        .font(BOLD)
                                        .style(c.primary)
                                        .shaping(Shaping::Advanced)
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
                .on_press(Message::OpenSearchResult(module));

                list = list.push(item_card);
            }

            column![header, Space::with_height(14), list]
                .spacing(8)
                .into()
        }
    }
}

fn home_page<'a>(env: &Env, children: Vec<Element<'a, Message>>) -> Element<'a, Message> {
    let mut col = column![].spacing(14).width(Length::Fill).max_width(1240);
    for child in children {
        col = col.push(child);
    }
    let pad: [u16; 2] = if env.narrow() {
        [14, 14]
    } else if env.is_desktop_wide() {
        [20, 26]
    } else {
        [18, 22]
    };
    scrollable(container(col).padding(pad).width(Length::Fill).center_x())
        .height(Length::Fill)
        .width(Length::Fill)
        .into()
}

fn card_spacer<'a>(fixed_h: Option<f32>) -> Element<'a, Message> {
    if fixed_h.is_some() {
        Space::with_height(Length::Fill).into()
    } else {
        Space::with_height(Length::Fixed(2.0)).into()
    }
}

fn bento_panel<'a>(
    env: &Env,
    fixed_h: Option<f32>,
    body: Element<'a, Message>,
) -> Element<'a, Message> {
    let mut box_ = container(body)
        .padding(if env.narrow() { 14 } else { 16 })
        .width(Length::Fill)
        .style(theme::bento_card);
    if let Some(h) = fixed_h {
        box_ = box_.height(Length::Fixed(h));
    }
    box_.into()
}

fn home_action_btn<'a>(
    label: impl ToString,
    kind: Btn,
    msg: Option<Message>,
) -> Element<'a, Message> {
    button(
        container(
            text(label)
                .size(12)
                .font(SEMIBOLD)
                .shaping(Shaping::Advanced),
        )
        .center_x()
        .width(Length::Fill),
    )
    .padding([8, 10])
    .width(Length::Fill)
    .style(theme::btn(kind))
    .on_press_maybe(msg)
    .into()
}

fn home_link_btn<'a>(label: impl ToString, msg: Option<Message>) -> Element<'a, Message> {
    button(
        text(label)
            .size(11)
            .font(SEMIBOLD)
            .shaping(Shaping::Advanced),
    )
    .padding([4, 8])
    .style(theme::btn(Btn::Ghost))
    .on_press_maybe(msg)
    .into()
}

fn quick_nav_tile<'a>(icon: &'a str, title: &'a str, msg: Message) -> Element<'a, Message> {
    button(
        container(
            row![
                text(icon).size(12).shaping(Shaping::Advanced),
                text(title)
                    .size(11)
                    .font(SEMIBOLD)
                    .shaping(Shaping::Advanced),
            ]
            .spacing(6)
            .align_items(Alignment::Center),
        )
        .center_x()
        .padding([7, 8])
        .width(Length::Fill)
        .style(theme::inner_box),
    )
    .width(Length::FillPortion(1))
    .style(theme::btn(Btn::Ghost))
    .on_press(msg)
    .into()
}

fn short_timestamp(ts: &str) -> String {
    let parts: Vec<&str> = ts.split_whitespace().collect();
    if parts.len() == 2 {
        let date = parts[0];
        let time = parts[1];
        let short_date = if date.len() >= 5 { &date[..5] } else { date };
        let short_time = if time.len() >= 5 { &time[..5] } else { time };
        format!("{short_date} {short_time}")
    } else {
        truncate_label(ts, 12)
    }
}

fn truncate_label(s: &str, max_chars: usize) -> String {
    let mut chars = s.chars();
    let collected: String = chars.by_ref().take(max_chars).collect();
    if chars.next().is_some() {
        format!("{collected}…")
    } else {
        collected
    }
}

fn format_duration_short(d: std::time::Duration) -> String {
    if d.as_secs() > 0 {
        format!("{:.1}s", d.as_secs_f32())
    } else {
        format!("{} ms", d.as_millis())
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shared::i18n::Lang;

    fn test_app() -> AxoloteSuite {
        let ctx = Ctx {
            settings: crate::core::config::Settings::default(),
            history: crate::core::history::History::empty_for_test(50),
            jobs: crate::core::jobs::Jobs::default(),
        };
        AxoloteSuite {
            ctx,
            active: Module::Home,
            mandatory_setup: false,
            home_search: String::new(),
            window_width: 1380.0,
            window_height: 880.0,
            sidebar_collapsed: None,
            sidebar_open_t: 1.0,
            nav_pulse: 0.0,
            last_tick: None,
            quick_convert: quick_convert::State::new(),
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
            installer: installer::State::new(installer::WizardStep::Welcome),
        }
    }

    #[test]
    fn default_window_activates_desktop_wide_layout() {
        let app = test_app();
        let sidebar_w = 258.0;
        let env = Env {
            width: app.window_width - sidebar_w,
            dark: true,
            lang: Lang::Pt,
        };
        assert!(env.is_desktop_wide());
        assert!(!env.is_medium());
        assert!(!env.narrow());

        let medium_env = Env {
            width: 820.0,
            dark: true,
            lang: Lang::Pt,
        };
        assert!(medium_env.is_medium());
        assert!(!medium_env.is_desktop_wide());

        let narrow_env = Env {
            width: 500.0,
            dark: true,
            lang: Lang::Pt,
        };
        assert!(narrow_env.narrow());
    }

    #[test]
    fn search_matches_real_features_with_accent_normalization() {
        let app = test_app();

        let video_hits = app.search_matches("vídeo");
        assert_eq!(video_hits.first().map(|x| x.0), Some(Module::Video));

        let pdf_exact = app.search_matches("pdf");
        assert_eq!(pdf_exact.first().map(|x| x.0), Some(Module::Pdf));

        let hist_hits = app.search_matches("Histórico");
        assert_eq!(hist_hits.first().map(|x| x.0), Some(Module::History));

        let pwd_hits = app.search_matches("senhas");
        assert_eq!(pwd_hits.first().map(|x| x.0), Some(Module::DevTools));

        let pdf_split_hits = app.search_matches("dividir pdf");
        assert_eq!(pdf_split_hits.first().map(|x| x.0), Some(Module::Pdf));
        assert_eq!(pdf_split_hits.len(), 1);

        assert!(app.search_matches("   ").is_empty());

        // Ensure unsupported/removed features are not advertised in search results
        assert!(app.search_matches("avif").is_empty());
        assert!(app.search_matches("nvenc").is_empty());
        assert!(app.search_matches("h265").is_empty());
        assert!(app.search_matches("sha256").is_empty());
        assert!(app.search_matches("zstandard").is_empty());
    }

    #[test]
    fn quick_launch_and_search_open_correct_module_and_tab() {
        let mut app = test_app();

        let _ = app.update(Message::QuickLaunchDevTools(devtools::Tab::Passwords));
        assert_eq!(app.active, Module::DevTools);
        assert_eq!(app.devtools.tab, devtools::Tab::Passwords);

        let _ = app.update(Message::QuickLaunchPdf(pdf::Mode::Split));
        assert_eq!(app.active, Module::Pdf);
        assert_eq!(app.pdf.mode, pdf::Mode::Split);

        let _ = app.update(Message::QuickLaunchImages(images::Mode::Batch));
        assert_eq!(app.active, Module::Images);
        assert_eq!(app.images.mode, images::Mode::Batch);

        let _ = app.update(Message::QuickLaunchVideo(video::Mode::ConvertCompress));
        assert_eq!(app.active, Module::Video);
        assert_eq!(app.video.mode, video::Mode::ConvertCompress);

        let _ = app.update(Message::QuickLaunchQuickConvert(
            quick_convert::WheelCategory::Image,
        ));
        assert_eq!(app.active, Module::QuickConvert);
        assert_eq!(
            app.quick_convert.category,
            quick_convert::WheelCategory::Image
        );

        let _ = app.update(Message::SearchChanged("tarefas".into()));
        let _ = app.update(Message::SearchSubmitted);
        assert_eq!(app.active, Module::Productivity);
        assert_eq!(app.productivity.tab, productivity::Tab::Tasks);
        assert!(app.home_search.is_empty());

        let _ = app.update(Message::SearchChanged("comprimir video".into()));
        let _ = app.update(Message::SearchSubmitted);
        assert_eq!(app.active, Module::Video);
        assert_eq!(app.video.mode, video::Mode::ConvertCompress);

        let inst_hits = app.search_matches("desinstalar");
        assert_eq!(inst_hits.first().map(|x| x.0), Some(Module::Installer));
    }

    #[test]
    fn truncate_label_preserves_utf8_boundaries() {
        assert_eq!(truncate_label("Olá", 10), "Olá");
        assert_eq!(truncate_label("Conversão de vídeo", 9), "Conversão…");
        assert_eq!(short_timestamp("11/11/2026 22:05:33"), "11/11 22:05");
    }

    #[test]
    fn home_view_renders_across_sizes_themes_and_languages() {
        let mut app = test_app();
        let job_id = app.ctx.jobs.start("PDF", "Dividir contrato.pdf");
        let done_res = crate::shared::types::OperationResult::ok("3 páginas extraídas");
        let _ = app.ctx.jobs.finish(job_id, &done_res);
        let fail_id = app.ctx.jobs.start("ZIP", "Extrair corrompido.zip");
        let fail_res = crate::shared::types::OperationResult::err("Arquivo ZIP corrompido");
        let _ = app.ctx.jobs.finish(fail_id, &fail_res);
        app.ctx
            .history
            .add_in_memory("PDF", "Dividir contrato.pdf", "3 páginas extraídas", true);
        let _running_id = app.ctx.jobs.start("Vídeo", "Comprimir clipe.mp4");

        for (w, h) in [
            (560, 480),
            (880, 720),
            (1300, 760),
            (1380, 880),
            (1920, 1080),
        ] {
            app.window_width = w as f32;
            app.window_height = h as f32;
            for dark in [true, false] {
                app.ctx.settings.dark_mode = dark;
                for lang_mode in [
                    crate::shared::i18n::LanguageMode::PtBr,
                    crate::shared::i18n::LanguageMode::EnUs,
                ] {
                    app.ctx.settings.language = lang_mode;
                    app.home_search.clear();
                    let _ = app.view();

                    app.home_search = "pdf".into();
                    let _ = app.view();

                    app.home_search = "termo_inexistente_xyz".into();
                    let _ = app.view();
                }
            }
        }
    }
}
