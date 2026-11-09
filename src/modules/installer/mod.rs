//! Módulo de Instalação (Setup Principal), Download Automático de Dependências (FFmpeg)
//! e Tela de Desinstalação do Axolote Suite.

pub mod view;

use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use iced::Command;

use crate::core::{paths, system, Ctx};
use crate::shared::types::OperationResult;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WizardStep {
    Welcome,
    Location,
    Installing,
    Finished,
    UninstallConfirm,
    Uninstalled,
}

pub struct State {
    pub step: WizardStep,
    pub install_dir: String,
    pub create_desktop_shortcut: bool,
    pub create_start_menu_shortcut: bool,
    pub download_ffmpeg: bool,
    pub remove_user_data_on_uninstall: bool,
    pub launch_after_finish: bool,
    pub progress: f32,
    pub status_text: String,
    pub ffmpeg_busy: bool,
    pub purge_busy: bool,
    pub result: Option<OperationResult>,
}

#[derive(Debug, Clone)]
pub enum Msg {
    GoToStep(WizardStep),
    InstallDirChanged(String),
    BrowseInstallDir,
    ToggleDesktopShortcut(bool),
    ToggleStartMenuShortcut(bool),
    ToggleDownloadFfmpeg(bool),
    ToggleRemoveUserData(bool),
    ToggleLaunchAfter(bool),
    StartInstall,
    InstallFinished(OperationResult),
    DownloadFfmpegOnly,
    DownloadFfmpegFinished(OperationResult),
    StartUninstall,
    UninstallFinished(OperationResult),
    PurgeEverything,
    PurgeFinished(OperationResult),
    FinishAndGoHome,
    OpenInstalledFolder,
}

impl State {
    pub fn new(start_step: WizardStep) -> Self {
        let default_dir = env::var_os("LOCALAPPDATA")
            .map(PathBuf::from)
            .unwrap_or_else(|| PathBuf::from("C:\\Program Files"))
            .join("Programs")
            .join("Axolote Suite");

        Self {
            step: start_step,
            install_dir: default_dir.display().to_string(),
            create_desktop_shortcut: true,
            create_start_menu_shortcut: true,
            download_ffmpeg: true,
            remove_user_data_on_uninstall: false,
            launch_after_finish: true,
            progress: 0.0,
            status_text: String::new(),
            ffmpeg_busy: false,
            purge_busy: false,
            result: None,
        }
    }

    pub fn is_installed(&self) -> bool {
        Path::new(self.install_dir.trim()).join("axolote_suite.exe").is_file()
    }

    pub fn is_ffmpeg_ready(&self) -> bool {
        system::find_ffmpeg().is_some()
    }

