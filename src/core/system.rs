//! Utilitários de sistema: execução em segundo plano, diálogos nativos de
//! arquivos, abrir pastas no Explorer e integração com o FFmpeg.

use std::future::Future;
use std::path::{Path, PathBuf};
use std::process::Command;

// ---------------------------------------------------------------------------
// Segundo plano
// ---------------------------------------------------------------------------

/// Executa uma função pesada numa thread separada e devolve um `Future`
/// com o resultado — assim a janela continua respondendo.
pub fn run_blocking<T, F>(f: F) -> impl Future<Output = T>
where
    T: Send + 'static,
    F: FnOnce() -> T + Send + 'static,
{
    let (tx, rx) = iced::futures::channel::oneshot::channel();
    std::thread::spawn(move || {
        let _ = tx.send(f());
    });
    async move { rx.await.expect("a operação em segundo plano foi interrompida") }
}

// ---------------------------------------------------------------------------
// Explorer
// ---------------------------------------------------------------------------

/// Abre uma pasta (ou seleciona um arquivo) no Explorer do Windows.
pub fn open_path(path: &Path) {
    #[cfg(windows)]
    {
        if path.is_file() {
            let _ = Command::new("explorer").arg(format!("/select,{}", path.display())).spawn();
        } else {
            let _ = Command::new("explorer").arg(path).spawn();
        }
    }
    #[cfg(not(windows))]
    {
        let target = if path.is_file() { path.parent().unwrap_or(path) } else { path };
        let _ = Command::new("xdg-open").arg(target).spawn();
    }
}

// ---------------------------------------------------------------------------
// Diálogos nativos
// ---------------------------------------------------------------------------

pub fn pick_files(title: &str, filter_name: &str, exts: &[&str]) -> Vec<PathBuf> {
    let mut d = rfd::FileDialog::new().set_title(title);
    if !exts.is_empty() {
        d = d.add_filter(filter_name, exts);
    }
    d = d.add_filter("Todos os arquivos", &["*"]);
    d.pick_files().unwrap_or_default()
}

pub fn pick_file(title: &str, filter_name: &str, exts: &[&str]) -> Option<PathBuf> {
    let mut d = rfd::FileDialog::new().set_title(title);
    if !exts.is_empty() {
        d = d.add_filter(filter_name, exts);
    }
    d = d.add_filter("Todos os arquivos", &["*"]);
    d.pick_file()
}

pub fn pick_folder(title: &str) -> Option<PathBuf> {
    rfd::FileDialog::new().set_title(title).pick_folder()
}

pub fn save_file(title: &str, default_name: &str, filter_name: &str, exts: &[&str]) -> Option<PathBuf> {
    rfd::FileDialog::new()
        .set_title(title)
        .set_file_name(default_name)
        .add_filter(filter_name, exts)
        .save_file()
}

// ---------------------------------------------------------------------------
// Processos externos
// ---------------------------------------------------------------------------

/// Cria um `Command` que não abre janela de console no Windows.
pub fn hidden_command(program: impl AsRef<std::ffi::OsStr>) -> Command {
    #[allow(unused_mut)]
    let mut cmd = Command::new(program);
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        const CREATE_NO_WINDOW: u32 = 0x0800_0000;
        cmd.creation_flags(CREATE_NO_WINDOW);
    }
    cmd
}

