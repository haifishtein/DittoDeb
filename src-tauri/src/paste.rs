//! Paste fix: hide popup FIRST (return focus), then Ctrl+V.
//! Wayland: `wtype -M ctrl v -m ctrl` (no existe `wtype paste`).
//! X11: `xdotool key ctrl+v`. Flatpak: `flatpak-spawn --host <cmd>`.
use tauri::{AppHandle, Manager};

fn is_wayland() -> bool {
    std::env::var("WAYLAND_DISPLAY").is_ok()
}

fn key_args() -> (&'static str, Vec<&'static str>) {
    if is_wayland() {
        ("wtype", vec!["-M", "ctrl", "v", "-m", "ctrl"])
    } else {
        ("xdotool", vec!["key", "ctrl+v"])
    }
}

fn fetch_clip(id: i64) -> Result<(String, Option<String>), String> {
    let conn = ditto_deb_core::init_db(&ditto_deb_core::db_path()).map_err(|e| e.to_string())?;
    let mut stmt = conn
        .prepare("SELECT text, html FROM clips WHERE id=?1")
        .map_err(|e| e.to_string())?;
    let (text, html): (String, Option<String>) = stmt
        .query_row([id], |r| Ok((r.get(0)?, r.get(1)?)))
        .map_err(|e| e.to_string())?;
    Ok((text, html))
}

fn do_paste(app: AppHandle, id: i64, plain: bool) -> Result<(), String> {
    let (text, html) = fetch_clip(id)?;
    let mut cb = arboard::Clipboard::new().map_err(|e| { crate::errlog::log("paste_clipboard", &e.to_string()); e.to_string() })?;
    if plain || html.as_deref().map(|h| h.trim().is_empty()).unwrap_or(true) {
        cb.set_text(&text).map_err(|e| { crate::errlog::log("paste_set_text", &e.to_string()); e.to_string() })?;
    } else if let Some(h) = html {
        cb.set_html(h, Some(text)).map_err(|e| { crate::errlog::log("paste_set_html", &e.to_string()); e.to_string() })?;
    } else {
        cb.set_text(&text).map_err(|e| { crate::errlog::log("paste_set_text", &e.to_string()); e.to_string() })?;
    }
    // Let Wayland/X11 settle clipboard ownership before hiding + pasting.
    std::thread::sleep(std::time::Duration::from_millis(50));
    // 1. Hide popup FIRST so focus returns to previous app.
    if let Some(win) = app.get_webview_window("main") {
        let _ = win.hide();
    }
    // 2. Small delay for WM focus restore, else Ctrl+V goes nowhere.
    let _cfg = crate::config::load();
    std::thread::sleep(std::time::Duration::from_millis(_cfg.paste_delay_ms));
    // 3. Simulate Ctrl+V in target app.
    let (bin, args) = key_args();
    let in_flatpak = std::path::Path::new("/.flatpak-info").exists();
    let status = if in_flatpak {
        let mut full = vec!["--host", bin];
        full.extend(args);
        std::process::Command::new("flatpak-spawn")
            .args(full)
            .status()
    } else {
        std::process::Command::new(bin).args(args).status()
    };
    status
        .map(|_| ())
        .map_err(|e| { crate::errlog::log("paste_key", &format!("{bin}: {e}")); format!("paste key failed ({bin}): {e}") })
}

#[tauri::command]
pub fn paste_clip_cmd(app: AppHandle, id: i64) -> Result<(), String> {
    do_paste(app, id, false)
}

#[tauri::command]
pub fn paste_clip_plain_cmd(app: AppHandle, id: i64) -> Result<(), String> {
    do_paste(app, id, true)
}
