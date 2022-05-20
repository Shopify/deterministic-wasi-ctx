# deterministic-wasi-ctx

## About this repo

A Rust crate for creating a [wasi-common](https://crates.io/crates/wasi-common) [WASI context](https://docs.rs/wasi-common/0.36.0/wasi_common/struct.WasiCtx.html) implementation that is fully deterministic.

Determinism refers to the property that a provided WASI function will **always** return the same series of results for the same series of invocations. For example, invoking `clock_time_get` against the system clock will always return a timestamp corresponding to December 25, 1999 at midnight. Or invoking `random_get` will return `155` followed by `111` on the second invocation.

This can be useful in a variety of contexts. For example, caching the results of invoking a function in a Wasm module. Or mitigating attacks that attempt to time how long certain operations take to complete using a monotonic clock.

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
