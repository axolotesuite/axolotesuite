//! Operações com arquivos ZIP: listar conteúdo, extrair (com proteção
//! contra Zip-Slip) e criar arquivos compactados (incluindo pastas inteiras).

use std::fs::{self, File};
use std::io::{self, Read};
use std::path::{Component, Path, PathBuf};

use walkdir::WalkDir;
use zip::write::FileOptions;
use zip::{CompressionMethod, ZipArchive, ZipWriter};

use crate::shared::types::OperationResult;
use crate::shared::ui::human_size;

#[derive(Debug, Clone)]
pub struct ZipEntryInfo {
    pub name: String,
    pub size: u64,
    pub compressed: u64,
    pub is_dir: bool,
}

/// Limites de segurança contra ataques de negação de serviço e ZIP Bombs
pub const MAX_ZIP_ENTRIES: usize = 20_000;
pub const MAX_ENTRY_BYTES: u64 = 1024 * 1024 * 1024; // 1 GiB por arquivo
pub const MAX_TOTAL_UNCOMPRESSED_BYTES: u64 = 4 * 1024 * 1024 * 1024; // 4 GiB acumulados

pub fn is_windows_reserved_name(comp: &str) -> bool {
    let stem = comp.split('.').next().unwrap_or(comp);
    let upper = stem.to_ascii_uppercase();
    matches!(
        upper.as_str(),
        "CON" | "PRN" | "AUX" | "NUL"
            | "COM1" | "COM2" | "COM3" | "COM4" | "COM5" | "COM6" | "COM7" | "COM8" | "COM9"
            | "LPT1" | "LPT2" | "LPT3" | "LPT4" | "LPT5" | "LPT6" | "LPT7" | "LPT8" | "LPT9"
    )
}

/// Verifica se o caminho interno do ZIP é seguro (rejeita `..`, raízes, prefixos de disco,
/// NTFS Alternate Data Streams, nomes reservados Windows e caracteres de controle).
pub fn sanitize_relative_path(name: &str) -> Option<PathBuf> {
    if name.contains('\0') {
        return None;
    }
    let normalized = name.replace('\\', "/");
    let path = Path::new(&normalized);
    let mut out = PathBuf::new();
    for comp in path.components() {
        match comp {
            Component::Normal(part) => {
                let s = part.to_str()?;
                if s.is_empty()
                    || s.ends_with([' ', '.'])
                    || s.chars().any(|c| "<>:\"|?*".contains(c) || c.is_control())
                    || is_windows_reserved_name(s)
                {
                    return None;
                }
                out.push(s);
            }
            Component::CurDir => {}
            Component::ParentDir | Component::RootDir | Component::Prefix(_) => return None,
        }
    }
    if out.as_os_str().is_empty() {
        None
    } else {
        Some(out)
    }
}

/// Lê a lista de itens dentro de um arquivo `.zip`.
pub fn list_contents(zip_path: &Path) -> Result<Vec<ZipEntryInfo>, String> {
    if !zip_path.is_file() {
        return Err("Arquivo ZIP não encontrado. Escolha um arquivo .zip válido.".into());
    }
    let file = File::open(zip_path).map_err(|e| format!("Não foi possível abrir o ZIP: {e}"))?;
    let mut archive = ZipArchive::new(file).map_err(|e| format!("Arquivo ZIP inválido ou corrompido: {e}"))?;
    let mut list = Vec::with_capacity(archive.len());
    for i in 0..archive.len() {
        if let Ok(f) = archive.by_index(i) {
            list.push(ZipEntryInfo {
                name: f.name().to_string(),
                size: f.size(),
                compressed: f.compressed_size(),
                is_dir: f.is_dir(),
            });
        }
    }
    Ok(list)
}

/// Extrai o arquivo ZIP em `dest_dir`, bloqueando qualquer tentativa de
/// escrever fora da pasta de destino (Zip-Slip).
pub fn extract(zip_path: PathBuf, dest_dir: PathBuf) -> OperationResult {
    if !zip_path.is_file() {
        return OperationResult::err("Arquivo ZIP não encontrado.");
    }
    let file = match File::open(&zip_path) {
        Ok(f) => f,
        Err(e) => return OperationResult::err(format!("Erro ao abrir o ZIP: {e}")),
    };
    let mut archive = match ZipArchive::new(file) {
        Ok(a) => a,
        Err(e) => return OperationResult::err(format!("Arquivo ZIP inválido: {e}")),
    };
    if let Err(e) = fs::create_dir_all(&dest_dir) {
        return OperationResult::err(format!("Não foi possível criar a pasta de destino: {e}"));
    }

    if archive.len() > MAX_ZIP_ENTRIES {
        return OperationResult::err(format!(
            "Arquivo ZIP contém entradas em excesso ({} > limite de {}). Bloqueado por segurança.",
            archive.len(),
            MAX_ZIP_ENTRIES
        ));
    }

    let mut extracted = 0usize;
    let mut blocked = 0usize;
    let mut total_bytes = 0u64;

    for i in 0..archive.len() {
        let Ok(mut entry) = archive.by_index(i) else { continue };

        // Rejeita links simbólicos em arquivos ZIP
        if entry.unix_mode().map(|m| (m & 0o170000) == 0o120000).unwrap_or(false) {
            blocked += 1;
            continue;
        }

        let Some(rel) = sanitize_relative_path(entry.name()) else {
            blocked += 1;
            continue;
        };
        let target = dest_dir.join(rel);
        if entry.is_dir() {
            let _ = fs::create_dir_all(&target);
            continue;
        }
        if let Some(parent) = target.parent() {
            let _ = fs::create_dir_all(parent);
        }

        // Não sobrescreve silenciosamente arquivos existentes: gera caminho único seguro
        let safe_target = if target.exists() {
            crate::core::paths::unique_path(target)
        } else {
            target
        };

        if let Ok(mut out) = File::create(&safe_target) {
            let mut limited = (&mut entry).take(MAX_ENTRY_BYTES);
            if let Ok(n) = io::copy(&mut limited, &mut out) {
                extracted += 1;
                total_bytes += n;
                if total_bytes > MAX_TOTAL_UNCOMPRESSED_BYTES {
                    blocked += 1;
                    break;
                }
            }
        }
    }

    let mut details = vec![
        format!("Pasta de destino: {}", dest_dir.display()),
        format!("Total extraído: {}", human_size(total_bytes)),
    ];
    if blocked > 0 {
        details.push(format!("🛡 {blocked} caminho(s) suspeito(s) bloqueado(s) por segurança."));
    }
    OperationResult::ok(format!("{extracted} arquivo(s) extraído(s) com sucesso."))
        .with_details(details)
        .with_output(dest_dir)
}

