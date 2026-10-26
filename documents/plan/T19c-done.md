# T19c done — poll 500ms + hotkey en setup
main.rs setup: thread poll clipboard::poll_once -> dittox_core::insert_clip + hotkey::register_toggle.
Falta T19d-e: loop eventos hotkey show/hide + hide tras paste.
Verificar en dittox-dev: cargo check -p dittox 2>&1 | tail -n 20
