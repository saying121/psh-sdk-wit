PSH SDK Wit Project
===================

This project is used to share wit defintions between projects that depend on.

Each directory represents some kind of performance data source or an I/O operator, for example: `perf` directory contains operators that can be used to develop a WASM module for performance engineering engineer. 

How To Use It?
==============

- Add this project as a submodule in project that you're working on.

```bash
    git submodule add git@github.com:OptimatistOpenSource/psh-sdk-wit.git /path/to/psh-sdk-wit
```
- Initialize and Update Submodule:
```bash
    git submodule update --init --recursive
```

Rust
====

To automatically use the latest wit files while you're running cargo build, add this code snippet in your build.rs. For example:

```rust
use std::process::Command;

fn main() {
    // Update the psh-proto submodule
    let _ = Command::new("git")
        .args(&["submodule", "update", "--remote", "--merge"])
        .status();

    // Additional build steps if needed

    // Cargo will continue with its default build process after this script exits
}
```
