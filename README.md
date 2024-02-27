PSH SDK Wit Project
===================

This project is used to share wit definitions between projects that depend on.

Each wit file represents some kind of performance data source or an I/O operator, for example, `perf.``wit` contains operators that collect Linux perf data and can be used to develop a WASM module for performance engineering engineers.

Test cases:
===========

- Guest side (wasm_module)
  
The project under `tests/wasm_module/Rust` depends on `wit-bindgen`. Before you are going to use it, install `wit-bindgen` by
```
cargo install --git https://github.com/bytecodealliance/wit-bindgen wit-bindgen-cli
```

- Host side
  * Wasmtime: TODO

How To Use It?
==============

- Add this project as a submodule in a project that you're working on.

```bash
    git submodule add git@github.com:OptimatistOpenSource/psh-sdk-wit.git /path/to/psh-sdk-wit
```
- Initialize and Update Submodule:
```bash
    git submodule update --init --recursive
```

Rust
====

To automatically use the latest wit files while you're running the cargo build, add this code snippet in your build.rs. For example:

```rust
use std::process::Command;

fn main() {
    // Update the psh-sdk-wit submodule
    let _ = Command::new("git")
        .args(&["submodule", "update", "--remote", "--merge"])
        .status();

    // Additional build steps if needed

    // Cargo will continue with its default build process after this script exits
}
```
