#! /bin/sh

svd2rust --target cortex-m -i "$1" || exit 1
rustfmt lib.rs
rustfmt build.rs
mkdir -p src
mv lib.rs src/
