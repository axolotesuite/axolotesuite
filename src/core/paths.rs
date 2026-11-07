//! Pastas usadas pelo programa.
//!
//! Todos os dados ficam em `%APPDATA%\AxoloteSuite` (configurações,
//! histórico, tarefas e notas). Nada é enviado para a internet.

use std::path::PathBuf;

/// Pasta de dados do aplicativo (criada automaticamente).
pub fn data_dir() -> PathBuf {
    let base = std::env::var_os("APPDATA")
        .map(PathBuf::from)
        .or_else(|| std::env::var_os("HOME").map(|h| PathBuf::from(h).join(".config")))
        .unwrap_or_else(|| PathBuf::from("."));
    let dir = base.join("AxoloteSuite");
    let _ = std::fs::create_dir_all(&dir);
    dir
}

/// Pasta onde as notas de texto são guardadas.
pub fn notes_dir() -> PathBuf {
    let dir = data_dir().join("notas");
    let _ = std::fs::create_dir_all(&dir);
    dir
}

/// Pasta onde o instalador automático guarda dependências essenciais (como `ffmpeg.exe`).
pub fn bin_dir() -> PathBuf {
    let dir = data_dir().join("bin");
    let _ = std::fs::create_dir_all(&dir);
    dir
}

pub fn config_file() -> PathBuf {
    data_dir().join("config.json")
}

pub fn history_file() -> PathBuf {
    data_dir().join("historico.json")
}

pub fn tasks_file() -> PathBuf {
    data_dir().join("tarefas.json")
}

/// Gera um caminho que ainda não existe, acrescentando " (2)", " (3)"...
pub fn unique_path(path: PathBuf) -> PathBuf {
    if !path.exists() {
        return path;
    }
    let parent = path.parent().map(|p| p.to_path_buf()).unwrap_or_default();
    let stem = path.file_stem().map(|s| s.to_string_lossy().to_string()).unwrap_or_default();
    let ext = path.extension().map(|e| e.to_string_lossy().to_string());
    for i in 2.. {
        let name = match &ext {
            Some(ext) => format!("{stem} ({i}).{ext}"),
            None => format!("{stem} ({i})"),
        };
        let candidate = parent.join(name);
        if !candidate.exists() {
            return candidate;
        }
    }
    unreachable!()
}

/// Monta o caminho de saída `pasta/nome_sufixo.ext`, sem sobrescrever nada.
pub fn output_path(input: &std::path::Path, out_dir: Option<&std::path::Path>, suffix: &str, ext: &str) -> PathBuf {
    let dir = out_dir
        .map(|d| d.to_path_buf())
        .or_else(|| input.parent().map(|p| p.to_path_buf()))
        .unwrap_or_default();
    let stem = input.file_stem().map(|s| s.to_string_lossy().to_string()).unwrap_or_else(|| "arquivo".into());
    let name = if suffix.is_empty() { format!("{stem}.{ext}") } else { format!("{stem}_{suffix}.{ext}") };
    unique_path(dir.join(name))
}
