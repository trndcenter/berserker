use libc::{AF_INET, SOCK_STREAM};
use std::collections::HashMap;
use std::{fs::File, os::fd::FromRawFd};

use super::{SysCaller, get_argument};
use log::info;
use syscalls::{Errno, Sysno, syscall};

#[derive(Debug)]
pub struct SocketCall {
    pub domain: usize,
    pub stype: usize,
    pub protocol: usize,
}

impl SocketCall {
    pub fn new(socket_args: &HashMap<String, String>) -> Self {
        let domain = get_argument(socket_args, "domain", AF_INET as usize);
        let stype = get_argument(socket_args, "type", SOCK_STREAM as usize);
        let protocol = get_argument(socket_args, "protocol", 0);

        Self {
            domain,
            stype,
            protocol,
        }
    }
}

impl SysCaller for SocketCall {
    fn call(&self) -> Result<usize, Errno> {
        let res = unsafe {
            syscall!(
                Sysno::socket,
                self.domain,
                self.stype | libc::SOCK_NONBLOCK as usize,
                self.protocol
            )
        };

        if let Ok(fd) = res {
            // Close file descriptor
            unsafe { File::from_raw_fd(fd as i32) };
        }

        res
    }
}
