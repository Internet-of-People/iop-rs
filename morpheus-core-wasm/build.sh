# Prerequisites:
# - OpenSSH headers, e.g. sudo apt install libssh-dev
# - clang, e.g. sudo apt install clang
# - wasm-pack binary: e.g. cargo install wasm-pack
# - optinally wasm-opt to autooptimize binaries: e.g. sudo apt install binaryen
cargo build
wasm-pack build --target nodejs
