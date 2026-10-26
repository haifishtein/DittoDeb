# Comparativa de Stacks — DittoX para SteamOS
Fecha: 2026-10-14
Objetivo: Clon Ditto (GPL-3.0) para SteamOS (Arch+KDE+Gamescope). Local-first.

## Resumen ejecutivo

| # | Stack | Idea | Veredicto |
|---|-------|------|-----------|
| A | Tauri2+Rust+TS | Daemon Rust ligero + UI web nativa | Recomendado: ligero, moderno, Flatpak-friendly |
| B | Qt6/C++ nativo | App KDE nativa como Ditto original | Max integracion Plasma, mas lento |
| C | Python+PySide6 | Prototipo rapido Qt en Python | Rapido inicio, fragil producto Deck |

Elige A si priorizas ligereza+UI moderna, B si priorizas integracion KDE total, C si quieres MVP en dias para luego reescribir.
