use std::time::Duration;

use wasmtime_wasi::{HostMonotonicClock, HostWallClock};

pub(super) struct DeterministicWallClock;

impl HostWallClock for DeterministicWallClock {
    fn resolution(&self) -> std::time::Duration {
        Duration::from_millis(0)
    }

    fn now(&self) -> std::time::Duration {
        Duration::from_millis(0)
    }
}

pub(super) struct DeterministicMonotonicClock;

impl HostMonotonicClock for DeterministicMonotonicClock {
    fn resolution(&self) -> u64 {
        0
    }

    fn now(&self) -> u64 {
        0
    }
}
