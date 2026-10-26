# Plan A — DittoX Tauri+Rust para SteamOS
Fecha: 2026-10-14 / Stack confirmado: A
Origen: clon Ditto sabrogden/Ditto GPL-3.0. Local-first.

## Milestones
- M0 scaffold + docs + LICENSE + Flatpak base (hoy)
- M1 MVP Desktop texto: daemon+SQLite FTS5+popup Ctrl+`+tray+autostart
- M2 multimedia: imagen/HTML/pins/grupos/editar/limites
- M3 GameMode Decky plugin + chord mando + paste seguro
- M4 paridad Ditto: sync/cifrado/import Ditto.db/temas/i18n
- M5 distribucion Flathub/AUR/Decky Store

## Principios
- Siempre planes + docs en documents/plan/ y documents/analisis/
- Local-first, sin cloud/telemetria
- GPL-3.0 (heredado Ditto si portamos logica)
- Desktop Mode ONLY (decision 2026-10-14: sin integracion visual/hotkeys GameMode, sin Decky, sin chord mando)
- Si lo Desktop funciona en Gaming por casualidad, bien, pero no se desarrolla ni testea para Gamescope
- Flatpak primero, AUR nativo para paste fuera sandbox
