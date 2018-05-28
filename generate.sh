#! /bin/sh

svd2rust --target cortex-m -i "$1" || exit 1
rustfmt lib.rs
rustfmt build.rs
mkdir -p src
form -i lib.rs -o src
rm lib.rs
rustfmt `find . -name \*.rs`
