use core::arch::asm;

use alloc::vec::Vec;

pub(crate) const SYS_VESA_INFO: u32 = 18;
pub(crate) fn syscall_vesa_info(a1: i32) -> i32 {
    let mut ret: i32;
    unsafe {
        asm!(
            // "push ebx",
            "mov eax, {0:e}",
            "mov ebx, {1:e}",
            "int 0x80",
            // "pop ebx",
            in(reg) SYS_VESA_INFO,
            in(reg) a1,
            lateout("eax") ret,
        );
    }
    ret
}

fn set(x: u32, y: u32, col: u32, pitch: u32, fb: *mut u32) {
    unsafe {
        let offset = y * pitch + x;
        *fb.add(offset.try_into().unwrap()) = col;
    }
}

pub fn set_pixel(x: u32, y: u32, color: u32) {
    let pitch = syscall_vesa_info(2) as u32;
    let fb = syscall_vesa_info(3) as *mut u32;
    set(x, y, color, pitch, fb);
}

pub fn set_pixels(x: Vec<u32>, y: Vec<u32>, colors: Vec<u32>) {
    let pitch = syscall_vesa_info(2) as u32;
    let fb = syscall_vesa_info(3) as *mut u32;
    for ((x_val, y_val), color_val) in x.iter().zip(y.iter()).zip(colors.iter()) {
        set(*x_val, *y_val, *color_val, pitch, fb);
    }
}
