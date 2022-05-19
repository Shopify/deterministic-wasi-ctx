use std::time::Instant;

mod common;

#[macro_use]
extern crate more_asserts;

#[test]
fn test_sleep() {
    let wasi = common::create_default_wasi_ctx();
    let (store, instance) = common::create_instance(wasi, "scheduler.wasm");
    let start = Instant::now();
    common::invoke_func::<(), ()>(store, instance, "sleep", ());
    let duration = start.elapsed();
    assert_lt!(duration.as_millis(), 100);
}

#[test]
fn test_yield() {
    let wasi = common::create_default_wasi_ctx();
    let (store, instance) = common::create_instance(wasi, "scheduler.wasm");

    // it's difficult to test that yielding isn't yielding in practice since sched_yield is pretty fast
    // it's still worth testing that it doesn't panic and doesn't take a while to execute
    let start = Instant::now();
    common::invoke_func::<(), ()>(store, instance, "yield", ());
    let duration = start.elapsed();
    assert_lt!(duration.as_millis(), 100);
}
