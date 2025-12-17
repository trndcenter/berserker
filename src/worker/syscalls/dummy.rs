use std::collections::HashMap;
use syscalls::{Errno, Sysno, syscall};

use super::SysCaller;

#[derive(Debug)]
pub struct DummyCall {
    pub syscall: Sysno,
}

impl DummyCall {
    pub fn new(_dummy_args: &HashMap<String, String>, syscall: Sysno) -> Self {
        Self { syscall }
    }
}

impl SysCaller for DummyCall {
    fn call(&self) -> Result<usize, Errno> {
        unsafe { syscall!(self.syscall) }
    }
}
