fn increment(x: &mut i32) {
    *x += 1;
}

fn main() {
    let mut value = 5;
    increment(&mut value);
    assert_eq!(value, 7); // prints 6
}