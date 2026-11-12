//! Serviços puros em Rust do módulo "Canivete Dev & Estúdio de Cores / Senhas":
//! - Conversão cromática (RGB ↔ HEX ↔ HSL ↔ CMYK) + Extrator de Paleta Dominante de Imagens
//! - Gerador de Senhas com cálculo de entropia em bits + Gerador de UUID v4
//! - Codificador/Decodificador Base64, Formatador/Validador JSON e Transformador de Texto.

use std::io::Write;
use std::path::Path;
use std::process::Stdio;
use std::time::{SystemTime, UNIX_EPOCH};

use image::GenericImageView;

use crate::core::system;

// ---------------------------------------------------------------------------
// 1. Área de Transferência do Windows (Clipboard nativo sem dependências extras)
// ---------------------------------------------------------------------------

pub fn copy_to_clipboard(text: &str) -> bool {
    if let Ok(mut child) = system::hidden_command("clip").stdin(Stdio::piped()).spawn() {
        if let Some(mut stdin) = child.stdin.take() {
            let _ = stdin.write_all(text.as_bytes());
        }
        return child.wait().map(|s| s.success()).unwrap_or(false);
    }
    false
}

// ---------------------------------------------------------------------------
// 2. Estúdio de Cores & Extrator de Paleta de Imagem
// ---------------------------------------------------------------------------

pub fn parse_hex_color(hex: &str) -> Option<(u8, u8, u8)> {
    let clean = hex.trim().trim_start_matches('#');
    if clean.len() == 6 {
        let r = u8::from_str_radix(&clean[0..2], 16).ok()?;
        let g = u8::from_str_radix(&clean[2..4], 16).ok()?;
        let b = u8::from_str_radix(&clean[4..6], 16).ok()?;
        Some((r, g, b))
    } else if clean.len() == 3 {
        let r = u8::from_str_radix(&clean[0..1], 16).ok()? * 17;
        let g = u8::from_str_radix(&clean[1..2], 16).ok()? * 17;
        let b = u8::from_str_radix(&clean[2..3], 16).ok()? * 17;
        Some((r, g, b))
    } else {
        None
    }
}

pub fn rgb_to_hex(r: u8, g: u8, b: u8) -> String {
    format!("#{:02X}{:02X}{:02X}", r, g, b)
}

pub fn rgb_to_hsl(r: u8, g: u8, b: u8) -> (f32, f32, f32) {
    let rf = r as f32 / 255.0;
    let gf = g as f32 / 255.0;
    let bf = b as f32 / 255.0;
    let max = rf.max(gf).max(bf);
    let min = rf.min(gf).min(bf);
    let l = (max + min) * 0.5;
    let d = max - min;
    if d.abs() < 1e-5 {
        return (0.0, 0.0, l * 100.0);
    }
    let s = if l > 0.5 {
        d / (2.0 - max - min)
    } else {
        d / (max + min)
    };
    let mut h = if (max - rf).abs() < 1e-5 {
        (gf - bf) / d + (if gf < bf { 6.0 } else { 0.0 })
    } else if (max - gf).abs() < 1e-5 {
        (bf - rf) / d + 2.0
    } else {
        (rf - gf) / d + 4.0
    };
    h *= 60.0;
    (h, s * 100.0, l * 100.0)
}

pub fn hsl_to_rgb(h: f32, s_pct: f32, l_pct: f32) -> (u8, u8, u8) {
    let h = ((h % 360.0) + 360.0) % 360.0;
    let s = (s_pct / 100.0).clamp(0.0, 1.0);
    let l = (l_pct / 100.0).clamp(0.0, 1.0);
    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let x = c * (1.0 - (((h / 60.0) % 2.0) - 1.0).abs());
    let m = l - c * 0.5;
    let (r1, g1, b1) = if h < 60.0 {
        (c, x, 0.0)
    } else if h < 120.0 {
        (x, c, 0.0)
    } else if h < 180.0 {
        (0.0, c, x)
    } else if h < 240.0 {
        (0.0, x, c)
    } else if h < 300.0 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };
    (
        ((r1 + m) * 255.0).round().clamp(0.0, 255.0) as u8,
        ((g1 + m) * 255.0).round().clamp(0.0, 255.0) as u8,
        ((b1 + m) * 255.0).round().clamp(0.0, 255.0) as u8,
    )
}

