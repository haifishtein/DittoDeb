# DittoDeb — Politica de ejecucion (workspace guardrails)
Fecha: 2026-10-14 / Owner: deck
Scope permitido: /home/deck/Documents/git/DittoDeb/** (+ ~/.local/node20/bin y ~/.cargo/bin para toolchain T00 ya instalada)

## Permitido sin preguntar
- read_files dentro de REPO_ROOT
- editor create/edit dentro de REPO_ROOT/documents/**, REPO_ROOT/src/**, REPO_ROOT/src-tauri/**, REPO_ROOT/flatpak/**
- run_commands SOLO si cwd o paths absolutos estan dentro de /home/deck/Documents/git/DittoDeb
  Ej: `ls /home/deck/Documents/git/DittoDeb/...`, `cat .../DittoDeb/...`

## Requiere confirmacion explicita
- run_commands fuera de DittoDeb (~/.cargo, ~/.local, /tmp, flatpak host, Konsole)
- instalacion/desinstalacion toolchain, cambios PATH, ~/.bashrc, autostart ~/.config
- git add/commit/push, rm -rf, curl|sh,owers escritura fuera de repo
- acceso red (fetch_web_content ya permitido solo lectura docs)

## Prohibido
- `rm -rf /`, `rm -rf ~`, `pacman -S`, `sudo`, `flatpak install --system`, `chmod +x /usr/*`
- ejecucion de binarios descargados fuera de ~/.local/node20 y ~/.cargo
- exfiltrar env-log secretos (no hay, pero regla)

## Nota tecnica
Yo no puedo auto-bloquear shell del IDE; esto es contrato documentado. Si un comando viola scope, paralo y pideme replan.
Ver: documents/plan/env.template.md, env-log.md
