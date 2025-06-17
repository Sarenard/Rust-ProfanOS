#![no_std]
#![no_main]
#![allow(unused)]

extern crate rust_profanos;

use core::ffi::CStr;

use alloc::string::String;
use alloc::vec;
use rust_profanos::libs::std::env;
use rust_profanos::libs::std::io::Read;
use rust_profanos::libs as libs;
use rust_profanos::println;
use rust_profanos::utilities as utilities;

extern crate alloc;

use alloc::boxed::Box;

use rust_profanos::libs::std::fs::File;

pub mod panichandler;

#[no_mangle]
pub extern "C" fn main() {
    // example code
    println!("Hello from Rust!");

    let mut file = File::open("/user/hello.c").unwrap();

    println!("{:?}", file);

    println!("{:?}", env::args().collect());

    let len = file.metadata().unwrap().len();

    let mut buf = vec![0; len];

    file.read(&mut buf);

    println!("{:?}", buf);

    println!("{}", String::from_utf8_lossy(&buf));

    let x = Box::new(3);

    if *x == 3 {
        println!("3");
    } else {
        println!("4");
    }

    println!("End of function");
}