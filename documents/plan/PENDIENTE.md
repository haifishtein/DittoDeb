# PENDIENTE — DittoDeb (2026-10-26, Desktop-only + solo-GitHub)
Verificado: T00-T40 + T19a-e hechos; commit final 5e33e10 en main; core tests 7/7; clippy/fmt limpios; npm build OK.

## 1. Wiring Tauri — HECHO 2026-10-14
- [x] T19a main Builder + handlers
- [x] T19b ipc delega a ditto-deb-core
- [x] T19c poll 500ms + insert
- [x] T19d hotkey toggle show/hide
- [x] T19e Enter + hide tras paste + fallback "a"
- [x] cargo check -p ditto-deb sin errores en ditto-deb-dev

## 2. UI popup (resta polish, no bloquea)
- [x] CSS tema Deck dark aplicado; pins UI; polish responsive/scrollbar/focus (hecho).
- [x] Lista virtualizada + preview imagen/html (hecho).
- [x] `npm run build` corrido OK (src/dist).


## 3. Core tests
- [x] 4 tests (insert/search, dedup, image_limits, import_fake)
- [x] sanitize_html + toggle_pin tests existen; clippy/fmt limpios.


## 4. Assets y autostart
- [x] icon.png 1024 generado; screenshots placeholder + metainfo actualizado.
- [x] cp .desktop a autostart manual (DittoDeb.desktop en ~/.config/autostart).

## 5. Git + Releases
- [x] Commit M0+M1+M2 (5e33e10) con COMMIT-FINAL.txt.
- [x] T50 workflow release.yml (hecho 2026-10-15: build binario real + tar.gz instalable, ver T50-done.md).

Siguiente: todo lo principal completado; pendientes opcionales: screenshots/metainfo, release publish.
