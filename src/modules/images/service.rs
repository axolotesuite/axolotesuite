//! Processamento e Edição de Fotos (estilo Photo Studio):
//! - Pré-visualização ao vivo (pixels RGBA em memória)
//! - Ajustes de Brilho, Contraste e Saturação
//! - Filtros criativos (Original, Preto e Branco, Sépia, Quente, Frio, Cyberpunk, Inverter)
//! - Transformações (Girar 90° Dir/Esq, 180°, Espelhar Horizontal/Vertical)
//! - Recorte por proporção (Livre/Original, 1:1, 16:9, 9:16, 4:3)
//! - Redimensionamento Lanczos3, Conversão (PNG/JPG/WebP/BMP/ICO) e Compressão JPEG

use std::fmt;
use std::fs::{self, File};
use std::io::BufWriter;
use std::path::{Path, PathBuf};

use iced::widget::image::Handle;
use image::codecs::jpeg::JpegEncoder;
use image::imageops::FilterType;
use image::{DynamicImage, GenericImageView, ImageFormat};

use crate::shared::types::OperationResult;
use crate::shared::ui::human_size;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutFormat {
    Png,
    Jpg,
    Webp,
    Bmp,
    Ico,
}

impl OutFormat {
    pub const ALL: &'static [OutFormat] = &[
        OutFormat::Png,
        OutFormat::Jpg,
        OutFormat::Webp,
        OutFormat::Bmp,
        OutFormat::Ico,
    ];

    pub fn ext(&self) -> &'static str {
        match self {
            OutFormat::Png => "png",
            OutFormat::Jpg => "jpg",
            OutFormat::Webp => "webp",
            OutFormat::Bmp => "bmp",
            OutFormat::Ico => "ico",
        }
    }

    pub fn image_format(&self) -> ImageFormat {
        match self {
            OutFormat::Png => ImageFormat::Png,
            OutFormat::Jpg => ImageFormat::Jpeg,
            OutFormat::Webp => ImageFormat::WebP,
            OutFormat::Bmp => ImageFormat::Bmp,
            OutFormat::Ico => ImageFormat::Ico,
        }
    }
}

impl fmt::Display for OutFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            OutFormat::Png => "PNG (sem perda, suporta transparência)",
            OutFormat::Jpg => "JPG / JPEG (fotos leves)",
            OutFormat::Webp => "WebP (moderno para web)",
            OutFormat::Bmp => "BMP (bitmap sem compressão)",
            OutFormat::Ico => "ICO (ícone do Windows, até 256×256)",
        };
        write!(f, "{s}")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
pub enum PhotoFilter {
    #[default]
    None,
    Grayscale,
    Sepia,
    Warm,
    Cool,
    Cyberpunk,
    Invert,
}

impl PhotoFilter {
    pub const ALL: &'static [PhotoFilter] = &[
        PhotoFilter::None,
        PhotoFilter::Grayscale,
        PhotoFilter::Sepia,
        PhotoFilter::Warm,
        PhotoFilter::Cool,
        PhotoFilter::Cyberpunk,
        PhotoFilter::Invert,
    ];

    pub fn label(&self) -> &'static str {
        match self {
            PhotoFilter::None => "✨ Original",
            PhotoFilter::Grayscale => "🖤 P&B",
            PhotoFilter::Sepia => "📜 Sépia",
            PhotoFilter::Warm => "☀ Quente",
            PhotoFilter::Cool => "❄ Frio",
            PhotoFilter::Cyberpunk => "💜 Neon/Axolote",
            PhotoFilter::Invert => "🔄 Negativo",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
pub enum AspectCrop {
    #[default]
    Original,
    Square1x1,
    Wide16x9,
    Story9x16,
    Classic4x3,
}

impl AspectCrop {
    pub const ALL: &'static [AspectCrop] = &[
        AspectCrop::Original,
        AspectCrop::Square1x1,
        AspectCrop::Wide16x9,
        AspectCrop::Story9x16,
        AspectCrop::Classic4x3,
    ];

    pub fn label(&self) -> &'static str {
        match self {
            AspectCrop::Original => "Livre (Original)",
            AspectCrop::Square1x1 => "1:1 (Instagram / Avatar)",
            AspectCrop::Wide16x9 => "16:9 (YouTube / Widescreen)",
            AspectCrop::Story9x16 => "9:16 (Reels / TikTok / Shorts)",
            AspectCrop::Classic4x3 => "4:3 (Foto Clássica)",
        }
    }
}

