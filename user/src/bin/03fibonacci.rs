#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

#[no_mangle]
fn main() -> i32 {
    println!("Fibonacci Sequence:");
    let mut a = 0;
    let mut b = 1;
    for i in 0..10 {
        println!("F({}) = {}", i, a);
        let next = a + b;
        a = b;
        b = next;
    }
    0
}
