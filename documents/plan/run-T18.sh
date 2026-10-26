# T18 run en distrobox — 2026-10-14
Container dittox-dev listo. Comandos dentro de distrobox:
sudo pacman -Syu base-devel pkg-config webkit2gtk-4.1 gtk3 libsoup3 --noconfirm
export PATH="$HOME/.cargo/bin:$HOME/.local/node20/bin:$PATH"
cd /home/deck/Documents/git/DittoX && cargo test -p dittox-core 2>&1 | tee /tmp/dittox-test.log; tail -n 30 /tmp/dittox-test.log
Pegar output para cerrar T18.
