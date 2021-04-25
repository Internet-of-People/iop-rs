# Internet of People

Internet of People (IoP) is a software project creating a decentralized software stack that provides the building blocks and tools to support a decentralized society.

This repository contains our Rust codebase that serves as common implementation and used with different bindings (WebAssembly, C FFI) in SDKs for other languages like Typescript or Dart as well.

## Usage

After installing Rust using [rustup](https://rustup.rs/), use the `iop-sdk` crate with the latest version as a dependency with `cargo` in file `Cargo.toml` of your project.

## Overview

You can read a  overview and descriptions of different components on our
[developer portal](https://developer.iop.technology). We especially suggest reading [glossary page](https://developer.iop.technology/glossary) for a detailed explanation of terms, concept and design principles of different software stack components.

## Components

- `json-digest` provides a canonical Json format, derived content IDs and
  selectively building [Merkle trees](https://en.wikipedia.org/wiki/Merkle_tree)
  from parts of a Json document
- `keyvault` implements a "generic cryptographic calculator": starting from a list of words
  it can deterministically derive an enormous number of private keys for any purpose
  like cryptocurrency addresses, DIDs, device keys, etc.
- `vault` adds encryption support, pluggability with state handling and persistence to the pure calculator features of the `keyvault`
- `morpheus` supports Self-Sovereign Identity (SSI) with Decentralized Identifiers (DIDs) and Verifiable Claims/Credentials (VCs). It defines a state machine for keeping a queriable history of DIDs, their keys and rights with atomic transactions to change the state.
- `coeus` implements a generic decentralized naming system (DDNS). It defines a state machine for managing resolvable names with atomic transactions to change the state.
- `hydra` supports using the Hydra blockchain by building transactions for cryptocurrency operations (transfer, delegate voting, etc), or custom transactions with SSI and DNS operations.
- `sdk` exports features of libraries above for clients in a single crate
