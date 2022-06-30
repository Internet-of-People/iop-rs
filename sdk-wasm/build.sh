#!/bin/bash

set -ex
echo Starting

# Prerequisites:
# - OpenSSH headers, e.g. sudo apt install libssh-dev
# - clang, e.g. sudo apt install clang
# - wasm-pack binary: e.g. cargo install wasm-pack
# - optionally wasm-opt to autooptimize binaries: e.g. sudo apt install binaryen

rm -rf pkg/
wasm-pack build --release --target browser --out-dir pkg/browser --out-name iop_sdk_wasm
wasm-pack build --release --target nodejs --out-dir pkg/node --out-name iop_sdk_wasm

mv pkg/browser/*.d.ts pkg/
mv pkg/browser/README.md pkg/
mv pkg/browser/LICENSE pkg/
rm pkg/browser/.gitignore
rm pkg/browser/package.json
rm pkg/node/*.d.ts
rm pkg/node/README.md
rm pkg/node/LICENSE
rm pkg/node/.gitignore
rm pkg/node/package.json

# We add some description of the git version of the rust code into the created package.json to make debugging easier
git describe --dirty --all --long | jq -R '{"git-describe":.}' | cat .package.json - | jq -s add > pkg/package.json
echo Ending
