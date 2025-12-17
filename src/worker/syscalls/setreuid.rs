use super::{SysCaller, get_argument};
use std::collections::HashMap;
use syscalls::{Errno, Sysno, syscall};

#[derive(Debug)]
pub struct SetreuidCall {
    pub ruid: usize,
    pub euid: usize,
}

impl SetreuidCall {
    pub fn new(setreuid_args: &HashMap<String, String>) -> Self {
        let ruid = get_argument(setreuid_args, "ruid", 0);
        let euid = get_argument(setreuid_args, "euid", 0);

        Self { ruid, euid }
    }
}

impl SysCaller for SetreuidCall {
    fn call(&self) -> Result<usize, Errno> {
        unsafe { syscall!(Sysno::setreuid, self.ruid, self.euid) }
    }
}
