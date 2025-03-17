use anyhow::Result;
use std::path::Path;
use wasi_common::WasiCtx;
use wasmtime::*;

pub fn test_instance<Params, Results, F>(module_name: &str, testcase: F) -> Result<()>
where
    Params: WasmParams,
    Results: WasmResults,
    F: Fn(Box<dyn FnOnce(&str, Params) -> Result<Results>>) -> Result<()>,
{
    let wasi = deterministic_wasi_ctx::build_wasi_ctx();
    let engine = Engine::default();
    let mut linker = Linker::new(&engine);
    wasi_common::sync::add_to_linker(&mut linker, |s| s)?;
    let module_path = Path::new("../../target/wasm32-wasip1/debug").join(module_name);
    let module = Module::from_file(&engine, module_path)?;
    let mut store = Store::new(&engine, wasi);
    linker.module(&mut store, "", &module)?;
    let instance = linker.instantiate(&mut store, &module)?;
    testcase(Box::new(move |func_name, params| {
        invoke_func(store, instance, func_name, params)
    }))
}

fn invoke_func<Params, Results>(
    mut store: Store<WasiCtx>,
    instance: Instance,
    func_name: &str,
    params: Params,
) -> Result<Results>
where
    Params: WasmParams,
    Results: WasmResults,
{
    let answer = instance
        .get_func(&mut store, func_name)
        .unwrap_or_else(|| panic!("`{}` was not an exported function", func_name));
    let answer = answer.typed::<Params, Results>(&store)?;
    answer.call(&mut store, params)
}
