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

Furthermore, to bring support for compressed proofs from `plonky2`, we use a custom format of [Proof](./src/proof.rs).

### Install:

```bash
cargo install --features converter --path .
```

### Usage:

```bash
plonky2-converter --help
```

```
Converts plonky2 formats

Usage: 

Commands:
  vk     Serialize VerifierCircuitData into zkVerify format
  proof  Serialize Proof or CompressedProof into zkVerify format
  help   Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version

```

## License

This code is released under the GPL 3.0 license.