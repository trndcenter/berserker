use super::{SysCaller, get_argument};
use std::collections::HashMap;
use syscalls::{Errno, Sysno, syscall};

#[derive(Debug)]
pub struct SetresuidCall {
    pub ruid: usize,
    pub euid: usize,
    pub suid: usize,
}

impl SetresuidCall {
    pub fn new(setresuid_args: &HashMap<String, String>) -> Self {
        let ruid = get_argument(setresuid_args, "ruid", 0);
        let euid = get_argument(setresuid_args, "euid", 0);
        let suid = get_argument(setresuid_args, "suid", 0);

        Self { ruid, euid, suid }
    }
}

impl SysCaller for SetresuidCall {
    fn call(&self) -> Result<usize, Errno> {
        unsafe { syscall!(Sysno::setresuid, self.ruid, self.euid, self.suid) }
    }
}
