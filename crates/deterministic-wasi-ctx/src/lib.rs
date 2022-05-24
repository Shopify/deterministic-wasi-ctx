use rand_core::SeedableRng;
use rand_pcg::Pcg64Mcg;
use wasi_common::WasiCtx;

mod clocks;

mod noop_scheduler;
use noop_scheduler::NoopScheduler;

pub fn build_wasi_ctx() -> WasiCtx {
    // Using Pcg64Mcg because it balances memory usage, performance, is adequately random, does not have major issues,
    // and has reproducible results across different platforms. SmallRng and StdRng were considered but are documented
    // as deterministic but not reproducible.
    // See https://rust-random.github.io/book/guide-rngs.html#basic-pseudo-random-number-generators-prngs
    // and https://docs.rs/rand_pcg/latest/rand_pcg/struct.Mcg128Xsl64.html for further details.
    const RANDOM_SEED: u64 = 42; // the answer to life, the universe, and everything
    let random = Box::new(Pcg64Mcg::seed_from_u64(RANDOM_SEED));

    let clocks = clocks::new_clocks();

    let scheduler = Box::new(NoopScheduler::new());
    let table = wasi_common::table::Table::new();
    WasiCtx::new(random, clocks, scheduler, table)
}
