# T50 done — release solo-GitHub 2026-10-15
- `.github/workflows/release.yml`: 2 jobs (build+publish) en ubuntu-22.04.
- build: System deps WebKit+Tauri, Node20 build UI (`src/dist`), `cargo test -p dittox-core`, `cargo build --release -p dittox` (binario real, antes solo core).
- stage: `pkg/dittox-linux-x86_64/` con `dittox` + `.desktop` (Exec=dittox) + README+LICENSE+INSTALL.txt, tar `dittox-linux-x86_64.tar.gz`.
- publish: `softprops/action-gh-release@v2` + `generate_release_notes:true` (Opcion A comparativa-release-actions).
- fix: `src-tauri/tauri.conf.json` `frontendDist` `../dist` -> `../src/dist` (Vite outDir default `src/dist`, antes CI rompia).
- trigger: `push tags v*`. Solo github.com, sin Flathub/AUR (ADR-002).
- Probar: `git tag v0.1.0-test -d` no; en CI real con `v0.1.0`. Local: `npm --prefix src run build && cargo build --release -p dittox`.
