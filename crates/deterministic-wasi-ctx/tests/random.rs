mod common;

#[test]
fn test_random() {
    let (store, instance) = common::create_instance("random.wasm");
    let result = common::invoke_func::<(), i32>(store, instance, "random", ());
    let expected_random_output = 155;
    assert_eq!(result, expected_random_output);
}

#[test]
fn test_random_two_invocations() {
    let (store, instance) = common::create_instance("random.wasm");
    let result = common::invoke_func::<(), i32>(store, instance, "random_two_invocations", ());
    let expected_random_output = 111;
    assert_eq!(result, expected_random_output);
}
