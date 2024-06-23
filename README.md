# Java Decl

Project to explain java bytecode in plain english, and convert back to original source code. Inspired by cdecl and godbolt.

## Getting Started

To build the WebAssembly module, follow these steps:

1. Install `wasm-pack` by running the following command:
   ```
   cargo install wasm-pack
   ```

2. Build the WebAssembly module by running the following command:
   ```
   wasm-pack build
   ```

3. The generated WebAssembly module will be available in the `pkg` directory.
