use libc::{S_IRWXG, S_IRWXO, S_IRWXU, S_ISVTX};
use std::collections::HashMap;
use std::ffi::CString;

use super::{SysCaller, get_argument};
use syscalls::{self, Sysno};

#[derive(Debug)]
pub struct ChmodCall {
    pub pathname: CString,
    pub mode: usize,
}

impl ChmodCall {
    pub fn new(chmod_args: &HashMap<String, String>) -> Self {
        let pathname =
            get_argument(chmod_args, "pathname", CString::new("/tmp").unwrap());
        let mode = get_argument(
            chmod_args,
            "mode",
            (S_ISVTX | S_IRWXU | S_IRWXG | S_IRWXO) as usize,
        );

        Self { pathname, mode }
    }
}

impl SysCaller for ChmodCall {
    fn call(&self) -> Result<usize, syscalls::Errno> {
        unsafe {
            syscalls::syscall!(Sysno::chmod, self.pathname.as_ptr(), self.mode)
        }
    }
}
