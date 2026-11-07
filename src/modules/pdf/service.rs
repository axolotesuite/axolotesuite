//! Operações reais em documentos PDF usando `lopdf`:
//! consultar metadados (páginas, versão, criptografia, tamanho),
//! unir múltiplos PDFs em um único documento e dividir páginas em arquivos individuais.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use lopdf::{Bookmark, Dictionary, Document, Object, ObjectId};

use crate::shared::types::OperationResult;
use crate::shared::ui::human_size;

#[derive(Debug, Clone)]
pub struct PdfMeta {
    pub pages: usize,
    pub version: String,
    pub encrypted: bool,
    pub size_bytes: u64,
    pub objects: usize,
}

/// Lê as informações reais de um arquivo PDF.
pub fn inspect(path: &Path) -> Result<PdfMeta, String> {
    if !path.is_file() {
        return Err("Arquivo PDF não encontrado.".into());
    }
    let size_bytes = fs::metadata(path).map(|m| m.len()).unwrap_or(0);
    let doc = Document::load(path).map_err(|e| format!("Não foi possível ler este PDF: {e}"))?;
    Ok(PdfMeta {
        pages: doc.get_pages().len(),
        version: doc.version.clone(),
        encrypted: doc.is_encrypted(),
        size_bytes,
        objects: doc.objects.len(),
    })
}

/// Une dois ou mais arquivos PDF em um único arquivo `output`.
pub fn merge(output: PathBuf, inputs: Vec<PathBuf>) -> OperationResult {
    if inputs.len() < 2 {
        return OperationResult::err("Adicione pelo menos 2 arquivos PDF para unir.");
    }
    let mut documents = Vec::with_capacity(inputs.len());
    for p in &inputs {
        match Document::load(p) {
            Ok(doc) => documents.push(doc),
            Err(e) => {
                return OperationResult::err(format!(
                    "Erro ao abrir \"{}\": {e}",
                    crate::shared::ui::file_name(p)
                ));
            }
        }
    }

    let mut max_id = 1u32;
    let mut pagenum = 1u32;
    let mut documents_pages: BTreeMap<ObjectId, Object> = BTreeMap::new();
    let mut documents_objects: BTreeMap<ObjectId, Object> = BTreeMap::new();
    let mut document = Document::with_version("1.5");

    for mut doc in documents {
        let mut first = false;
        doc.renumber_objects_with(max_id);
        max_id = doc.max_id + 1;

        for (_, object_id) in doc.get_pages() {
            if !first {
                let b = Bookmark::new(format!("Documento_{pagenum}"), [0.0, 0.0, 1.0], 0, object_id);
                document.add_bookmark(b, None);
                first = true;
                pagenum += 1;
            }
            if let Ok(obj) = doc.get_object(object_id) {
                documents_pages.insert(object_id, obj.to_owned());
            }
        }
        documents_objects.extend(doc.objects);
    }

    let mut catalog_object: Option<(ObjectId, Object)> = None;
    let mut pages_object: Option<(ObjectId, Object)> = None;

    for (object_id, object) in documents_objects.iter() {
        match object.type_name().unwrap_or("") {
            "Catalog" => {
                catalog_object = Some((
                    if let Some((id, _)) = catalog_object { id } else { *object_id },
                    object.clone(),
                ));
            }
            "Pages" => {
                if let Ok(dictionary) = object.as_dict() {
                    let mut dictionary = dictionary.clone();
                    if let Some((_, ref obj)) = pages_object {
                        if let Ok(old_dictionary) = obj.as_dict() {
                            dictionary.extend(old_dictionary);
                        }
                    }
                    pages_object = Some((
                        if let Some((id, _)) = pages_object { id } else { *object_id },
                        Object::Dictionary(dictionary),
                    ));
                }
            }
            "Page" | "Outlines" | "Outline" => {}
            _ => {
                document.objects.insert(*object_id, object.clone());
            }
        }
    }

    let Some((page_id, page_obj)) = pages_object else {
        return OperationResult::err("Estrutura de páginas não encontrada nos PDFs informados.");
    };
    let Some((catalog_id, catalog_obj)) = catalog_object else {
        return OperationResult::err("Catálogo PDF não encontrado nos arquivos informados.");
    };

    for (object_id, object) in documents_pages.iter() {
        if let Ok(dictionary) = object.as_dict() {
            let mut dictionary = dictionary.clone();
            dictionary.set("Parent", page_id);
            document.objects.insert(*object_id, Object::Dictionary(dictionary));
        }
    }

    if let Ok(dictionary) = page_obj.as_dict() {
        let mut dictionary = dictionary.clone();
        dictionary.set("Count", documents_pages.len() as u32);
        dictionary.set(
            "Kids",
            documents_pages
                .keys()
                .map(|&id| Object::Reference(id))
                .collect::<Vec<_>>(),
        );
        document.objects.insert(page_id, Object::Dictionary(dictionary));
    }

    if let Ok(dictionary) = catalog_obj.as_dict() {
        let mut dictionary = dictionary.clone();
        dictionary.set("Pages", page_id);
        dictionary.remove(b"Outlines");
        document.objects.insert(catalog_id, Object::Dictionary(dictionary));
    }

    document.trailer.set("Root", catalog_id);
    document.max_id = document.objects.len() as u32;
    document.renumber_objects();
    document.adjust_zero_pages();
    if let Some(n) = document.build_outline() {
        if let Ok(Object::Dictionary(dict)) = document.get_object_mut(catalog_id) {
            dict.set("Outlines", Object::Reference(n));
        }
    }
    document.compress();

    if let Some(parent) = output.parent() {
        let _ = fs::create_dir_all(parent);
    }
    if let Err(e) = document.save(&output) {
        return OperationResult::err(format!("Erro ao salvar o PDF unido: {e}"));
    }
    let out_size = fs::metadata(&output).map(|m| m.len()).unwrap_or(0);
    let total_pages = documents_pages.len();
    OperationResult::ok(format!(
        "{} PDFs unidos em um documento com {total_pages} página(s) ({})",
        inputs.len(),
        human_size(out_size)
    ))
    .with_detail(format!("Arquivo salvo em: {}", output.display()))
    .with_output(output)
}

