//! Log simple de errores a ~/.local/share/dittox/error.log
use std::fs::OpenOptions;
use std::io::Write;

fn path() -> std::path::PathBuf {
    dirs::data_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("."))
        .join("dittox")
        .join("error.log")
}

pub fn log(context: &str, err: &str) {
    let p = path();
    if let Some(parent) = p.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    if let Ok(mut f) = OpenOptions::new().create(true).append(true).open(&p) {
        let _ = writeln!(f, "[{}] {}: {}", now(), context, err);
    }
}

fn now() -> String {
    // RFC3339-ish using SystemTime
    use std::time::{SystemTime, UNIX_EPOCH};
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    format!("{}", secs)
}
