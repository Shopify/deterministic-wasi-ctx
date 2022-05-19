mod common;

#[test]
fn test_realtime_clock() {
    let wasi = common::create_default_wasi_ctx();
    let (store, instance) = common::create_instance(wasi, "clocks.wasm");
    let result = common::invoke_func::<(), u64>(store, instance, "realtime", ());
    let expected_clock_output = 946080000000000000; // Dec 25, 1999
    assert_eq!(result, expected_clock_output);
}

#[test]
fn test_realtime_clock_seq_calls() {
    let wasi = common::create_default_wasi_ctx();
    let (store, instance) = common::create_instance(wasi, "clocks.wasm");
    let result = common::invoke_func::<(), u64>(store, instance, "realtime_seq_calls", ());
    let expected_clock_diff = 0;
    assert_eq!(result, expected_clock_diff);
}

#[test]
fn test_monotonic_clock() {
    let mut wasi = common::create_default_wasi_ctx();
    wasi.set_stdout(Box::new(wasi_common::pipe::WritePipe::new(std::io::stdout())));
    let (store, instance) = common::create_instance(wasi, "clocks.wasm");
    let result = common::invoke_func::<(), u64>(store, instance, "monotonic", ());
    let expected_clock_output = 0;
    assert_eq!(result, expected_clock_output);
}

#[test]
fn test_monotonic_clock_seq_calls() {
    let wasi = common::create_default_wasi_ctx();
    let (store, instance) = common::create_instance(wasi, "clocks.wasm");
    let result = common::invoke_func::<(), u64>(store, instance, "monotonic_seq_calls", ());
    let expected_clock_diff = 0;
    assert_eq!(result, expected_clock_diff);
}
