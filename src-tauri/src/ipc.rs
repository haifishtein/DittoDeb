//! T12 IPC Tauri — search_clips + get_clip por id. Path DB via DITTOX_DB o ~/.local/share/dittox/ditto.db
use serde::Serialize;

#[derive(Serialize)]
pub struct ClipItem {
    pub id: i64,
    pub text: String,
}

#[tauri::command]
pub fn search_clips_cmd(q: String, limit: i64) -> Result<Vec<ClipItem>, String> {
    let conn = dittox_core::init_db(&dittox_core::db_path()).map_err(|e| e.to_string())?;
    let rows = dittox_core::search_clips(&conn, &q, limit).map_err(|e| e.to_string())?;
    Ok(rows
        .into_iter()
        .map(|(id, text)| ClipItem { id, text })
        .collect())
}

#[tauri::command]
pub fn get_clip_cmd(id: i64) -> Result<Option<String>, String> {
    let conn = dittox_core::init_db(&dittox_core::db_path()).map_err(|e| e.to_string())?;
    let mut stmt = conn
        .prepare("SELECT text FROM clips WHERE id=?1")
        .map_err(|e| e.to_string())?;
    let mut rows = stmt.query([id]).map_err(|e| e.to_string())?;
    if let Some(r) = rows.next().map_err(|e| e.to_string())? {
        Ok(Some(r.get(0).map_err(|e: rusqlite::Error| e.to_string())?))
    } else {
        Ok(None)
    }
}

#[tauri::command]
pub fn toggle_pin_cmd(id: i64) -> Result<bool, String> {
    let conn = dittox_core::init_db(&dittox_core::db_path()).map_err(|e| e.to_string())?;
    dittox_core::pins::toggle_pin(&conn, id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn list_pinned_cmd() -> Result<Vec<ClipItem>, String> {
    let conn = dittox_core::init_db(&dittox_core::db_path()).map_err(|e| e.to_string())?;
    let rows = dittox_core::pins::list_pinned(&conn, 100).map_err(|e| e.to_string())?;
    Ok(rows
        .into_iter()
        .map(|(id, text)| ClipItem { id, text })
        .collect())
}

#[tauri::command]
pub fn import_ditto_cmd(path: String) -> Result<usize, String> {
    let conn = dittox_core::init_db(&dittox_core::db_path()).map_err(|e| e.to_string())?;
    dittox_core::import::import_ditto_db(&conn, &path).map_err(|e| e.to_string())
}

#[derive(Serialize)]
pub struct ClipDetail {
    pub id: i64,
    pub text: String,
    pub html: Option<String>,
    pub kind: String,
}

#[tauri::command]
pub fn get_clip_detail_cmd(id: i64) -> Result<Option<ClipDetail>, String> {
    let conn = dittox_core::init_db(&dittox_core::db_path()).map_err(|e| e.to_string())?;
    let detail = dittox_core::get_clip_detail(&conn, id).map_err(|e| e.to_string())?;
    Ok(detail.map(|(text, html, kind)| ClipDetail {
        id,
        text,
        html,
        kind,
    }))
}

#[tauri::command]
pub fn get_hotkey_cmd() -> String {
    crate::hotkey::current()
}

#[tauri::command]
pub fn set_hotkey_cmd(hotkey: String) -> Result<(), String> {
    crate::hotkey::set_hotkey(&hotkey)
}

#[tauri::command]
pub fn get_config_cmd() -> crate::config::AppConfig {
    crate::config::load()
}

#[tauri::command]
pub fn set_config_cmd(cfg: crate::config::AppConfig) -> Result<(), String> {
    crate::config::save(&cfg)
}

#[tauri::command]
pub fn get_error_log_cmd() -> String {
    let p = dirs::data_dir().unwrap_or_default().join("dittox").join("error.log");
    std::fs::read_to_string(p).unwrap_or_default()
}
