use anyhow::Result;

mod common;

#[test]
fn test_random() -> Result<()> {
    common::test_instance("random.wasm", |invoke_func| {
        let result: i32 = invoke_func("random", ())?;
        let expected_random_output = 155;
        assert_eq!(result, expected_random_output);
        Ok(())
    })
}

#[test]
fn test_random_two_invocations() -> Result<()> {
    common::test_instance("random.wasm", |invoke_func| {
        let result: i32 = invoke_func("random_two_invocations", ())?;
        let expected_random_output = 111;
        assert_eq!(result, expected_random_output);
        Ok(())
    })
}
