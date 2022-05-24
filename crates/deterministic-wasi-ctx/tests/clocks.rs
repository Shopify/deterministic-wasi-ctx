mod common;

#[test]
fn test_realtime_clock() {
    let (store, instance) = common::create_instance("clocks.wasm");
    let result = common::invoke_func::<(), u64>(store, instance, "realtime", ());
    let expected_clock_output = 0;
    assert_eq!(result, expected_clock_output);
}

#[test]
fn test_realtime_clock_seq_calls() {
    let (store, instance) = common::create_instance("clocks.wasm");
    let result = common::invoke_func::<(), u64>(store, instance, "realtime_seq_calls", ());
    let expected_clock_diff = 0;
    assert_eq!(result, expected_clock_diff);
}

#[test]
fn test_monotonic_clock() {
    let (store, instance) = common::create_instance("clocks.wasm");
    let result = common::invoke_func::<(), u64>(store, instance, "monotonic", ());
    let expected_clock_output = 0;
    assert_eq!(result, expected_clock_output);
}

#[test]
fn test_monotonic_clock_seq_calls() {
    let (store, instance) = common::create_instance("clocks.wasm");
    let result = common::invoke_func::<(), u64>(store, instance, "monotonic_seq_calls", ());
    let expected_clock_diff = 0;
    assert_eq!(result, expected_clock_diff);
}