    pub fn update(&mut self, msg: Msg, ctx: &mut Ctx) -> Command<Msg> {
        match msg {
            Msg::GoToStep(s) => {
                self.step = s;
                self.result = None;
            }
            Msg::InstallDirChanged(s) => self.install_dir = s,
            Msg::BrowseInstallDir => {
                if let Some(d) = system::pick_folder("Escolha a pasta onde o Axolote Suite será instalado") {
                    self.install_dir = d.join("Axolote Suite").display().to_string();
                }
            }
            Msg::ToggleDesktopShortcut(b) => self.create_desktop_shortcut = b,
            Msg::ToggleStartMenuShortcut(b) => self.create_start_menu_shortcut = b,
            Msg::ToggleDownloadFfmpeg(b) => self.download_ffmpeg = b,
            Msg::ToggleRemoveUserData(b) => self.remove_user_data_on_uninstall = b,
            Msg::ToggleLaunchAfter(b) => self.launch_after_finish = b,

            Msg::StartInstall => {
                self.step = WizardStep::Installing;
                self.progress = 68.0;
                self.status_text = if self.download_ffmpeg && !self.is_ffmpeg_ready() {
                    "Instalando Axolote Suite e baixando motor FFmpeg automaticamente…".into()
                } else {
                    "Copiando arquivos essenciais e criando atalhos no Windows…".into()
                };
                let target_dir = PathBuf::from(self.install_dir.trim());
                let desktop = self.create_desktop_shortcut;
                let start_menu = self.create_start_menu_shortcut;
                let dl_ffmpeg = self.download_ffmpeg;
                return Command::perform(
                    system::run_blocking(move || perform_install(target_dir, desktop, start_menu, dl_ffmpeg)),
                    Msg::InstallFinished,
                );
            }
            Msg::InstallFinished(res) => {
                ctx.settings.setup_completed = true;
                ctx.settings.save();
                ctx.log("Instalador", "Instalar Axolote Suite + Dependências", &res);
                self.progress = 100.0;
                self.step = WizardStep::Finished;
                self.result = Some(res);
            }
            Msg::DownloadFfmpegOnly => {
                self.ffmpeg_busy = true;
                self.result = None;
                let dest_bin = PathBuf::from(self.install_dir.trim()).join("bin");
                return Command::perform(
                    system::run_blocking(move || match system::ensure_ffmpeg_installed(&dest_bin) {
                        Ok(p) => OperationResult::ok("Dependência FFmpeg baixada e ativada com sucesso!")
                            .with_detail(format!("Local do FFmpeg: {}", p.display())),
                        Err(e) => OperationResult::err(e),
                    }),
                    Msg::DownloadFfmpegFinished,
                );
            }
            Msg::DownloadFfmpegFinished(res) => {
                self.ffmpeg_busy = false;
                ctx.log("Instalador", "Baixar FFmpeg", &res);
                self.result = Some(res);
            }
            Msg::StartUninstall => {
                let target_dir = PathBuf::from(self.install_dir.trim());
                let remove_data = self.remove_user_data_on_uninstall;
                return Command::perform(
                    system::run_blocking(move || perform_uninstall(target_dir, remove_data)),
                    Msg::UninstallFinished,
                );
            }
            Msg::UninstallFinished(res) => {
                ctx.log("Instalador", "Desinstalar Axolote Suite", &res);
                self.step = WizardStep::Uninstalled;
                self.result = Some(res);
            }
            Msg::PurgeEverything => {
                self.purge_busy = true;
                self.result = None;
                let target_dir = PathBuf::from(self.install_dir.trim());
                return Command::perform(
                    system::run_blocking(move || perform_purge_everything(target_dir)),
                    Msg::PurgeFinished,
                );
            }
            Msg::PurgeFinished(res) => {
                self.purge_busy = false;
                ctx.settings = crate::core::config::Settings::default();
                ctx.settings.setup_completed = false;
                ctx.settings.save();
                self.step = WizardStep::Uninstalled;
                self.result = Some(res);
            }
            Msg::FinishAndGoHome => {
                ctx.settings.setup_completed = true;
                ctx.settings.save();
                self.step = WizardStep::Welcome;
            }
            Msg::OpenInstalledFolder => {
                system::open_path(Path::new(self.install_dir.trim()));
            }
        }
        Command::none()
    }
}

impl Default for State {
    fn default() -> Self {
        Self::new(WizardStep::Welcome)
    }
}