/// Todos os ajustes do Editor de Fotos. É salvo em JSON nos presets do usuário,
/// por isso campos novos precisam de `#[serde(default)]`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct PhotoEdits {
    // Geometria
    pub rotation: u16, // 0, 90, 180, 270
    pub flip_h: bool,
    pub flip_v: bool,
    pub crop: AspectCrop,
    pub filter: PhotoFilter,
    // Luz (-100..=100)
    pub exposure: i32,
    pub brightness: i32,
    pub contrast: i32,
    pub highlights: i32,
    pub shadows: i32,
    pub whites: i32,
    pub blacks: i32,
    // Cor
    pub temperature: i32, // -100..=100
    pub tint: i32,        // -100..=100
    pub vibrance: i32,    // -100..=100
    pub saturation: i32,  // -100..=100
    pub hue: i32,         // -180..=180
    // Efeitos
    pub clarity: i32,  // -100..=100
    pub sharpen: i32,  // 0..=100
    pub blur: i32,     // 0..=100
    pub vignette: i32, // -100..=100
    pub grain: i32,    // 0..=100
    pub pixelate: i32, // 0..=100
}

#[derive(Debug, Clone)]
pub struct ImageMeta {
    pub width: u32,
    pub height: u32,
    pub size_bytes: u64,
    pub format_label: String,
}

/// Lê as dimensões e o tamanho de uma imagem.
pub fn inspect(path: &Path) -> Result<ImageMeta, String> {
    if !path.is_file() {
        return Err("Arquivo de imagem não encontrado.".into());
    }
    let size_bytes = fs::metadata(path).map(|m| m.len()).unwrap_or(0);
    let (width, height) = image::image_dimensions(path)
        .map_err(|e| format!("Não foi possível ler as dimensões da imagem: {e}"))?;
    let ext = path
        .extension()
        .map(|e| e.to_string_lossy().to_uppercase())
        .unwrap_or_else(|| "IMAGEM".into());
    Ok(ImageMeta {
        width,
        height,
        size_bytes,
        format_label: ext,
    })
}

/// Carrega uma miniatura base (máx 440px) para o preview do editor responder em tempo real.
pub fn load_preview_base(path: &Path) -> Option<DynamicImage> {
    let img = image::open(path).ok()?;
    Some(img.thumbnail(440, 320))
}

/// Gera o `Handle` de pré-visualização aplicando todos os ajustes do editor na miniatura.
pub fn build_preview_handle(base: &DynamicImage, edits: &PhotoEdits) -> Handle {
    let processed = apply_edits(base.clone(), edits);
    let rgba = processed.to_rgba8();
    let (w, h) = rgba.dimensions();
    Handle::from_pixels(w, h, rgba.into_raw())
}

/// Aplica todas as edições (geometria, luz, cor e efeitos). Veja develop.rs.
pub fn apply_edits(img: DynamicImage, edits: &PhotoEdits) -> DynamicImage {
    super::develop::develop(img, edits)
}

pub(super) fn crop_to_aspect(mut img: DynamicImage, crop: AspectCrop) -> DynamicImage {
    let ratio = match crop {
        AspectCrop::Original => return img,
        AspectCrop::Square1x1 => 1.0,
        AspectCrop::Wide16x9 => 16.0 / 9.0,
        AspectCrop::Story9x16 => 9.0 / 16.0,
        AspectCrop::Classic4x3 => 4.0 / 3.0,
    };
    let (w, h) = img.dimensions();
    if w == 0 || h == 0 {
        return img;
    }
    let current = w as f32 / h as f32;
    let (new_w, new_h) = if current > ratio {
        (((h as f32) * ratio).round() as u32, h)
    } else {
        (w, ((w as f32) / ratio).round() as u32)
    };
    let new_w = new_w.clamp(1, w);
    let new_h = new_h.clamp(1, h);
    let x = (w - new_w) / 2;
    let y = (h - new_h) / 2;
    img.crop(x, y, new_w, new_h)
}

/// Salva a foto em resolução total aplicando todas as edições do Editor de Fotos.
pub fn save_edited_photo(input: PathBuf, output: PathBuf, edits: PhotoEdits) -> OperationResult {
    let img = match image::open(&input) {
        Ok(i) => i,
        Err(e) => return OperationResult::err(format!("Erro ao abrir a foto: {e}")),
    };
    let edited = apply_edits(img, &edits);
    let (w, h) = edited.dimensions();
    if let Err(e) = save_auto(&edited, &output) {
        return OperationResult::err(format!("Erro ao salvar a foto editada: {e}"));
    }
    let out_size = fs::metadata(&output).map(|m| m.len()).unwrap_or(0);
    OperationResult::ok(format!(
        "Foto editada salva ({w}×{h} px, {})",
        human_size(out_size)
    ))
    .with_detail(format!("Arquivo salvo em: {}", output.display()))
    .with_output(output)
}

