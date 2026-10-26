# T19a/b done — 2026-10-14
- main.rs: fn main + Builder + invoke_handler(search/get/paste) + init ditto-deb-core db.
- ipc.rs delega a ditto-deb-core (no duplica db::).
- Falta: loop hotkey/poll/hide (T19c-e), cargo check verde en ditto-deb-dev.
Corre en ditto-deb-dev: cargo check -p ditto-deb 2>&1 | tail -n 30