#!/usr/bin/env bash

export edition=$1

function replace_toml() {
    toml_file=$1
    sed -i -E 's#^(\s*edition\s*=\s*)".+"$#\1"'"$edition"'"#g' "$toml_file"
}

declare -a crates=(
    "coeus-node"
    "coeus-proto"
    "hydra-proto"
    "hydra-sdk"
    "journal-proto"
    "json-digest"
    "json-digest-wasm"
    "keyvault"
    "keyvault-wasm"
    "morpheus-node"
    "morpheus-proto"
    "morpheus-sdk"
    "node-wasm"
    "proto-wasm"
    "sdk"
    "sdk-ffi"
    "sdk-wasm"
    "vault"
)

for crate in "${crates[@]}"; do
    replace_toml "$crate/Cargo.toml"
done
