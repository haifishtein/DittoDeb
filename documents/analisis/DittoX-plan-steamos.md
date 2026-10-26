# DittoX — Clon de Ditto para SteamOS

## Objetivo
Clon funcional de Ditto (sabrogden/Ditto, GPL-3.0) para SteamOS (Arch Linux + KDE Plasma + Gamescope).
Local-first, sin cloud, sin telemetría.

## Decisión de stack (propuesta — falta tu confirmación)

Recomendado: **Tauri 2 + Rust + TypeScript/React (o Svelte) + SQLite**

Por qué:
- Rust da acceso nativo a clipboard X11/Wayland, hotkeys globales, SQLite, daemon ligero — ideal para Steam Deck con poca RAM.
- Tauri genera ventana popup rápida + tray icon + autostart, mucho más liviano que Electron (~10MB vs ~150MB).
- Frontend web = UI moderna fácil de iterar (búsqueda fuzzy, preview imágenes/HTML, temas Deck).
- Flatpak-friendly, que es el formato preferido en SteamOS (inmutable, no se rompe con updates).
- Licencia compatible con GPL-3.0 si derivamos lógica de Ditto (debemos mantener GPL-3.0 o compatible).

Alternativas descartadas (por ahora):
- **Electron**: más pesado, peor para Deck en Game Mode. Solo si priorizas velocidad de desarrollo JS puro.
- **Qt6/C++ nativo**: máxima integración Plasma, pero desarrollo más lento. Bueno si quieres paridad 1:1 con Ditto C++. Considerar si equipo domina Qt.
- **Python + PyQt/PySide**: prototipado rápido, pero packaging Flatpak + daemon + hotkeys es más frágil.
- **Go + Wails**: viable, similar a Tauri pero ecosistema clipboard Linux más maduro en Rust (`arboard`, `clipboard-rs`).

> Pregunta para ti: ¿prefieres Tauri+Rust (recomendado), Electron, Qt C++, o Python? Por defecto avanzo con Tauri.

## Arquitectura propuesta

```
┌─────────────┐     D-Bus / wlr-data-control / X11     ┌──────────────┐
│ KDE Plasma  │◄──────────────────────────────────────►│  dittox-     │
│ Gamescope   │        clipboard events                │  daemon (Rust)│
│ Steam Input │                                        └──────┬───────┘
└─────────────┘                                               │ SQLite
       ▲ hotkey (KGlobalShortcut / Gamescope hook)            ▼
┌─────────────┐  paste via wtype/ydotool/kdotool / Ctrl+V  ┌──────────────┐
│ Popup UI    │──────────────────────────────────────────►│ ditto.db     │
│ (Tauri)     │                                           │ (sqlcipher?) │
└─────────────┘                                           └──────────────┘
       ▲ tray (StatusNotifier / AppIndicator)
```

Componentes:
1. `dittox-daemon` (Rust): vigila clipboard (poll + eventos), guarda en SQLite, expone API local (Tauri IPC / Unix socket / HTTP localhost).
2. `dittox-ui` (Tauri WebView): popup búsqueda (`Ctrl+`` `), lista virtualizada, preview, editor, settings.
3. `dittox-tray`: StatusNotifierItem para Desktop Mode.
4. Integración SteamOS:
   - Desktop Mode: autostart `.desktop`, atajo global KDE, tray OK.
   - Game Mode: Decky Loader plugin o shortcut `steam://` + chord de mando (ej. `Steam + V`), overlay limitado. Pegado via `ydotool`/`wtype` según compositor.
5. Persistencia: `~/.local/share/dittox/ditto.db` (SQLite + FTS5 para búsqueda, SQLCipher opcional).
6. Formatos: `text/plain`, `text/html`, `image/png`, `text/uri-list` (archivos). Límite configurable (ej. no guardar >50MB imágenes).

## Roadmap

- **M0 — Docs y scaffold**: este documento + `documents/` + scaffold Tauri + Flatpak manifest. ✅ en curso
- **M1 — MVP Desktop**: daemon clipboard X11/Wayland texto plano + SQLite + popup búsqueda + Enter para pegar + tray + autostart. Funciona en Desktop Mode.
- **M2 — Multimedia**: imágenes, HTML preview, pins/favoritos, grupos, editar/borrar, límites, ignorar apps.
- **M3 — Game Mode**: atajo mando, overlay Decky, pegado en juegos (notas: anti-cheat — solo texto, opt-in).
- **M4 — Paridad Ditto**: network sync, cifrado, import/export desde Ditto `.db`, temas, i18n (es/en).
- **M5 — Distribución**: Flathub, AUR `dittox-bin`, Decky Store.

## Riesgos SteamOS específicos
1. Wayland restringe lectura clipboard en background (privacidad). Necesita portal o `wlr-data-control` (solo en compositor, no Flatpak sin permisos). Fallback: modo XWayland + polling + extensión KWin/Klipper sync.
2. Gamescope fullscreen captura teclado/mando — hotkey global difícil. Solución: Decky plugin + Steam Input chord.
3. Flatpak sandbox bloquea `ydotool`/`wtype` para auto-paste. Necesita `flatpak-spawn --host` o modo no-sandbox (AUR/native).
4. Anti-cheat en juegos puede flaggear inyección de teclas — pegar solo bajo demanda explícita, nunca automático.
5. GPL-3.0: si portas código de Ditto, DittoX debe ser GPL-3.0. Previsto.

## Siguiente paso inmediato
1. Confirmar stack (Tauri vs alternativas).
2. Generar scaffold: `cargo-tauri` + estructura `src-tauri/`, `src/`, `flatpak/`, `decky/`.
3. Implementar M1 daemon clipboard.

## Pregunta
¿Qué stack eliges y quieres que empiece con el scaffold M0/M1 en `src-tauri/` + `src/`?
