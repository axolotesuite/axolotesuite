//! Motor da Matriz Cristalina Axolote (Conversão Inteligente por Gesto / Atalho Ctrl+Alt+N):
//! - Design 100% autoral Axolote Hex-Core: Núcleo Hexagonal Neon + Cristais dinâmicos (1 a 8 destinos reais)
//! - Destinos inteligentes por extensão real do arquivo (idêntico à filosofia de preservação:
//!   nunca exibe o mesmo formato de origem e sempre oferece ZIP para arquivos não-ZIP)
//! - Preservação total do arquivo original (somente leitura)
//! - Sem sobrescrita silenciosa: gera `arquivo.ext`, `arquivo (1).ext`, `arquivo (2).ext` ao lado do original
//! - Finalização atômica: grava em arquivo temporário `.axolote_tmp` e renomeia ao concluir (limpa em caso de erro)
//! - Motores reais 100% locais:
//!   1. Imagens (PNG, JPG, WEBP, BMP, GIF, TIFF, ICO, PDF)
//!   2. Texto / Markdown / HTML (TXT, MD, HTML, DOCX, PDF)
//!   3. Documentos (DOCX ↔ PDF, HTML, TXT, MD)
//!   4. Vídeo & Áudio via FFmpeg (MP4, WEBM, MKV, GIF animado, MOV, AVI, MP3, WAV, FLAC, OGG, AAC)
//!   5. Compactação ZIP universal (qualquer arquivo → ZIP e ZIP → EXTRAIR).

use std::fs::{self, File};
use std::io::{Cursor, Read, Write};
use std::path::{Path, PathBuf};

use image::{DynamicImage, GenericImageView, ImageFormat};
use lopdf::content::{Content, Operation};
use lopdf::{dictionary, Document, Object, Stream};
use zip::write::FileOptions;
use zip::{CompressionMethod, ZipArchive, ZipWriter};

use crate::core::system;
use crate::shared::types::OperationResult;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WheelCategory {
    Image,
    Document,
    Video,
    Audio,
    Archive,
}

impl WheelCategory {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Image => "🖼  Imagens",
            Self::Document => "📄  Texto & Docs (TXT/MD/HTML/DOCX/PDF)",
            Self::Video => "🎬  Vídeo & GIF",
            Self::Audio => "🎵  Áudio",
            Self::Archive => "🗜  Compactação ZIP",
        }
    }

    pub fn default_source_ext(&self) -> &'static str {
        match self {
            Self::Image => "PNG",
            Self::Document => "DOCX",
            Self::Video => "MP4",
            Self::Audio => "MP3",
            Self::Archive => "ARQUIVO",
        }
    }

    pub fn from_extension(ext: &str) -> Self {
        match ext.trim().trim_start_matches('.').to_ascii_lowercase().as_str() {
            "png" | "jpg" | "jpeg" | "webp" | "bmp" | "gif" | "tiff" | "tif" | "ico" => Self::Image,
            "txt" | "md" | "markdown" | "html" | "htm" | "docx" | "pdf" => Self::Document,
            "mp4" | "mkv" | "webm" | "mov" | "avi" | "wmv" | "flv" | "m4v" | "mpg" | "mpeg" => Self::Video,
            "mp3" | "wav" | "flac" | "aac" | "ogg" | "m4a" | "opus" | "wma" => Self::Audio,
            _ => Self::Archive,
        }
    }
}

