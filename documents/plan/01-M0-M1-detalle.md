# M0-M1 Detalle Tecnico — Tauri+Rust

## M0 scaffold (hoy)
Archivos:
- Cargo.toml workspace + src-tauri/Cargo.toml (tauri2, rusqlite+fts5, arboard, clipboard-rs, global-hotkey, tray-icon, dirs, serde, tokio)
- src-tauri/tauri.conf.json (popup 900x600, centered, alwaysOnTop, singleInstance, tray, autostart)
- src/ Vite+TS: App.tsx popup busqueda, main.ts, styles Deck dark
- flatpak/com.haifishtein.DittoDeb.yml (runtime freedesktop 23.08, WebKitGTK, permisos wayland+x11+klipper+host paste opt)
- LICENSE GPL-3.0 + README.md raiz
- documents/plan/*.md (este set)

DB schema v1:
CREATE TABLE clips(id INTEGER PK, text TEXT, html TEXT, image BLOB, kind TEXT, app TEXT, created_at INT, pinned INT DEFAULT 0);
CREATE VIRTUAL TABLE clips_fts USING fts5(text, content=clips, content_rowid=id);
Triggers ai/ad/au para sync FTS.

## M1 MVP Desktop (3-5d)
1. daemon clipboard: poll 500ms + eventos arboard, dedup hash, ignora vacio/password managers, guarda texto plano.
2. IPC Tauri: cmd search_clips(query,limit), get_clip(id), delete_clip(id), toggle_pin(id), set_clipboard(id) + paste.
3. UI popup: input busqueda, lista virtualizada, Enter pega (set clipboard + wtype/xdotool Ctrl+V + auto-hide + foco previo), Esc cierra, Ctrl+` toggle.
4. Tray: show/hide, quit, StatusNotifier Desktop Mode.
5. Autostart: .desktop ~/.config/autostart.
6. Paste: detecta WAYLAND_DISPLAY -> wtype, sino xdotool. Flatpak -> flatpak-spawn --host. Documentar.
7. Test manual Desktop Mode KDE X11 luego Wayland.

Criterio done M1: copiar 20 textos, abrir popup <200ms, buscar fuzzy, Enter pega en Kate/KWrite, sobrevive reboot via autostart.
