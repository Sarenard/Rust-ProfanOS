unsafe extern "C" {
    pub fn printf(fmt: *const u8, ...) -> i32;
}

unsafe extern "C" {
    pub fn malloc(size: usize, ...) -> *mut u8;
}

unsafe extern "C" {
    pub fn free(mem: *mut u8, ...);
}

unsafe extern "C" {
    unsafe fn _exit(id: u8, ...);
}

pub fn exit(id: u8) {
    unsafe {
        _exit(id);
    }
}
