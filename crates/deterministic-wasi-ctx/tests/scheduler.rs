use std::time::Instant;

mod common;

#[macro_use]
extern crate more_asserts;

#[test]
fn test_sleep() {
    let (store, instance) = common::create_instance("scheduler.wasm");
    let start = Instant::now();
    common::invoke_func::<(), ()>(store, instance, "sleep", ());
    let duration = start.elapsed();
    assert_lt!(duration.as_millis(), 100);
}

#[test]
fn test_yield() {
    let (store, instance) = common::create_instance("scheduler.wasm");

    // it's difficult to test that yielding isn't yielding in practice since sched_yield is very fast
    // it's still worth testing that it doesn't panic
    common::invoke_func::<(), ()>(store, instance, "yield", ());
}