/// Retorna SOMENTE as conversões reais suportadas para a extensão do arquivo selecionado:
/// - Nunca oferece o mesmo formato de origem
/// - Sempre oferece `ZIP` para qualquer arquivo que não seja `.zip`
/// - Para `.zip`, oferece `EXTRAIR` (descompactação segura anti Zip-Slip).
pub fn targets_for_extension(ext: &str, ffmpeg_available: bool) -> Vec<&'static str> {
    let clean = ext.trim().trim_start_matches('.').to_ascii_uppercase();
    let norm = match clean.as_str() {
        "JPEG" => "JPG",
        "TIF" => "TIFF",
        "HTM" => "HTML",
        "MARKDOWN" => "MD",
        "MPEG" => "MPG",
        other => other,
    };

    match norm {
        // Imagens -> demais formatos de imagem + PDF + ZIP
        "PNG" | "JPG" | "WEBP" | "BMP" | "GIF" | "TIFF" | "ICO" => {
            let all = ["PNG", "JPG", "WEBP", "PDF", "GIF", "BMP", "TIFF", "ICO", "ZIP"];
            all.into_iter()
                .filter(|&f| f != norm)
                .take(8)
                .collect()
        }
        // TXT -> MD, HTML, DOCX, PDF + ZIP
        "TXT" => vec!["MD", "HTML", "DOCX", "PDF", "ZIP"],
        // MD/Markdown -> TXT, HTML, DOCX, PDF + ZIP
        "MD" => vec!["TXT", "HTML", "DOCX", "PDF", "ZIP"],
        // HTML/HTM -> TXT, MD, DOCX, PDF + ZIP
        "HTML" => vec!["TXT", "MD", "DOCX", "PDF", "ZIP"],
        // DOCX -> PDF, HTML, TXT, MD + ZIP
        "DOCX" => vec!["PDF", "HTML", "TXT", "MD", "ZIP"],
        // PDF -> DOCX, HTML, TXT, MD + ZIP
        "PDF" => vec!["DOCX", "HTML", "TXT", "MD", "ZIP"],
        // Vídeos -> MP4, WEBM, MKV, GIF animado, MP3 + ZIP (ou apenas ZIP se sem FFmpeg)
        "MP4" | "MKV" | "WEBM" | "AVI" | "MOV" | "WMV" | "FLV" | "M4V" | "MPG" => {
            if !ffmpeg_available {
                return vec!["ZIP"];
            }
            let all = ["MP4", "WEBM", "MKV", "GIF", "MOV", "AVI", "MP3", "ZIP"];
            all.into_iter().filter(|&f| f != norm).take(8).collect()
        }
        // Áudios -> MP3, WAV, FLAC, OGG, AAC, M4A + ZIP
        "MP3" | "WAV" | "FLAC" | "AAC" | "OGG" | "M4A" | "OPUS" | "WMA" => {
            if !ffmpeg_available {
                return vec!["ZIP"];
            }
            let all = ["MP3", "WAV", "FLAC", "OGG", "AAC", "M4A", "ZIP"];
            all.into_iter().filter(|&f| f != norm).take(8).collect()
        }
        // ZIP -> EXTRAIR pasta segura
        "ZIP" => vec!["EXTRAIR"],
        // Qualquer outro formato (DOC, ODT, RTF, CSV, JSON, XML, etc.) -> ZIP
        _ => vec!["ZIP"],
    }
}

/// Gera um caminho de saída ao lado do arquivo original (ou em `custom_dir`)
/// SEM sobrescrever arquivos existentes: adiciona `(1)`, `(2)`, `(3)`...
pub fn unique_output_path(src: &Path, target_ext: &str, custom_dir: Option<&Path>) -> PathBuf {
    let parent = custom_dir
        .or_else(|| src.parent())
        .unwrap_or_else(|| Path::new("."));
    let stem = src
        .file_stem()
        .and_then(|s| s.to_str())
        .filter(|s| !s.is_empty())
        .unwrap_or("arquivo");
    let ext_lower = target_ext.trim().trim_start_matches('.').to_ascii_lowercase();

    let first = parent.join(format!("{stem}.{ext_lower}"));
    if !first.exists() && first != src {
        return first;
    }

    for i in 1..10_000 {
        let candidate = parent.join(format!("{stem} ({i}).{ext_lower}"));
        if !candidate.exists() && candidate != src {
            return candidate;
        }
    }
    parent.join(format!("{stem}_convertido.{ext_lower}"))
}

/// Gera um caminho de pasta único sem sobrescrever (`nome (1)`, `nome (2)`...).
fn unique_output_dir(src: &Path, custom_dir: Option<&Path>) -> PathBuf {
    let parent = custom_dir
        .or_else(|| src.parent())
        .unwrap_or_else(|| Path::new("."));
    let stem = src
        .file_stem()
        .and_then(|s| s.to_str())
        .filter(|s| !s.is_empty())
        .unwrap_or("extraido");

    let first = parent.join(stem);
    if !first.exists() {
        return first;
    }
    for i in 1..10_000 {
        let candidate = parent.join(format!("{stem} ({i})"));
        if !candidate.exists() {
            return candidate;
        }
    }
    parent.join(format!("{stem}_extraido"))
}

/// Cria o caminho de arquivo temporário ao lado do destino final para escrita atômica.
fn temp_path_for(final_path: &Path) -> PathBuf {
    let name = final_path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("output");
    let pid = std::process::id();
    final_path.with_file_name(format!(".{name}.{pid}.axolote_tmp"))
}

