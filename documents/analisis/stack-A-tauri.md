## A — Tauri2 + Rust + TypeScript

Backend Rust (daemon, SQLite FTS5, clipboard, hotkey, tray) + frontend TS (popup Ctrl+`, busqueda fuzzy, preview). WebView sistema, no Chromium.

Pros:
- Ligero Deck: 10-20MB bin, 60-120MB RAM vs Electron 200-400MB.
- Crates maduros: arboard, clipboard-rs, x11rb, wayland-client, global-hotkey, tray-icon, rusqlite.
- Flatpak-friendly: WebKitGTK runtime, permisos Klipper/Wayland via Flatseal.
- UI rapida: Vite + lista virtualizada, temas Deck, i18n facil.
- Distribucion: Flathub + AUR + AppImage. flatpak-spawn --host para wtype/ydotool.
- GPL-3.0 compatible.

Contras:
- Wayland restringe lectura background (portal/wlr-data-control). Hay que codearlo.
- GameMode Gamescope sin tray/hotkey: necesita plugin Decky + chord Steam+V.
- Auto-paste Flatpak requiere --host o build AUR nativo.
- Curva Rust.

Estructura:
src-tauri/src/clipboard.rs, db.rs, hotkey.rs, paste.rs + src/ UI + flatpak/ + decky/

Esfuerzo: M0 0.5d, M1 texto Desktop 3-5d, M2 multimedia 1-2sem, M3 Decky 1-2sem.
