# deterministic-wasi-ctx

## About this repo

A Rust crate for creating a [wasi-common](https://crates.io/crates/wasi-common) [WASI context](https://docs.rs/wasi-common/0.36.0/wasi_common/struct.WasiCtx.html) implementation that is fully deterministic.

Determinism refers to the property that a provided WASI function will **always** return the same series of results for the same series of invocations. For example, invoking `clock_time_get` against the system clock will always return the same timestamp. Or invoking `random_get` will always return `155` on the first invocation followed by always returning `111` on the second invocation.

This can be useful in a variety of contexts. For example, caching the results of invoking a function in a Wasm module.

## Usage

```rust
let wasi = deterministic_wasi_ctx::build_wasi_ctx();
let engine = Engine::default();
let mut linker = Linker::new(&engine);
wasmtime_wasi::add_to_linker(&mut linker, |s| s).unwrap();
let module_path = ...; // path to a Wasm module
let module = Module::from_file(&engine, module_path).unwrap();
let mut store = Store::new(&engine, wasi);
linker.module(&mut store, "", &module).unwrap();
let instance = linker.instantiate(&mut store, &module).unwrap();
... // invoke functions on `instance`
```

## Contributing

`deterministic-wasi-ctx` is a beta project and will be under major development. We welcome feedback, bug reports and bug fixes. We're also happy to discuss feature development but please discuss the features in an issue before contributing.

## Build dependencies

- [rustup](https://rustup.rs/)
- The latest release of Rust on the stable channel
- `wasm32-wasi` target, can be installed by running `rustup target add wasm32-wasi`

## Building

After all the dependencies are installed, run `make build-deterministic-wasi-ctx` to build the crate.

## Testing

Run `make test` to run integration tests.

## Structure

The `deterministic-wasi-ctx-test-programs` crate is used to build a collection of Wasm files invoking WASI functions that the integration tests in the `deterministic-wasi-ctx` crate use to verify the output from those Wasm files is deterministic.
