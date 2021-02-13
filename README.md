# Advanca is now [Automata](https://ata.network).

The development is moved to https://github.com/automata-network/automata. Come and join us there!

----------

# Advanca Node

This repository contains the source code for Advanca Node.

See [advanca/advanca](https://github.com/advanca/advanca) for more information.

## Build

Install Rust:

```bash
curl https://sh.rustup.rs -sSf | sh
```

Initialize your Wasm Build environment:

```bash
./scripts/init.sh
```

Build Wasm and native code:

```bash
cargo build --release
```

## Run

### Single node development chain

Purge any existing developer chain state:

```bash
./target/release/advanca-node purge-chain --dev
```

Start a development chain with:

```bash
./target/release/advanca-node --dev
```

Detailed logs may be shown by running the node with the following environment variables set: `RUST_LOG=debug RUST_BACKTRACE=1 cargo run -- --dev`.

Additional CLI usage options are available and may be shown by running `cargo run -- --help`.

## License

[GPL 3.0](./LICENSE)
