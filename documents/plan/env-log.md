# LOG ambiente — DittoX (T00+)
Proposito: registrar TODO lo instalado/modificado desde T00 para poder desinstalar/limpiar.
Formato: 1 entrada por accion. No borrar entradas, solo agregar REVERT.

### 2026-10-14 — rustup install (parcial, interrumpido)
- Que: rustup + stable minimal
- Como: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain stable --profile minimal`
- Donde: ~/.cargo, ~/.rustup (creados 11:40)
- Por que: T00 toolchain
- Revert: `rustup self uninstall -y` + `rm -rf ~/.cargo ~/.rustup`
- Temp: ~/.rustup/downloads/*, ~/.cargo/.package-cache
- Estado: OK 2026-10-14 11:45 — rustc 1.99.0 + cargo 1.99.0 via `rustup toolchain install stable --force --profile minimal` tras manifest corrupto. Verificado `rustc --version` + `cargo --version`.

### 2026-10-14 — node20 install (~/.local, sin root)
- Que: Node v20.19.0 + npm 10.8.2
- Como: `curl -fsSL https://nodejs.org/dist/v20.19.0/node-v20.19.0-linux-x64.tar.xz -o /tmp/node.tar.xz` + `mkdir -p ~/.local/node20; tar -xJf /tmp/node.tar.xz -C ~/.local/node20 --strip-components=1` + `rm /tmp/node.tar.xz`
- Donde: ~/.local/node20/bin/node, ~/.local/node20/bin/npm
- Por que: T00 toolchain (Vite+Tauri CLI)
- Revert: `rm -rf ~/.local/node20` (+ quitar PATH)
- Temp: /tmp/node.tar.xz 25M (eliminado)
- Estado: OK — `node --version` v20.19.0, `npm --version` 10.8.2 con PATH=$HOME/.local/node20/bin
- Pendiente: export PATH en ~/.bashrc: `export PATH="$HOME/.cargo/bin:$HOME/.local/node20/bin:$PATH"`


## Template entrada
```
### YYYY-MM-DD HH:MM — <accion>
- Que: <paquete/file/config>
- Como: <comando exacto>
- Donde: <path>
- Por que: <Txx>
- Revert: <comando desinstalacion>
- Temp: <files temp generados>
```

## Historial

### 2026-10-14 — T00 env check (solo lectura)
- Que: verificacion toolchain
- Como: `rustc --version; cargo --version; node --version`
- Donde: /home/deck/Documents/git/DittoX
- Por que: T00
- Revert: N/A (solo lectura, no instalo nada)
- Temp: ninguno
- Resultado: rustc/cargo/node NOT FOUND. Ver env.md
