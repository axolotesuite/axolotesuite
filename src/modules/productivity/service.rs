//! Serviços de produtividade (lógica pura, sem interface):
//! notas, tarefas, calculadora e organizador de arquivos.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::core::paths;
use crate::shared::types::OperationResult;

// ---------------------------------------------------------------------------
// Notas
// ---------------------------------------------------------------------------

/// Lista as notas (.txt) da pasta de notas, em ordem alfabética.
pub fn list_notes(dir: &Path) -> Vec<String> {
    let mut notes: Vec<String> = fs::read_dir(dir)
        .into_iter()
        .flatten()
        .flatten()
        .filter(|e| e.path().extension().map_or(false, |x| x.eq_ignore_ascii_case("txt")))
        .filter_map(|e| e.path().file_stem().map(|s| s.to_string_lossy().to_string()))
        .collect();
    notes.sort_by_key(|n| n.to_lowercase());
    notes
}

/// Remove caracteres que o Windows não aceita em nomes de arquivo.
pub fn sanitize_name(name: &str) -> String {
    let cleaned: String = name
        .trim()
        .chars()
        .map(|c| if "\\/:*?\"<>|".contains(c) || c.is_control() { '_' } else { c })
        .collect();
    let cleaned = cleaned.trim_end_matches(".txt").trim().trim_end_matches('.').to_string();
    cleaned
}

pub fn read_note(dir: &Path, name: &str) -> std::io::Result<String> {
    fs::read_to_string(dir.join(format!("{name}.txt")))
}

pub fn save_note(dir: &Path, name: &str, content: &str) -> OperationResult {
    let name = sanitize_name(name);
    if name.is_empty() {
        return OperationResult::err("Dê um nome para a nota antes de salvar.");
    }
    let path = dir.join(format!("{name}.txt"));
    match fs::write(&path, content) {
        Ok(_) => OperationResult::ok(format!("Nota \"{name}\" salva.")).with_output(path),
        Err(e) => OperationResult::err(format!("Erro ao salvar a nota: {e}")),
    }
}

pub fn delete_note(dir: &Path, name: &str) -> OperationResult {
    match fs::remove_file(dir.join(format!("{name}.txt"))) {
        Ok(_) => OperationResult::ok(format!("Nota \"{name}\" excluída.")),
        Err(e) => OperationResult::err(format!("Erro ao excluir a nota: {e}")),
    }
}

// ---------------------------------------------------------------------------
// Tarefas
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: u64,
    pub title: String,
    pub done: bool,
    pub created: String,
}

pub fn load_tasks() -> Vec<Task> {
    fs::read_to_string(paths::tasks_file())
        .ok()
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap_or_default()
}

pub fn save_tasks(tasks: &[Task]) {
    if let Ok(json) = serde_json::to_string_pretty(tasks) {
        let _ = fs::write(paths::tasks_file(), json);
    }
}

// ---------------------------------------------------------------------------
// Calculadora
// ---------------------------------------------------------------------------

/// Avalia uma expressão matemática com segurança (sem `eval`).
///
/// Suporta `+ - * / % ^`, parênteses, sinais negativos e vírgula como
/// separador decimal. Também aceita `x`, `×` e `÷`.
pub fn evaluate(expr: &str) -> Result<f64, String> {
    let normalized: String = expr
        .chars()
        .map(|c| match c {
            ',' => '.',
            'x' | 'X' | '×' => '*',
            '÷' => '/',
            c => c,
        })
        .filter(|c| !c.is_whitespace())
        .collect();
    if normalized.is_empty() {
        return Err("Digite uma expressão, por exemplo 12,5 * (3 + 2)".into());
    }
    let mut parser = Parser { chars: normalized.chars().collect(), pos: 0 };
    let value = parser.expr()?;
    if parser.pos < parser.chars.len() {
        return Err(format!("Caractere inesperado: \"{}\"", parser.chars[parser.pos]));
    }
    if !value.is_finite() {
        return Err("Resultado indefinido (divisão por zero?)".into());
    }
    Ok(value)
}

