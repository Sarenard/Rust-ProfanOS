use crate::libs::libc::_exit_c;

pub fn exit(id: u8) -> !{
    unsafe {
        _exit_c(id);
    }
}