/// Grava bytes de forma atômica: escreve no `.axolote_tmp` e renomeia para `final_path`.
/// Se falhar, remove o arquivo temporário.
fn write_bytes_atomic(final_path: &Path, bytes: &[u8]) -> Result<(), String> {
    let tmp = temp_path_for(final_path);
    if let Err(e) = fs::write(&tmp, bytes) {
        let _ = fs::remove_file(&tmp);
        return Err(format!("Falha ao gravar arquivo temporário: {e}"));
    }
    if let Err(e) = fs::rename(&tmp, final_path) {
        let _ = fs::remove_file(&tmp);
        return Err(format!("Falha ao finalizar arquivo {}: {e}", final_path.display()));
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Execução da Conversão com Preservação Atômica e Sufixos (1), (2)...
// ---------------------------------------------------------------------------

pub fn execute_wheel_conversion(
    files: Vec<PathBuf>,
    category: WheelCategory,
    target_format: String,
    custom_out_dir: Option<PathBuf>,
    strip_metadata: bool,
) -> OperationResult {
    if files.is_empty() {
        return OperationResult::err("Nenhum arquivo selecionado na Matriz").with_details(vec![
            "Segure Ctrl + Alt e arraste um arquivo do Windows Explorer (ou clique em 'Adicionar Arquivos').".into(),
        ]);
    }

    let fmt = target_format.trim().to_ascii_uppercase();
    let custom_dir_ref = custom_out_dir.as_deref();
    if let Some(d) = custom_dir_ref {
        let _ = fs::create_dir_all(d);
    }

    // 1. Se o destino escolhido for ZIP (disponível universalmente para qualquer arquivo não-ZIP)
    if fmt == "ZIP" {
        return compress_files_to_zip(&files, custom_dir_ref);
    }

    // 2. Se a origem for um .zip e o destino for EXTRAIR
    if fmt == "EXTRAIR" {
        return extract_zip_archives(&files, custom_dir_ref);
    }

    // 3. Caso Imagens -> PDF multi-página
    if category == WheelCategory::Image && fmt == "PDF" {
        return convert_images_to_pdf(&files, custom_dir_ref);
    }

    match category {
        WheelCategory::Image => convert_images_batch(&files, &fmt, custom_dir_ref, strip_metadata),
        WheelCategory::Document => convert_documents_batch(&files, &fmt, custom_dir_ref),
        WheelCategory::Video | WheelCategory::Audio => convert_media_batch(&files, &fmt, custom_dir_ref),
        WheelCategory::Archive => compress_files_to_zip(&files, custom_dir_ref),
    }
}

// ---------------------------------------------------------------------------
// Engine 1: Compactação Universal ZIP e Extração Segura (Anti Zip-Slip)
// ---------------------------------------------------------------------------

fn compress_files_to_zip(files: &[PathBuf], custom_dir: Option<&Path>) -> OperationResult {
    let out_zip = unique_output_path(&files[0], "zip", custom_dir);
    let tmp_zip = temp_path_for(&out_zip);

    let build_res = (|| -> Result<Vec<String>, String> {
        let f = File::create(&tmp_zip).map_err(|e| format!("Erro ao criar temporário ZIP: {e}"))?;
        let mut zw = ZipWriter::new(f);
        let opts = FileOptions::default()
            .compression_method(CompressionMethod::Deflated)
            .unix_permissions(0o644);
        let mut details = Vec::new();

        for src in files {
            if !src.is_file() {
                continue;
            }
            let name = src
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("arquivo");
            zw.start_file(name, opts)
                .map_err(|e| format!("Erro ao iniciar entrada ZIP ({name}): {e}"))?;
            let data = fs::read(src).map_err(|e| format!("Erro ao ler {}: {e}", src.display()))?;
            zw.write_all(&data)
                .map_err(|e| format!("Erro ao comprimir {name}: {e}"))?;
            details.push(format!("✓ Adicionado ao pacote (original intacto): {}", src.display()));
        }
        zw.finish().map_err(|e| format!("Erro ao finalizar ZIP: {e}"))?;
        Ok(details)
    })();

    match build_res {
        Ok(mut details) => {
            if let Err(e) = fs::rename(&tmp_zip, &out_zip) {
                let _ = fs::remove_file(&tmp_zip);
                return OperationResult::err("Falha ao renomear pacote ZIP final").with_detail(e.to_string());
            }
            details.push(format!("🗜 Pacote salvo ao lado do original: {}", out_zip.display()));
            OperationResult::ok(format!("Compactado com sucesso em {}!", out_zip.display()))
                .with_details(details)
                .with_output(out_zip)
        }
        Err(e) => {
            let _ = fs::remove_file(&tmp_zip);
            OperationResult::err("Erro ao compactar para ZIP").with_detail(e)
        }
    }
}

fn extract_zip_archives(files: &[PathBuf], custom_dir: Option<&Path>) -> OperationResult {
    let mut details = Vec::new();
    let mut last_dir = None;
    let mut ok_count = 0usize;

    for src in files {
        let Ok(file) = File::open(src) else {
            details.push(format!("✖ Não foi possível abrir {}", src.display()));
            continue;
        };
        let Ok(mut archive) = ZipArchive::new(file) else {
            details.push(format!("✖ Arquivo ZIP inválido: {}", src.display()));
            continue;
        };
        let dest_dir = unique_output_dir(src, custom_dir);
        let _ = fs::create_dir_all(&dest_dir);

        let mut extracted_here = 0usize;
        for i in 0..archive.len() {
            let Ok(mut entry) = archive.by_index(i) else { continue };
            let Some(safe_rel) = entry.enclosed_name().map(|p| p.to_path_buf()) else {
                continue;
            };
            let out_path = dest_dir.join(safe_rel);
            if entry.is_dir() {
                let _ = fs::create_dir_all(&out_path);
            } else {
                if let Some(p) = out_path.parent() {
                    let _ = fs::create_dir_all(p);
                }
                if let Ok(mut out_f) = File::create(&out_path) {
                    let _ = std::io::copy(&mut entry, &mut out_f);
                    extracted_here += 1;
                }
            }
        }
        ok_count += 1;
        details.push(format!(
            "✓ {} ({} itens extraídos) → {}",
            src.display(),
            extracted_here,
            dest_dir.display()
        ));
        last_dir = Some(dest_dir);
    }

    if ok_count == 0 {
        OperationResult::err("Não foi possível extrair o arquivo ZIP").with_details(details)
    } else {
        OperationResult::ok("Arquivo(s) ZIP extraído(s) sem alterar o original!")
            .with_details(details)
            .with_output(last_dir.unwrap_or_else(|| files[0].clone()))
    }
}

// ---------------------------------------------------------------------------
// Engine 2: Imagens (PNG, JPG, WEBP, BMP, TIFF, GIF, ICO, PDF) com Escrita Atômica
// ---------------------------------------------------------------------------

fn convert_images_batch(
    files: &[PathBuf],
    fmt: &str,
    custom_dir: Option<&Path>,
    strip_metadata: bool,
) -> OperationResult {
    let ext = fmt.to_ascii_lowercase();
    let img_fmt = match ext.as_str() {
        "png" => ImageFormat::Png,
        "jpg" | "jpeg" => ImageFormat::Jpeg,
        "webp" => ImageFormat::WebP,
        "bmp" => ImageFormat::Bmp,
        "tiff" | "tif" => ImageFormat::Tiff,
        "gif" => ImageFormat::Gif,
        "ico" => ImageFormat::Ico,
        _ => ImageFormat::Png,
    };

    let mut details = Vec::new();
    let mut ok_count = 0usize;
    let mut last_out = None;

    for src in files {
        match image::open(src) {
            Ok(img) => {
                let out_path = unique_output_path(src, &ext, custom_dir);
                let final_img = if img_fmt == ImageFormat::Ico {
                    let (w, h) = img.dimensions();
                    if w > 256 || h > 256 {
                        img.resize(256, 256, image::imageops::FilterType::Lanczos3)
                    } else {
                        img
                    }
                } else if strip_metadata {
                    DynamicImage::ImageRgba8(img.to_rgba8())
                } else {
                    img
                };

                // Codifica em memória primeiro e grava atomicamente via .axolote_tmp -> rename
                let mut mem = Cursor::new(Vec::new());
                match final_img.write_to(&mut mem, img_fmt) {
                    Ok(_) => match write_bytes_atomic(&out_path, &mem.into_inner()) {
                        Ok(_) => {
                            ok_count += 1;
                            details.push(format!("✓ {} → {}", src.display(), out_path.display()));
                            last_out = Some(out_path);
                        }
                        Err(e) => details.push(format!("✖ {e}")),
                    },
                    Err(e) => details.push(format!("✖ Erro ao codificar {}: {e}", src.display())),
                }
            }
            Err(e) => details.push(format!("✖ Não foi possível ler {}: {e}", src.display())),
        }
    }

    if ok_count == 0 {
        OperationResult::err("Falha na conversão de imagens").with_details(details)
    } else {
        OperationResult::ok(format!(
            "{ok_count} de {} imagem(ns) convertida(s) para {fmt} (original preservado)!",
            files.len()
        ))
        .with_details(details)
        .with_output(last_out.unwrap_or_else(|| files[0].clone()))
    }
}

fn convert_images_to_pdf(files: &[PathBuf], custom_dir: Option<&Path>) -> OperationResult {
    let mut doc = Document::with_version("1.5");
    let pages_id = doc.new_object_id();
    let mut page_ids = Vec::new();
    let mut details = Vec::new();

    for src in files {
        let Ok(img) = image::open(src) else {
            details.push(format!("✖ Ignorado (não é imagem suportada): {}", src.display()));
            continue;
        };
        let rgb = img.to_rgb8();
        let (w, h) = rgb.dimensions();
        let mut jpeg_bytes = Vec::new();
        let mut encoder = image::codecs::jpeg::JpegEncoder::new_with_quality(&mut jpeg_bytes, 92);
        if encoder
            .encode(rgb.as_raw(), w, h, image::ExtendedColorType::Rgb8)
            .is_err()
        {
            continue;
        }

        let img_stream = Stream::new(
            dictionary! {
                "Type" => "XObject",
                "Subtype" => "Image",
                "Width" => w as i64,
                "Height" => h as i64,
                "ColorSpace" => "DeviceRGB",
                "BitsPerComponent" => 8,
                "Filter" => "DCTDecode",
            },
            jpeg_bytes,
        );
        let img_id = doc.add_object(img_stream);

        let page_w = 595.0f64;
        let page_h = (595.0 * (h as f64 / w.max(1) as f64)).clamp(200.0, 1200.0);

        let content = Content {
            operations: vec![
                Operation::new("q", vec![]),
                Operation::new(
                    "cm",
                    vec![
                        page_w.into(),
                        0.into(),
                        0.into(),
                        page_h.into(),
                        0.into(),
                        0.into(),
                    ],
                ),
                Operation::new("Do", vec![Object::Name(b"Im0".to_vec())]),
                Operation::new("Q", vec![]),
            ],
        };
        let content_id = doc.add_object(Stream::new(dictionary! {}, content.encode().unwrap_or_default()));

        let resources_id = doc.add_object(dictionary! {
            "XObject" => dictionary! {
                "Im0" => img_id,
            },
        });

        let page_id = doc.add_object(dictionary! {
            "Type" => "Page",
            "Parent" => pages_id,
            "MediaBox" => vec![0.into(), 0.into(), page_w.into(), page_h.into()],
            "Contents" => content_id,
            "Resources" => resources_id,
        });
        page_ids.push(Object::Reference(page_id));
        details.push(format!("✓ Página adicionada ao PDF: {} ({w}×{h})", src.display()));
    }

    if page_ids.is_empty() {
        return OperationResult::err("Nenhuma imagem válida pôde ser convertida em PDF").with_details(details);
    }

    let count = page_ids.len() as i64;
    doc.objects.insert(
        pages_id,
        Object::Dictionary(dictionary! {
            "Type" => "Pages",
            "Kids" => page_ids,
            "Count" => count,
        }),
    );
    let catalog_id = doc.add_object(dictionary! {
        "Type" => "Catalog",
        "Pages" => pages_id,
    });
    doc.trailer.set("Root", catalog_id);

    let out_pdf = unique_output_path(&files[0], "pdf", custom_dir);
    let mut buffer = Cursor::new(Vec::new());
    if let Err(e) = doc.save_to(&mut buffer) {
        return OperationResult::err("Erro ao montar PDF em memória").with_detail(e.to_string());
    }
    if let Err(e) = write_bytes_atomic(&out_pdf, &buffer.into_inner()) {
        return OperationResult::err("Erro ao salvar PDF final").with_detail(e);
    }

    details.push(format!("📄 Arquivo PDF salvo ao lado do original: {}", out_pdf.display()));
    OperationResult::ok(format!("PDF gerado com {count} página(s)!"))
        .with_details(details)
        .with_output(out_pdf)
}

// ---------------------------------------------------------------------------
// Engine 3: Texto, Markdown, HTML, DOCX e PDF (Estilo ShiftConvert!)
// ---------------------------------------------------------------------------

fn convert_documents_batch(
    files: &[PathBuf],
    target_fmt: &str,
    custom_dir: Option<&Path>,
) -> OperationResult {
    let ext = target_fmt.to_ascii_lowercase();
    let mut details = Vec::new();
    let mut ok_count = 0usize;
    let mut last_out = None;

    for src in files {
        let src_ext = src
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
            .to_ascii_lowercase();

        // 1. Extrai o texto puro de qualquer formato de entrada (TXT, MD, HTML, DOCX, PDF)
        let plain_text = match extract_text_from_document(src, &src_ext) {
            Ok(t) => t,
            Err(e) => {
                details.push(format!("✖ {}: {e}", src.display()));
                continue;
            }
        };

        let out_path = unique_output_path(src, &ext, custom_dir);
        let title = src.file_stem().and_then(|s| s.to_str()).unwrap_or("Documento");

        // 2. Converte o texto para o formato de destino (TXT, MD, HTML, DOCX, PDF)
        let write_res = match ext.as_str() {
            "txt" => write_bytes_atomic(&out_path, plain_text.as_bytes()),
            "md" => {
                let md = if src_ext == "html" || src_ext == "htm" {
                    html_to_markdown(&plain_text)
                } else {
                    format!("# {title}\n\n{plain_text}")
                };
                write_bytes_atomic(&out_path, md.as_bytes())
            }
            "html" => {
                let html = render_clean_html(title, &plain_text, src_ext == "md" || src_ext == "markdown");
                write_bytes_atomic(&out_path, html.as_bytes())
            }
            "docx" => build_docx_bytes(&plain_text).and_then(|b| write_bytes_atomic(&out_path, &b)),
            "pdf" => build_text_pdf_bytes(&plain_text).and_then(|b| write_bytes_atomic(&out_path, &b)),
            _ => Err(format!("Formato de documento não suportado: {ext}")),
        };

        match write_res {
            Ok(_) => {
                ok_count += 1;
                details.push(format!("✓ {} → {}", src.display(), out_path.display()));
                last_out = Some(out_path);
            }
            Err(e) => details.push(format!("✖ Falha ao converter {}: {e}", src.display())),
        }
    }

    if ok_count == 0 {
        OperationResult::err("Não foi possível converter o(s) documento(s)").with_details(details)
    } else {
        OperationResult::ok(format!(
            "{ok_count} de {} documento(s) convertido(s) para {target_fmt}!",
            files.len()
        ))
        .with_details(details)
        .with_output(last_out.unwrap_or_else(|| files[0].clone()))
    }
}

fn extract_text_from_document(path: &Path, src_ext: &str) -> Result<String, String> {
    match src_ext {
        "txt" | "md" | "markdown" => {
            fs::read_to_string(path).map_err(|e| format!("Erro ao ler texto: {e}"))
        }
        "html" | "htm" => {
            let raw = fs::read_to_string(path).map_err(|e| format!("Erro ao ler HTML: {e}"))?;
            Ok(strip_html_tags(&raw))
        }
        "docx" => extract_docx_text(path),
        "pdf" => extract_pdf_text(path),
        _ => fs::read_to_string(path).map_err(|e| format!("Formato não textual: {e}")),
    }
}

fn strip_html_tags(html: &str) -> String {
    let mut out = String::with_capacity(html.len());
    let mut in_tag = false;
    let mut tag_buf = String::new();

    for ch in html.chars() {
        if ch == '<' {
            in_tag = true;
            tag_buf.clear();
        } else if ch == '>' {
            in_tag = false;
            let t = tag_buf.trim().to_ascii_lowercase();
            if t.starts_with("p") || t.starts_with("/p") || t.starts_with("br") || t.starts_with("div") || t.starts_with("li") || t.starts_with("h1") || t.starts_with("h2") || t.starts_with("h3") {
                out.push('\n');
            }
        } else if in_tag {
            tag_buf.push(ch);
        } else {
            out.push(ch);
        }
    }
    out.replace("&nbsp;", " ")
        .replace("&amp;", "&")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .collect::<Vec<_>>()
        .join("\n\n")
}

fn html_to_markdown(text: &str) -> String {
    text.to_string()
}

fn render_clean_html(title: &str, body: &str, from_md: bool) -> String {
    let mut html_body = String::new();
    for line in body.lines() {
        let t = line.trim();
        if t.is_empty() {
            continue;
        }
        if from_md && t.starts_with("### ") {
            html_body.push_str(&format!("<h3>{}</h3>\n", escape_xml(&t[4..])));
        } else if from_md && t.starts_with("## ") {
            html_body.push_str(&format!("<h2>{}</h2>\n", escape_xml(&t[3..])));
        } else if from_md && t.starts_with("# ") {
            html_body.push_str(&format!("<h1>{}</h1>\n", escape_xml(&t[2..])));
        } else if from_md && (t.starts_with("- ") || t.starts_with("* ")) {
            html_body.push_str(&format!("<li>{}</li>\n", escape_xml(&t[2..])));
        } else {
            html_body.push_str(&format!("<p>{}</p>\n", escape_xml(t)));
        }
    }
    format!(
        "<!DOCTYPE html>\n<html lang=\"pt-BR\">\n<head>\n<meta charset=\"utf-8\">\n<title>{}</title>\n<style>body{{font-family:Segoe UI,system-ui,sans-serif;max-width:780px;margin:40px auto;padding:0 20px;line-height:1.6;color:#1e293b;}}</style>\n</head>\n<body>\n{}\n</body>\n</html>\n",
        escape_xml(title),
        html_body
    )
}

fn extract_docx_text(path: &Path) -> Result<String, String> {
    let file = File::open(path).map_err(|e| format!("Não foi possível abrir DOCX: {e}"))?;
    let mut zip = ZipArchive::new(file).map_err(|e| format!("DOCX inválido: {e}"))?;
    let mut doc_xml = String::new();
    zip.by_name("word/document.xml")
        .map_err(|_| "word/document.xml não encontrado no DOCX".to_string())?
        .read_to_string(&mut doc_xml)
        .map_err(|e| format!("Erro ao ler XML do DOCX: {e}"))?;

    let mut out = String::new();
    let mut in_tag = false;
    let mut tag = String::new();
    for ch in doc_xml.chars() {
        if ch == '<' {
            in_tag = true;
            tag.clear();
        } else if ch == '>' {
            in_tag = false;
            if tag == "/w:p" || tag.starts_with("w:br") {
                out.push('\n');
            }
        } else if in_tag {
            tag.push(ch);
        } else {
            out.push(ch);
        }
    }
    Ok(out
        .replace("&amp;", "&")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .trim()
        .to_string())
}

fn extract_pdf_text(path: &Path) -> Result<String, String> {
    let doc = Document::load(path).map_err(|e| format!("Não foi possível abrir PDF: {e}"))?;
    let pages: Vec<u32> = doc.get_pages().keys().copied().collect();
    if pages.is_empty() {
        return Err("O PDF não possui páginas legíveis.".into());
    }
    let text = doc
        .extract_text(&pages)
        .map_err(|e| format!("Erro ao extrair texto do PDF: {e}"))?;
    if text.trim().is_empty() {
        return Err("Este PDF parece ser digitalizado (imagem) e não contém texto selecionável (sem OCR).".into());
    }
    Ok(text)
}

fn escape_xml(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

/// Gera um arquivo `.docx` OpenXML válido em memória a partir de texto puro.
fn build_docx_bytes(text: &str) -> Result<Vec<u8>, String> {
    let mut buf = Cursor::new(Vec::new());
    {
        let mut zw = ZipWriter::new(&mut buf);
        let opts = FileOptions::default().compression_method(CompressionMethod::Deflated);

        let content_types = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types">
  <Default Extension="rels" ContentType="application/vnd.openxmlformats-package.relationships+xml"/>
  <Default Extension="xml" ContentType="application/xml"/>
  <Override PartName="/word/document.xml" ContentType="application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml"/>
</Types>"#;
        zw.start_file("[Content_Types].xml", opts).map_err(|e| e.to_string())?;
        zw.write_all(content_types.as_bytes()).map_err(|e| e.to_string())?;

        let rels = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
  <Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument" Target="word/document.xml"/>
</Relationships>"#;
        zw.start_file("_rels/.rels", opts).map_err(|e| e.to_string())?;
        zw.write_all(rels.as_bytes()).map_err(|e| e.to_string())?;

        let mut paragraphs = String::new();
        for line in text.lines() {
            let escaped = escape_xml(line);
            paragraphs.push_str(&format!(
                "<w:p><w:r><w:t xml:space=\"preserve\">{escaped}</w:t></w:r></w:p>"
            ));
        }

        let doc_xml = format!(
            "<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?>\
            <w:document xmlns:w=\"http://schemas.openxmlformats.org/wordprocessingml/2006/main\">\
            <w:body>{paragraphs}</w:body></w:document>"
        );
        zw.start_file("word/document.xml", opts).map_err(|e| e.to_string())?;
        zw.write_all(doc_xml.as_bytes()).map_err(|e| e.to_string())?;
        zw.finish().map_err(|e| e.to_string())?;
    }
    Ok(buf.into_inner())
}

/// Gera um arquivo PDF textual multi-página em memória usando `lopdf` e fonte padrão Helvetica.
fn build_text_pdf_bytes(text: &str) -> Result<Vec<u8>, String> {
    let mut doc = Document::with_version("1.5");
    let pages_id = doc.new_object_id();
    let font_id = doc.add_object(dictionary! {
        "Type" => "Font",
        "Subtype" => "Type1",
        "BaseFont" => "Helvetica",
        "Encoding" => "WinAnsiEncoding",
    });
    let resources_id = doc.add_object(dictionary! {
        "Font" => dictionary! {
            "F1" => font_id,
        },
    });

    // Quebra linhas longas em até 82 colunas e agrupa em páginas de 48 linhas
    let mut wrapped_lines = Vec::new();
    for raw_line in text.lines() {
        if raw_line.trim().is_empty() {
            wrapped_lines.push(String::new());
            continue;
        }
        let mut cur = String::new();
        for word in raw_line.split_whitespace() {
            if cur.len() + word.len() + 1 > 82 && !cur.is_empty() {
                wrapped_lines.push(cur.clone());
                cur.clear();
            }
            if !cur.is_empty() {
                cur.push(' ');
            }
            cur.push_str(word);
        }
        if !cur.is_empty() {
            wrapped_lines.push(cur);
        }
    }
    if wrapped_lines.is_empty() {
        wrapped_lines.push(" ".into());
    }

    let mut page_ids = Vec::new();
    for chunk in wrapped_lines.chunks(48) {
        let mut ops = vec![
            Operation::new("BT", vec![]),
            Operation::new("Tf", vec!["F1".into(), 11.into()]),
            Operation::new("Td", vec![50.into(), 785.into()]),
            Operation::new("TL", vec![15.into()]),
        ];
        for line in chunk {
            let win_ansi: Vec<u8> = line
                .chars()
                .map(|c| if (c as u32) <= 0xFF { c as u8 } else { b'?' })
                .collect();
            ops.push(Operation::new("Tj", vec![Object::String(win_ansi, lopdf::StringFormat::Literal)]));
            ops.push(Operation::new("T*", vec![]));
        }
        ops.push(Operation::new("ET", vec![]));

        let content = Content { operations: ops };
        let content_id = doc.add_object(Stream::new(dictionary! {}, content.encode().unwrap_or_default()));
        let page_id = doc.add_object(dictionary! {
            "Type" => "Page",
            "Parent" => pages_id,
            "MediaBox" => vec![0.into(), 0.into(), 595.into(), 842.into()],
            "Contents" => content_id,
            "Resources" => resources_id,
        });
        page_ids.push(Object::Reference(page_id));
    }

    let count = page_ids.len() as i64;
    doc.objects.insert(
        pages_id,
        Object::Dictionary(dictionary! {
            "Type" => "Pages",
            "Kids" => page_ids,
            "Count" => count,
        }),
    );
    let catalog_id = doc.add_object(dictionary! {
        "Type" => "Catalog",
        "Pages" => pages_id,
    });
    doc.trailer.set("Root", catalog_id);

    let mut buffer = Cursor::new(Vec::new());
    doc.save_to(&mut buffer).map_err(|e| e.to_string())?;
    Ok(buffer.into_inner())
}

// ---------------------------------------------------------------------------
// Engine 4: Vídeo, GIF Animado e Áudio via FFmpeg com Escrita Atômica (.tmp -> rename)
// ---------------------------------------------------------------------------

fn convert_media_batch(
    files: &[PathBuf],
    fmt: &str,
    custom_dir: Option<&Path>,
) -> OperationResult {
    let Some(ffmpeg) = system::find_ffmpeg() else {
        return OperationResult::err("FFmpeg não encontrado")
            .with_detail("Sem FFmpeg, arquivos de vídeo oferecem apenas ZIP. Vá em 'Instalar / Setup' para instalar o FFmpeg incluído.");
    };

    let ext = fmt.to_ascii_lowercase();
    let mut details = Vec::new();
    let mut ok_count = 0usize;
    let mut last_out = None;

    for src in files {
        let out_path = unique_output_path(src, &ext, custom_dir);
        // O FFmpeg precisa da extensão final para detectar o container, então colocamos .axolote_tmp ANTES da extensão
        let stem = out_path.file_stem().and_then(|s| s.to_str()).unwrap_or("midia");
        let tmp_path = out_path.with_file_name(format!(".{stem}.axolote_tmp.{ext}"));

        let mut cmd = system::hidden_command(&ffmpeg);
        cmd.arg("-y").arg("-i").arg(src);

        match ext.as_str() {
            "gif" => {
                cmd.args(["-vf", "fps=12,scale=540:-1:flags=lanczos", "-loop", "0"]);
            }
            "mp3" => {
                cmd.args(["-vn", "-c:a", "libmp3lame", "-b:a", "192k"]);
            }
            "wav" => {
                cmd.args(["-vn", "-c:a", "pcm_s16le"]);
            }
            "flac" => {
                cmd.args(["-vn", "-c:a", "flac"]);
            }
            "aac" | "m4a" => {
                cmd.args(["-vn", "-c:a", "aac", "-b:a", "192k"]);
            }
            "ogg" | "opus" => {
                cmd.args(["-vn", "-b:a", "160k"]);
            }
            _ => {}
        }
        cmd.arg(&tmp_path);

        match cmd.output() {
            Ok(o) if o.status.success() && tmp_path.is_file() => {
                match fs::rename(&tmp_path, &out_path) {
                    Ok(_) => {
                        ok_count += 1;
                        details.push(format!("✓ {} → {}", src.display(), out_path.display()));
                        last_out = Some(out_path);
                    }
                    Err(e) => {
                        let _ = fs::remove_file(&tmp_path);
                        details.push(format!("✖ Erro ao finalizar {}: {e}", out_path.display()));
                    }
                }
            }
            Ok(o) => {
                let _ = fs::remove_file(&tmp_path);
                let err = String::from_utf8_lossy(&o.stderr);
                details.push(format!("✖ Falha em {}: {}", src.display(), err.lines().last().unwrap_or("erro")));
            }
            Err(e) => {
                let _ = fs::remove_file(&tmp_path);
                details.push(format!("✖ Erro ao executar FFmpeg: {e}"));
            }
        }
    }

    if ok_count == 0 {
        OperationResult::err("Não foi possível converter os arquivos de mídia").with_details(details)
    } else {
        OperationResult::ok(format!(
            "{ok_count} de {} arquivo(s) convertido(s) para {fmt} (original preservado)!",
            files.len()
        ))
        .with_details(details)
        .with_output(last_out.unwrap_or_else(|| files[0].clone()))
    }
}
