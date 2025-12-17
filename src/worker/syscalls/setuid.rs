use super::{SysCaller, get_argument};
use std::collections::HashMap;
use syscalls::{Errno, Sysno, syscall};

#[derive(Debug)]
pub struct SetuidCall {
    pub uid: usize,
}

impl SetuidCall {
    pub fn new(setuid_args: &HashMap<String, String>) -> Self {
        let uid = get_argument(setuid_args, "uid", 0);

        Self { uid }
    }
}

impl SysCaller for SetuidCall {
    fn call(&self) -> Result<usize, Errno> {
        unsafe { syscall!(Sysno::setuid, self.uid) }
    }
}
