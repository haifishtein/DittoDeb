# T50 — GitHub Releases only (redefinido 2026-10-14 por ADR-002)
Antes: Flathub submit. Ahora: SOLO GitHub, sin remotos.

## Distribucion
- GitHub Releases: `ditto-deb-v0.x-linux-x86_64.tar.gz` (bin + .desktop + README install) + opcional AppImage.
- Install: descargar asset o `git clone https://github.com/haifishtein/DittoDeb && cargo build --release -p ditto-deb`.
- No Flathub PR, no AUR, no Decky Store.

## Pre-requisitos
- [ ] cargo test 4 passed + cargo check verde en ditto-deb-dev
- [ ] icon.png + UI final + version en Cargo/tauri.conf
- [ ] GitHub Actions: build release + upload asset (solo github.com, sin registros externos)

## Flatpak local (opcional, no publicar)
- `flatpak-builder --force-clean build-dir flatpak/com.haifishtein.DittoDeb.yml` solo para probar en Deck.

