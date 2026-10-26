## B — Qt6 / C++ nativo Plasma

App Qt Widgets/QML C++17/20. Equivalente a portar Ditto MFC a Qt: QClipboard, KGlobalShortcuts, KStatusNotifierItem, KConfig.

Pros:
- Integracion Plasma perfecta: CLIPBOARD+PRIMARY, Klipper D-Bus, atajos KDE, tray nativo, autostart .desktop, Breeze/Deck.
- Paridad con Ditto: mismo lenguaje, portar grupos/expiracion/SQL/cifrado directo.
- Rendimiento: nativo, arranque <150ms, 40-80MB RAM, ideal daemon.
- Acceso directo X11/Wayland si instalas nativo AUR, sin pelear portals para pegar.
- Qt abstrae Wayland mejor que Rust manual.

Contras:
- Desarrollo lento: UI QML, busqueda fuzzy, preview HTML, settings = boilerplate vs web.
- Packaging: Flatpak Qt runtime KDE ~800MB base compartido. SteamOS inmutable -> friccion, mejor AUR.
- GameMode tampoco lo resuelve: igual necesitas Decky.
- Build CMake + Qt6 dev necesita contenedor, no ensuciar base.
- Menos ejemplos clipboard+Decky que Rust/TS.

Esfuerzo: M0 1d, M1 texto 1-2sem, M2 multimedia 2-3sem. Bueno si quieres app sistema KDE para siempre.
