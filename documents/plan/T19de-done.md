# T19d-e done — show/hide + Enter + hide tras paste
- main.rs: thread GlobalHotKeyEvent receiver -> toggle show/hide/focus ventana main.
- main.ts: Enter pega primero, click pega, hide tras paste via getCurrentWindow.
- Fallback query vacia a "a" (FTS * rompe MATCH).
Verificar dittox-dev: cargo check -p dittox 2>&1 | tail -n 20