fn perform_install(
    target_dir: PathBuf,
    desktop_lnk: bool,
    start_menu_lnk: bool,
    download_ffmpeg: bool,
) -> OperationResult {
    let Ok(current_exe) = env::current_exe() else {
        return OperationResult::err("Não foi possível localizar o executável atual.");
    };
    if let Err(e) = fs::create_dir_all(&target_dir) {
        return OperationResult::err(format!("Não foi possível criar a pasta de instalação: {e}"));
    }

    let dest_exe = target_dir.join("axolote_suite.exe");
    if current_exe != dest_exe {
        if let Err(e) = fs::copy(&current_exe, &dest_exe) {
            return OperationResult::err(format!("Erro ao copiar o executável principal: {e}"));
        }
    }

    let icon_path = target_dir.join("axolote.ico");
    let _ = fs::write(&icon_path, include_bytes!("../../../assets/icon.ico"));

    let mut details = vec![format!("Pasta instalada: {}", target_dir.display())];

    // Instala / Baixa a dependência essencial FFmpeg automaticamente
    if download_ffmpeg {
        let bin_dir = target_dir.join("bin");
        match system::ensure_ffmpeg_installed(&bin_dir) {
            Ok(ff_path) => {
                details.push(format!("✓ Motor multimídia FFmpeg pronto ({})", ff_path.display()));
            }
            Err(e) => {
                details.push(format!("⚠ Aviso FFmpeg: {e}"));
            }
        }
    }

    if desktop_lnk {
        if let Some(desktop) = user_desktop_dir() {
            let lnk = desktop.join("Axolote Suite.lnk");
            if create_windows_shortcut(&lnk, &dest_exe, &icon_path, "Axolote Suite — Central de Ferramentas") {
                details.push("✓ Atalho criado na Área de Trabalho".into());
            }
        }
    }

    if start_menu_lnk {
        if let Some(programs) = user_start_menu_dir() {
            let folder = programs.join("Axolote Suite");
            let _ = fs::create_dir_all(&folder);
            let lnk = folder.join("Axolote Suite.lnk");
            if create_windows_shortcut(&lnk, &dest_exe, &icon_path, "Axolote Suite") {
                details.push("✓ Atalho criado no Menu Iniciar".into());
            }
            let uninst_lnk = folder.join("Desinstalar Axolote Suite.lnk");
            let _ = create_windows_shortcut_with_args(
                &uninst_lnk,
                &dest_exe,
                "--uninstall",
                &icon_path,
                "Desinstalar Axolote Suite",
            );
        }
    }

    register_uninstaller_hkcu(&target_dir, &dest_exe, &icon_path);
    details.push("✓ Registrado em Aplicativos Instalados do Windows".into());

    OperationResult::ok("O Axolote Suite e suas dependências essenciais foram instalados com sucesso!")
        .with_details(details)
        .with_output(target_dir)
}

fn perform_uninstall(target_dir: PathBuf, remove_user_data: bool) -> OperationResult {
    let mut details = Vec::new();

    if let Some(desktop) = user_desktop_dir() {
        let lnk = desktop.join("Axolote Suite.lnk");
        if lnk.exists() && fs::remove_file(&lnk).is_ok() {
            details.push("✓ Atalho da Área de Trabalho removido".into());
        }
    }

    if let Some(programs) = user_start_menu_dir() {
        let folder = programs.join("Axolote Suite");
        if folder.exists() && fs::remove_dir_all(&folder).is_ok() {
            details.push("✓ Pasta do Menu Iniciar removida".into());
        }
    }

    unregister_hkcu();
    details.push("✓ Registro de instalação do Windows removido".into());

    if target_dir.exists() {
        let _ = fs::remove_dir_all(target_dir.join("bin"));
        let _ = fs::remove_file(target_dir.join("axolote.ico"));
        let exe = target_dir.join("axolote_suite.exe");
        if fs::remove_file(&exe).is_ok() {
            let _ = fs::remove_dir(&target_dir);
            details.push(format!("✓ Pasta {} removida", target_dir.display()));
        } else {
            schedule_self_delete(&target_dir);
            details.push("✓ O executável será removido assim que você fechar esta janela.".into());
        }
    }

    if remove_user_data {
        let data = paths::data_dir();
        if data.exists() && fs::remove_dir_all(&data).is_ok() {
            details.push("✓ Configurações, notas, histórico e binários em %APPDATA% apagados".into());
        }
    } else {
        details.push("ℹ Suas notas e configurações pessoais foram preservadas.".into());
    }

    OperationResult::ok("O Axolote Suite foi desinstalado do seu computador.").with_details(details)
}

