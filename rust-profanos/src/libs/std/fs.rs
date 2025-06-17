extern crate alloc;

use alloc::ffi::CString;

use crate::libs::libc::{self, CFILE, SEEK};

use super::io::Read;

#[derive(Debug)]
pub struct File {
    inner: *mut CFILE,
    metadata: Metadata
}

#[derive(Debug, Clone, Copy)]
pub struct Metadata {
    length: usize,
}

impl Metadata {
    pub fn len(&self) -> usize {
        self.length
    }
}

impl File {
    pub fn open(path: &str) -> Option<File> {
        let filepath = CString::new(path).unwrap();
        let mode = CString::new("r").unwrap();

        use crate::libs::libc::fopen_c;

        unsafe {
            // we open with the C interface
            let inner_file = fopen_c(filepath.as_ptr() as *const u8, mode.as_ptr() as *const u8);
            
            // we get the length of the file
            let base_pos = libc::ftell_c(inner_file);
            libc::fseek_c(inner_file, 0, SEEK::SEEK_END as usize);
            let size = libc::ftell_c(inner_file);
            libc::fseek_c(inner_file, 0, base_pos);

            // we transmute into a File
            let myfile = File {
                inner: inner_file,
                metadata: Metadata {
                    length: size
                }
            };

            Some(myfile)
        }
    }

    pub fn metadata(&self) -> Option<Metadata> {
        Some(self.metadata)
    }
}


impl Read for File {
    fn read(&mut self, buf: &mut [u8]) -> Option<usize> {
        let fd = self.inner;
        
        let bytes_read = unsafe {
            libc::fread_c(
                buf.as_mut_ptr(),
                1,
                buf.len(), 
                fd
            ) as usize
        };
        
        // If `bytes_read` is 0, we hit the end of the file
        if bytes_read == 0 {
            Some(0)
        } else if bytes_read < buf.len() {
            // Partially read data
            Some(bytes_read)
        } else {
            // Successfully read into the buffer
            Some(bytes_read)
        }
    }
}
