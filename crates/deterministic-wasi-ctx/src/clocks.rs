use cap_std::time::{Duration, Instant, SystemTime};
use wasi_common::{WasiClocks, WasiMonotonicClock, WasiSystemClock};

// There is no resolution given it's a stubbed clock
const TIMER_RESOLUTION_MS: u64 = 0;

pub fn new_clocks() -> WasiClocks {
    let instant = Instant::from_std(std::time::Instant::now());
    WasiClocks {
        system: Box::new(DeterministicSystemClock::new()),
        monotonic: Box::new(DeterministicMonotonicClock::new(instant)),
        creation_time: instant,
    }
}

struct DeterministicSystemClock {
    stubbed_time: SystemTime,
}

impl DeterministicSystemClock {
    pub fn new() -> DeterministicSystemClock {
        let stubbed_time = SystemTime::from_std(std::time::UNIX_EPOCH);
        DeterministicSystemClock { stubbed_time }
    }
}

impl WasiSystemClock for DeterministicSystemClock {
    fn resolution(&self) -> Duration {
        Duration::from_millis(TIMER_RESOLUTION_MS)
    }

    fn now(&self, _precision: Duration) -> SystemTime {
        self.stubbed_time
    }
}

// `std::time::Instant` (and `cap_primitives::time::Instant`) are opaque and cannot be constructed from a particular
// datetime. This implementation results in a value of `0` being written into the memory for the Wasm module on each
// invocation of `clock_time_get`.
struct DeterministicMonotonicClock {
    instant: Instant,
}

impl DeterministicMonotonicClock {
    pub fn new(instant: Instant) -> DeterministicMonotonicClock {
        DeterministicMonotonicClock { instant }
    }
}

impl WasiMonotonicClock for DeterministicMonotonicClock {
    fn resolution(&self) -> Duration {
        Duration::from_millis(TIMER_RESOLUTION_MS)
    }

    fn now(&self, _precision: Duration) -> Instant {
        self.instant
    }
}
