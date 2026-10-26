//! Clipboard monitor M1 — poll arboard 500ms + dedup sha256, ignora vacio.
//! Captura texto e imagenes. Wayland: requiere Klipper sync / portal. X11: CLIPBOARD.
use sha2::{Digest, Sha256};

pub fn hash_text(text: &str) -> String {
    let mut h = Sha256::new();
    h.update(text.as_bytes());
    format!("{:x}", h.finalize())
}

pub fn should_save(text: &str, last_hash: &str) -> bool {
    if text.trim().is_empty() || text.len() > 1_000_000 {
        return false;
    }
    hash_text(text) != last_hash
}

pub fn poll_once(last_hash: &mut String) -> Option<(String, Option<String>, &'static str)> {
    let mut cb = arboard::Clipboard::new().ok()?;
    // 1) Texto
    if let Ok(text) = cb.get_text() {
        if should_save(&text, last_hash) {
            *last_hash = hash_text(&text);
            return Some((text, None, "text"));
        }
    }
    // 2) Imagen
    if let Ok(img) = cb.get_image() {
        let bytes = img.bytes.into_owned();
        let mut h = Sha256::new();
        h.update(&bytes);
        let img_hash = format!("{:x}", h.finalize());
        if img_hash != *last_hash {
            *last_hash = img_hash;
            if let Ok(path) = ditto_deb_core::save_image_png(&bytes, img.width, img.height) {
                return Some(("[image]".to_string(), Some(path), "image"));
            }
        }
    }
    None
}
