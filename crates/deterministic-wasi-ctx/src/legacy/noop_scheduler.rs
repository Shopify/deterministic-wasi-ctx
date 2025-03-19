use wasi_common::sched::{Duration, WasiSched};
use wasi_common::{Error, Poll};

pub struct NoopScheduler;

impl NoopScheduler {
    pub fn new() -> NoopScheduler {
        NoopScheduler {}
    }
}

#[async_trait::async_trait]
impl WasiSched for NoopScheduler {
    async fn poll_oneoff<'a>(&self, _poll: &mut Poll<'a>) -> anyhow::Result<(), Error> {
        Ok(())
    }

    async fn sched_yield(&self) -> anyhow::Result<(), Error> {
        Ok(())
    }

    async fn sleep(&self, _duration: Duration) -> anyhow::Result<(), Error> {
        Ok(())
    }
}
