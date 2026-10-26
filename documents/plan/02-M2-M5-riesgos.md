# M2-M5 + Riesgos SteamOS

## M2 multimedia (1-2sem)
- imagen png/jpg clipboard -> BLOB + thumbnail cache ~/.cache/dittox/thumbs
- html preview sanitizado
- uri-list archivos (solo paths, no copiar bytes)
- pins/favoritos, grupos/carpetas, editar/borrar, contadores
- limites: max 5000 clips, max 10MB imagen, expiracion 30d opt, ignorar apps (keepass, etc)
- settings UI: hotkey, tema, idioma es/en

## M3 GameMode — CANCELADO 2026-10-14 por decision usuario (Desktop-only)
- No Decky plugin, no chord Steam+V, no overlay Gamescope, no test GameMode.
- Razon: ahorro tokens + scope Desktop KDE Plasma Wayland/X11 unicamente.
- Si build Desktop arranca en Gaming, es bonus sin soporte.

## M4 paridad Ditto
- network sync LAN (opt, TLS + token), como Ditto friends
- SQLCipher opt, import Ditto.db sqlite, export json/csv
- temas Deck/Breeze, i18n es/en, atajos custom

## M5 distribucion
- Flathub: manifest + metainfo + screenshots
- AUR dittox-bin + dittox-git PKGBUILD
- Decky Store submit
- CI github actions: cargo clippy/fmt/test + tauri build + flatpak-builder lint

## Riesgos y mitigacion
1. Wayland bg read bloqueado -> Klipper D-Bus sync + portal + fallback XWayland poll. Doc en troubleshooting.
2. Gamescope captura input -> Decky + Steam Input, no hotkey global en GameMode.
3. Flatpak no deja wtype -> flatpak-spawn --host o build AUR nativo. Ofrecer ambos.
4. Anti-cheat flag -> solo paste manual texto, disclaimer.
5. GPL-3.0 -> LICENSE + NOTICE origen Ditto, mantener compatible.