/// Localiza o executável do FFmpeg:
/// 1. Na pasta `bin` de dados do Axolote (`%APPDATA%\AxoloteSuite\bin\ffmpeg.exe`)
/// 2. Na pasta de instalação padrão (`%LOCALAPPDATA%\Programs\Axolote Suite\bin\ffmpeg.exe`)
/// 3. Ao lado do executável atual
/// 4. No PATH do sistema Windows
pub fn find_ffmpeg() -> Option<PathBuf> {
    let mut candidates = vec![
        crate::core::paths::bin_dir().join("ffmpeg.exe"),
        crate::core::paths::data_dir().join("ffmpeg.exe"),
    ];
    if let Some(local) = std::env::var_os("LOCALAPPDATA") {
        let base = PathBuf::from(local).join("Programs").join("Axolote Suite");
        candidates.push(base.join("bin").join("ffmpeg.exe"));
        candidates.push(base.join("ffmpeg.exe"));
    }
    for candidate in candidates {
        if candidate.is_file() {
            return Some(candidate);
        }
    }
    // Só recorre ao executável local se estiver explicitamente na pasta do próprio executável
    if let Ok(exe) = std::env::current_exe() {
        if let Some(dir) = exe.parent() {
            let local_bin = dir.join("bin").join("ffmpeg.exe");
            if local_bin.is_file() {
                return Some(local_bin);
            }
            let local_ffmpeg = dir.join("ffmpeg.exe");
            if local_ffmpeg.is_file() {
                return Some(local_ffmpeg);
            }
        }
    }
    // Não executa "ffmpeg" cego do PATH para evitar CWE-426 Binary Planting
    None
}

/// Baixa e instala automaticamente o `ffmpeg.exe` para a pasta `dest_bin_dir` (e também
/// copia para `%APPDATA%\AxoloteSuite\bin\ffmpeg.exe`), caso ainda não esteja disponível.
pub fn ensure_ffmpeg_installed(dest_bin_dir: &Path) -> Result<PathBuf, String> {
    let _ = std::fs::create_dir_all(dest_bin_dir);
    let target_ffmpeg = dest_bin_dir.join("ffmpeg.exe");
    let appdata_ffmpeg = crate::core::paths::bin_dir().join("ffmpeg.exe");

    // Se já existir localmente no computador, aproveita e copia para a pasta de instalação
    if let Some(existing) = find_ffmpeg() {
        if existing.is_file() {
            if existing != target_ffmpeg {
                let _ = std::fs::copy(&existing, &target_ffmpeg);
            }
            if existing != appdata_ffmpeg {
                let _ = std::fs::copy(&existing, &appdata_ffmpeg);
            }
            return Ok(if target_ffmpeg.is_file() { target_ffmpeg } else { existing });
        }
        return Ok(existing);
    }

    // 1ª tentativa: Download direto do zip oficial de release do FFmpeg e extração via crate `zip`
    let urls = [
        "https://github.com/BtbN/FFmpeg-Builds/releases/download/latest/ffmpeg-master-latest-win64-gpl.zip",
        "https://www.gyan.dev/ffmpeg/builds/ffmpeg-release-essentials.zip",
    ];
    let tmp_zip = std::env::temp_dir().join("axolote_ffmpeg_setup.zip");

    for url in urls {
        let dl_cmd = format!(
            "[Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12; $ProgressPreference = 'SilentlyContinue'; Invoke-WebRequest -Uri '{}' -OutFile '{}' -UseBasicParsing",
            url,
            tmp_zip.display().to_string().replace('\'', "''")
        );
        let status = hidden_command("powershell")
            .args(["-NoProfile", "-NonInteractive", "-Command", &dl_cmd])
            .status();

        if status.map(|s| s.success()).unwrap_or(false) && tmp_zip.is_file() {
            if extract_ffmpeg_from_zip(&tmp_zip, &target_ffmpeg).is_ok() && target_ffmpeg.is_file() {
                let _ = std::fs::copy(&target_ffmpeg, &appdata_ffmpeg);
                let _ = std::fs::remove_file(&tmp_zip);
                return Ok(target_ffmpeg);
            }
        }
        let _ = std::fs::remove_file(&tmp_zip);
    }

    // 2ª tentativa (fallback): winget oficial da Microsoft
    let _ = hidden_command("winget")
        .args([
            "install",
            "--id",
            "Gyan.FFmpeg",
            "-e",
            "--silent",
            "--accept-package-agreements",
            "--accept-source-agreements",
        ])
        .status();

    if let Some(found) = find_ffmpeg() {
        return Ok(found);
    }

    Err("Não foi possível baixar o FFmpeg automaticamente (verifique sua conexão com a internet).".into())
}

