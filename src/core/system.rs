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
    if let Ok(exe) = std::env::current_exe() {
        if let Some(dir) = exe.parent() {
            candidates.push(dir.join("ffmpeg.exe"));
            candidates.push(dir.join("bin").join("ffmpeg.exe"));
            candidates.push(dir.join("ffmpeg").join("ffmpeg.exe"));
        }
    }
    for candidate in candidates {
        if candidate.is_file() {
            return Some(candidate);
        }
    }
    let ok = hidden_command("ffmpeg")
        .arg("-version")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false);
    ok.then(|| PathBuf::from("ffmpeg"))
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
        .args(["-hide_banner", "-loglevel", "error", "-y"])
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
    let out = match hidden_command(ffmpeg).arg("-hide_banner").arg("-i").arg(input).output() {
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
