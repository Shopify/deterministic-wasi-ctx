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
