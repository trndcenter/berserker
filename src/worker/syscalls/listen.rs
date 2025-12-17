use std::collections::HashMap;
use std::{fs::File, os::fd::FromRawFd};

use syscalls::{Errno, Sysno, syscall};

use super::SysCaller;
use crate::worker::syscalls::socket::SocketCall;

#[derive(Debug)]
pub struct ListenCall {
    pub socket_call: SocketCall,
    pub sockfd: usize,
}

impl ListenCall {
    pub fn new(listen_args: &HashMap<String, String>) -> Self {
        let socket_call = SocketCall::new(listen_args);
        let sockfd = 0;

        Self {
            socket_call,
            sockfd,
        }
    }
}

impl Drop for ListenCall {
    fn drop(&mut self) {
        unsafe { File::from_raw_fd(self.sockfd as i32) };
    }
}

impl SysCaller for ListenCall {
    fn init(&mut self) -> Result<usize, Errno> {
        // Create socket directly instead of calling socket_call.call()
        self.sockfd = unsafe {
            syscall!(
                Sysno::socket,
                self.socket_call.domain,
                self.socket_call.stype | libc::SOCK_NONBLOCK as usize,
                self.socket_call.protocol
            )?
        };
        Ok(self.sockfd)
    }
    fn call(&self) -> Result<usize, Errno> {
        unsafe { syscall!(Sysno::listen, self.sockfd, 10) }
    }
}
