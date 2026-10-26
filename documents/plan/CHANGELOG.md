# CHANGELOG — DittoX
Formato Keep-a-Changelog. Stack A Tauri+Rust.

## [0.1.0-M0] — 2026-10-14
### Added
- documents/analisis/: Ditto-original.md, DittoX-plan-steamos.md, comparativa-stacks.md, stack-A/B/C
- documents/plan/: 00-plan-A-overview.md, 01-M0-M1-detalle.md, 02-M2-M5-riesgos.md
- README.md, LICENSE GPL-3.0 placeholder, .gitignore, Cargo workspace
- src-tauri/: Cargo.toml, tauri.conf.json, src/main.rs, db.rs (SQLite FTS5 schema), clipboard.rs, hotkey.rs, paste.rs stubs
- src/: package.json, main.ts popup stub, index.html
- flatpak/com.haifishtein.DittoX.yml base
### Known limitations
- No compila aun: falta cargo tauri init completo, icons, capabilities. Stub M1 pendiente implementar.
- Wayland bg read + GameMode Decky pendientes M1/M3.
