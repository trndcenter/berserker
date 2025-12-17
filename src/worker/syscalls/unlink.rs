use std::collections::HashMap;
use std::ffi::CString;

use super::{SysCaller, get_argument};
use syscalls::{Errno, Sysno, syscall};

#[derive(Debug)]
pub struct UnlinkCall {
    pub pathname: CString,
}

impl UnlinkCall {
    pub fn new(unlink_args: &HashMap<String, String>) -> Self {
        let pathname = get_argument(
            unlink_args,
            "pathname",
            CString::new("/privileged_dir/file").unwrap(),
        );

        Self { pathname }
    }
}

impl SysCaller for UnlinkCall {
    fn call(&self) -> Result<usize, Errno> {
        unsafe { syscall!(Sysno::unlink, self.pathname.as_ptr()) }
    }
}
