use cap_std::time::{Duration, Instant, SystemTime};
use wasi_common::{WasiClocks, WasiMonotonicClock, WasiSystemClock};

// doesn't matter too much what this is set to since the clocks are going to be stubbed
const TIMER_RESOLUTION_MS: u64 = 100;

pub fn new_clocks() -> WasiClocks {
    let instant = Instant::from_std(std::time::Instant::now());
    WasiClocks {
        system: Box::new(DeterministicSystemClock::new()),
        monotonic: Box::new(DeterministicMonotonicClock::new(instant)),
        creation_time: instant,
    }
}

struct DeterministicSystemClock {
    stubbed_time: SystemTime
}

impl DeterministicSystemClock {
    pub fn new() -> DeterministicSystemClock {
        // Make this Dec 25, 1999 so it's obvious it's not correct but also not the epoch since someone
        // could think it was unintentionally set to the epoch in that case.
        // Using an offset from the Unix epoch since computing this from a datetime would require pulling
        // a crate.
        const SECONDS_IN_A_YEAR: u64 = 31_536_000;
        let stubbed_time = SystemTime::from_std(std::time::UNIX_EPOCH + Duration::from_secs(SECONDS_IN_A_YEAR * 30));
        DeterministicSystemClock {
            stubbed_time
        }
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
