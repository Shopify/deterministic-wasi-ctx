mod clocks;
mod scheduling;

use clocks::{DeterministicMonotonicClock, DeterministicWallClock};
use rand_core::SeedableRng;
use rand_pcg::Pcg64Mcg;
pub use scheduling::{
    replace_scheduling_functions, replace_scheduling_functions_for_wasi_preview_0,
};
use wasmtime_wasi::WasiCtxBuilder;

pub fn add_determinism_to_wasi_ctx_builder(
    wasi_builder: &mut WasiCtxBuilder,
) -> &mut WasiCtxBuilder {
    // Using Pcg64Mcg because it balances memory usage, performance, is adequately random, does not have major issues,
    // and has reproducible results across different platforms. SmallRng and StdRng were considered but are documented
    // as deterministic but not reproducible.
    // See https://rust-random.github.io/book/guide-rngs.html#basic-pseudo-random-number-generators-prngs
    // and https://docs.rs/rand_pcg/latest/rand_pcg/struct.Mcg128Xsl64.html for further details.
    const RANDOM_SEED: u64 = 42; // the answer to life, the universe, and everything
    let random = Box::new(Pcg64Mcg::seed_from_u64(RANDOM_SEED));

    wasi_builder
        .allow_tcp(false)
        .allow_udp(false)
        .insecure_random(random.clone())
        .secure_random(random)
        .wall_clock(DeterministicWallClock)
        .monotonic_clock(DeterministicMonotonicClock)
}
