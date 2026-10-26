# ADR-001 — Desktop-only, sin GameMode
Fecha: 2026-10-14 / Estado: aceptado
Contexto: clon Ditto para SteamOS. GameMode (Gamescope fullscreen) requiere Decky plugin + Steam Input chord + overlay, costo 1-2sem + anti-cheat riesgo.
Decision: solo Desktop Mode KDE Plasma (Wayland/X11). Sin integracion visual, hotkeys, tray, ni paste especifico Gaming.
Consecuencias:
- No se desarrolla decky/, no chord Steam+V, no test Gamescope.
- Tray + hotkey Ctrl+` + wtype/xdotool solo Desktop. Si funciona en Gaming, bonus sin soporte.
- Ahorro: ~T30 completo + riesgos 2 y 4 mitigados por scope (no aplican).
- Extra Desktop necesario: nada nuevo. Lo ya hecho (Klipper sync, KGlobalShortcut doc, StatusNotifier, autostart .desktop) es suficiente.
