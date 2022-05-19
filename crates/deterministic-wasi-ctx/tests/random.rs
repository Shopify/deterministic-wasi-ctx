mod common;
pub use common::create_default_wasi_ctx; // used to suppress dead code warning for common code that's used for other tests

#[test]
fn test_random() {
    let wasi = deterministic_wasi_ctx::build_wasi_ctx(42);
    let (store, instance) = common::create_instance(wasi, "random.wasm");
    let result = common::invoke_func::<(), i32>(store, instance, "random", ());
    let expected_random_output = 155;
    assert_eq!(result, expected_random_output);
}

#[test]
fn test_random_two_invocations() {
    let wasi = deterministic_wasi_ctx::build_wasi_ctx(42);
    let (store, instance) = common::create_instance(wasi, "random.wasm");
    let result = common::invoke_func::<(), i32>(store, instance, "random_two_invocations", ());
    let expected_random_output = 111;
    assert_eq!(result, expected_random_output);
}

#[test]
fn test_random_diff_seed() {
    let wasi = deterministic_wasi_ctx::build_wasi_ctx(123456);
    let (store, instance) = common::create_instance(wasi, "random.wasm");
    let result = common::invoke_func::<(), i32>(store, instance, "random", ());
    let expected_random_output = 103;
    assert_eq!(result, expected_random_output);
}
