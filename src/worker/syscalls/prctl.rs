use super::{SysCaller, get_argument};
use libc::PR_GET_KEEPCAPS;
use std::collections::HashMap;
use syscalls::{self, Sysno, syscall};

#[derive(Debug)]
pub struct PrctlCall {
    pub op: usize,
    pub arg2: usize,
    pub arg3: usize,
    pub arg4: usize,
    pub arg5: usize,
}

impl PrctlCall {
    pub fn new(prctl_args: &HashMap<String, String>) -> Self {
        let op = get_argument(prctl_args, "op", PR_GET_KEEPCAPS as usize);
        let arg2 = get_argument(prctl_args, "arg2", 0);
        let arg3 = get_argument(prctl_args, "arg3", 0);
        let arg4 = get_argument(prctl_args, "arg4", 0);
        let arg5 = get_argument(prctl_args, "arg5", 0);

        Self {
            op,
            arg2,
            arg3,
            arg4,
            arg5,
        }
    }
}

impl SysCaller for PrctlCall {
    fn call(&self) -> Result<usize, syscalls::Errno> {
        unsafe {
            syscall!(
                Sysno::prctl,
                self.op,
                self.arg2,
                self.arg3,
                self.arg4,
                self.arg5
            )
        }
    }
}
