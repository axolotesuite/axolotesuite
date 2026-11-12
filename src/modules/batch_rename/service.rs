//! Lógica de renomeação em lote com pré-visualização ao vivo.

use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};

use crate::core::paths;
use crate::shared::types::OperationResult;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Rule {
    Replace,
    Prefix,
    Suffix,
    Number,
    Uppercase,
    Lowercase,
}

impl Rule {
    pub const ALL: &'static [Rule] = &[
        Rule::Replace,
        Rule::Prefix,
        Rule::Suffix,
        Rule::Number,
        Rule::Uppercase,
        Rule::Lowercase,
    ];

    pub fn description(&self) -> &'static str {
        match self {
            Rule::Replace => {
                "Troca um trecho do nome por outro (ou apaga se deixar o segundo campo vazio)."
            }
            Rule::Prefix => "Adiciona um texto no início de cada nome de arquivo.",
            Rule::Suffix => "Adiciona um texto no final do nome (antes da extensão .jpg, .pdf...).",
            Rule::Number => {
                "Numera os arquivos em sequência: 01, 02, 03... com um nome base opcional."
            }
            Rule::Uppercase => "Converte todo o nome do arquivo para LETRAS MAIÚSCULAS.",
            Rule::Lowercase => "Converte todo o nome do arquivo para letras minúsculas.",
        }
    }
}

impl fmt::Display for Rule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Rule::Replace => "Substituir texto",
            Rule::Prefix => "Adicionar prefixo (no início)",
            Rule::Suffix => "Adicionar sufixo (no fim)",
            Rule::Number => "Numerar sequencialmente (01, 02, 03…)",
            Rule::Uppercase => "Converter para MAIÚSCULAS",
            Rule::Lowercase => "Converter para minúsculas",
        };
        write!(f, "{s}")
    }
}

#[derive(Debug, Clone)]
pub struct PreviewItem {
    pub original_path: PathBuf,
    pub old_name: String,
    pub new_name: String,
    pub changed: bool,
}

/// Calcula como cada arquivo se chamará após aplicar a regra.
pub fn compute_new_name(
    old_name: &str,
    rule: Rule,
    find: &str,
    value: &str,
    index: usize,
    total: usize,
) -> String {
    let p = Path::new(old_name);
    let stem = p.file_stem().and_then(|s| s.to_str()).unwrap_or(old_name);
    let ext = p.extension().and_then(|e| e.to_str()).unwrap_or("");

    let new_stem = match rule {
        Rule::Replace => {
            if find.is_empty() {
                stem.to_string()
            } else {
                stem.replace(find, value)
            }
        }
        Rule::Prefix => format!("{value}{stem}"),
        Rule::Suffix => format!("{stem}{value}"),
        Rule::Number => {
            let digits = total.max(1).to_string().len().max(2);
            let num = format!("{:0width$}", index + 1, width = digits);
            let base = value.trim();
            if base.is_empty() {
                format!("{num}_{stem}")
            } else {
                format!("{base}_{num}")
            }
        }
        Rule::Uppercase => stem.to_uppercase(),
        Rule::Lowercase => stem.to_lowercase(),
    };

    let clean_stem = if new_stem.trim().is_empty() {
        stem.to_string()
    } else {
        new_stem
    };
    if ext.is_empty() {
        clean_stem
    } else {
        format!("{clean_stem}.{ext}")
    }
}

/// Gera a lista de pré-visualização para todos os arquivos selecionados.
pub fn preview(files: &[PathBuf], rule: Rule, find: &str, value: &str) -> Vec<PreviewItem> {
    let total = files.len();
    files
        .iter()
        .enumerate()
        .map(|(i, path)| {
            let old_name = crate::shared::ui::file_name(path);
            let new_name = compute_new_name(&old_name, rule, find, value, i, total);
            let changed = old_name != new_name;
            PreviewItem {
                original_path: path.clone(),
                old_name,
                new_name,
                changed,
            }
        })
        .collect()
}

/// Executa a renomeação nos arquivos reais, evitando sobrescritas.
pub fn execute(
    files: Vec<PathBuf>,
    rule: Rule,
    find: String,
    value: String,
) -> (OperationResult, Vec<PathBuf>) {
    if files.is_empty() {
        return (
            OperationResult::err("Adicione pelo menos um arquivo antes de renomear."),
            files,
        );
    }
    let items = preview(&files, rule, &find, &value);
    let mut renamed = 0usize;
    let mut details = Vec::new();
    let mut updated_paths = Vec::with_capacity(files.len());
    let mut first_parent: Option<PathBuf> = None;

    for item in items {
        if !item.changed {
            updated_paths.push(item.original_path);
            continue;
        }
        let Some(parent) = item.original_path.parent().map(|p| p.to_path_buf()) else {
            updated_paths.push(item.original_path);
            continue;
        };
        if first_parent.is_none() {
            first_parent = Some(parent.clone());
        }
        let desired = parent.join(&item.new_name);
        // Se for apenas mudança de maiúsculas/minúsculas do mesmo caminho no Windows,
        // permitimos renomear diretamente; caso contrário garantimos caminho único.
        let same_ci = item.old_name.eq_ignore_ascii_case(&item.new_name);
        let target = if same_ci {
            desired
        } else {
            paths::unique_path(desired)
        };
        match fs::rename(&item.original_path, &target) {
            Ok(_) => {
                renamed += 1;
                details.push(format!(
                    "{}  →  {}",
                    item.old_name,
                    crate::shared::ui::file_name(&target)
                ));
                updated_paths.push(target);
            }
            Err(e) => {
                details.push(format!("Erro em {}: {e}", item.old_name));
                updated_paths.push(item.original_path);
            }
        }
    }

    let mut res = if renamed > 0 {
        OperationResult::ok(format!("{renamed} arquivo(s) renomeado(s) com sucesso."))
            .with_details(details)
    } else {
        OperationResult::err("Nenhum arquivo precisava ser alterado com a regra atual.")
            .with_details(details)
    };
    if let Some(dir) = first_parent {
        res = res.with_output(dir);
    }
    (res, updated_paths)
}

/// Lista todos os arquivos (não pastas) existentes dentro de `dir`.
pub fn files_in_folder(dir: &Path) -> Vec<PathBuf> {
    let mut list: Vec<PathBuf> = fs::read_dir(dir)
        .into_iter()
        .flatten()
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.is_file())
        .collect();
    list.sort();
    list
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn regras_de_renomeacao() {
        assert_eq!(
            compute_new_name("foto_antiga.jpg", Rule::Replace, "antiga", "nova", 0, 1),
            "foto_nova.jpg"
        );
        assert_eq!(
            compute_new_name("relatorio.pdf", Rule::Prefix, "", "2026_", 0, 1),
            "2026_relatorio.pdf"
        );
        assert_eq!(
            compute_new_name("relatorio.pdf", Rule::Suffix, "", "_final", 0, 1),
            "relatorio_final.pdf"
        );
        assert_eq!(
            compute_new_name("img.png", Rule::Number, "", "ferias", 0, 12),
            "ferias_01.png"
        );
        assert_eq!(
            compute_new_name(" teste.txt", Rule::Uppercase, "", "", 0, 1),
            " TESTE.txt"
        );
    }
}
