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
fn test_poll_oneoff_with_bad_in_ptr() -> Result<()> {
    common::test_instance("scheduler.wasm", |invoke_func| {
        // we want to ensure this doesn't cause a panic in the host and the error looks relevant
        let result: Result<i32, anyhow::Error> =
            invoke_func("poll_oneoff_test", (i32::MAX - 16, 0, 1, 0));
        assert_has_ptr_related_error(result)
    })
}

#[test]
fn test_poll_oneoff_with_bad_out_ptr() -> Result<()> {
    common::test_instance("scheduler.wasm", |invoke_func| {
        // we want to ensure this doesn't cause a panic in the host and the error looks relevant
        let result: Result<i32, anyhow::Error> =
            invoke_func("poll_oneoff_test", (0, i32::MAX - 16, 1, 0));
        assert_has_ptr_related_error(result)
    })
}

#[test]
fn test_poll_oneoff_with_bad_nevents_ptr() -> Result<()> {
    common::test_instance("scheduler.wasm", |invoke_func| {
        // we want to ensure this doesn't cause a panic in the host and the error looks relevant
        let result: Result<i32, anyhow::Error> =
            invoke_func("poll_oneoff_test", (0, 0, 1, i32::MAX - 16));
        assert_has_ptr_related_error(result)
    })
}

fn assert_has_ptr_related_error(result: Result<i32, anyhow::Error>) -> Result<()> {
    let error = result.unwrap_err();
    let inner_error = error.source().unwrap();
    let err_message = inner_error.to_string();
    if err_message.contains("Pointer out of bounds")
        || err_message.contains("out of bounds memory access")
    {
        Ok(())
    } else {
        Err(error)
    }
}

#[test]
fn test_poll_oneoff_with_missing_memory_export() -> Result<()> {
    let wat = r#"
(module
    (import "wasi_snapshot_preview1" "poll_oneoff" (func (;0;) (param i32 i32 i32 i32) (result i32)))
    (func (export "")
        (call 0 (i32.const 0) (i32.const 0) (i32.const 0) (i32.const 0))
        drop
    )
)    
    "#;
    common::test_instance_with_bytes(wat.as_bytes(), |invoke_func| {
        let result: Result<(), anyhow::Error> = invoke_func("", ());
        let error = result.unwrap_err();
        let inner_error = error.source().unwrap();
        let err_message = inner_error.to_string();
        if err_message.contains("missing required memory export") {
            Ok(())
        } else {
            Err(error)
        }
    })
}

#[test]
fn test_poll_oneoff_with_memory_export_that_isnt_memory() -> Result<()> {
    let wat = r#"
(module
    (import "wasi_snapshot_preview1" "poll_oneoff" (func (;0;) (param i32 i32 i32 i32) (result i32)))
    (global (export "memory") i32 (i32.const 0))
    (func (export "")
        (call 0 (i32.const 0) (i32.const 0) (i32.const 0) (i32.const 0))
        drop
    )
)    
    "#;
    common::test_instance_with_bytes(wat.as_bytes(), |invoke_func| {
        let result: Result<(), anyhow::Error> = invoke_func("", ());
        let error = result.unwrap_err();
        let inner_error = error.source().unwrap();
        let err_message = inner_error.to_string();
        if err_message.contains("missing required memory export") {
            Ok(())
        } else {
            Err(error)
        }
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
