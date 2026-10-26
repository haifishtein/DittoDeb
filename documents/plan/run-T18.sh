# T18 run en distrobox — 2026-10-14
Container ditto-deb-dev listo. Comandos dentro de distrobox:
sudo pacman -Syu base-devel pkg-config webkit2gtk-4.1 gtk3 libsoup3 --noconfirm
export PATH="$HOME/.cargo/bin:$HOME/.local/node20/bin:$PATH"
cd /home/deck/Documents/git/DittoDeb && cargo test -p ditto-deb-core 2>&1 | tee /tmp/ditto-deb-test.log; tail -n 30 /tmp/ditto-deb-test.log
Pegar output para cerrar T18.