pub fn rgb_to_cmyk(r: u8, g: u8, b: u8) -> (u8, u8, u8, u8) {
    let rf = r as f32 / 255.0;
    let gf = g as f32 / 255.0;
    let bf = b as f32 / 255.0;
    let k = 1.0 - rf.max(gf).max(bf);
    if (1.0 - k).abs() < 1e-5 {
        return (0, 0, 0, 100);
    }
    let c = ((1.0 - rf - k) / (1.0 - k) * 100.0).round() as u8;
    let m = ((1.0 - gf - k) / (1.0 - k) * 100.0).round() as u8;
    let y = ((1.0 - bf - k) / (1.0 - k) * 100.0).round() as u8;
    let kp = (k * 100.0).round() as u8;
    (c, m, y, kp)
}

pub fn color_harmonies(r: u8, g: u8, b: u8) -> Vec<(&'static str, String, (u8, u8, u8))> {
    let (h, s, l) = rgb_to_hsl(r, g, b);
    let specs: [(&str, f32, f32, f32); 5] = [
        ("Complementar (180°)", h + 180.0, s, l),
        ("Análoga (-30°)", h - 30.0, s, l),
        ("Análoga (+30°)", h + 30.0, s, l),
        ("Tríade (120°)", h + 120.0, s, l),
        ("Tríade (240°)", h + 240.0, s, l),
    ];
    specs
        .into_iter()
        .map(|(name, nh, ns, nl)| {
            let rgb = hsl_to_rgb(nh, ns, nl);
            (name, rgb_to_hex(rgb.0, rgb.1, rgb.2), rgb)
        })
        .collect()
}

pub fn render_color_swatch(r: u8, g: u8, b: u8, w: u32, h: u32) -> Vec<u8> {
    let mut buf = vec![0u8; (w * h * 4) as usize];
    for y in 0..h {
        for x in 0..w {
            let border = x < 2 || y < 2 || x >= w - 2 || y >= h - 2;
            let idx = ((y * w + x) * 4) as usize;
            if border {
                buf[idx] = 255;
                buf[idx + 1] = 255;
                buf[idx + 2] = 255;
                buf[idx + 3] = 90;
            } else {
                buf[idx] = r;
                buf[idx + 1] = g;
                buf[idx + 2] = b;
                buf[idx + 3] = 255;
            }
        }
    }
    buf
}

/// Cor dominante extraída: `(hex, (r, g, b), percentual)`.
pub type PaletteColor = (String, (u8, u8, u8), u32);

/// Extrai as 6 cores dominantes de uma imagem por quantização de histograma RGB.
pub fn extract_image_palette(path: &Path) -> Result<Vec<PaletteColor>, String> {
    let img = image::open(path).map_err(|e| format!("Erro ao abrir imagem: {e}"))?;
    let small = img.thumbnail(96, 96);
    let mut buckets = std::collections::HashMap::<(u8, u8, u8), (u64, u64, u64, u32)>::new();

    for (_, _, p) in small.pixels() {
        if p[3] < 40 {
            continue;
        }
        // Quantiza em blocos de 32 níveis por canal
        let key = (p[0] / 32, p[1] / 32, p[2] / 32);
        let entry = buckets.entry(key).or_insert((0, 0, 0, 0));
        entry.0 += p[0] as u64;
        entry.1 += p[1] as u64;
        entry.2 += p[2] as u64;
        entry.3 += 1;
    }

    let total_pixels: u32 = buckets.values().map(|v| v.3).sum::<u32>().max(1);
    let mut sorted: Vec<_> = buckets.into_values().collect();
    sorted.sort_by_key(|a| std::cmp::Reverse(a.3));

    let mut out = Vec::new();
    for (sr, sg, sb, count) in sorted.into_iter().take(6) {
        let r = (sr / count.max(1) as u64) as u8;
        let g = (sg / count.max(1) as u64) as u8;
        let b = (sb / count.max(1) as u64) as u8;
        let pct = ((count as f32 / total_pixels as f32) * 100.0).round() as u32;
        out.push((rgb_to_hex(r, g, b), (r, g, b), pct.max(1)));
    }
    Ok(out)
}

