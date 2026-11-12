//! Motor de revelação de fotos do Axolote, inspirado nos ajustes do
//! PhotoCraft e do LightCraft (implementação própria, sem código deles):
//!
//! - **Luz:** exposição, realces, sombras, brancos e pretos
//! - **Cor:** temperatura, matiz (tint), vibração e rotação de matiz
//! - **Efeitos:** claridade, nitidez, desfoque, vinheta, granulado e pixelizar
//! - **Histograma** RGB + luminância ao vivo
//! - **Looks prontos** e presets do usuário salvos em JSON
//!
//! Todos os raios espaciais são relativos ao tamanho da imagem, então o preview
//! em miniatura e a exportação em resolução máxima ficam iguais.

use std::fs;
use std::path::{Path, PathBuf};

use iced::widget::image::Handle;
use image::{imageops, DynamicImage, GenericImageView, RgbaImage};
use serde::{Deserialize, Serialize};

use super::service::{PhotoEdits, PhotoFilter};
use crate::core::paths;

/// Cada controle deslizante do revelador.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Adj {
    // Luz
    Exposure,
    Brightness,
    Contrast,
    Highlights,
    Shadows,
    Whites,
    Blacks,
    // Cor
    Temperature,
    Tint,
    Vibrance,
    Saturation,
    Hue,
    // Efeitos
    Clarity,
    Sharpen,
    Blur,
    Vignette,
    Grain,
    Pixelate,
}

impl Adj {
    pub const LIGHT: &'static [Adj] = &[
        Adj::Exposure,
        Adj::Brightness,
        Adj::Contrast,
        Adj::Highlights,
        Adj::Shadows,
        Adj::Whites,
        Adj::Blacks,
    ];
    pub const COLOR: &'static [Adj] = &[
        Adj::Temperature,
        Adj::Tint,
        Adj::Vibrance,
        Adj::Saturation,
        Adj::Hue,
    ];
    pub const EFFECTS: &'static [Adj] = &[
        Adj::Clarity,
        Adj::Sharpen,
        Adj::Blur,
        Adj::Vignette,
        Adj::Grain,
        Adj::Pixelate,
    ];

    pub fn label(&self) -> &'static str {
        match self {
            Adj::Exposure => "☀  Exposição",
            Adj::Brightness => "💡  Brilho",
            Adj::Contrast => "◐  Contraste",
            Adj::Highlights => "⬜  Realces",
            Adj::Shadows => "⬛  Sombras",
            Adj::Whites => "○  Brancos",
            Adj::Blacks => "●  Pretos",
            Adj::Temperature => "🌡  Temperatura",
            Adj::Tint => "🟢  Matiz (Tint)",
            Adj::Vibrance => "✨  Vibração",
            Adj::Saturation => "🌈  Saturação",
            Adj::Hue => "🎡  Girar cores (Hue)",
            Adj::Clarity => "🔍  Claridade",
            Adj::Sharpen => "🔪  Nitidez",
            Adj::Blur => "🌫  Desfoque",
            Adj::Vignette => "🎯  Vinheta",
            Adj::Grain => "🎞  Granulado",
            Adj::Pixelate => "🟪  Pixelizar",
        }
    }

    pub fn range(&self) -> (i32, i32) {
        match self {
            Adj::Hue => (-180, 180),
            Adj::Sharpen | Adj::Blur | Adj::Grain | Adj::Pixelate => (0, 100),
            _ => (-100, 100),
        }
    }

    pub fn get(&self, e: &PhotoEdits) -> i32 {
        match self {
            Adj::Exposure => e.exposure,
            Adj::Brightness => e.brightness,
            Adj::Contrast => e.contrast,
            Adj::Highlights => e.highlights,
            Adj::Shadows => e.shadows,
            Adj::Whites => e.whites,
            Adj::Blacks => e.blacks,
            Adj::Temperature => e.temperature,
            Adj::Tint => e.tint,
            Adj::Vibrance => e.vibrance,
            Adj::Saturation => e.saturation,
            Adj::Hue => e.hue,
            Adj::Clarity => e.clarity,
            Adj::Sharpen => e.sharpen,
            Adj::Blur => e.blur,
            Adj::Vignette => e.vignette,
            Adj::Grain => e.grain,
            Adj::Pixelate => e.pixelate,
        }
    }

    pub fn set(&self, e: &mut PhotoEdits, v: i32) {
        let (lo, hi) = self.range();
        let v = v.clamp(lo, hi);
        match self {
            Adj::Exposure => e.exposure = v,
            Adj::Brightness => e.brightness = v,
            Adj::Contrast => e.contrast = v,
            Adj::Highlights => e.highlights = v,
            Adj::Shadows => e.shadows = v,
            Adj::Whites => e.whites = v,
            Adj::Blacks => e.blacks = v,
            Adj::Temperature => e.temperature = v,
            Adj::Tint => e.tint = v,
            Adj::Vibrance => e.vibrance = v,
            Adj::Saturation => e.saturation = v,
            Adj::Hue => e.hue = v,
            Adj::Clarity => e.clarity = v,
            Adj::Sharpen => e.sharpen = v,
            Adj::Blur => e.blur = v,
            Adj::Vignette => e.vignette = v,
            Adj::Grain => e.grain = v,
            Adj::Pixelate => e.pixelate = v,
        }
    }
}

