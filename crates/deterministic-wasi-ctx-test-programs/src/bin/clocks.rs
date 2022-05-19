fn main() {}

#[export_name = "realtime"]
pub extern "C" fn realtime() -> u64 {
    unsafe {
        let resolution = wasi::clock_res_get(wasi::CLOCKID_REALTIME).unwrap();
        wasi::clock_time_get(wasi::CLOCKID_REALTIME, resolution).unwrap()
    }
}

#[export_name = "realtime_seq_calls"]
pub extern "C" fn realitime_seq_calls() -> u64 {
    unsafe {
        let resolution = wasi::clock_res_get(wasi::CLOCKID_REALTIME).unwrap();
        let first_read = wasi::clock_time_get(wasi::CLOCKID_REALTIME, resolution).unwrap();
        let second_read = wasi::clock_time_get(wasi::CLOCKID_REALTIME, resolution).unwrap();
        second_read - first_read
    }
}

#[export_name = "monotonic"]
pub extern "C" fn monotonic() -> u64 {
    unsafe {
        let resolution = wasi::clock_res_get(wasi::CLOCKID_MONOTONIC).unwrap();
        wasi::clock_time_get(wasi::CLOCKID_MONOTONIC, resolution).unwrap()
    }
}

#[export_name = "monotonic_seq_calls"]
pub extern "C" fn monotonic_seq_calls() -> u64 {
    unsafe {
        let resolution = wasi::clock_res_get(wasi::CLOCKID_MONOTONIC).unwrap();
        let first_read = wasi::clock_time_get(wasi::CLOCKID_MONOTONIC, resolution).unwrap();
        let second_read = wasi::clock_time_get(wasi::CLOCKID_MONOTONIC, resolution).unwrap();
        second_read - first_read
    }
}
