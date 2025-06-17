use alloc::{string::String, vec::Vec};

#[derive(Debug, Clone)]
pub struct Args {
    argc: usize,
    argv: Vec<String>,
}

impl Args {
    pub fn new(argc: usize, argv: Vec<String>) -> Args {
        Args {
            argc,
            argv
        }
    }

    pub fn len(&self) -> usize {
        self.argc
    }

    pub fn iter(&self) -> impl Iterator<Item = &String> {
        self.argv.iter()
    }

    pub fn collect(&self) -> Vec<String> {
        self.argv.clone()
    }
}

pub(crate) static mut ARGS: Option<Args> = None;

#[allow(static_mut_refs)] // maybe REALLY BAD ??
pub fn args() -> Args {
    unsafe {
        ARGS.clone().unwrap()
    }
}