/// Formata o número no padrão brasileiro (vírgula decimal).
pub fn format_number(v: f64) -> String {
    if v.fract() == 0.0 && v.abs() < 1e15 {
        return format!("{}", v as i64);
    }
    let s = format!("{:.10}", v);
    let s = s.trim_end_matches('0').trim_end_matches('.');
    s.replace('.', ",")
}

struct Parser {
    chars: Vec<char>,
    pos: usize,
}

impl Parser {
    fn peek(&self) -> Option<char> {
        self.chars.get(self.pos).copied()
    }

    fn expr(&mut self) -> Result<f64, String> {
        let mut v = self.term()?;
        while let Some(c) = self.peek() {
            match c {
                '+' => {
                    self.pos += 1;
                    v += self.term()?;
                }
                '-' => {
                    self.pos += 1;
                    v -= self.term()?;
                }
                _ => break,
            }
        }
        Ok(v)
    }

    fn term(&mut self) -> Result<f64, String> {
        let mut v = self.power()?;
        while let Some(c) = self.peek() {
            match c {
                '*' => {
                    self.pos += 1;
                    v *= self.power()?;
                }
                '/' => {
                    self.pos += 1;
                    let d = self.power()?;
                    if d == 0.0 {
                        return Err("Não é possível dividir por zero.".into());
                    }
                    v /= d;
                }
                '%' => {
                    self.pos += 1;
                    let d = self.power()?;
                    if d == 0.0 {
                        return Err("Não é possível calcular resto por zero.".into());
                    }
                    v %= d;
                }
                _ => break,
            }
        }
        Ok(v)
    }

    fn power(&mut self) -> Result<f64, String> {
        let base = self.unary()?;
        if self.peek() == Some('^') {
            self.pos += 1;
            let exp = self.power()?; // associativa à direita
            return Ok(base.powf(exp));
        }
        Ok(base)
    }

    fn unary(&mut self) -> Result<f64, String> {
        match self.peek() {
            Some('-') => {
                self.pos += 1;
                Ok(-self.unary()?)
            }
            Some('+') => {
                self.pos += 1;
                self.unary()
            }
            _ => self.primary(),
        }
    }

    fn primary(&mut self) -> Result<f64, String> {
        match self.peek() {
            Some('(') => {
                self.pos += 1;
                let v = self.expr()?;
                if self.peek() != Some(')') {
                    return Err("Falta fechar um parêntese \")\".".into());
                }
                self.pos += 1;
                Ok(v)
            }
            Some(c) if c.is_ascii_digit() || c == '.' => {
                let start = self.pos;
                while let Some(c) = self.peek() {
                    if c.is_ascii_digit() || c == '.' {
                        self.pos += 1;
                    } else {
                        break;
                    }
                }
                let s: String = self.chars[start..self.pos].iter().collect();
                s.parse::<f64>().map_err(|_| format!("Número inválido: \"{s}\""))
            }
            Some(c) => Err(format!("Caractere não permitido: \"{c}\"")),
            None => Err("A expressão terminou de forma incompleta.".into()),
        }
    }
}

// ---------------------------------------------------------------------------
// Organizador de arquivos
// ---------------------------------------------------------------------------

/// Categoria (subpasta) de destino para cada extensão.
pub fn category_for(ext: &str) -> &'static str {
    match ext.to_lowercase().as_str() {
        "jpg" | "jpeg" | "png" | "gif" | "bmp" | "webp" | "svg" | "heic" | "ico" | "tif" | "tiff" | "raw" => "Imagens",
        "pdf" => "PDFs",
        "doc" | "docx" | "odt" | "rtf" | "txt" | "md" | "xls" | "xlsx" | "ods" | "csv" | "ppt" | "pptx" | "odp" => "Documentos",
        "mp3" | "wav" | "flac" | "ogg" | "m4a" | "aac" | "wma" | "opus" => "Áudios",
        "mp4" | "mkv" | "avi" | "mov" | "wmv" | "webm" | "flv" | "m4v" => "Vídeos",
        "zip" | "rar" | "7z" | "tar" | "gz" | "bz2" | "xz" => "Compactados",
        "exe" | "msi" | "bat" | "cmd" | "apk" => "Programas",
        _ => "Outros",
    }
}

