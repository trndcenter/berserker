use std::collections::HashMap;
use std::ffi::CString;

use super::{SysCaller, get_argument};
use syscalls::{self, Sysno, syscall};

#[derive(Debug)]
pub struct ChownCall {
    pub pathname: CString,
    pub owner: usize,
    pub group: usize,
}

impl ChownCall {
    pub fn new(chown_args: &HashMap<String, String>) -> Self {
        let pathname =
            get_argument(chown_args, "pathname", CString::new("/tmp").unwrap());
        let owner = get_argument(chown_args, "owner", 0);
        let group = get_argument(chown_args, "group", 0);

        Self {
            pathname,
            owner,
            group,
        }
    }
}

impl SysCaller for ChownCall {
    fn call(&self) -> Result<usize, syscalls::Errno> {
        unsafe {
            syscall!(
                Sysno::chown,
                self.pathname.as_ptr(),
                self.owner,
                self.group
            )
        }
    }
}
