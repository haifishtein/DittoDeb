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
    std::panic::set_hook(Box::new(|info| { errlog::log("panic", &info.to_string()); }));
    let _ = ditto_deb_core::init_db(&ditto_deb_core::db_path());
    tauri::Builder::default()
        .setup(|app| {
            let handle = app.handle().clone();
            std::thread::spawn(move || {
                let mut last = String::new();
                loop {
                    if let Some((text, html, kind)) = clipboard::poll_once(&mut last) {
                        if let Ok(conn) = ditto_deb_core::init_db(&ditto_deb_core::db_path()) {
                            if let Err(e) = ditto_deb_core::insert_clip_full(&conn, &text, html.as_deref(), kind) {
                                errlog::log("insert_clip", &e.to_string());
                            }
                        } else {
                            errlog::log("init_db", "failed");
                        }
                    }
                    std::thread::sleep(std::time::Duration::from_millis(500));
                }
            });
            hotkey::init();
            let handle2 = handle.clone();
            std::thread::spawn(move || {
                use global_hotkey::GlobalHotKeyEvent;
                for event in GlobalHotKeyEvent::receiver().iter() {
                    if event.state == global_hotkey::HotKeyState::Pressed {
                        if let Some(win) = handle2.get_webview_window("main") {
                            if win.is_visible().unwrap_or(false) {
                                let _ = win.hide();
                            } else {
                                let _ = win.show();
                                let _ = win.set_focus();
                            }
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
            paste::paste_clip_cmd
        ])
        .run(tauri::generate_context!())
        .expect("DittoDeb Tauri run failed");
}
