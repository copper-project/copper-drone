#/bin/bash
cargo build -r --target aarch64-unknown-linux-musl
scp target/aarch64-unknown-linux-musl/release/drone copper8:

