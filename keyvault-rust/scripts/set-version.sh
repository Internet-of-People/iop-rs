#!/usr/bin/env bash

export version=$1

function replace() {
    toml_file=$1
    sed -i -E 's#^(version = )".+"$#\1"'"$version"'"#g' "$toml_file"
    sed -i -E 's#^(iop-.* = )".+"$#\1"'"$version"'"#g' "$toml_file"
}

declare -a crates=("keyvault" "keyvault-wasm")

for crate in "${crates[@]}"; do
    replace "$crate/Cargo.toml"
done
