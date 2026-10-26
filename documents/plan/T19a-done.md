# T19a/b done — 2026-10-14
- main.rs: fn main + Builder + invoke_handler(search/get/paste) + init dittox-core db.
- ipc.rs delega a dittox-core (no duplica db::).
- Falta: loop hotkey/poll/hide (T19c-e), cargo check verde en dittox-dev.
Corre en dittox-dev: cargo check -p dittox 2>&1 | tail -n 30