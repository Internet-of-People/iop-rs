#!/usr/bin/env bash

export version=$1

function replace_toml() {
    toml_file=$1
    sed -i -E 's#^(\s*version\s*=\s*)".+"$#\1"'"$version"'"#g' "$toml_file"
    sed -i -E 's#(iop-.*=.*)".+"#\1"'"$version"'"#g' "$toml_file"
    sed -i -E 's#(json-digest.*=.*)".+"#\1"'"$version"'"#g' "$toml_file"
}

function replace_json() {
    json_file=$1
    sed -i -E 's#^(\s*"version"\s*:\s*)".+",$#\1"'"$version"'",#g' "$json_file"
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
declare -a wasm_packages=("node-wasm" "sdk-wasm")

for crate in "${crates[@]}"; do
    replace_toml "$crate/Cargo.toml"
done

for wasm_package in "${wasm_packages[@]}"; do
    replace_json "$wasm_package/.package.json"
done
