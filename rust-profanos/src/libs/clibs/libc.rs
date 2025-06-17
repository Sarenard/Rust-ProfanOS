#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub(crate) struct CFILE {
    pub(crate) fd: i32,
    pub(crate) mode: i32,
    pub(crate) error: u8,
    pub(crate) eof: u8,
    pub(crate) ungetchar: i32,
    pub(crate) buffer: *mut u8,
    pub(crate) buffer_size: i32,
}

#[allow(non_camel_case_types, dead_code)]
pub(crate) enum SEEK {
    SEEK_CUR = 1,
    SEEK_END = 2,
    SEEK_SET = 0,
}

unsafe extern "C" {
    #[link_name = "printf"]
    pub(crate) fn printf(fmt: *const u8, ...) -> i32;

    #[link_name = "malloc"]
    pub(crate) fn malloc(size: usize, ...) -> *mut u8;

    #[link_name = "free"]
    pub(crate) fn free(mem: *mut u8, ...);

    #[link_name = "_exit"]
    pub(crate) fn _exit_c(id: u8, ...) -> !;

    #[link_name = "fopen"]
    pub(crate) fn fopen_c(filename: *const u8, mode: *const u8) -> *mut CFILE;

    #[link_name = "fread"]
    pub(crate) fn fread_c(buffer: *mut u8 , size: usize, count: usize, stream: *mut CFILE) -> usize;

    #[link_name = "fseek"]
    pub(crate) fn fseek_c(stream: *mut CFILE, offset: usize, whence: usize) -> usize;

    #[link_name = "ftell"]
    pub(crate) fn ftell_c(stream: *mut CFILE) -> usize;
}
    