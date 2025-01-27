# Plonky2 Verifier

[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)
[![Rust CI🌌](https://github.com/distributed-lab/plonky2-verifier/actions/workflows/rust.yml/badge.svg)](https://github.com/distributed-lab/plonky2-verifier/actions/workflows/rust.yml)
[![Docs 🌌](https://github.com/distributed-lab/plonky2-verifier/actions/workflows/docs.yml/badge.svg)](https://github.com/distributed-lab/plonky2-verifier/actions/workflows/docs.yml)

> A verifier for [plonky2](https://github.com/0xPolygonZero/plonky2/) proofs.

This rust crate provides functionality to deserialize and verify proof, public inputs and verification key. 

## plonky2-converter
`Plonky2` has a certain number of generics for its constraint system, such as used field, hasher etc.
Since we are limited by the nature of passing them in `zkVerify`, we use a custom format of [Vk](./src/vk.rs).

For that reason, this crate also provides a binary to convert verification keys into acceptable by `zkVerify` format.

### Install:

```bash
cargo install --features converter --path .
```

### Usage:
```bash
plonky2-converter vk --help
Serialize VerifierCircuitData into zkVerify format

Usage: 

Arguments:
  <INPUT>

  [OUTPUT]
          
Options:
  -i, --in-fmt <IN_FMT>
          [default: bytes]

          Possible values:
          - bytes: Raw binary bytes
          - hex:   Hex-encoded string (with optional "0x" prefix)

  -o, --out-fmt <OUT_FMT>
          [default: json]

          Possible values:
          - json:  JSON format (pretty-printed)
          - bytes: Raw binary bytes
          - hex:   Hex-encoded string

  -c, --config <CONFIG>
          [default: poseidon]

          Possible values:
          - keccak:   Preset Keccak over Goldilocks config available in `plonky2`
          - poseidon: Preset Poseidon over Goldilocks config available in `plonky2`

  -h, --help
          Print help (see a summary with '-h')

```

## License

This code is released under the GPL 3.0 license.