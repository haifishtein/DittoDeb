// T19a-e Tauri entrypoint — Desktop-only. Handlers + poll + hotkey toggle show/hide.
use tauri::menu::{Menu, MenuItem};
use tauri::tray::TrayIconBuilder;
use tauri::Manager;
mod clipboard;
mod errlog;
mod config;
mod hotkey;
mod ipc;
mod paste;
mod tray;

fn main() {
    // Single instance: avoid two DittoDeb processes/windows.
    let lock_path = dirs::data_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("."))
        .join("ditto-deb")
        .join("instance.lock");
    if let Some(parent) = lock_path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    let lock_file = std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .open(&lock_path)
        .ok();
    let mut already_running = false;
    if let Some(f) = &lock_file {
        use std::os::unix::io::AsRawFd;
        extern "C" {
            fn flock(fd: i32, operation: i32) -> i32;
        }
        const LOCK_EX: i32 = 2;
        const LOCK_NB: i32 = 4;
        if unsafe { flock(f.as_raw_fd(), LOCK_EX | LOCK_NB) } != 0 {
            already_running = true;
        }
    }
    if already_running {
        // Focus/show the existing instance window.
        if let Ok(out) = std::process::Command::new("xdotool")
            .args(["search", "--name", "^DittoDeb$"])
            .output()
        {
            for line in String::from_utf8_lossy(&out.stdout).lines() {
                let id = line.trim();
                if !id.is_empty() {
                    let _ = std::process::Command::new("xdotool")
                        .args(["windowmap", id])
                        .status();
                    let _ = std::process::Command::new("xdotool")
                        .args(["windowactivate", id])
                        .status();
                    break;
                }
            }
        }
        std::process::exit(0);
    }
    let _lock_file = lock_file;

    std::panic::set_hook(Box::new(|info| { errlog::log("panic", &info.to_string()); }));
    let _ = ditto_deb_core::init_db(&ditto_deb_core::db_path()).map_err(|e| errlog::log("init_db", &e.to_string()));
    tauri::Builder::default()
        .setup(|app| {
            let handle = app.handle().clone();
            std::thread::spawn(move || {
                let mut last = String::new();
                loop {
                    if let Some((text, html, kind)) = clipboard::poll_once(&mut last) {
                        match ditto_deb_core::init_db(&ditto_deb_core::db_path()) {
                            Ok(conn) => {
                                if let Err(e) = ditto_deb_core::insert_clip_full(&conn, &text, html.as_deref(), kind) {
                                    errlog::log("insert_clip", &e.to_string());
                                }
                            }
                            Err(e) => errlog::log("init_db", &e.to_string()),
                        }
                    }
                    std::thread::sleep(std::time::Duration::from_millis(500));
                }
            });
            hotkey::init();
            let handle2 = handle.clone();
            std::thread::spawn(move || {
                let rx = hotkey::receiver().lock().unwrap();
                for _ in rx.iter() {
                    if let Some(win) = handle2.get_webview_window("main") {
                        if win.is_visible().unwrap_or(false) {
                            let _ = win.hide();
                        } else {
                            let _ = win.show();
                            let _ = win.set_focus();
                            // Boost focus on X11 after the window is mapped (xdotool is a recommended dependency).
                            std::thread::spawn(|| {
                                std::thread::sleep(std::time::Duration::from_millis(500));
                                let _ = std::process::Command::new("xdotool")
                                    .args(["search", "--name", "^DittoDeb$", "windowactivate"])
                                    .status();
                            });
                        }
                    }
                }
            });

            // Tray icon + menu (Show/Hide/Quit)
            let tray_menu = Menu::with_items(
                app,
                &[
                    &MenuItem::with_id(app, "show", "Show DittoDeb", true, None::<&str>)?,
                    &MenuItem::with_id(app, "hide", "Hide", true, None::<&str>)?,
                    &MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?,
                ],
            )?;
            let _tray = TrayIconBuilder::new()
                .icon(app.default_window_icon().cloned().expect("default icon"))
                .menu(&tray_menu)
                .on_menu_event(|app, event| match event.id().as_ref() {
                    "show" => {
                        if let Some(w) = app.get_webview_window("main") {
                            let _ = w.show();
                            let _ = w.set_focus();
                            std::thread::spawn(|| {
                                std::thread::sleep(std::time::Duration::from_millis(500));
                                let _ = std::process::Command::new("xdotool")
                                    .args(["search", "--name", "^DittoDeb$", "windowactivate"])
                                    .status();
                            });
                        }
                    }
                    "hide" => {
                        if let Some(w) = app.get_webview_window("main") {
                            let _ = w.hide();
                        }
                    }
                    "quit" => app.exit(0),
                    _ => {}
                })
                .build(app)?;
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            ipc::search_clips_cmd,
            ipc::get_clip_cmd,
            ipc::toggle_pin_cmd,
            ipc::list_pinned_cmd,
            ipc::get_clip_detail_cmd,
            ipc::import_ditto_cmd,
            ipc::get_hotkey_cmd,
            ipc::set_hotkey_cmd,
            ipc::get_config_cmd,
            ipc::set_config_cmd,
            ipc::get_error_log_cmd,
            ipc::hide_window_cmd,
            paste::paste_clip_cmd,
            paste::paste_clip_plain_cmd
        ])
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                // Close button (X) hides to tray instead of quitting.
                api.prevent_close();
                let _ = window.hide();
            }
        })
        .run(tauri::generate_context!())
        .expect("DittoDeb Tauri run failed");
}
