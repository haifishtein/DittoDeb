# Comparativa Release Actions — solo GitHub (T50)
Fecha: 2026-10-14 / Contexto: DittoDeb Desktop-only, tags v*.

## Opcion A — softprops/action-gh-release (recomendada)
- Repo: github.com/softprops/action-gh-release (JS/TS, ~5k stars, mantenido).
- Pros: crea Release + sube archivos en 1 step, soporta draft/prerelease, body desde file, overwrite, múltiples files glob, sin instalar gh CLI.
- Uso: `with: { files: target/release/..., body_path: CHANGELOG }`, token GITHUB_TOKEN default.
- Contras: dependencia terceros (pin a SHA para seguridad), menos control que gh CLI para notas complejas.
- Costo tokens YML: ~15 líneas.
- Seguridad: requiere `permissions: contents: write`.

## Opcion B — gh CLI oficial (gh release create)
- Repo oficial: cli/cli (gh). En Actions via `run: gh release create ...` (gh preinstalado en runners).
- Pros: oficial GitHub, cero dependencias terceros, control total (notas, assets, flags --draft --prerelease --generate-notes).
- Contras: más bash manual (crear tar.gz, loop upload), ~25-30 líneas, manejo errores manual.
- Costo tokens YML: ~30 líneas.
- Seguridad: mismo GITHUB_TOKEN, menos supply-chain.

## Tabla
- Mantenimiento: A alto (softprops activo), B oficial garantizado.
- Simplicidad: A 1 step, B 3-4 runs.
- Control: A medio, B total.
- Supply-chain: A pin SHA recomendado, B nativo.
- Chungo Deck: ninguno, ambos solo CI Ubuntu.

## Recomendacion
A (softprops) para T50: 1 step, menos tokens, suficiente para tar.gz + dist. Pin a v2 SHA. Si luego necesitas notas auto-generadas complejas, migra a B.
Siguiente micro: T50b añadir job publish con softprops + tar.gz.
