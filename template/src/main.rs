#![no_std]
#![no_main]
#![allow(unused)]

extern crate rust_profanos;

use rust_profanos::libs as libs;
use rust_profanos::println;
use rust_profanos::utilities as utilities;


extern crate alloc;

use alloc::boxed::Box;


pub mod entry;
pub mod panichandler;


fn main() {
    // example code
    println!("Hello from Rust!");

    let x = Box::new(3);

    if *x == 3 {
        println!("3");
    } else {
        println!("4");
    }

    println!("End of function");
}