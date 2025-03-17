use anyhow::Result;
use std::time::Instant;

mod common;

#[macro_use]
extern crate more_asserts;

#[test]
fn test_sleep() -> Result<()> {
    common::test_instance::<_, (), _>("scheduler.wasm", |invoke_func| {
        let start = Instant::now();
        invoke_func("sleep", ())?;
        let duration = start.elapsed();
        assert_lt!(duration.as_millis(), 100);
        Ok(())
    })
}

#[test]
fn test_yield() -> Result<()> {
    common::test_instance("scheduler.wasm", |invoke_func| {
        // it's difficult to test that yielding isn't yielding in practice since sched_yield is very fast
        // it's still worth testing that it doesn't panic
        invoke_func("yield", ())
    })
}
