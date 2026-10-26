# ADR-002 — Solo GitHub, sin remotos
Fecha: 2026-10-14 / Estado: aceptado
Decision usuario: no quiero instalaciones remotas, todo se debe instalar de GitHub.
Consecuencias:
- T50 Flathub submit CANCELADO. No Flathub, no AUR, no Decky Store, no Winget/Choco.
- Distribucion: solo GitHub Releases con binarios + instrucciones `git clone + cargo build` o descargar .tar.gz/.AppImage desde Releases.
- Flatpak yml/metainfo se mantienen como build local opcional (`flatpak-builder build-dir`), pero NO se publica.
- Toolchain: rustup (sh.rustup.rs) + Node tarball nodejs.org ya instalados se mantienen; a futuro preferir releases GitHub (ej. tauri-cli desde github.com/tauri-apps/tauri).
- CI: solo GitHub Actions (build + test + release assets), sin submit externo.
