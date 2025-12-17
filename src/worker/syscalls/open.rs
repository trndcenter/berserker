use std::collections::HashMap;
use std::{ffi::CString, fs::File, os::fd::FromRawFd};

use syscalls::{Errno, Sysno, syscall};

use super::SysCaller;
use super::get_argument;

#[derive(Debug)]
pub struct OpenCall {
    pub pathname: CString,
    pub flags: usize,
    pub mode: usize,
}

impl OpenCall {
    pub fn new(open_args: &HashMap<String, String>) -> Self {
        let pathname =
            get_argument(open_args, "pathname", CString::new("/tmp").unwrap());
        let flags = get_argument(open_args, "flags", 0);
        let mode = get_argument(open_args, "mode", 0);

        Self {
            pathname,
            flags,
            mode,
        }
    }
}

impl SysCaller for OpenCall {
    fn call(&self) -> Result<usize, Errno> {
        let res = unsafe {
            syscall!(Sysno::open, self.pathname.as_ptr(), self.flags, self.mode)
        };

        if let Ok(fd) = res {
            // Close file descriptor
            unsafe { File::from_raw_fd(fd as i32) };
        }

        res
    }
}
