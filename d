#/bin/bash
set -E
cargo build --target aarch64-unknown-linux-musl
scp target/aarch64-unknown-linux-musl/debug/drone copper8:

