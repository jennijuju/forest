#!/usr/bin/env bash
set -euxo pipefail

avx2=target_avx2
no_avx2=target_no_avx2

# Build
for dir in $avx2 $no_avx2; do
    mkdir --parents -- "$dir"
    echo '*' > "$dir/.gitignore"
done

CARGO_TARGET_DIR=$avx2 \
RUSTFLAGS="-C target-feature=+avx2,+fma" \
cargo build --release --bins

CARGO_TARGET_DIR=$no_avx2 \
RUSTFLAGS="" \
cargo build --release --bins