fn extract_ffmpeg_from_zip(zip_path: &Path, target_ffmpeg: &Path) -> std::io::Result<()> {
    let file = std::fs::File::open(zip_path)?;
    let mut archive = zip::ZipArchive::new(file).map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
    for i in 0..archive.len() {
        let mut entry = archive.by_index(i).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        let name = entry.name().replace('\\', "/");
        if name.ends_with("/ffmpeg.exe") || name.eq_ignore_ascii_case("ffmpeg.exe") {
            if let Some(parent) = target_ffmpeg.parent() {
                let _ = std::fs::create_dir_all(parent);
            }
            let mut out = std::fs::File::create(target_ffmpeg)?;
            std::io::copy(&mut entry, &mut out)?;
            return Ok(());
        }
    }
    Err(std::io::Error::new(std::io::ErrorKind::NotFound, "ffmpeg.exe não encontrado dentro do pacote ZIP"))
}

/// Executa o FFmpeg com os argumentos dados. Em caso de erro devolve as
/// últimas linhas da saída para ajudar a entender o problema.
pub fn run_ffmpeg(ffmpeg: &Path, args: &[String]) -> Result<(), String> {
    let out = hidden_command(ffmpeg)
        .args(["-hide_banner", "-loglevel", "error"])
        .args(args)
        .output()
        .map_err(|e| format!("Não foi possível iniciar o FFmpeg: {e}"))?;
    if out.status.success() {
        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&out.stderr);
        let tail: Vec<&str> = stderr.lines().filter(|l| !l.trim().is_empty()).rev().take(3).collect();
        let msg = tail.into_iter().rev().collect::<Vec<_>>().join(" | ");
        Err(if msg.is_empty() { "O FFmpeg terminou com erro.".into() } else { msg })
    }
}

/// Lê informações básicas de um arquivo de mídia (duração, codecs...).
pub fn probe_media(ffmpeg: &Path, input: &Path) -> Vec<String> {
    if !input.is_file() {
        return vec!["Arquivo de mídia não encontrado.".into()];
    }
    let out = match hidden_command(ffmpeg)
        .args(["-hide_banner", "-protocol_whitelist", "file"])
        .arg("-i")
        .arg(input)
        .output()
    {
        Ok(o) => o,
        Err(e) => return vec![format!("Erro ao executar o FFmpeg: {e}")],
    };
    let stderr = String::from_utf8_lossy(&out.stderr);
    let mut lines = Vec::new();
    for line in stderr.lines() {
        let t = line.trim();
        if t.starts_with("Duration:") {
            lines.push(t.replace("Duration:", "Duração:").replace("start:", "início:"));
        } else if t.starts_with("Stream #") {
            let pretty = t
                .splitn(2, ": ")
                .nth(1)
                .unwrap_or(t)
                .replace("Video:", "Vídeo:")
                .replace("Audio:", "Áudio:");
            lines.push(pretty);
        }
    }
    if lines.is_empty() {
        lines.push("Não foi possível ler informações deste arquivo.".into());
    }
    lines
}

// ---------------------------------------------------------------------------
// Detector Global de Atalho (Ctrl + Alt + N) e Gesto (Ctrl + Alt + Arrastar)
// ---------------------------------------------------------------------------

#[cfg(windows)]
mod win_gesture {
    use std::sync::atomic::{AtomicBool, Ordering};

    #[link(name = "user32")]
    extern "system" {
        fn GetAsyncKeyState(vkey: i32) -> i16;
        fn GetForegroundWindow() -> isize;
        fn GetClassNameW(hwnd: isize, lp_class_name: *mut u16, max_count: i32) -> i32;
        fn EnumWindows(lp_enum_func: unsafe extern "system" fn(isize, isize) -> i32, lparam: isize) -> i32;
        fn GetWindowThreadProcessId(hwnd: isize, lpdw_process_id: *mut u32) -> u32;
        fn IsWindowVisible(hwnd: isize) -> i32;
        fn ShowWindow(hwnd: isize, cmd_show: i32) -> i32;
        fn SetForegroundWindow(hwnd: isize) -> i32;
    }

