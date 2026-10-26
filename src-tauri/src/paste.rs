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

#[tauri::command]
pub fn paste_clip_cmd(app: AppHandle, id: i64) -> Result<(), String> {
    let conn = dittox_core::init_db(&dittox_core::db_path()).map_err(|e| e.to_string())?;
    let mut stmt = conn
        .prepare("SELECT text FROM clips WHERE id=?1")
        .map_err(|e| e.to_string())?;
    let text: String = stmt
        .query_row([id], |r| r.get(0))
        .map_err(|e| e.to_string())?;
    let mut cb = arboard::Clipboard::new().map_err(|e| { crate::errlog::log("paste_clipboard", &e.to_string()); e.to_string() })?;
    cb.set_text(&text).map_err(|e| { crate::errlog::log("paste_set_text", &e.to_string()); e.to_string() })?;
    // Let Wayland/X11 settle clipboard ownership before hiding + pasting.
    std::thread::sleep(std::time::Duration::from_millis(50));
    // 1. Hide popup FIRST so focus returns to previous app (Kate/KWrite).
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
