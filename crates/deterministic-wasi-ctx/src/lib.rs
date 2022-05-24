use rand_core::SeedableRng;
use rand_pcg::Pcg64Mcg;
use wasi_common::WasiCtx;

mod clocks;

mod noop_scheduler;
use noop_scheduler::NoopScheduler;

pub fn build_wasi_ctx(random_seed: u64) -> WasiCtx {
    // Using Pcg64Mcg because it balances memory usage, performance, is adequately random, does not have major issues,
    // and has reproducible results across different platforms. SmallRng and StdRng were considered but are documented
    // as deterministic but not reproducible.
    // See https://rust-random.github.io/book/guide-rngs.html#basic-pseudo-random-number-generators-prngs
    // and https://docs.rs/rand_pcg/latest/rand_pcg/struct.Mcg128Xsl64.html for further details.
    // Using a u64 to seed the generator since we cannot guarantee that the input for the seed will not contain low
    // Hamming Weight numbers like 0 and 1 and we still want sufficiently random output
    let random = Box::new(Pcg64Mcg::seed_from_u64(random_seed));

    let clocks = clocks::new_clocks();

    let scheduler = Box::new(NoopScheduler::new());
    let table = wasi_common::table::Table::new();
    WasiCtx::new(random, clocks, scheduler, table)
}
