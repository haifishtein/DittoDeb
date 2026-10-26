# T18 fix cc linker — 2026-10-14 12:25
Error /tmp/dittox-test.log: `error: linker cc not found` en num-traits/proc-macro2.
Causa: rusqlite bundled-full necesita compilar C (libsqlite3-sys cc crate). Falta gcc/base-devel en host.

Fix host Konsole (elige 1):
A) Distrobox (recomendado, no toca base inmutable):
```
distrobox create -n dittox-dev -i archlinux:latest
distrobox enter dittox-dev
sudo pacman -Syu base-devel pkg-config webkit2gtk-4.1 gtk3 libsoup3 --noconfirm
export PATH="$HOME/.cargo/bin:$HOME/.local/node20/bin:$PATH"
cd /home/deck/Documents/git/DittoX && cargo test -p dittox-core
```
B) Steamos readonly disable (se pierde en update, no recomendado):
```
sudo steamos-readonly disable
sudo pacman -Syu base-devel --noconfirm
sudo steamos-readonly enable
```

Nota: en Flatpak SDK ya viene gcc, por eso M5 Flathub no tendra este problema. Solo dev host.
