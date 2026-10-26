//! Configuracion avanzada persistente en ~/.config/ditto-deb/config.json
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub max_clips: i64,
    pub max_text_bytes: usize,
    pub max_image_mb: u64,
    pub poll_interval_ms: u64,
    pub paste_delay_ms: u64,
    pub search_case_sensitive: bool,
    pub autostart: bool,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            max_clips: 1000,
            max_text_bytes: 1_000_000,
            max_image_mb: 10,
            poll_interval_ms: 500,
            paste_delay_ms: 150,
            search_case_sensitive: false,
            autostart: true,
        }
    }
}

fn config_path() -> std::path::PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("."))
        .join("ditto-deb")
        .join("config.json")
}

pub fn load() -> AppConfig {
    let p = config_path();
    if let Ok(data) = fs::read_to_string(&p) {
        if let Ok(cfg) = serde_json::from_str::<AppConfig>(&data) {
            return cfg;
        }
    }
    let cfg = AppConfig::default();
    let _ = save(&cfg);
    cfg
}

pub fn save(cfg: &AppConfig) -> Result<(), String> {
    let p = config_path();
    if let Some(parent) = p.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    let data = serde_json::to_string_pretty(cfg).map_err(|e| e.to_string())?;
    fs::write(p, data).map_err(|e| { crate::errlog::log("config_save", &e.to_string()); e.to_string() })?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn roundtrip() {
        let c = AppConfig::default();
        let s = serde_json::to_string(&c).unwrap();
        let c2: AppConfig = serde_json::from_str(&s).unwrap();
        assert_eq!(c.max_clips, c2.max_clips);
    }
}
