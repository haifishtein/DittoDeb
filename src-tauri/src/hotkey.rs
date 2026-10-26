//! Hotkey global personalizable. Persiste en ~/.config/ditto-deb/hotkey.
//! Implementación propia sobre X11 (x11-dl) para soportar teclas no ASCII
//! como la 'ñ' (XK_ntilde), que global-hotkey no mapea.
use std::sync::mpsc;
use std::sync::{Mutex, OnceLock};
use std::time::Duration;
use x11_dl::xlib;
use x11_dl::keysym;

#[derive(Clone, Copy)]
pub struct Spec {
    mods: u32,
    keysym: u32,
}

enum Cmd {
    Register {
        spec: Spec,
        reply: mpsc::Sender<Result<(), String>>,
    },
}

static EVENT_TX: OnceLock<mpsc::Sender<()>> = OnceLock::new();
static EVENT_RX: OnceLock<Mutex<mpsc::Receiver<()>>> = OnceLock::new();
static CMD_TX: OnceLock<mpsc::Sender<Cmd>> = OnceLock::new();

const IGNORED_MODS: [u32; 4] = [
    0,
    xlib::Mod2Mask, // NumLock
    xlib::LockMask, // CapsLock
    xlib::Mod2Mask | xlib::LockMask,
];

fn config_path() -> std::path::PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("."))
        .join("ditto-deb")
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

fn parse_mods(tokens: &[&str]) -> u32 {
    let mut m = 0u32;
    for t in tokens {
        match t.to_lowercase().as_str() {
            "ctrl" | "control" => m |= xlib::ControlMask,
            "shift" => m |= xlib::ShiftMask,
            "alt" => m |= xlib::Mod1Mask,
            "super" | "meta" | "win" => m |= xlib::Mod4Mask,
            _ => {}
        }
    }
    m
}

fn parse_keysym(tok: &str) -> Result<u32, String> {
    let t = tok.to_lowercase();
    if t.len() == 1 {
        let c = t.chars().next().unwrap();
        if c.is_ascii_lowercase() {
            return Ok(match c {
                'a' => keysym::XK_a,
                'b' => keysym::XK_b,
                'c' => keysym::XK_c,
                'd' => keysym::XK_d,
                'e' => keysym::XK_e,
                'f' => keysym::XK_f,
                'g' => keysym::XK_g,
                'h' => keysym::XK_h,
                'i' => keysym::XK_i,
                'j' => keysym::XK_j,
                'k' => keysym::XK_k,
                'l' => keysym::XK_l,
                'm' => keysym::XK_m,
                'n' => keysym::XK_n,
                'o' => keysym::XK_o,
                'p' => keysym::XK_p,
                'q' => keysym::XK_q,
                'r' => keysym::XK_r,
                's' => keysym::XK_s,
                't' => keysym::XK_t,
                'u' => keysym::XK_u,
                'v' => keysym::XK_v,
                'w' => keysym::XK_w,
                'x' => keysym::XK_x,
                'y' => keysym::XK_y,
                'z' => keysym::XK_z,
                _ => return Err(format!("tecla no soportada: {tok}")),
            });
        }
        if c.is_ascii_digit() {
            return Ok(match c {
                '0' => keysym::XK_0,
                '1' => keysym::XK_1,
                '2' => keysym::XK_2,
                '3' => keysym::XK_3,
                '4' => keysym::XK_4,
                '5' => keysym::XK_5,
                '6' => keysym::XK_6,
                '7' => keysym::XK_7,
                '8' => keysym::XK_8,
                '9' => keysym::XK_9,
                _ => return Err(format!("tecla no soportada: {tok}")),
            });
        }
    }
    match t.as_str() {
        "backquote" | "`" => Ok(keysym::XK_quoteleft),
        "ñ" => Ok(keysym::XK_ntilde),
        "semicolon" => Ok(keysym::XK_semicolon),
        "comma" => Ok(keysym::XK_comma),
        "period" => Ok(keysym::XK_period),
        "slash" => Ok(keysym::XK_slash),
        "minus" => Ok(keysym::XK_minus),
        "equal" => Ok(keysym::XK_equal),
        "space" => Ok(keysym::XK_space),
        "enter" | "return" => Ok(keysym::XK_Return),
        "escape" | "esc" => Ok(keysym::XK_Escape),
        "tab" => Ok(keysym::XK_Tab),
        "backspace" => Ok(keysym::XK_BackSpace),
        "f1" => Ok(keysym::XK_F1),
        "f2" => Ok(keysym::XK_F2),
        "f3" => Ok(keysym::XK_F3),
        "f4" => Ok(keysym::XK_F4),
        "f5" => Ok(keysym::XK_F5),
        "f6" => Ok(keysym::XK_F6),
        "f7" => Ok(keysym::XK_F7),
        "f8" => Ok(keysym::XK_F8),
        "f9" => Ok(keysym::XK_F9),
        "f10" => Ok(keysym::XK_F10),
        "f11" => Ok(keysym::XK_F11),
        "f12" => Ok(keysym::XK_F12),
        _ => Err(format!("tecla no soportada: {tok}")),
    }
}