/// Cria um arquivo `.zip` a partir de uma lista de arquivos e/ou pastas.
pub fn create(output: PathBuf, sources: Vec<PathBuf>) -> OperationResult {
    if sources.is_empty() {
        return OperationResult::err("Adicione pelo menos um arquivo ou pasta para compactar.");
    }
    if let Some(parent) = output.parent() {
        if !parent.as_os_str().is_empty() {
            let _ = fs::create_dir_all(parent);
        }
    }
    let file = match File::create(&output) {
        Ok(f) => f,
        Err(e) => return OperationResult::err(format!("Não foi possível criar o arquivo ZIP: {e}")),
    };
    let mut zip = ZipWriter::new(file);
    let options = FileOptions::default()
        .compression_method(CompressionMethod::Deflated)
        .unix_permissions(0o644);

    let mut count = 0usize;
    let mut errors = Vec::new();

    for src in &sources {
        // Evita tentar incluir o próprio arquivo de saída caso esteja na mesma pasta
        if src == &output {
            continue;
        }
        if src.is_file() {
            let name = crate::shared::ui::file_name(src);
            match add_file_to_zip(&mut zip, src, &name, options) {
                Ok(_) => count += 1,
                Err(e) => errors.push(format!("{name}: {e}")),
            }
        } else if src.is_dir() {
            let base_parent = src.parent().unwrap_or(src);
            for entry in WalkDir::new(src).into_iter().flatten() {
                let p = entry.path();
                if p == output || !p.is_file() {
                    continue;
                }
                let rel = p.strip_prefix(base_parent).unwrap_or(p);
                let zip_name = rel.to_string_lossy().replace('\\', "/");
                match add_file_to_zip(&mut zip, p, &zip_name, options) {
                    Ok(_) => count += 1,
                    Err(e) => errors.push(format!("{zip_name}: {e}")),
                }
            }
        }
    }

    if let Err(e) = zip.finish() {
        return OperationResult::err(format!("Erro ao finalizar o arquivo ZIP: {e}"));
    }

    let final_size = fs::metadata(&output).map(|m| m.len()).unwrap_or(0);
    let mut details = vec![
        format!("Arquivo gerado: {}", output.display()),
        format!("Tamanho final do ZIP: {}", human_size(final_size)),
    ];
    for err in errors.iter().take(10) {
        details.push(format!("Aviso: {err}"));
    }
    OperationResult::ok(format!("ZIP criado com {count} arquivo(s) ({})", human_size(final_size)))
        .with_details(details)
        .with_output(output)
}

fn add_file_to_zip<W: io::Write + io::Seek>(
    zip: &mut ZipWriter<W>,
    src: &Path,
    zip_name: &str,
    options: FileOptions,
) -> io::Result<()> {
    let mut f = File::open(src)?;
    zip.start_file(zip_name, options).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    io::copy(&mut f, zip)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bloqueia_zip_slip() {
        assert!(sanitize_relative_path("../segredo.txt").is_none());
        assert!(sanitize_relative_path("pasta/../../etc/passwd").is_none());
        assert!(sanitize_relative_path("/absoluto.txt").is_none());
        assert!(sanitize_relative_path("C:\\Windows\\system.ini").is_none());
        assert!(sanitize_relative_path("CON").is_none());
        assert!(sanitize_relative_path("NUL.txt").is_none());
        assert!(sanitize_relative_path("doc.txt:ads.exe").is_none());
        assert!(sanitize_relative_path("arquivo_com_espaco ").is_none());
        assert_eq!(sanitize_relative_path("fotos/praia.jpg"), Some(PathBuf::from("fotos").join("praia.jpg")));
    }

    #[test]
    fn cria_e_extrai_zip() {
        let dir = std::env::temp_dir().join(format!("axolote_zip_test_{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        let src_file = dir.join("ola.txt");
        fs::write(&src_file, "conteudo de teste").unwrap();

        let zip_path = dir.join("saida.zip");
        let res = create(zip_path.clone(), vec![src_file]);
        assert!(res.success);

        let items = list_contents(&zip_path).unwrap();
        assert_eq!(items.len(), 1);

        let out_dir = dir.join("extraido");
        let ext = extract(zip_path, out_dir.clone());
        assert!(ext.success);
        assert_eq!(fs::read_to_string(out_dir.join("ola.txt")).unwrap(), "conteudo de teste");
        let _ = fs::remove_dir_all(&dir);
    }
}