/// Redimensiona a imagem usando filtro Lanczos3 (alta nitidez).
pub fn resize(
    input: PathBuf,
    output: PathBuf,
    width: u32,
    height: u32,
    keep_aspect: bool,
) -> OperationResult {
    if width == 0 || height == 0 {
        return OperationResult::err("Informe uma largura e altura maiores que zero.");
    }
    let img = match image::open(&input) {
        Ok(i) => i,
        Err(e) => return OperationResult::err(format!("Erro ao abrir a imagem: {e}")),
    };
    let (orig_w, orig_h) = img.dimensions();
    let resized = if keep_aspect {
        img.resize(width, height, FilterType::Lanczos3)
    } else {
        img.resize_exact(width, height, FilterType::Lanczos3)
    };
    let (new_w, new_h) = resized.dimensions();
    if let Err(e) = save_auto(&resized, &output) {
        return OperationResult::err(format!("Erro ao salvar a imagem redimensionada: {e}"));
    }
    let out_size = fs::metadata(&output).map(|m| m.len()).unwrap_or(0);
    OperationResult::ok(format!(
        "Imagem redimensionada: {orig_w}×{orig_h} → {new_w}×{new_h} px"
    ))
    .with_details(vec![
        format!("Arquivo salvo em: {}", output.display()),
        format!("Tamanho final: {}", human_size(out_size)),
    ])
    .with_output(output)
}

/// Converte a imagem para o formato escolhido.
pub fn convert(input: PathBuf, output: PathBuf, fmt: OutFormat) -> OperationResult {
    let img = match image::open(&input) {
        Ok(i) => i,
        Err(e) => return OperationResult::err(format!("Erro ao abrir a imagem: {e}")),
    };
    let to_save = match fmt {
        OutFormat::Ico => {
            let (w, h) = img.dimensions();
            if w > 256 || h > 256 {
                img.resize(256, 256, FilterType::Lanczos3)
            } else {
                img
            }
        }
        OutFormat::Jpg => DynamicImage::ImageRgb8(img.to_rgb8()),
        _ => img,
    };
    if let Some(parent) = output.parent() {
        let _ = fs::create_dir_all(parent);
    }
    if let Err(e) = to_save.save_with_format(&output, fmt.image_format()) {
        return OperationResult::err(format!("Erro ao converter a imagem: {e}"));
    }
    let out_size = fs::metadata(&output).map(|m| m.len()).unwrap_or(0);
    OperationResult::ok(format!(
        "Imagem convertida para {} ({})",
        fmt.ext().to_uppercase(),
        human_size(out_size)
    ))
    .with_detail(format!("Arquivo salvo em: {}", output.display()))
    .with_output(output)
}

/// Comprime a imagem como JPEG ajustando a qualidade (10 a 100).
pub fn compress(input: PathBuf, output: PathBuf, quality: u8) -> OperationResult {
    let orig_size = fs::metadata(&input).map(|m| m.len()).unwrap_or(0);
    let img = match image::open(&input) {
        Ok(i) => i,
        Err(e) => return OperationResult::err(format!("Erro ao abrir a imagem: {e}")),
    };
    let rgb = img.to_rgb8();
    if let Some(parent) = output.parent() {
        let _ = fs::create_dir_all(parent);
    }
    let file = match File::create(&output) {
        Ok(f) => f,
        Err(e) => return OperationResult::err(format!("Erro ao criar o arquivo de saída: {e}")),
    };
    let mut writer = BufWriter::new(file);
    let mut encoder = JpegEncoder::new_with_quality(&mut writer, quality.clamp(10, 100));
    if let Err(e) = encoder.encode_image(&rgb) {
        return OperationResult::err(format!("Erro ao comprimir a imagem: {e}"));
    }
    drop(writer);

    let new_size = fs::metadata(&output).map(|m| m.len()).unwrap_or(0);
    let reduction = if orig_size > 0 && new_size < orig_size {
        let pct = ((orig_size - new_size) as f64 / orig_size as f64) * 100.0;
        format!("redução de {:.0}%", pct)
    } else {
        format!("qualidade {quality}%")
    };

    OperationResult::ok(format!(
        "Imagem comprimida: {} → {} ({reduction})",
        human_size(orig_size),
        human_size(new_size)
    ))
    .with_detail(format!("Arquivo salvo em: {}", output.display()))
    .with_output(output)
}

