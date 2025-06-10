# deterministic-wasi-ctx

## About this repo

A Rust crate for creating a [wasmtime-wasi](https://crates.io/crates/wasmtime-wasi) [WASI context](https://docs.rs/wasmtime-wasi/30.0.0/wasmtime_wasi/preview1/struct.WasiP1Ctx.html) implementation that is fully deterministic.

Determinism refers to the property that a provided WASI function will **always** return the same series of results for the same series of invocations. For example, invoking `clock_time_get` against the system clock will always return the same timestamp. Or invoking `random_get` will always return `155` on the first invocation followed by always returning `111` on the second invocation.

This can be useful in a variety of contexts. For example, caching the results of invoking a function in a Wasm module.

## Usage

```rust
let engine = Engine::default();
let mut wasi_builder = WasiCtxBuilder::new();
let mut linker = Linker::new(&engine);

deterministic_wasi_ctx::add_determinism_to_wasi_ctx_builder(&mut wasi_builder);
let wasi = wasi_builder.build_p1();
wasmtime_wasi::preview1::add_to_linker_sync(&mut linker, |s| s)?;
deterministic_wasi_ctx::replace_scheduling_functions(&mut linker)?;

let mut store = Store::new(&engine, wasi);
let module_path = ...; // path to a Wasm module
let module = Module::from_file(&engine, module_path).unwrap();
let instance = linker.instantiate(&mut store, &module).unwrap();
... // invoke functions on `instance`
```

## Contributing

We welcome feedback, bug reports and bug fixes. We're also happy to discuss feature development but please discuss the features in an issue before contributing.

## Build dependencies

- [rustup](https://rustup.rs/)
- The latest release of Rust on the stable channel
- `wasm32-wasip1` target, can be installed by running `rustup target add wasm32-wasip1`

## Building

After all the dependencies are installed, run `make build-deterministic-wasi-ctx` to build the crate.

## Testing

Run `make test` to run integration tests.

## Structure

The `deterministic-wasi-ctx-test-programs` crate is used to build a collection of Wasm files invoking WASI functions that the integration tests in the `deterministic-wasi-ctx` crate use to verify the output from those Wasm files is deterministic.

## Releasing

1. Create and merge a PR incrementing the crates' versions in accordance with [SemVer](https://semver.org/) based on changes from the previous release
1. Create a new release in [Github](https://github.com/Shopify/deterministic-wasi-ctx/releases/new) with a name like `v0.1.0` where the version matches the crates' versions
1. Publish the new version of the crate by running `make publish`
