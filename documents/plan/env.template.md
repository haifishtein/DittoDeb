# DittoDeb — Configuracion de ambiente (Deck/SteamOS)
Fecha: 2026-10-14 / Editar segun tu Deck. No commitear secretos.

## Paths
REPO_ROOT=/home/deck/Documents/git/DittoDeb
DOCS=$REPO_ROOT/documents
PLAN=$DOCS/plan
ANALISIS=$DOCS/analisis
SRC_TAURI=$REPO_ROOT/src-tauri
SRC_UI=$REPO_ROOT/src
FLATPAK_DIR=$REPO_ROOT/flatpak
DECKY_DIR=$REPO_ROOT/decky
DB_PATH=~/.local/share/ditto-deb/ditto.db
CACHE_THUMBS=~/.cache/ditto-deb/thumbs
AUTOSTART=~/.config/autostart/com.haifishtein.DittoDeb.desktop

## Toolchain requerida (T00 env check)
- rustc/cargo (>=1.75, rustup default stable)
- node/npm (>=20, para Vite+Tauri CLI)
- cargo-tauri: cargo install tauri-cli --version ^2
- flatpak + flatpak-builder (Flathub)
- wayland: wtype + wl-clipboard (wl-copy/wl-paste)
- x11: xdotool + xclip/xsel
- kde: klipper, kglobalshortcuts, qdbus
- decky: Decky Loader (solo M3 GameMode)

## Alias sugeridos (pegar en ~/.bashrc o ~/.zshrc)
alias dx='cd /home/deck/Documents/git/DittoDeb'
alias dx-docs='ls -R /home/deck/Documents/git/DittoDeb/documents/plan /home/deck/Documents/git/DittoDeb/documents/analisis'
alias dx-dev='cd /home/deck/Documents/git/DittoDeb && cargo tauri dev'
alias dx-lint='cargo clippy -- -D warnings && cargo fmt --check && cargo test'
alias dx-flat='flatpak-builder --force-clean build-dir flatpak/com.haifishtein.DittoDeb.yml'
alias dx-db='sqlite3 ~/.local/share/ditto-deb/ditto.db "SELECT id,substr(text,1,60),created_at FROM clips ORDER BY id DESC LIMIT 20;"'
alias dx-clip='wl-paste 2>/dev/null || xclip -o -selection clipboard 2>/dev/null || echo NO_CLIP_TOOL'

## Env vars
export DITTO_DEB_DB=~/.local/share/ditto-deb/ditto.db
export DITTO_DEB_HOTKEY='Ctrl+`'
export RUST_BACKTRACE=1

## Notas Deck
- SteamOS inmutable: no pacman -S en base, usar flatpak o distrobox/toolbox para Qt/dev deps.
- Desktop Mode = KDE Plasma Wayland por defecto, X11 opcional.
- GameMode = Gamescope fullscreen, sin tray/hotkey global, requiere Decky M3.
- Flatpak paste fuera sandbox: flatpak-spawn --host wtype / xdotool.
