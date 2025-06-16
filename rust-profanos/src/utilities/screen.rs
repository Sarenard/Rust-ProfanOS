extern crate alloc;
use alloc::ffi::CString;

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {{
        $crate::utilities::screen::print_internal(format_args!($($arg)*));
    }};
}

#[macro_export]
macro_rules! println {
    () => {{
        $crate::print!("\n");
    }};
    ($($arg:tt)*) => {{
        $crate::print!("{}\n", format_args!($($arg)*));
    }};
}

// Internal function used by the macro
pub fn print_internal(args: core::fmt::Arguments) {
    use core::fmt::Write;
    use alloc::string::String;

    let mut buf = String::new();
    buf.write_fmt(args).unwrap();

    // Convert to C-compatible null-terminated string
    let cstr = CString::new(buf).unwrap();
    unsafe {
        crate::libs::libc::printf(cstr.as_ptr() as *const u8);
    }
}