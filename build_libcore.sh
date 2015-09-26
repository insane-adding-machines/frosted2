
#Builds Rust's libcore library

set -e

export LIBCORE_DIR=libcore

git clone https://github.com/rust-lang/rust.git
cd rust
#git checkout 522d09dfecbeca1595f25ac58c6d0178bbd21d7d
cd ..

rm -rf $LIBCORE_DIR
mkdir $LIBCORE_DIR
rustc -C opt-level=2 -Z no-landing-pads --target thumbv7m-none-eabi -g rust/src/libcore/lib.rs --out-dir $LIBCORE_DIR

rm -rf rust
