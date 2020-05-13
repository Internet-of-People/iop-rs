#!/bin/bash

# Prerequisites:
# - OpenSSH headers, e.g. sudo apt install libssh-dev
# - clang, e.g. sudo apt install clang
# - wasm-pack binary: e.g. cargo install wasm-pack
# - optionally wasm-opt to autooptimize binaries: e.g. sudo apt install binaryen

cargo build

rm -rf pkg/
wasm-pack build --target browser --out-dir pkg/browser --out-name iop_morpheus_core_wasm
wasm-pack build --target nodejs --out-dir pkg/node --out-name iop_morpheus_core_wasm

mv pkg/browser/*.d.ts pkg/
rm pkg/browser/.gitignore
rm pkg/browser/package.json
rm pkg/node/*.d.ts
rm pkg/node/.gitignore
rm pkg/node/package.json
cp .package.json pkg/package.json
