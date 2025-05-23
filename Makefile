.PHONY: clean
CMD?=check

# Use a Makefile since the repo contains crates that target different architectures so we need an alternative way to
# manage dependencies since Cargo currently can't manage dependencies for crates that depend on the binary output of
# other crates

clean:
	cargo clean

build-deterministic-wasi-ctx:
	cargo build --locked --package deterministic-wasi-ctx

build-deterministic-wasi-ctx-test-programs:
	cargo build --locked --package deterministic-wasi-ctx-test-programs --target wasm32-wasip1

crate-checks:
	cargo install --locked --version ~0.12 cargo-deny && cargo deny check

deterministic-wasi-ctx:
	cargo $(CMD) --package deterministic-wasi-ctx

deterministic-wasi-ctx-test-programs:
	cargo $(CMD) --package deterministic-wasi-ctx-test-programs --target wasm32-wasip1

fmt: fmt-deterministic-wasi-ctx fmt-deterministic-wasi-ctx-test-programs

fmt-deterministic-wasi-ctx:
	cargo fmt --package deterministic-wasi-ctx -- --check
	cargo clippy --package deterministic-wasi-ctx -- -D warnings
	cargo clippy --package deterministic-wasi-ctx --features wasi-common -- -D warnings

fmt-deterministic-wasi-ctx-test-programs:
	cargo fmt --package deterministic-wasi-ctx-test-programs -- --check \
		&& cargo clippy --package deterministic-wasi-ctx-test-programs --target wasm32-wasip1 -- -D warnings

publish:
	cargo publish --package deterministic-wasi-ctx

test: build-deterministic-wasi-ctx-test-programs
	cargo test --locked --package deterministic-wasi-ctx
	cargo test --locked --package deterministic-wasi-ctx --features wasi-common

unused-dependencies:
	cargo install cargo-udeps --locked --version ~0.1
	cargo +nightly udeps
