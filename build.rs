use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=assets/icon.ico");
    println!("cargo:rerun-if-changed=build.rs");

    if env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("windows") {
        return;
    }

    let manifest_dir = match env::var("CARGO_MANIFEST_DIR") {
        Ok(d) => PathBuf::from(d),
        Err(_) => return,
    };
    let ico_path = manifest_dir.join("assets").join("icon.ico");
    let Ok(ico_bytes) = fs::read(&ico_path) else {
        panic!("assets/icon.ico not found at {}", ico_path.display());
    };

    let res_bytes = build_win32_icon_res(&ico_bytes)
        .unwrap_or_else(|e| panic!("failed to build Windows icon resource: {e}"));

    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR missing"));
    let res_path = out_dir.join("axolote_icon.res");
    fs::write(&res_path, res_bytes).expect("failed to write axolote_icon.res");

    if env::var("CARGO_CFG_TARGET_ENV").as_deref() == Ok("msvc") {
        println!(
            "cargo:rustc-link-arg-bin=axolote_suite={}",
            res_path.display()
        );
    }
}

fn push_res_entry(out: &mut Vec<u8>, res_type: u16, res_id: u16, memory_flags: u16, data: &[u8]) {
    // RESOURCEHEADER with numeric Type and Name (HeaderSize = 32)
    out.extend_from_slice(&(data.len() as u32).to_le_bytes()); // DataSize
    out.extend_from_slice(&32u32.to_le_bytes()); // HeaderSize
    out.extend_from_slice(&0xFFFFu16.to_le_bytes()); // Type ordinal marker
    out.extend_from_slice(&res_type.to_le_bytes()); // Type ID
    out.extend_from_slice(&0xFFFFu16.to_le_bytes()); // Name ordinal marker
    out.extend_from_slice(&res_id.to_le_bytes()); // Name ID
    out.extend_from_slice(&0u32.to_le_bytes()); // DataVersion
    out.extend_from_slice(&memory_flags.to_le_bytes()); // MemoryFlags
    out.extend_from_slice(&0x0409u16.to_le_bytes()); // LanguageId (en-US)
    out.extend_from_slice(&0u32.to_le_bytes()); // Version
    out.extend_from_slice(&0u32.to_le_bytes()); // Characteristics
    out.extend_from_slice(data);
    let pad = (4 - (out.len() & 3)) & 3;
    out.resize(out.len() + pad, 0);
}

fn build_win32_icon_res(ico: &[u8]) -> Result<Vec<u8>, String> {
    if ico.len() < 6 {
        return Err("ICO file too small".into());
    }
    let reserved = u16::from_le_bytes([ico[0], ico[1]]);
    let ico_type = u16::from_le_bytes([ico[2], ico[3]]);
    let count = u16::from_le_bytes([ico[4], ico[5]]) as usize;
    if reserved != 0 || ico_type != 1 || count == 0 {
        return Err("Invalid ICO header".into());
    }
    let dir_end = 6 + count * 16;
    if ico.len() < dir_end {
        return Err("ICO directory truncated".into());
    }

    let mut res = Vec::with_capacity(ico.len() + 256);
    // Mandatory null 32-byte header at start of .res file
    res.extend_from_slice(&[0u8; 4]); // DataSize = 0
    res.extend_from_slice(&32u32.to_le_bytes()); // HeaderSize = 32
    res.extend_from_slice(&0xFFFFu16.to_le_bytes());
    res.extend_from_slice(&0u16.to_le_bytes());
    res.extend_from_slice(&0xFFFFu16.to_le_bytes());
    res.extend_from_slice(&0u16.to_le_bytes());
    res.extend_from_slice(&[0u8; 16]);

    let mut group_data = Vec::with_capacity(6 + count * 14);
    group_data.extend_from_slice(&0u16.to_le_bytes()); // idReserved
    group_data.extend_from_slice(&1u16.to_le_bytes()); // idType (1 = icon)
    group_data.extend_from_slice(&(count as u16).to_le_bytes()); // idCount

    const RT_ICON: u16 = 3;
    const RT_GROUP_ICON: u16 = 14;

    for i in 0..count {
        let off = 6 + i * 16;
        let entry = &ico[off..off + 16];
        let size = u32::from_le_bytes([entry[8], entry[9], entry[10], entry[11]]) as usize;
        let img_offset = u32::from_le_bytes([entry[12], entry[13], entry[14], entry[15]]) as usize;
        let img_end = img_offset
            .checked_add(size)
            .ok_or_else(|| "ICO image offset overflow".to_string())?;
        if img_end > ico.len() {
            return Err(format!("ICO frame {} out of bounds", i + 1));
        }
        let icon_id = (i + 1) as u16;
        push_res_entry(
            &mut res,
            RT_ICON,
            icon_id,
            0x1010,
            &ico[img_offset..img_end],
        );

        // GRPICONDIRENTRY: first 12 bytes of ICONDIRENTRY + 2-byte nID
        group_data.extend_from_slice(&entry[0..12]);
        group_data.extend_from_slice(&icon_id.to_le_bytes());
    }

    push_res_entry(&mut res, RT_GROUP_ICON, 1, 0x1030, &group_data);
    Ok(res)
}