// ---------------------------------------------------------------------------
// Ajustes de tom e cor (por pixel)
// ---------------------------------------------------------------------------

fn smoothstep(e0: f32, e1: f32, x: f32) -> f32 {
    let t = ((x - e0) / (e1 - e0)).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

/// Matriz de rotação de matiz (preserva a luminância).
fn hue_matrix(degrees: f32) -> [[f32; 3]; 3] {
    let (s, c) = degrees.to_radians().sin_cos();
    let (lr, lg, lb) = (0.213, 0.715, 0.072);
    [
        [
            lr + c * (1.0 - lr) + s * (-lr),
            lg + c * (-lg) + s * (-lg),
            lb + c * (-lb) + s * (1.0 - lb),
        ],
        [
            lr + c * (-lr) + s * 0.143,
            lg + c * (1.0 - lg) + s * 0.140,
            lb + c * (-lb) + s * (-0.283),
        ],
        [
            lr + c * (-lr) + s * (-(1.0 - lr)),
            lg + c * (-lg) + s * lg,
            lb + c * (1.0 - lb) + s * lb,
        ],
    ]
}

/// Aplica luz, cor e filtro criativo pixel a pixel.
pub fn apply_tone(rgba: &mut RgbaImage, e: &PhotoEdits) {
    let exposure = 2f32.powf(e.exposure as f32 / 50.0);
    let bright = e.brightness as f32 / 100.0 * 0.5;
    let contrast = ((100.0 + e.contrast as f32) / 100.0).powi(2);
    let temp = e.temperature as f32 / 100.0;
    let tint = e.tint as f32 / 100.0;
    let (hl, sh) = (e.highlights as f32 / 100.0, e.shadows as f32 / 100.0);
    let (wh, bl) = (e.whites as f32 / 100.0, e.blacks as f32 / 100.0);
    let sat = (100.0 + e.saturation as f32) / 100.0;
    let vib = e.vibrance as f32 / 100.0;
    let hue = (e.hue != 0).then(|| hue_matrix(e.hue as f32));

    for px in rgba.pixels_mut() {
        let mut r = px[0] as f32 / 255.0;
        let mut g = px[1] as f32 / 255.0;
        let mut b = px[2] as f32 / 255.0;

        // Balanço de branco
        r *= 1.0 + temp * 0.18 + tint * 0.06;
        g *= 1.0 - tint * 0.14;
        b *= 1.0 - temp * 0.18 + tint * 0.06;

        // Exposição (±2 stops) e brilho
        r = r * exposure + bright;
        g = g * exposure + bright;
        b = b * exposure + bright;

        // Realces / sombras / brancos / pretos por faixa de luminância
        let l = (0.2126 * r + 0.7152 * g + 0.0722 * b).clamp(0.0, 1.0);
        let delta = sh * 0.45 * (1.0 - smoothstep(0.0, 0.55, l))
            + hl * 0.45 * smoothstep(0.45, 1.0, l)
            + wh * 0.30 * smoothstep(0.75, 1.0, l)
            + bl * 0.30 * (1.0 - smoothstep(0.0, 0.25, l));
        r += delta;
        g += delta;
        b += delta;

        // Contraste em torno do cinza médio
        r = (r - 0.5) * contrast + 0.5;
        g = (g - 0.5) * contrast + 0.5;
        b = (b - 0.5) * contrast + 0.5;

        // Vibração (protege cores já saturadas) + saturação
        let luma = 0.299 * r + 0.587 * g + 0.114 * b;
        let cur_sat = (r.max(g).max(b) - r.min(g).min(b)).clamp(0.0, 1.0);
        let factor = sat * (1.0 + vib * (1.0 - cur_sat));
        r = luma + (r - luma) * factor;
        g = luma + (g - luma) * factor;
        b = luma + (b - luma) * factor;

        // Rotação de matiz
        if let Some(m) = &hue {
            let (nr, ng, nb) = (
                m[0][0] * r + m[0][1] * g + m[0][2] * b,
                m[1][0] * r + m[1][1] * g + m[1][2] * b,
                m[2][0] * r + m[2][1] * g + m[2][2] * b,
            );
            r = nr;
            g = ng;
            b = nb;
        }

        let (mut r8, mut g8, mut b8) = (r * 255.0, g * 255.0, b * 255.0);
        apply_filter(e.filter, &mut r8, &mut g8, &mut b8);

        px[0] = r8.clamp(0.0, 255.0) as u8;
        px[1] = g8.clamp(0.0, 255.0) as u8;
        px[2] = b8.clamp(0.0, 255.0) as u8;
    }
}

fn apply_filter(f: PhotoFilter, r: &mut f32, g: &mut f32, b: &mut f32) {
    match f {
        PhotoFilter::None => {}
        PhotoFilter::Grayscale => {
            let y = 0.299 * *r + 0.587 * *g + 0.114 * *b;
            (*r, *g, *b) = (y, y, y);
        }
        PhotoFilter::Sepia => {
            let (sr, sg, sb) = (
                0.393 * *r + 0.769 * *g + 0.189 * *b,
                0.349 * *r + 0.686 * *g + 0.168 * *b,
                0.272 * *r + 0.534 * *g + 0.131 * *b,
            );
            (*r, *g, *b) = (sr, sg, sb);
        }
        PhotoFilter::Warm => {
            *r += 22.0;
            *g += 8.0;
            *b -= 18.0;
        }
        PhotoFilter::Cool => {
            *r -= 16.0;
            *g += 4.0;
            *b += 24.0;
        }
        PhotoFilter::Cyberpunk => {
            *r = *r * 1.12 + 12.0;
            *g *= 0.88;
            *b = *b * 1.25 + 20.0;
        }
        PhotoFilter::Invert => {
            *r = 255.0 - *r;
            *g = 255.0 - *g;
            *b = 255.0 - *b;
        }
    }
}

// ---------------------------------------------------------------------------
// Efeitos espaciais
// ---------------------------------------------------------------------------

/// Escala dos raios: 1.0 para uma imagem de 1000 px no lado maior.
fn scale_of(w: u32, h: u32) -> f32 {
    (w.max(h) as f32 / 1000.0).max(0.05)
}

/// `orig + amount * (orig - blurred)` (máscara de nitidez com intensidade livre).
fn unsharp(img: &RgbaImage, sigma: f32, amount: f32) -> RgbaImage {
    let blurred = imageops::fast_blur(img, sigma.max(0.3));
    let mut out = img.clone();
    for (o, bl) in out.pixels_mut().zip(blurred.pixels()) {
        for c in 0..3 {
            let v = o[c] as f32 + amount * (o[c] as f32 - bl[c] as f32);
            o[c] = v.clamp(0.0, 255.0) as u8;
        }
    }
    out
}

fn pixelate(img: &RgbaImage, block: u32) -> RgbaImage {
    let (w, h) = img.dimensions();
    let small = imageops::resize(
        img,
        (w / block).max(1),
        (h / block).max(1),
        imageops::FilterType::Triangle,
    );
    imageops::resize(&small, w, h, imageops::FilterType::Nearest)
}

/// Ruído determinístico (o mesmo pixel sempre recebe o mesmo grão).
fn hash_noise(x: u32, y: u32) -> f32 {
    let mut n = x.wrapping_mul(374_761_393) ^ y.wrapping_mul(668_265_263);
    n = (n ^ (n >> 13)).wrapping_mul(1_274_126_177);
    n ^= n >> 16;
    (n & 0xffff) as f32 / 65535.0 - 0.5
}

pub fn apply_effects(mut rgba: RgbaImage, e: &PhotoEdits) -> RgbaImage {
    let (w, h) = rgba.dimensions();
    if w == 0 || h == 0 {
        return rgba;
    }
    let s = scale_of(w, h);

    if e.clarity != 0 {
        rgba = unsharp(&rgba, 18.0 * s, e.clarity as f32 / 100.0 * 0.6);
    }
    if e.sharpen > 0 {
        rgba = unsharp(&rgba, 1.2 * s, e.sharpen as f32 / 100.0 * 1.5);
    }
    if e.blur > 0 {
        rgba = imageops::fast_blur(&rgba, e.blur as f32 / 100.0 * 25.0 * s);
    }
    if e.pixelate > 0 {
        let block = ((e.pixelate as f32 / 100.0) * 60.0 * s).round().max(2.0) as u32;
        rgba = pixelate(&rgba, block);
    }
    if e.vignette != 0 || e.grain > 0 {
        let amt = e.vignette as f32 / 100.0;
        let grain = e.grain as f32 / 100.0 * 60.0;
        let grain_scale = s.max(1.0);
        let (cx, cy) = (w as f32 / 2.0, h as f32 / 2.0);
        let max_d = (cx * cx + cy * cy).sqrt();
        for (x, y, px) in rgba.enumerate_pixels_mut() {
            let mut k = 1.0;
            if amt != 0.0 {
                let d = (((x as f32 - cx).powi(2) + (y as f32 - cy).powi(2)).sqrt() / max_d)
                    .clamp(0.0, 1.0);
                let v = smoothstep(0.35, 1.0, d);
                // Negativo escurece as bordas, positivo clareia
                k = 1.0 + amt * v * 0.85;
            }
            let n = if grain > 0.0 {
                hash_noise(
                    (x as f32 / grain_scale) as u32,
                    (y as f32 / grain_scale) as u32,
                ) * grain
            } else {
                0.0
            };
            for c in 0..3 {
                px[c] = (px[c] as f32 * k + n).clamp(0.0, 255.0) as u8;
            }
        }
    }
    rgba
}

/// Pipeline completo: geometria → tom/cor → efeitos.
pub fn develop(mut img: DynamicImage, e: &PhotoEdits) -> DynamicImage {
    img = match e.rotation % 360 {
        90 => img.rotate90(),
        180 => img.rotate180(),
        270 => img.rotate270(),
        _ => img,
    };
    if e.flip_h {
        img = img.fliph();
    }
    if e.flip_v {
        img = img.flipv();
    }
    img = super::service::crop_to_aspect(img, e.crop);

    let neutral = PhotoEdits {
        rotation: e.rotation,
        flip_h: e.flip_h,
        flip_v: e.flip_v,
        crop: e.crop,
        ..PhotoEdits::default()
    };
    if *e == neutral {
        return img;
    }
    let mut rgba = img.to_rgba8();
    apply_tone(&mut rgba, e);
    DynamicImage::ImageRgba8(apply_effects(rgba, e))
}

// ---------------------------------------------------------------------------
// Histograma
// ---------------------------------------------------------------------------

pub const HIST_W: u32 = 256;
pub const HIST_H: u32 = 96;

/// Conta os valores R, G, B e luminância (256 faixas cada).
pub fn histogram(img: &DynamicImage) -> [[u32; 256]; 4] {
    let mut h = [[0u32; 256]; 4];
    for (_, _, p) in img.pixels() {
        h[0][p[0] as usize] += 1;
        h[1][p[1] as usize] += 1;
        h[2][p[2] as usize] += 1;
        let y = (0.2126 * p[0] as f32 + 0.7152 * p[1] as f32 + 0.0722 * p[2] as f32) as usize;
        h[3][y.min(255)] += 1;
    }
    h
}

/// Desenha o histograma (RGB sobrepostos + luminância) como imagem.
pub fn histogram_handle(img: &DynamicImage) -> Handle {
    let h = histogram(img);
    // Ignora as pontas (0 e 255) na escala para não achatar o gráfico.
    let peak = h[..3]
        .iter()
        .flat_map(|c| c[1..255].iter())
        .copied()
        .max()
        .unwrap_or(1)
        .max(1) as f32;
    let mut out = RgbaImage::from_pixel(HIST_W, HIST_H, image::Rgba([14, 12, 24, 255]));
    let colors = [[239u8, 68, 68], [34, 197, 94], [59, 130, 246]];
    for x in 0..HIST_W {
        for (ch, col) in colors.iter().enumerate() {
            let bar =
                ((h[ch][x as usize] as f32 / peak).sqrt().min(1.0) * (HIST_H - 1) as f32) as u32;
            for y in (HIST_H - bar)..HIST_H {
                let p = out.get_pixel_mut(x, y);
                for c in 0..3 {
                    p[c] = p[c].saturating_add(col[c] / 2);
                }
            }
        }
        let lbar = ((h[3][x as usize] as f32 / peak).sqrt().min(1.0) * (HIST_H - 1) as f32) as u32;
        if lbar > 0 {
            let y = HIST_H - lbar;
            out.put_pixel(x, y.min(HIST_H - 1), image::Rgba([235, 225, 255, 255]));
        }
    }
    Handle::from_pixels(HIST_W, HIST_H, out.into_raw())
}

// ---------------------------------------------------------------------------
// Looks prontos e presets do usuário
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Preset {
    pub name: String,
    pub edits: PhotoEdits,
}

/// Looks inspirados em revelação fotográfica e color grading de cinema.
pub fn builtin_looks() -> Vec<Preset> {
    let d = PhotoEdits::default();
    let p = |name: &str, edits: PhotoEdits| Preset {
        name: name.into(),
        edits,
    };
    vec![
        p(
            "🌅 Vívido",
            PhotoEdits {
                vibrance: 45,
                contrast: 12,
                clarity: 20,
                shadows: 15,
                ..d
            },
        ),
        p(
            "🎬 Cinema",
            PhotoEdits {
                temperature: 12,
                tint: -6,
                contrast: 18,
                highlights: -25,
                shadows: 10,
                saturation: -10,
                vignette: -35,
                ..d
            },
        ),
        p(
            "📼 Retrô Desbotado",
            PhotoEdits {
                blacks: 35,
                contrast: -20,
                saturation: -25,
                temperature: 18,
                grain: 35,
                vignette: -20,
                ..d
            },
        ),
        p(
            "🖤 P&B Dramático",
            PhotoEdits {
                filter: PhotoFilter::Grayscale,
                contrast: 35,
                clarity: 40,
                blacks: -20,
                vignette: -40,
                ..d
            },
        ),
        p(
            "🌸 Retrato Suave",
            PhotoEdits {
                exposure: 8,
                highlights: -20,
                shadows: 20,
                clarity: -25,
                temperature: 8,
                vibrance: 10,
                ..d
            },
        ),
        p(
            "🏔 Paisagem",
            PhotoEdits {
                vibrance: 35,
                clarity: 35,
                highlights: -35,
                shadows: 25,
                sharpen: 30,
                ..d
            },
        ),
        p(
            "❄ Inverno",
            PhotoEdits {
                temperature: -35,
                tint: 5,
                saturation: -15,
                exposure: 10,
                whites: 15,
                ..d
            },
        ),
        p(
            "💜 Axolote Neon",
            PhotoEdits {
                tint: 30,
                vibrance: 40,
                contrast: 20,
                hue: -12,
                vignette: -30,
                ..d
            },
        ),
    ]
}

fn presets_file() -> PathBuf {
    paths::data_dir().join("presets_fotos.json")
}

pub fn load_user_presets() -> Vec<Preset> {
    load_presets_from(&presets_file())
}

pub fn save_user_presets(list: &[Preset]) -> Result<(), String> {
    save_presets_to(&presets_file(), list)
}

fn load_presets_from(path: &Path) -> Vec<Preset> {
    fs::read_to_string(path)
        .ok()
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap_or_default()
}

fn save_presets_to(path: &Path, list: &[Preset]) -> Result<(), String> {
    let json = serde_json::to_string_pretty(list).map_err(|e| e.to_string())?;
    let tmp = path.with_extension("axolote_tmp");
    fs::write(&tmp, json).map_err(|e| e.to_string())?;
    fs::rename(&tmp, path).map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn gray(v: u8) -> DynamicImage {
        DynamicImage::ImageRgba8(RgbaImage::from_pixel(40, 30, image::Rgba([v, v, v, 255])))
    }

    #[test]
    fn neutral_edits_keep_pixels() {
        let img = gray(120);
        let out = develop(img.clone(), &PhotoEdits::default());
        assert_eq!(img.to_rgba8(), out.to_rgba8());
    }

    #[test]
    fn exposure_brightens_and_darkens() {
        let up = develop(
            gray(100),
            &PhotoEdits {
                exposure: 50,
                ..Default::default()
            },
        );
        let down = develop(
            gray(100),
            &PhotoEdits {
                exposure: -50,
                ..Default::default()
            },
        );
        assert!(up.to_rgba8().get_pixel(5, 5)[0] > 150);
        assert!(down.to_rgba8().get_pixel(5, 5)[0] < 70);
    }

    #[test]
    fn temperature_warms_image() {
        let out = develop(
            gray(128),
            &PhotoEdits {
                temperature: 80,
                ..Default::default()
            },
        )
        .to_rgba8();
        let p = out.get_pixel(3, 3);
        assert!(p[0] > p[2], "vermelho deve ficar maior que azul: {p:?}");
    }

    #[test]
    fn hue_rotation_keeps_gray() {
        let out = develop(
            gray(128),
            &PhotoEdits {
                hue: 90,
                ..Default::default()
            },
        )
        .to_rgba8();
        let p = out.get_pixel(1, 1);
        assert!((p[0] as i32 - 128).abs() <= 2 && (p[2] as i32 - 128).abs() <= 2);
    }

    #[test]
    fn vignette_darkens_corners_only() {
        let out = develop(
            gray(200),
            &PhotoEdits {
                vignette: -100,
                ..Default::default()
            },
        )
        .to_rgba8();
        assert!(out.get_pixel(0, 0)[0] < out.get_pixel(20, 15)[0]);
    }

    #[test]
    fn histogram_counts_every_pixel() {
        let h = histogram(&gray(77));
        assert_eq!(h[0][77], 40 * 30);
        assert_eq!(h[0].iter().sum::<u32>(), 40 * 30);
    }

    #[test]
    fn presets_round_trip() {
        let dir = std::env::temp_dir().join(format!("axolote_presets_{}", std::process::id()));
        fs::create_dir_all(&dir).unwrap();
        let file = dir.join("p.json");
        let list = builtin_looks();
        save_presets_to(&file, &list).unwrap();
        assert_eq!(load_presets_from(&file), list);
        let _ = fs::remove_dir_all(dir);
    }
}