    static WAS_TRIGGERED: AtomicBool = AtomicBool::new(false);

    const VK_LBUTTON: i32 = 0x01;
    const VK_CONTROL: i32 = 0x11;
    const VK_MENU: i32 = 0x12; // Alt
    const VK_N: i32 = 0x4E;
    const SW_RESTORE: i32 = 9;

    fn key_down(vk: i32) -> bool {
        unsafe { (GetAsyncKeyState(vk) as u16 & 0x8000) != 0 }
    }

    fn is_explorer_foreground() -> bool {
        unsafe {
            let fg = GetForegroundWindow();
            if fg == 0 {
                return false;
            }
            let mut buf = [0u16; 64];
            let len = GetClassNameW(fg, buf.as_mut_ptr(), 63);
            if len <= 0 {
                return false;
            }
            let cls = String::from_utf16_lossy(&buf[..len as usize]);
            matches!(cls.as_str(), "CabinetWClass" | "ExploreWClass" | "Progman" | "WorkerW")
        }
    }

    unsafe extern "system" fn enum_proc(hwnd: isize, lparam: isize) -> i32 {
        let mut pid = 0u32;
        GetWindowThreadProcessId(hwnd, &mut pid);
        if pid == std::process::id() && IsWindowVisible(hwnd) != 0 {
            let out_ptr = lparam as *mut isize;
            *out_ptr = hwnd;
            return 0;
        }
        1
    }

    pub fn bring_self_to_front() {
        unsafe {
            let mut my_hwnd: isize = 0;
            EnumWindows(enum_proc, (&mut my_hwnd as *mut isize) as isize);
            if my_hwnd != 0 {
                ShowWindow(my_hwnd, SW_RESTORE);
                SetForegroundWindow(my_hwnd);
            }
        }
    }

    pub fn poll_ctrl_alt_n_or_drag() -> Option<bool> {
        let ctrl_alt = key_down(VK_CONTROL) && key_down(VK_MENU);
        let n_pressed = ctrl_alt && key_down(VK_N);
        let explorer_drag = ctrl_alt && key_down(VK_LBUTTON) && is_explorer_foreground();
        let active = n_pressed || explorer_drag;

        let prev = WAS_TRIGGERED.swap(active, Ordering::SeqCst);
        if active && !prev {
            bring_self_to_front();
            // true = foi gesto de arrastar no Explorer; false = foi tecla Ctrl+Alt+N
            Some(explorer_drag && !n_pressed)
        } else {
            None
        }
    }
}

/// Verifica globalmente no Windows se o usuário apertou `Ctrl + Alt + N` ou
/// segurou `Ctrl + Alt` arrastando um arquivo no Windows Explorer.
/// Retorna `Some(true)` se foi gesto de arrastar no Explorer, `Some(false)` se foi `Ctrl + Alt + N`.
pub fn poll_global_quick_convert() -> Option<bool> {
    #[cfg(windows)]
    {
        win_gesture::poll_ctrl_alt_n_or_drag()
    }
    #[cfg(not(windows))]
    {
        None
    }
}

/// Subscription em segundo plano (0% CPU quando ocioso) que escuta globalmente no Windows
/// o atalho `Ctrl + Alt + N` e o gesto `Ctrl + Alt + Arrastar no Explorer`.
pub fn global_gesture_subscription() -> iced::Subscription<bool> {
    struct GlobalGestureWatcher;
    iced::subscription::unfold(
        std::any::TypeId::of::<GlobalGestureWatcher>(),
        (),
        |_| async move {
            let is_drag = run_blocking(|| loop {
                std::thread::sleep(std::time::Duration::from_millis(80));
                if let Some(drag) = poll_global_quick_convert() {
                    return drag;
                }
            })
            .await;
            (is_drag, ())
        },
    )
}
