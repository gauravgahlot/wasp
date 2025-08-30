#[no_mangle]
pub extern "C" fn run(n: i32) -> i32 {
    n * n * n
}
