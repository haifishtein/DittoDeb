# Plan — DittoDeb: versión Linux Mint empaquetada como .deb

Fecha: 2026-10-26
Base: fork de DittoDeb (Tauri 2 + Rust + TypeScript + SQLite FTS5)
Objetivo: clon de Ditto (sabrogden/Ditto, GPL-3.0) para Linux Mint, distribuido como paquete `.deb`.

## 1. Contexto

- **Ditto original**: extensión de portapapeles para Windows (C++/MFC, Win32). Funcionalidades clave: monitor de portapapeles en background, historial persistente con búsqueda full-text, popup con hotkey global (`Ctrl + `` `), pegado en ventana anterior, multi-formato (texto/imagen/HTML/archivos), pins/favoritos, grupos, tray icon, autostart, import/export.
- **DittoDeb**: reimplementación para SteamOS (Arch + KDE Plasma). Stack Tauri 2 + Rust + TS + SQLite FTS5. Desktop-only (ADR-001), solo-GitHub (ADR-002). Ya tiene daemon clipboard, popup, tray, autostart, paste X11/Wayland, import Ditto.db.
- **DittoDeb**: adaptación de DittoDeb a **Linux Mint** (Ubuntu/Debian base) empaquetada como **.deb**, manteniendo stack y licencia GPL-3.0.

Linux Mint 22.x = Ubuntu 24.04 (noble). Entornos por defecto: Cinnamon/MATE/XFCE, **X11 por defecto** (Wayland experimental). Esto simplifica paste/clipboard respecto a Wayland puro.

## 2. Decisiones (ADRs DittoDeb)

- **D01 — Reutilizar stack Tauri 2** (no reescribir). Hereda toda la lógica de DittoDeb.
- **D02 — X11-first**: paste con `xdotool` (disponible en Mint por defecto-ish; se declara dependencia). Fallback `wtype` si `WAYLAND_DISPLAY`. Documentar limitaciones Wayland.
- **D03 — Renombrado completo** `ditto-deb` → `ditto-deb`/`DittoDeb` (crate, binario, rutas de datos/config, frontend, desktop file, identifier). Rutas: `~/.local/share/ditto-deb/ditto.db`, `~/.config/ditto-deb/config.json`.
- **D04 — Empaquetado via Tauri bundle** (`targets: ["deb"]`) + metadatos Debian (depends/recommends/section). Genera `.deb` en CI y local.
- **D05 — Tray con Ayatana AppIndicator** (`libayatana-appindicator3`) para Cinnamon.
- **D06 — Autostart por usuario** (`~/.config/autostart/ditto-deb.desktop`), no sistema.
- **D07 — Solo GitHub** (ADR-002): `.deb` adjunto a GitHub Releases. Sin apt repo propio. Instalación con `sudo apt install ./ditto-deb_*.deb` o `gdebi`.

## 3. Análisis comparativo

| Funcionalidad Ditto | DittoDeb | DittoDeb (Mint .deb) |
|---|---|---|
| Monitor clipboard bg | poll arboard 500ms | igual, X11 estable |
| Historial + FTS5 | SQLite + FTS5 | igual |
| Popup hotkey global | global-hotkey (X11) | igual; Wayland limitado |
| Pegar en ventana previa | xdotool/wtype | xdotool primario (Mint X11) |
| Multi-formato | texto/imagen/html | igual |
| Pins/grupos | pins.rs | igual |
| Tray icon | StatusNotifier | Ayatana AppIndicator |
| Autostart | .desktop | .desktop por usuario |
| Import Ditto.db | import.rs | igual |
| Empaquetado | tar.gz/Flatpak | **.deb** (Tauri bundle) |

## 4. Cambios técnicos respecto a DittoDeb

### 4.1 Renombrado (micro-tareas)
- `Cargo.toml` (workspace): members sin cambio; package `ditto-deb` → `ditto-deb` (crate bin).
- `src-tauri/Cargo.toml`: `name = "ditto-deb"`, dep `ditto-deb-core` (o mantener `ditto-deb-core`? Renombrar a `ditto-deb-core` para consistencia). `tauri.conf.json`: `productName = "DittoDeb"`, `identifier = "com.<maintainer>.DittoDeb"`, `bundle.targets = ["deb"]`.
- `crates/core`: crate name `ditto-deb-core`, rutas db/config `ditto-deb`.
- `config.rs`, `ipc.rs`, `errlog.rs`, `main.rs`: strings/rutas `ditto-deb` → `ditto-deb`.
- `paste.rs`: binarios igual (xdotool/wtype); logs `ditto-deb`.
- Frontend `src/`: título, package.json name, CSS.
- `flatpak/` y `.github/workflows/release.yml`: renombrar o dejar opcional; workflow principal pasa a build .deb.
- `README.md`, `LICENSE` (mantener GPL-3.0).

