#[link(wasm_import_module = "wasi_snapshot_preview1")]
extern "C" {
    fn poll_oneoff(in_ptr: i32, out_ptr: i32, nsubscriptions: i32, nevents_ptr: i32) -> i32;
}

fn main() {}

#[export_name = "sleep"]
pub extern "C" fn sleep() {
    // sleep is a little trickier to use directly so just going to use thread's sleep
    std::thread::sleep(std::time::Duration::from_secs(5));
}

#[export_name = "yield"]
pub extern "C" fn yield_func() {
    unsafe { wasi::sched_yield().unwrap() }
}

#[export_name = "poll_oneoff_test"]
pub extern "C" fn poll_oneoff_test(
    in_ptr: i32,
    out_ptr: i32,
    nsubscriptions: i32,
    nevents_ptr: i32,
) -> i32 {
    unsafe { poll_oneoff(in_ptr, out_ptr, nsubscriptions, nevents_ptr) }
}