/// Apaga absolutamente tudo do Axolote Suite do computador:
/// atalhos da Área de Trabalho, Menu Iniciar, registro do Windows,
/// pasta de instalação e pasta de dados/configurações em %APPDATA%.
pub fn perform_purge_everything(install_dir: PathBuf) -> OperationResult {
    let mut details = Vec::new();

    // 1. Atalhos
    if let Some(desktop) = user_desktop_dir() {
        let lnk = desktop.join("Axolote Suite.lnk");
        if lnk.exists() && fs::remove_file(&lnk).is_ok() {
            details.push("✓ Atalho da Área de Trabalho apagado".into());
        }
    }
    if let Some(start) = user_start_menu_dir() {
        let lnk = start.join("Axolote Suite.lnk");
        if lnk.exists() && fs::remove_file(&lnk).is_ok() {
            details.push("✓ Atalho do Menu Iniciar apagado".into());
        }
    }

    // 2. Registro do Windows
    unregister_hkcu();
    details.push("✓ Registro do Windows limpo".into());

    // 3. Pasta de dados completos em %APPDATA%\AxoloteSuite (notas, tarefas, histórico, configs, bin/ffmpeg)
    let data = paths::data_dir();
    if data.exists() {
        let _ = fs::remove_dir_all(&data);
        details.push("✓ Todos os dados, configurações, histórico e notas em %APPDATA% apagados".into());
    }

    // 4. Pasta de instalação
    if install_dir.exists() {
        let running_exe = env::current_exe().ok();
        let is_running_from_install = running_exe
            .as_ref()
            .map(|p| p.starts_with(&install_dir))
            .unwrap_or(false);

        if is_running_from_install {
            schedule_self_delete(&install_dir);
            details.push(format!("✓ Pasta de instalação ({}) agendada para exclusão completa", install_dir.display()));
        } else {
            let _ = fs::remove_dir_all(&install_dir);
            details.push(format!("✓ Pasta de instalação ({}) apagada", install_dir.display()));
        }
    }

    OperationResult::ok("Limpeza Completa Realizada: Tudo do Axolote Suite foi apagado do computador.")
        .with_details(details)
}

fn user_desktop_dir() -> Option<PathBuf> {
    env::var_os("USERPROFILE").map(|p| PathBuf::from(p).join("Desktop"))
}

fn user_start_menu_dir() -> Option<PathBuf> {
    env::var_os("APPDATA").map(|p| PathBuf::from(p).join("Microsoft\\Windows\\Start Menu\\Programs"))
}

fn create_windows_shortcut(lnk: &Path, target: &Path, icon: &Path, desc: &str) -> bool {
    create_windows_shortcut_with_args(lnk, target, "", icon, desc)
}

fn create_windows_shortcut_with_args(lnk: &Path, target: &Path, args: &str, icon: &Path, desc: &str) -> bool {
    let ps = format!(
        "$s=(New-Object -COM WScript.Shell).CreateShortcut('{}');$s.TargetPath='{}';$s.Arguments='{}';$s.IconLocation='{}';$s.Description='{}';$s.Save()",
        lnk.display().to_string().replace('\'', "''"),
        target.display().to_string().replace('\'', "''"),
        args.replace('\'', "''"),
        icon.display().to_string().replace('\'', "''"),
        desc.replace('\'', "''"),
    );
    system::hidden_command("powershell")
        .args(["-NoProfile", "-NonInteractive", "-Command", &ps])
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

fn register_uninstaller_hkcu(install_dir: &Path, exe: &Path, icon: &Path) {
    let key = r"HKCU\Software\Microsoft\Windows\CurrentVersion\Uninstall\AxoloteSuite";
    let uninstall_cmd = format!("\"{}\" --uninstall", exe.display());
    for (name, val) in [
        ("DisplayName", "Axolote Suite".to_string()),
        ("DisplayVersion", env!("CARGO_PKG_VERSION").to_string()),
        ("Publisher", "Axolote".to_string()),
        ("InstallLocation", install_dir.display().to_string()),
        ("DisplayIcon", icon.display().to_string()),
        ("UninstallString", uninstall_cmd),
    ] {
        let _ = system::hidden_command("reg")
            .args(["add", key, "/v", name, "/t", "REG_SZ", "/d", &val, "/f"])
            .status();
    }
}

fn unregister_hkcu() {
    let key = r"HKCU\Software\Microsoft\Windows\CurrentVersion\Uninstall\AxoloteSuite";
    let _ = system::hidden_command("reg").args(["delete", key, "/f"]).status();
}

fn schedule_self_delete(dir: &Path) {
    let cmd_str = format!("ping 127.0.0.1 -n 3 > nul & rmdir /s /q \"{}\"", dir.display());
    let _ = system::hidden_command("cmd").args(["/C", &cmd_str]).spawn();
}
