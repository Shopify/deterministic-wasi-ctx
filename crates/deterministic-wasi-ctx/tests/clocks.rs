use anyhow::Result;

mod common;

#[test]
fn test_realtime_clock() -> Result<()> {
    common::test_instance("clocks.wasm", |invoke_func| {
        let result: i64 = invoke_func("realtime", ())?;
        let expected_clock_output = 0;
        assert_eq!(result, expected_clock_output);
        Ok(())
    })
}

#[test]
fn test_realtime_clock_seq_calls() -> Result<()> {
    common::test_instance("clocks.wasm", |invoke_func| {
        let result: i64 = invoke_func("realtime_seq_calls", ())?;
        let expected_clock_diff = 0;
        assert_eq!(result, expected_clock_diff);
        Ok(())
    })
}

#[test]
fn test_monotonic_clock() -> Result<()> {
    common::test_instance("clocks.wasm", |invoke_func| {
        let result: i64 = invoke_func("monotonic", ())?;
        let expected_clock_output = 0;
        assert_eq!(result, expected_clock_output);
        Ok(())
    })
}

#[test]
fn test_monotonic_clock_seq_calls() -> Result<()> {
    common::test_instance("clocks.wasm", |invoke_func| {
        let result: i64 = invoke_func("monotonic_seq_calls", ())?;
        let expected_clock_diff = 0;
        assert_eq!(result, expected_clock_diff);
        Ok(())
    })
}
