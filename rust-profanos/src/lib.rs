#![no_std]

use core::ffi::CStr;

use alloc::{borrow::ToOwned, vec::Vec};
use libs::std::process::exit;

use libs::std::env::{Args, ARGS};

extern crate alloc;

pub mod libs;
pub mod utilities;

#[unsafe(no_mangle)]
pub extern "C" fn _start(argc: i32, argv: *const *const i8) -> ! {
    let mut args = Vec::new();

    for i in 0..argc {
        let cstr_ptr = unsafe { *argv.add(i as usize) };
        let cstr = unsafe { CStr::from_ptr(cstr_ptr) };
        let str = cstr.to_str().unwrap_or("<invalid UTF-8>").to_owned();
        args.push(str);  // Push the argument into the vector
    }

    unsafe { 
        ARGS = Some( Args::new(
            argc as usize,
            args,
        )) 
    }

    unsafe extern "C" {
        fn main();
    }
    
    unsafe {
        main();
    }

    exit(0);
}
