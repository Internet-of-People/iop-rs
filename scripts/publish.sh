#!/usr/bin/env bash

# Run `scripts/publish.sh --dry-run` from the root of the repository to
# check if everything is ready for publishing. Then remove --dry-run to
# actually publish all crates on https://crates.io/

declare -a crates=(
	"json-digest"
	"iop-journal-proto"
	"iop-keyvault"
	"iop-keyvault-wasm"
	"iop-morpheus-proto"
	"iop-coeus-proto"
	"iop-vault"
	"iop-hydra-proto"
	"iop-coeus-node"
	"iop-morpheus-node"
	"json-digest-wasm"
	"iop-proto-wasm"
	"iop-hydra-sdk"
	"iop-morpheus-sdk"
	"iop-sdk"
	"iop-sdk-wasm"
	"iop-node-wasm"
	"iop-sdk-ffi"
)

for crate in "${crates[@]}"; do
    cargo publish -p "$crate" "$@"
done
