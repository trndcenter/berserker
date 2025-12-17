use super::{SysCaller, get_argument};
use std::collections::HashMap;
use syscalls::{Errno, Sysno};

#[derive(Debug)]
pub struct UnshareCall {
    pub flags: usize,
}

impl UnshareCall {
    pub fn new(unshare_args: &HashMap<String, String>) -> Self {
        let flags = get_argument(unshare_args, "flags", 0);

        Self { flags }
    }
}

impl SysCaller for UnshareCall {
    fn call(&self) -> Result<usize, Errno> {
        unsafe { syscalls::syscall!(Sysno::unshare, self.flags) }
    }
}
