fn main() {}

#[export_name = "random"]
pub extern "C" fn random() -> i32 {
    let mut buf = Vec::new();
    buf.resize(1, 0);
    unsafe {
        wasi::random_get(buf.as_mut_ptr(), 1).expect("random_get failed");
    }
    buf[0].into()
}

#[export_name = "random_two_invocations"]
pub extern "C" fn random_two_invocations() -> i32 {
    let mut buf = Vec::new();
    buf.resize(1, 0);
    unsafe {
        wasi::random_get(buf.as_mut_ptr(), 1).expect("random_get failed");
        wasi::random_get(buf.as_mut_ptr(), 1).expect("random_get failed");
    }
    buf[0].into()
}