/// Divide cada página (ou um intervalo opcional `start..=end`) em um arquivo PDF separado.
pub fn split(pdf_path: PathBuf, out_dir: PathBuf, range: Option<(u32, u32)>) -> OperationResult {
    let base_doc = match Document::load(&pdf_path) {
        Ok(d) => d,
        Err(e) => return OperationResult::err(format!("Erro ao abrir o PDF: {e}")),
    };
    let all_pages: Vec<u32> = base_doc.get_pages().keys().copied().collect();
    if all_pages.is_empty() {
        return OperationResult::err("Este PDF não possui páginas legíveis.");
    }
    let total = all_pages.len() as u32;
    let (from, to) = range.unwrap_or((1, total));
    if from == 0 || from > to || to > total {
        return OperationResult::err(format!(
            "Intervalo inválido ({from} a {to}). O documento tem {total} página(s)."
        ));
    }

    if let Err(e) = fs::create_dir_all(&out_dir) {
        return OperationResult::err(format!("Erro ao criar a pasta de saída: {e}"));
    }

    let stem = pdf_path
        .file_stem()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_else(|| "pagina".into());
    let digits = total.to_string().len().max(2);
    let mut created = 0usize;
    let mut details = Vec::new();

    for page_num in from..=to {
        let mut doc = base_doc.clone();
        let to_delete: Vec<u32> = all_pages.iter().copied().filter(|&p| p != page_num).collect();
        doc.delete_pages(&to_delete);
        doc.prune_objects();
        doc.renumber_objects();
        doc.adjust_zero_pages();
        doc.compress();

        let file_name = format!("{stem}_pagina_{page_num:0digits$}.pdf");
        let target = out_dir.join(&file_name);
        match doc.save(&target) {
            Ok(_) => {
                created += 1;
                details.push(format!("Página {page_num} → {file_name}"));
            }
            Err(e) => details.push(format!("Erro na página {page_num}: {e}")),
        }
    }

    let _ = Dictionary::new(); // garante import limpo
    OperationResult::ok(format!("{created} página(s) extraída(s) em arquivos PDF individuais."))
        .with_details(details)
        .with_output(out_dir)
}