// ---------------------------------------------------------------------------
// 3. Gerador de Senhas Fortes, Entropia & UUID v4
// ---------------------------------------------------------------------------

pub fn generate_passwords(
    length: usize,
    upper: bool,
    lower: bool,
    digits: bool,
    symbols: bool,
    avoid_ambiguous: bool,
    count: usize,
) -> (Vec<String>, f32, &'static str) {
    let mut pool = String::new();
    if upper {
        pool.push_str(if avoid_ambiguous {
            "ABCDEFGHJKLMNPQRSTUVWXYZ"
        } else {
            "ABCDEFGHIJKLMNOPQRSTUVWXYZ"
        });
    }
    if lower {
        pool.push_str(if avoid_ambiguous {
            "abcdefghijkmnpqrstuvwxyz"
        } else {
            "abcdefghijklmnopqrstuvwxyz"
        });
    }
    if digits {
        pool.push_str(if avoid_ambiguous {
            "23456789"
        } else {
            "0123456789"
        });
    }
    if symbols {
        pool.push_str("!@#$%^&*()-_=+[]{};:,.<>?");
    }
    if pool.is_empty() {
        pool.push_str("abcdefghijkmnpqrstuvwxyz23456789");
    }

    let chars: Vec<char> = pool.chars().collect();
    let pool_len = chars.len();
    let entropy = (length as f32) * (pool_len as f32).log2();
    let rating = if entropy < 45.0 {
        "Fraca"
    } else if entropy < 70.0 {
        "Razoável"
    } else if entropy < 100.0 {
        "Forte"
    } else {
        "Excelente (Criptográfica)"
    };

    let mut list = Vec::with_capacity(count);
    for _ in 0..count {
        let mut pwd = String::with_capacity(length);
        for _ in 0..length {
            let idx = secure_random_index(pool_len);
            pwd.push(chars[idx]);
        }
        list.push(pwd);
    }

    (list, entropy, rating)
}

pub fn generate_uuids(count: usize) -> Vec<String> {
    let mut out = Vec::with_capacity(count);
    for _ in 0..count {
        let mut bytes = [0u8; 16];
        let _ = getrandom::fill(&mut bytes);
        // RFC 4122 v4
        bytes[6] = (bytes[6] & 0x0f) | 0x40;
        bytes[8] = (bytes[8] & 0x3f) | 0x80;
        out.push(format!(
            "{:02x}{:02x}{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
            bytes[0], bytes[1], bytes[2], bytes[3],
            bytes[4], bytes[5],
            bytes[6], bytes[7],
            bytes[8], bytes[9],
            bytes[10], bytes[11], bytes[12], bytes[13], bytes[14], bytes[15]
        ));
    }
    out
}

fn secure_random_u32() -> u32 {
    let mut buf = [0u8; 4];
    if getrandom::fill(&mut buf).is_ok() {
        u32::from_le_bytes(buf)
    } else {
        // Fallback defensivo com nanosegundos caso a chamada de sistema falhe
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_nanos() as u64)
            .unwrap_or(0x1234_5678_9ABC_DEF0);
        (nanos ^ (nanos >> 32)) as u32
    }
}

/// Sorteia um índice no intervalo [0, pool_len) sem viés de módulo (Rejection Sampling).
fn secure_random_index(pool_len: usize) -> usize {
    if pool_len <= 1 {
        return 0;
    }
    let n = pool_len as u32;
    let zone = u32::MAX - (u32::MAX % n);
    loop {
        let r = secure_random_u32();
        if r < zone {
            return (r % n) as usize;
        }
    }
}
