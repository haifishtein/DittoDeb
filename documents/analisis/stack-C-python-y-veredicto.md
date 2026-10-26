## C — Python + PySide6

Python 3.11+ + PySide6: QApplication.clipboard(), sqlite3 FTS5, pynput/evdev hotkeys, pystray/SNi tray.

Pros:
- Prototipo en 1-3 dias. Sin compilar, iteracion instantanea en Deck.
- Qt te da clipboard/tray sin C++.
- Facil, muchos ejemplos.

Contras:
- Fragil producto: daemon siempre activo 150-250MB RAM, arranque 800ms-2s, deps pip PySide6 ~150MB.
- Hotkeys: pynput flojo Wayland, evdev/uinput pide root/input group, se rompe Flatpak. Necesitarias helpers Rust/C igual.
- Packaging SteamOS: Flatpak Python pesado-lento; AUR python-pyside6 se rompe con updates inmutables.
- 100k clips + imagenes: OK texto, flojo preview/cache vs Rust/C++.
- Alto riesgo reescritura en M2.

Uso recomendado: spike 2 dias para validar, luego reescribir en A. No final salvo herramienta personal.

## Tabla comparativa

- RAM idle: A 60-120MB, B 40-80MB, C 150-250MB
- Arranque popup: A <200ms, B <150ms, C 800ms-2s
- Plasma nativo: A medio-manual, B perfecto, C bueno pero hotkey flojo
- Wayland bg read: A dificil, B medio-Qt abstrae, C medio pero hotkey mal
- Tray: A ok, B nativo, C fragil
- GameMode: todos necesitan Decky
- Auto-paste: A --host, B nativo AUR, C permisos root
- UI moderna: A web excelente, B QML medio, C Designer medio
- Flatpak: A facil, B medio-pesado, C pesado
- Dev M1: A 3-5d, B 1-2sem, C 1-3d
- Largo plazo: A bueno, B excelente si sabes Qt, C deuda alta

## Recomendacion

- Producto Deck real: A Tauri+Rust.
- App sistema KDE para siempre: B Qt/C++.
- Validar manana: C spike luego A.

Pendiente tu decision: [ ] A [ ] B [ ] C [ ] A+C hibrido
Siguiente: M0 scaffold + LICENSE GPL-3.0 + README + Flatpak/AUR base.
Dime letra y genero scaffold M0/M1 en documents/plan/.
