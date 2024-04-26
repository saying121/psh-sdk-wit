PSH SDK Wit Project
===================

This project contains wit definitions for developing performance engineering wasm component.

For now there are two main packages, `perf` and `system`. `perf` provides wrappers for the Linux perf APIs to collect Linux perf data, `system` provides APIs for quering system resources information, i.e., cpus/memory/processes.

These APIs are implemented in [PSH](https://github.com/OptimatistOpenSource/psh) project.

How To Use It?
==============

There are a few options available (we recommand `git submodule` and `wit-bindgen`):

 1. Add this project as a dependency as your wasm component project, i.e., add this project as a submodule or simply copy/paste these files under your project directory.

```bash
    git submodule add git@github.com:OptimatistOpenSource/psh-sdk-wit.git /path/to/psh-sdk-wit
```

 2. use `wit-bindgen` or it's command line interface `wit-bindgen-cli` to generate bindings for your programming language. For example, if you use `Rust` to develop your wasm component, you can either:

  - add `wit-bindgen` as a dependency to your project by `cargo add wit-bindgen`
  - use the `wit_bindgen::generate` macro to dynamically generate bindings.

```Rust
wit_bindgen::generate!({
    world: "bindings",
    path: "<path/to/wit>",
    // ...
});
```

or:

 - install `wit-bindgen-cli` by `cargo install wit-bindgen-cli`
 - use shell command: `./wit-bindgen rust <path/to/wit> --out-dir <path/to/output>` to statically generate these bindings.

 After these steps, you'll able to use these APIs in your Rust project.

Examples:
===========

We also provided a simple rust example in `./examples/wasm_module/Rust`, to build it, make sure you have installed rustup `wasm32-wasi` target toolchain and `cargo-component`, then run

```bash
cargo component build -r
```

This will produce a wasm component in `target/wasm32-wasi/release/psh-sdk-wit.wasm`, to run it, you'll need to use [PSH](https://github.com/OptimatistOpenSource/psh)

```bash
./psh -p <pash/to/psh-sdk-wit.wasm>
```
