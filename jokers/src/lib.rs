// function_lib/src/lib.rs
#[no_mangle]  // This attribute prevents name mangling, making the function accessible from other programs
pub fn Showman(x: f64, y: f64) -> f64 {
    x * y  // Example functionality
}