fn save_auto(img: &DynamicImage, output: &Path) -> Result<(), String> {
    if let Some(parent) = output.parent() {
        let _ = fs::create_dir_all(parent);
    }
    let ext = output
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("png")
        .to_lowercase();
    // Gravação atômica: escreve em `nome.axolote_tmp.ext` e só renomeia no fim.
    let stem = output
        .file_stem()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_else(|| "foto".into());
    let tmp = output.with_file_name(format!("{stem}.axolote_tmp.{ext}"));
    let res = if ext == "jpg" || ext == "jpeg" {
        DynamicImage::ImageRgb8(img.to_rgb8())
            .save(&tmp)
            .map_err(|e| e.to_string())
    } else {
        img.save(&tmp).map_err(|e| e.to_string())
    };
    if let Err(e) = res {
        let _ = fs::remove_file(&tmp);
        return Err(e);
    }
    fs::rename(&tmp, output).map_err(|e| {
        let _ = fs::remove_file(&tmp);
        e.to_string()
    })
}

/// Aplica as mesmas edições em todas as imagens de uma pasta (estilo "sincronizar
/// ajustes" do LightCraft). Os resultados vão para `pasta/Axolote_Editadas`.
pub fn batch_apply(folder: PathBuf, edits: PhotoEdits) -> OperationResult {
    const EXTS: &[&str] = &["png", "jpg", "jpeg", "webp", "bmp", "gif", "tif", "tiff"];
    let entries = match fs::read_dir(&folder) {
        Ok(e) => e,
        Err(e) => return OperationResult::err(format!("Não foi possível abrir a pasta: {e}")),
    };
    let mut files: Vec<PathBuf> = entries
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| {
            p.is_file()
                && p.extension()
                    .and_then(|e| e.to_str())
                    .map(|e| EXTS.contains(&e.to_lowercase().as_str()))
                    .unwrap_or(false)
        })
        .collect();
    files.sort();
    if files.is_empty() {
        return OperationResult::err("Nenhuma imagem encontrada nessa pasta.");
    }

    let out_dir = folder.join("Axolote_Editadas");
    let mut ok = 0usize;
    let mut errors = Vec::new();
    for f in &files {
        let ext = f
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("png")
            .to_lowercase();
        let out = crate::core::paths::output_path(f, Some(&out_dir), "", &ext);
        match image::open(f) {
            Ok(img) => match save_auto(&apply_edits(img, &edits), &out) {
                Ok(()) => ok += 1,
                Err(e) => errors.push(format!("{}: {e}", crate::shared::ui::file_name(f))),
            },
            Err(e) => errors.push(format!("{}: {e}", crate::shared::ui::file_name(f))),
        }
    }

    let mut details = vec![format!("Salvas em: {}", out_dir.display())];
    details.extend(errors.iter().take(8).cloned());
    let title = if errors.is_empty() {
        format!("{ok} foto(s) editada(s) em lote")
    } else {
        format!(
            "{ok} de {} foto(s) editada(s); {} com erro",
            files.len(),
            errors.len()
        )
    };
    let res = if ok > 0 {
        OperationResult::ok(title)
    } else {
        OperationResult::err(title)
    };
    res.with_details(details).with_output(out_dir)
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::RgbaImage;

    #[test]
    fn batch_apply_writes_to_subfolder_and_keeps_originals() {
        let dir = std::env::temp_dir().join(format!("axolote_batch_{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        for name in ["a.png", "b.png"] {
            RgbaImage::from_pixel(8, 8, image::Rgba([100, 100, 100, 255]))
                .save(dir.join(name))
                .unwrap();
        }
        let edits = PhotoEdits {
            exposure: 50,
            ..Default::default()
        };
        let res = batch_apply(dir.clone(), edits);
        assert!(res.success, "{}", res.title);
        let out = image::open(dir.join("Axolote_Editadas").join("a.png"))
            .unwrap()
            .to_rgba8();
        assert!(out.get_pixel(0, 0)[0] > 150);
        let orig = image::open(dir.join("a.png")).unwrap().to_rgba8();
        assert_eq!(orig.get_pixel(0, 0)[0], 100);
        let leftovers = fs::read_dir(dir.join("Axolote_Editadas"))
            .unwrap()
            .filter(|e| {
                e.as_ref()
                    .unwrap()
                    .file_name()
                    .to_string_lossy()
                    .contains("axolote_tmp")
            })
            .count();
        assert_eq!(leftovers, 0);
        let _ = fs::remove_dir_all(dir);
    }
}