### 4.2 Integración Mint
- `.desktop`: `Name=DittoDeb`, `Exec=ditto-deb`, `Icon=ditto-deb`, `Categories=Utility;`.
- Iconos hicolor: `usr/share/icons/hicolor/{16x16,...,512x512,scalable}/apps/ditto-deb.png` (Tauri bundle puede incluir; verificar `bundle.icon`).
- AppIndicator: asegurar `libayatana-appindicator3-dev` en build deps y runtime dep.
- Autostart: crear `~/.config/autostart/ditto-deb.desktop` si `autostart=true` (ya existe lógica? verificar; si no, añadir).
- Dependencias runtime .deb:
  - `libwebkit2gtk-4.1-0`, `libgtk-3-0`, `libsoup-3.0-0`, `libayatana-appindicator3-1`, `libx11-6`
  - Recommends: `xdotool`, `wl-clipboard` (para Wayland)
- Build deps (CI/dev): `build-essential curl wget file pkg-config libwebkit2gtk-4.1-dev libsoup-3.0-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev`.

### 4.3 Configuración Tauri bundle .deb
En `tauri.conf.json`:
```json
"bundle": {
  "active": true,
  "icon": ["icons/icon.png"],
  "targets": ["deb"],
  "linux": {
    "deb": {
      "depends": ["libwebkit2gtk-4.1-0", "libgtk-3-0", "libsoup-3.0-0", "libayatana-appindicator3-1", "libx11-6"],
      "recommends": ["xdotool", "wl-clipboard"],
      "section": "utils",
      "priority": "optional"
    }
  }
}
```
(Verificar schema real de Tauri v2 antes de commitear; ajustar campos.)

### 4.4 CI / Release
- Workflow GitHub Actions en `ubuntu-22.04` (o 24.04):
  1. instalar deps sistema + Node 20 + Rust.
  2. `npm --prefix src install && npm --prefix src run build`
  3. `cargo test -p ditto-deb-core`
  4. `cargo tauri build` (genera .deb en `src-tauri/target/release/bundle/deb/`)
  5. subir `.deb` + checksums a Release (tag `v*`).
- Mantener ADR-002: solo GitHub, sin repos externos.

## 5. Milestones

- **M0 — Fork + plan + renombrado base** (este documento).
- **M1 — Build .deb local en Mint** y smoke test (instalar, lanzar, tray, hotkey).
- **M2 — Clipboard/paste X11** verificado en Cinnamon (copiar 20 textos, buscar, Enter pega en xed/gedit).
- **M3 — Packaging polish** (deps, iconos, desktop, autostart).
- **M4 — Release workflow .deb** + checksums.
- **M5 — QA multi-DE** (Cinnamon/MATE/XFCE, X11; Wayland experimental documentado).

## 6. Riesgos

1. **Tauri deb bundle schema**: campos `deb.depends` etc. deben validarse contra la versión instalada de tauri-cli. Mitigación: instalar tauri-cli y correr `cargo tauri build` temprano.
2. **Hotkey global en Wayland**: `global-hotkey` puede no capturar en Wayland. Mitigación: documentar X11 requerido para hotkey; en Wayland usar tray.
3. **Clipboard en Wayland**: arboard requiere compositor/portal. Mitigación: X11-first; documentar.
4. **AppIndicator en Cinnamon**: verificar que el icono de tray aparece (puede requerir `libayatana-appindicator3`).
5. **Dependencias .deb**: si faltan, `dpkg -i` falla; usar `apt install ./file.deb` para resolver. Documentar.
6. **Nombre de paquete**: Debian requiere minúsculas; Tauri puede derivar de `productName`/identifier. Verificar nombre final del `.deb`.

## 7. Criterios de aceptación

- `sudo apt install ./ditto-deb_0.1.0_amd64.deb` exitoso en Mint 22 limpio.
- App arranca, icono tray visible, hotkey `Ctrl+`` ` alterna popup.
- Copiar ≥20 textos, búsqueda fuzzy funciona, Enter pega en app previa (X11).
- Desinstalar con `sudo apt remove ditto-deb` no borra datos de usuario (opcional documentado).

## 8. Próximos pasos inmediatos

1. Instalar herramientas: `nodejs`, `npm`, `cargo install tauri-cli` (o `@tauri-apps/cli`).
2. Ejecutar renombrado completo (script sed + ajustes manuales).
3. Primer build: `npm --prefix src install && npm --prefix src run build && cargo test -p ditto-deb-core && cargo tauri build`.
4. Instalar .deb generado en Mint y smoke test.

## 9. Fuentes

- Ditto original: https://github.com/sabrogden/Ditto (GPL-3.0)
- DittoDeb: `/home/mike/ClineWorkSpace/DittoDeb` (commit 5e33e10)
- Linux Mint 22.3 (Ubuntu 24.04 noble), dpkg 1.22.6

## 10. Resultado de build (2026-10-26)

- `cargo check -p ditto-deb` OK.
- Frontend `npm run build` OK (src/dist).
- `tauri build` compilo binario release OK (`target/release/ditto-deb`), pero el bundler .deb fallo por falta de `libayatana-appindicator3-dev` (deteccion pkg-config). 
- Se genero .deb manual con `dpkg-deb` en `/tmp/ditto-deb_0.1.0_amd64.deb` (5.0M) y se valido con `dpkg-deb --info/--contents`.
- Para que Tauri genere .deb automaticamente en CI, instalar `libayatana-appindicator3-dev` en el runner (requiere sudo/apt).
