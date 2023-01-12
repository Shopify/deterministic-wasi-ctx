use std::path::Path;
use wasi_common::WasiCtx;
use wasmtime::*;

pub fn create_instance(module_name: &str) -> (Store<WasiCtx>, Instance) {
    let wasi = deterministic_wasi_ctx::build_wasi_ctx();
    let engine = Engine::default();
    let mut linker = Linker::new(&engine);
    wasmtime_wasi::add_to_linker(&mut linker, |s| s).unwrap();
    let module_path = Path::new("../../target/wasm32-wasi/debug").join(module_name);
    let module = Module::from_file(&engine, module_path).unwrap();
    let mut store = Store::new(&engine, wasi);
    linker.module(&mut store, "", &module).unwrap();
    let instance = linker.instantiate(&mut store, &module).unwrap();
    (store, instance)
}

pub fn invoke_func<Params, Results>(
    mut store: Store<WasiCtx>,
    instance: Instance,
    func_name: &str,
    params: Params,
) -> Results
where
    Params: WasmParams,
    Results: WasmResults,
{
    let answer = instance
        .get_func(&mut store, func_name)
        .unwrap_or_else(|| panic!("`{}` was not an exported function", func_name));
    let answer = answer.typed::<Params, Results>(&store).unwrap();
    answer.call(&mut store, params).unwrap()
}
