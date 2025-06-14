#[macro_export]
macro_rules! print {
    // Just a format string (C-style)
    ($fmt:expr) => {{
        const CSTR: &str = concat!($fmt, "\0");
        unsafe {
            $crate::libs::libc::printf(CSTR.as_ptr());
        }
    }};
    // Format string + arguments, passed to `printf` directly
    ($fmt:expr, $($arg:expr),+ $(,)?) => {{
        unsafe {
            $crate::libs::libc::printf(concat!($fmt, "\0").as_ptr(), $($arg),+);
        }
    }};
}

#[macro_export]
macro_rules! println {
    () => {{
        $crate::print!("\n");
    }};
    ($fmt:expr) => {{
        $crate::print!(concat!($fmt, "\n"));
    }};
    ($fmt:expr, $($arg:expr),+ $(,)?) => {{
        $crate::print!(concat!($fmt, "\n"), $($arg),+);
    }};
}