/// Conta quantos arquivos iriam para cada categoria (não move nada).
pub fn organize_preview(dir: &Path) -> Result<BTreeMap<&'static str, usize>, String> {
    if !dir.is_dir() {
        return Err("Pasta não encontrada. Escolha uma pasta válida.".into());
    }
    let mut map = BTreeMap::new();
    for entry in fs::read_dir(dir).map_err(|e| e.to_string())?.flatten() {
        let p = entry.path();
        if p.is_file() {
            let ext = p.extension().map(|e| e.to_string_lossy().to_string()).unwrap_or_default();
            *map.entry(category_for(&ext)).or_insert(0) += 1;
        }
    }
    Ok(map)
}

/// Move os arquivos da pasta para subpastas por tipo.
pub fn organize(dir: PathBuf) -> OperationResult {
    if !dir.is_dir() {
        return OperationResult::err("Pasta não encontrada. Escolha uma pasta válida.");
    }
    let entries = match fs::read_dir(&dir) {
        Ok(e) => e,
        Err(e) => return OperationResult::err(format!("Não foi possível ler a pasta: {e}")),
    };
    let mut moved = 0;
    let mut errors = Vec::new();
    let mut per_cat: BTreeMap<&str, usize> = BTreeMap::new();
    for entry in entries.flatten() {
        let p = entry.path();
        if !p.is_file() {
            continue;
        }
        let ext = p.extension().map(|e| e.to_string_lossy().to_string()).unwrap_or_default();
        let cat = category_for(&ext);
        let target_dir = dir.join(cat);
        if let Err(e) = fs::create_dir_all(&target_dir) {
            errors.push(format!("{}: {e}", p.display()));
            continue;
        }
        let target = paths::unique_path(target_dir.join(p.file_name().unwrap_or_default()));
        match fs::rename(&p, &target) {
            Ok(_) => {
                moved += 1;
                *per_cat.entry(cat).or_insert(0) += 1;
            }
            Err(e) => errors.push(format!("{}: {e}", crate::shared::ui::file_name(&p))),
        }
    }
    let mut details: Vec<String> = per_cat.iter().map(|(c, n)| format!("{c}: {n} arquivo(s)")).collect();
    for e in errors.iter().take(10) {
        details.push(format!("Erro: {e}"));
    }
    let title = if errors.is_empty() {
        format!("{moved} arquivo(s) organizado(s) em subpastas.")
    } else {
        format!("{moved} arquivo(s) organizado(s), {} com erro.", errors.len())
    };
    OperationResult { success: errors.is_empty() || moved > 0, title, details, output: Some(dir) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculadora_respeita_precedencia() {
        assert_eq!(evaluate("2 + 2 * 3").unwrap(), 8.0);
        assert_eq!(evaluate("(2 + 2) * 3").unwrap(), 12.0);
        assert_eq!(evaluate("-3 + 5").unwrap(), 2.0);
        assert_eq!(evaluate("2^3^2").unwrap(), 512.0);
        assert_eq!(evaluate("10 % 4").unwrap(), 2.0);
        assert_eq!(evaluate("12,5 x 2").unwrap(), 25.0);
        assert!(evaluate("1 / 0").is_err());
        assert!(evaluate("2 +").is_err());
        assert!(evaluate("abc").is_err());
    }

    #[test]
    fn formata_numero_brasileiro() {
        assert_eq!(format_number(2.5), "2,5");
        assert_eq!(format_number(10.0), "10");
    }

    #[test]
    fn nome_de_nota_seguro() {
        assert_eq!(sanitize_name("  ideia: nova?.txt "), "ideia_ nova_");
    }
}
