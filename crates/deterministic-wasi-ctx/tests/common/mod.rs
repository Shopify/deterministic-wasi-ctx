use anyhow::Result;
use std::{fs, path::Path};
use wasmtime::*;
use wasmtime_wasi::{p2::WasiCtxBuilder, preview1::WasiP1Ctx};

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

    #[cfg(feature = "wasi-common")]
    {
        let (store, instance) = create_instance_with_legacy_ctx(&engine, &module)?;
        testcase(Box::new(move |func_name, params| {
            invoke_func(store, instance, func_name, params)
        }))?;
    }

    let (store, instance) = create_instance(&engine, &module)?;
    testcase(Box::new(move |func_name, params| {
        invoke_func(store, instance, func_name, params)
    }))?;

    Ok(())
}

#[cfg(feature = "wasi-common")]
fn create_instance_with_legacy_ctx(
    engine: &Engine,
    module: &Module,
) -> Result<(Store<wasi_common::WasiCtx>, Instance)> {
    let wasi = deterministic_wasi_ctx::build_wasi_ctx();

    let mut linker = Linker::new(&engine);
    wasi_common::sync::add_to_linker(&mut linker, |s| s)?;

    setup_store_and_instance(engine, module, wasi, &mut linker)
}

fn create_instance(engine: &Engine, module: &Module) -> Result<(Store<WasiP1Ctx>, Instance)> {
    let mut wasi_builder = WasiCtxBuilder::new();
    deterministic_wasi_ctx::add_determinism_to_wasi_ctx_builder(&mut wasi_builder);
    let wasi = wasi_builder.build_p1();

    let mut linker = Linker::new(&engine);
    wasmtime_wasi::preview1::add_to_linker_sync(&mut linker, |s| s)?;
    deterministic_wasi_ctx::replace_scheduling_functions(&mut linker)?;

    setup_store_and_instance(engine, module, wasi, &mut linker)
}

fn setup_store_and_instance<T>(
    engine: &Engine,
    module: &Module,
    wasi: T,
    linker: &mut Linker<T>,
) -> Result<(Store<T>, Instance)> {
    let mut store = Store::new(&engine, wasi);
    let instance = linker.instantiate(&mut store, &module)?;
    Ok((store, instance))
}

fn invoke_func<Params, Results>(
    mut store: impl AsContextMut,
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
