#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

const LEN: usize = 100;

#[no_mangle]
fn main() -> i32 {
    for i in 1..6 {
        println!("My_test {}", i);
    }
    println!("Yang Man test is OK!");
    0
}

