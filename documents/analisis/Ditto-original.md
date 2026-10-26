# Análisis — Ditto original (sabrogden/Ditto)

Fuente: https://github.com/sabrogden/Ditto

## Qué es
Ditto es una extensión del portapapeles estándar de Windows.
Guarda cada item copiado (Ctrl+C) en una base de datos para recuperarlo después.
Filosofía Local-First: sin login, sin cloud, sin telemetría.

- Stars: ~7.3k, Forks: ~462, Issues: ~775, Commits: ~1583
- Licencia: GPL-3.0
- Web: ditto-cp.sourceforge.io
- Distribución Windows: Installer, Portable, Chocolatey (`choco install ditto`), Winget (`winget install -e --id Ditto.Ditto`), Windows Store.

## Uso básico
1. Ejecutar Ditto (queda en system tray).
2. Copiar cosas normalmente.
3. Abrir Ditto con click en icono o Hot Key por defecto `Ctrl + `` ` (back-quote).
4. Doble-click o Enter sobre un item para pegarlo en la ventana anterior.

## Tipos de datos soportados
Cualquier formato del clipboard de Windows: texto plano, imágenes, HTML, formatos custom.

## Tech stack original (no reutilizable directo en Linux)
- Lenguaje: C++ con MFC (Microsoft Foundation Classes), COM/ATL (`CP_Main.vcxproj`, `CP_Main.sln`, `CP_Main.idl`, `CP_Main.rgs`, `resource.h`, `CP_Main.rc`).
- UI: Win32 nativa, system tray, hotkeys globales, ventana de búsqueda/pegado.
- Módulos en repo:
  - `src/` — app principal
  - `Shared/` — código compartido
  - `DittoSetup/` — instalador
  - `DittoUtil/`, `Addins/DittoUtil` — utilidades/addins
  - `EncryptDecrypt/`, `ICU_Loader/`, `FocusHighlight/`, `focusdll/`, `U3Stop/`, `res/`, `Debug/`
- Persistencia: base de datos local (históricamente SQLite + cifrado opcional).
- Integración OS: Clipboard Viewer Chain / APIs Win32 clipboard, hooks de teclado para hotkey global, paste simulado (SendKeys / focus tracking).
- Build: MSVC `.vcxproj/.sln`, `packages.config` (NuGet).

## Funcionalidades a clonar (MVP)
1. Monitor de portapapeles en background (daemon).
2. Historial persistente con búsqueda full-text.
3. Popup rápido con hotkey global (`Ctrl + `` ` configurable).
4. Pegado al pulsar Enter/doble-click en ventana anterior.
5. Soporte multi-formato: texto, imagen, HTML, archivos.
6. Favoritos / pins, grupos / carpetas, edición de clips.
7. Sync opcional entre dispositivos (Ditto tiene network sync) — v2.
8. Encriptación DB, expiración, límite de tamaño, ignorar apps.
9. Tray icon + autostart.
10. Import/export.

## Restricciones Windows que NO existen en SteamOS
- No hay MFC/Win32 clipboard chain. En Linux hay:
  - X11: `CLIPBOARD` + `PRIMARY`, ownership, `xclip/xsel`.
  - Wayland: `wl-copy/wl-paste`, protocolos `wlr-data-control` (privilegiado), portales.
  - KDE Plasma (SteamOS Desktop Mode): Klipper + D-Bus, KGlobalShortcuts.
  - SteamOS Game Mode (Gamescope): sin tray, sin hotkeys globales normales, fullscreen.
- No hay `.exe` installer. En SteamOS: Flatpak (recomendado), AppImage, o paquete pacman/AUR, o plugin Decky Loader.
