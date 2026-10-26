# T18 parcial — 2026-10-14 12:05
- Fix1: quitado clipboard-rs 0.1 (todo yanked crates.io) — solo arboard queda.
- Fix2: rusqlite fts5 no existe como feature; FTS5 viene en bundled. Cambiado a bundled-full (ver docs.rs rusqlite 0.31 features).
- cargo check en curso (descargando 200+ crates Tauri, tarda 5-15min Deck). Terminal sandbox no devuelve output largo.
- Pendiente: ver tail errores compilacion (tauri/ipc/tray wiring), cargo test db, npm build UI.
- Nota: no bloquear M2 hasta check verde.
