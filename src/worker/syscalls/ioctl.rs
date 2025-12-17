use super::SysCaller;
use std::collections::HashMap;
use std::fs::File;
use std::os::fd::IntoRawFd;
use std::os::unix::prelude::FromRawFd;
use syscalls::syscall;
use syscalls::{Errno, Sysno};

#[derive(Debug)]
pub struct IoctlCall {
    pub fd: usize,
    pub op: usize,
    pub argp: usize,
}

impl IoctlCall {
    pub fn new(_: &HashMap<String, String>) -> Self {
        let fd = 0; // Will be initialized in init()
        let op = 0; // Default value, can be overridden if needed
        let argp = 0; // Default value, can be overridden if needed

        Self { fd, op, argp }
    }
}

impl Drop for IoctlCall {
    fn drop(&mut self) {
        unsafe { File::from_raw_fd(self.fd as i32) };
    }
}

impl SysCaller for IoctlCall {
    fn init(&mut self) -> Result<usize, Errno> {
        self.fd = match File::open("/dev/null") {
            Ok(f) => f.into_raw_fd() as usize,
            Err(e) => return Err(Errno::new(e.raw_os_error().unwrap())),
        };
        Ok(self.fd)
    }
    fn call(&self) -> Result<usize, Errno> {
        unsafe { syscall!(Sysno::ioctl, self.fd, self.op, self.argp) }
    }
}