pub fn parse_hotkey(s: &str) -> Result<Spec, String> {
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
    let keysym = parse_keysym(key_token)?;
    Ok(Spec { mods, keysym })
}

fn event_loop(cmd_rx: mpsc::Receiver<Cmd>, event_tx: mpsc::Sender<()>) {
    let xlib = match xlib::Xlib::open() {
        Ok(x) => x,
        Err(_) => return,
    };
    unsafe {
        let display = (xlib.XOpenDisplay)(std::ptr::null());
        if display.is_null() {
            return;
        }
        let root: xlib::Window = (xlib.XDefaultRootWindow)(display);
        let mut supported_rtrn: i32 = 0;
        (xlib.XkbSetDetectableAutoRepeat)(display, 1, &mut supported_rtrn);
        (xlib.XSelectInput)(display, root, xlib::KeyPressMask);

        let mut current_keycode: Option<u32> = None;
        let mut current_mods: u32 = 0;

        loop {
            while let Ok(cmd) = cmd_rx.try_recv() {
                match cmd {
                    Cmd::Register { spec, reply } => {
                        if let Some(kc) = current_keycode {
                            for m in IGNORED_MODS {
                                (xlib.XUngrabKey)(display, kc as i32, (current_mods | m) as u32, root);
                            }
                        }
                        let keycode = (xlib.XKeysymToKeycode)(display, spec.keysym.into());
                        if keycode == 0 {
                            let _ = reply.send(Err(format!("tecla no soportada: {}", spec.keysym)));
                            current_keycode = None;
                            continue;
                        }
                        let mut ok = true;
                        for m in IGNORED_MODS {
                            let res = (xlib.XGrabKey)(
                                display,
                                keycode as i32,
                                (spec.mods | m) as u32,
                                root,
                                0,
                                xlib::GrabModeAsync,
                                xlib::GrabModeAsync,
                            );
                            if res == xlib::BadAccess as i32 {
                                ok = false;
                                break;
                            }
                        }
                        if !ok {
                            for m in IGNORED_MODS {
                                (xlib.XUngrabKey)(display, keycode as i32, (spec.mods | m) as u32, root);
                            }
                            let _ = reply.send(Err("hotkey en uso por otra aplicacion".to_string()));
                            current_keycode = None;
                            continue;
                        }
                        current_keycode = Some(keycode as u32);
                        current_mods = spec.mods;
                        let _ = reply.send(Ok(()));
                    }
                }
            }

            if (xlib.XPending)(display) > 0 {
                let mut event: xlib::XEvent = std::mem::zeroed();
                (xlib.XNextEvent)(display, &mut event);
                if event.get_type() == xlib::KeyPress {
                    let keycode = event.key.keycode;
                    if Some(keycode as u32) == current_keycode {
                        let event_mods = event.key.state
                            & (xlib::ControlMask
                                | xlib::ShiftMask
                                | xlib::Mod1Mask
                                | xlib::Mod4Mask);
                        if event_mods
                            == (current_mods
                                & (xlib::ControlMask
                                    | xlib::ShiftMask
                                    | xlib::Mod1Mask
                                    | xlib::Mod4Mask))
                        {
                            let _ = event_tx.send(());
                        }
                    }
                }
            } else {
                std::thread::sleep(Duration::from_millis(10));
            }
        }
    }
}

pub fn init() {
    let (etx, erx) = mpsc::channel();
    let _ = EVENT_TX.set(etx.clone());
    let _ = EVENT_RX.set(Mutex::new(erx));
    let (ctx, crx) = mpsc::channel();
    let _ = CMD_TX.set(ctx.clone());
    std::thread::spawn(move || event_loop(crx, etx));

    let s = load_config();
    if let Ok(spec) = parse_hotkey(&s) {
        let (rtx, _rrx) = mpsc::channel();
        let _ = ctx.send(Cmd::Register {
            spec,
            reply: rtx,
        });
    }
}

pub fn set_hotkey(s: &str) -> Result<(), String> {
    let spec = parse_hotkey(s)?;
    let cmd_tx = CMD_TX.get().ok_or_else(|| "hotkey no inicializado".to_string())?;
    let (rtx, rrx) = mpsc::channel();
    cmd_tx
        .send(Cmd::Register {
            spec,
            reply: rtx,
        })
        .map_err(|_| "hilo hotkey caido".to_string())?;
    let res = rrx.recv().map_err(|_| "sin respuesta hotkey".to_string())?;
    if res.is_ok() {
        save_config(s);
    }
    res
}

pub fn current() -> String {
    load_config()
}

pub fn receiver() -> &'static Mutex<mpsc::Receiver<()>> {
    EVENT_RX.get().expect("hotkey no inicializado")
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
