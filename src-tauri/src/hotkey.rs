//! Hotkey global personalizable. Persiste en ~/.config/dittox/hotkey.
use global_hotkey::hotkey::{Code, HotKey, Modifiers};
use std::sync::Mutex;

static MANAGER: Mutex<Option<global_hotkey::GlobalHotKeyManager>> = Mutex::new(None);
static CURRENT: Mutex<Option<HotKey>> = Mutex::new(None);

fn config_path() -> std::path::PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("."))
        .join("dittox")
        .join("hotkey")
}

pub fn load_config() -> String {
    std::fs::read_to_string(config_path()).unwrap_or_else(|_| "ctrl+ñ".to_string())
}

fn save_config(s: &str) {
    let p = config_path();
    if let Some(parent) = p.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    let _ = std::fs::write(p, s);
}

fn parse_mods(tokens: &[&str]) -> Option<Modifiers> {
    let mut m = Modifiers::empty();
    let mut any = false;
    for t in tokens {
        match t.to_lowercase().as_str() {
            "ctrl" | "control" => {
                m |= Modifiers::CONTROL;
                any = true;
            }
            "shift" => {
                m |= Modifiers::SHIFT;
                any = true;
            }
            "alt" => {
                m |= Modifiers::ALT;
                any = true;
            }
            "super" | "meta" | "win" => {
                m |= Modifiers::SUPER;
                any = true;
            }
            _ => {}
        }
    }
    if any {
        Some(m)
    } else {
        None
    }
}

fn parse_code(tok: &str) -> Result<Code, String> {
    let t = tok.to_lowercase();
    if t.len() == 1 {
        let c = t.chars().next().unwrap();
        if c.is_ascii_lowercase() {
            // map a-z
            return Ok(match c {
                'a' => Code::KeyA,
                'b' => Code::KeyB,
                'c' => Code::KeyC,
                'd' => Code::KeyD,
                'e' => Code::KeyE,
                'f' => Code::KeyF,
                'g' => Code::KeyG,
                'h' => Code::KeyH,
                'i' => Code::KeyI,
                'j' => Code::KeyJ,
                'k' => Code::KeyK,
                'l' => Code::KeyL,
                'm' => Code::KeyM,
                'n' => Code::KeyN,
                'o' => Code::KeyO,
                'p' => Code::KeyP,
                'q' => Code::KeyQ,
                'r' => Code::KeyR,
                's' => Code::KeyS,
                't' => Code::KeyT,
                'u' => Code::KeyU,
                'v' => Code::KeyV,
                'w' => Code::KeyW,
                'x' => Code::KeyX,
                'y' => Code::KeyY,
                'z' => Code::KeyZ,
                _ => return Err(format!("tecla no soportada: {tok}")),
            });
        }
        if c.is_ascii_digit() {
            return Ok(match c {
                '0' => Code::Digit0,
                '1' => Code::Digit1,
                '2' => Code::Digit2,
                '3' => Code::Digit3,
                '4' => Code::Digit4,
                '5' => Code::Digit5,
                '6' => Code::Digit6,
                '7' => Code::Digit7,
                '8' => Code::Digit8,
                '9' => Code::Digit9,
                _ => return Err(format!("tecla no soportada: {tok}")),
            });
        }
    }
    match t.as_str() {
        "backquote" | "`" => Ok(Code::Backquote),
        "ñ" | "semicolon" => Ok(Code::Semicolon),
        "space" => Ok(Code::Space),
        "enter" => Ok(Code::Enter),
        "escape" | "esc" => Ok(Code::Escape),
        "tab" => Ok(Code::Tab),
        "f1" => Ok(Code::F1),
        "f2" => Ok(Code::F2),
        "f3" => Ok(Code::F3),
        "f4" => Ok(Code::F4),
        "f5" => Ok(Code::F5),
        "f6" => Ok(Code::F6),
        "f7" => Ok(Code::F7),
        "f8" => Ok(Code::F8),
        "f9" => Ok(Code::F9),
        "f10" => Ok(Code::F10),
        "f11" => Ok(Code::F11),
        "f12" => Ok(Code::F12),
        _ => Err(format!("tecla no soportada: {tok}")),
    }
}

pub fn parse_hotkey(s: &str) -> Result<HotKey, String> {
    let parts: Vec<&str> = s
        .split('+')
        .map(str::trim)
        .filter(|x| !x.is_empty())
        .collect();
    if parts.is_empty() {
        return Err("hotkey vacio".to_string());
    }
    let (mods_tokens, key_token) = if parts.len() == 1 {
        (&parts[..0], parts[0])
    } else {
        (&parts[..parts.len() - 1], parts[parts.len() - 1])
    };
    let mods = parse_mods(mods_tokens);
    let code = parse_code(key_token)?;
    Ok(HotKey::new(mods, code))
}

pub fn init() {
    // no-op log on start
    let s = load_config();
    if let Ok(hk) = parse_hotkey(&s) {
        let mut mgr_guard = MANAGER.lock().unwrap();
        if mgr_guard.is_none() {
            *mgr_guard = global_hotkey::GlobalHotKeyManager::new().ok();
        }
        if let Some(mgr) = mgr_guard.as_ref() {
            let _ = mgr.register(hk);
            *CURRENT.lock().unwrap() = Some(hk);
        }
    }
}

pub fn set_hotkey(s: &str) -> Result<(), String> {
    let hk = match parse_hotkey(s) { Ok(h) => h, Err(e) => { crate::errlog::log("hotkey_parse", &e); return Err(e); } };
    let mut mgr_guard = MANAGER.lock().unwrap();
    if mgr_guard.is_none() {
        *mgr_guard = Some(global_hotkey::GlobalHotKeyManager::new().map_err(|e| e.to_string())?);
    }
    let mgr = mgr_guard.as_ref().unwrap();
    // unregister current
    if let Some(cur) = CURRENT.lock().unwrap().take() {
        let _ = mgr.unregister(cur);
    }
    mgr.register(hk).map_err(|e| { crate::errlog::log("hotkey", &e.to_string()); e.to_string() })?;
    *CURRENT.lock().unwrap() = Some(hk);
    save_config(s);
    Ok(())
}

pub fn current() -> String {
    load_config()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse_default() {
        let hk = parse_hotkey("ctrl+ñ").unwrap();
        let _ = hk;
    }
    #[test]
    fn parse_invalid() {
        assert!(parse_hotkey("").is_err());
        assert!(parse_hotkey("ctrl+zzz").is_err());
    }
}
