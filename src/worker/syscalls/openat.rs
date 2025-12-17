use std::collections::HashMap;
use std::{ffi::CString, fs::File, os::fd::FromRawFd};

use super::SysCaller;
use super::get_argument;
use syscalls::{Errno, Sysno, syscall};

#[derive(Debug)]
pub struct OpenatCall {
    pub dirfd: usize,
    pub pathname: CString,
    pub flags: usize,
    pub mode: usize,
}

impl OpenatCall {
    pub fn new(openat_args: &HashMap<String, String>) -> Self {
        let dirfd = 0; // Default value, can be overridden if needed
        let pathname = get_argument(
            openat_args,
            "pathname",
            CString::new("/tmp").unwrap(),
        );
        let flags = get_argument(openat_args, "flags", 0);
        let mode = get_argument(openat_args, "mode", 0);

        Self {
            dirfd,
            pathname,
            flags,
            mode,
        }
    }
}

impl SysCaller for OpenatCall {
    fn call(&self) -> Result<usize, Errno> {
        let res = unsafe {
            syscall!(
                Sysno::openat,
                self.dirfd,
                self.pathname.as_ptr(),
                self.flags,
                self.mode
            )
        };

        if let Ok(fd) = res {
            // Close file descriptor
            unsafe { File::from_raw_fd(fd as i32) };
        }

        res
    }
}
