# rlibphonenumber-wasm

WASM bindings for the [rlibphonenumber](https://github.com/1aim/rust-phonenumber) library (a Rust port of Google's original `libphonenumber`). 

![Demo Preview](https://img.shields.io/badge/Status-Active-success.svg)
![WASM](https://img.shields.io/badge/Target-WebAssembly-blue.svg)

## Features

- **100% Compatibility**: Mirrors the complete logic of the original Google `libphonenumber` C++ library.
- **Streaming Compilation**: The WASM binary is compiled by the browser on the fly. It doesn't block the Main Thread parsing a massive JavaScript AST, which is a known issue with the official Google JS port.
- **Predictable Memory Footprint**: Operates entirely within WASM's Linear Memory, preventing the JS garbage collector from being choked by tens of thousands of tiny instantiated objects.

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (version 1.70+)
- `wasm-bindgen-cli` (install via `cargo install wasm-bindgen-cli`)
- [Node.js](https://nodejs.org/) (version 18+) for running tests and benchmarks

## Building WASM

To generate the `.wasm` file and the JavaScript web wrappers, run the following command:

```bash
cargo build --target wasm32-unknown-unknown --release
wasm-bindgen target/wasm32-unknown-unknown/release/rlibphonenumber_wasm.wasm \
  --out-dir pkg \
  --target web