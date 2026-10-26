# T00 resultado — 2026-10-14 Deck
Terminal observado:
- rustc: NOT FOUND
- cargo: NOT FOUND
- node: NOT FOUND
- flatpak/wtype/wl-copy/xdotool: sin confirmar (shell integration fallo segundo comando)

Accion requerida (manual, SteamOS inmutable):
1. Rust: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh` luego `rustup default stable`
2. Node 20+: `flatpak install flathub org.nodejs.Node20` o `mise`/`nvm` en ~/.local, no pacman base
3. flatpak-builder: `flatpak install flathub org.flatpak.Builder`
4. wtype/wl-clipboard/xdotool: `flatpak run --host` o distrobox `distrobox create --name dev`
5. Re-ejecutar: `rustc --version; cargo --version; node --version; flatpak --version; which wtype wl-copy xdotool`

Estado: BLOQUEADO para compilar hasta instalar toolchain. Docs/scaffold OK sin compilar.
Ver: env.template.md para paths/alias.
