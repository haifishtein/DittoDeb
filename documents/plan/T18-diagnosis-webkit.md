# T18 diagnostico webkit — 2026-10-14 12:15
Error host Konsole:
`Package javascriptcoregtk-4.1 was not found` + `webkit2gtk-sys`, `soup3-sys` compilando.
Causa: Tauri2 Linux necesita libs sistema: webkit2gtk-4.1, gtk3, soup3, javascriptcoregtk.
En SteamOS inmutable no hay `pacman -S` directo sin `steamos-readonly disable`.

Opciones (Desktop-only):
1. Dev en Distrobox Arch (recomendado, no toca base): `distrobox create -n dittox-dev -i archlinux:latest` + dentro `sudo pacman -S webkit2gtk-4.1 gtk3 libsoup3 pkg-config` + cargo check.
2. Separar core sin Tauri (T18a): crate `dittox-core` solo rusqlite/sha2/arboard para `cargo test` sin webkit. Tauri UI solo compila en Flatpak SDK (que ya trae webkit).
3. Steamos readonly disable (no recomendado, se pierde en update): `sudo steamos-readonly disable; sudo pacman -S ...`

Siguiente micro: T18a split core (1 file Cargo + move db/clipboard). Sin instalar nada sistema aun.
