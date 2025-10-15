use anyhow::Result;
use std::{fs, path::Path};
use wasmtime::*;
use wasmtime_wasi::WasiCtxBuilder;

pub fn test_instance<Params, Results, F>(module_name: &str, testcase: F) -> Result<()>
where
    Params: WasmParams,
    Results: WasmResults,
    F: Fn(Box<dyn FnOnce(&str, Params) -> Result<Results>>) -> Result<()>,
{
    let module_path = Path::new("../../target/wasm32-wasip1/debug").join(module_name);
    let bytes = fs::read(module_path)?;
    test_instance_with_bytes(&bytes, testcase)
}

pub fn test_instance_with_bytes<Params, Results, F>(bytes: &[u8], testcase: F) -> Result<()>
where
    Params: WasmParams,
    Results: WasmResults,
    F: Fn(Box<dyn FnOnce(&str, Params) -> Result<Results>>) -> Result<()>,
{
    let engine = Engine::default();
    let module = Module::new(&engine, bytes)?;

    let mut wasi_builder = WasiCtxBuilder::new();
    deterministic_wasi_ctx::add_determinism_to_wasi_ctx_builder(&mut wasi_builder);
    let wasi = wasi_builder.build_p1();

    let mut linker = Linker::new(&engine);
    wasmtime_wasi::p1::add_to_linker_sync(&mut linker, |s| s)?;
    deterministic_wasi_ctx::replace_scheduling_functions(&mut linker)?;

    let mut store = Store::new(&engine, wasi);
    let instance = linker.instantiate(&mut store, &module)?;

    testcase(Box::new(move |func_name, params| {
        let answer = instance
            .get_func(&mut store, func_name)
            .unwrap_or_else(|| panic!("`{}` was not an exported function", func_name));
        let answer = answer.typed::<Params, Results>(&store)?;
        answer.call(&mut store, params)
    }))?;

    Ok(())
}
