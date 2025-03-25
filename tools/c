#/bin/bash
set -e
export PKG_CONFIG_SYSROOT_DIR=/home/gbin/projects/copper8-sysroot
export RUSTFLAGS="-C link-arg=--sysroot=/home/gbin/projects/copper8-sysroot -C link-arg=-L/home/gbin/projects/copper8-sysroot/usr/lib/aarch64-linux-gnu -C link-arg=-L/home/gbin/projects/copper8-sysroot/lib/aarch64-linux-gnu"
export RUSTFLAGS="$RUSTFLAGS -C link-arg=-Wl,-rpath,/home/gbin/projects/copper8-sysroot/usr/lib/aarch64-linux-gnu \
                                -C link-arg=-Wl,-rpath-link,/home/gbin/projects/copper8-sysroot/usr/lib/aarch64-linux-gnu \
                                -C link-arg=-L/home/gbin/projects/copper8-sysroot/usr/lib/aarch64-linux-gnu \
                                -C link-arg=-L/home/gbin/projects/copper8-sysroot/lib/aarch64-linux-gnu \
                                -C link-arg=-Wl,--unresolved-symbols=ignore-all \
                                -C link-arg=-Wl,--no-as-needed \
                                -C link-arg=-lgcc_s -C link-arg=-lpthread"
cargo build -r --target aarch64-unknown-linux-gnu
scp target/aarch64-unknown-linux-gnu/release/drone copper8:

