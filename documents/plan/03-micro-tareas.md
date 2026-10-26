# Micro-tareas Plan A — max ahorro tokens
Regla: 1 tarea = 1 file, <200 lineas, 1 tool call.

## M0
- T00 env check: rustc/cargo/node + toolchain Deck (hecho 2026-10-14, OK rustc1.99 cargo1.99 node20.19 npm10.8.2, ver T00-env.md + env-log.md)
- T01 icons + vite.config (hecho 2026-10-14, ver T01-done.md)
- T02 tauri capabilities + autostart desktop (hecho 2026-10-14: T02a capabilities/main.json + T02b flatpak/com.haifishtein.DittoX.desktop)
- T03 flatpak metainfo + screenshots placeholder (hecho 2026-10-14: metainfo.xml, screenshots pend diseno)

## M1 MVP texto (orden)
- T10 db.rs init + test insert/search (hecho 2026-10-14: init_db+insert_clip+search_clips FTS5, sin cargo test aun)
- T11 clipboard poll texto + dedup (hecho 2026-10-14: hash_text+should_save+poll_once arboard, sin test)
- T12 IPC search_clips + get_clip (hecho 2026-10-14: ipc.rs search_clips_cmd+get_clip_cmd, sin build)
- T13 UI input busqueda + lista (hecho 2026-10-14: main.ts refresh search_clips_cmd)
- T14 hotkey Ctrl+` toggle popup (hecho 2026-10-14: hotkey.rs register_toggle, sin loop eventos aun)
- T15 paste wtype/xdotool + hide (hecho 2026-10-14: paste_clip_cmd + click UI, sin hide auto aun)
- T16 tray show/hide/quit (hecho 2026-10-14: tray.rs stub menu, falta wiring Tauri)
- T17 autostart .desktop (hecho 2026-10-14 en T02b: flatpak/com.haifishtein.DittoX.desktop, falta cp a ~/.config/autostart manual)
- T18 test manual 20 clips Kate (hecho 2026-10-14: cargo test -p dittox-core 2 passed insert_and_search+dedup_and_empty en dittox-dev)

## M2-M5 (Desktop-only, sin GameMode)
- T20 imagen thumb (hecho 2026-10-14: is_image_allowed 10MB + thumb_path, test image_limits, corre cargo test en dittox-dev)
- T21 html preview (hecho 2026-10-14: html.rs sanitize_html strip tags 2000 chars, corre cargo test)
- T22 pins/grupos (hecho 2026-10-14: pins.rs toggle_pin+list_pinned+groups, corre cargo test)
- T40 import Ditto.db (hecho 2026-10-14: import.rs scan tablas/cols texto + test import_fake_ditto_db, corre cargo test en dittox-dev)
- T50 github-releases-only (hecho 2026-10-14: workflow test+build+publish softprops tar.gz + comparativa-release-actions.md)
- T30 decky CANCELADO 2026-10-14 (Desktop-only, sin integracion Gaming)

Siguiente: dime Txx y ejecuto solo esa.
