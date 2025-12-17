use libc::MS_PRIVATE;
use std::collections::HashMap;
use std::ffi::CString;

use super::{SysCaller, get_argument};
use syscalls::{Errno, Sysno};

#[derive(Debug)]
pub struct MountCall {
    pub source: CString,
    pub target: CString,
    pub filesystemtype: CString,
    pub mountflags: usize,
    pub data: usize,
}

impl MountCall {
    pub fn new(mount_args: &HashMap<String, String>) -> Self {
        let source =
            get_argument(mount_args, "source", CString::new("").unwrap());
        let target =
            get_argument(mount_args, "target", CString::new("/tmp").unwrap());
        let filesystemtype = get_argument(
            mount_args,
            "filesystemtype",
            CString::new("").unwrap(),
        );
        let mountflags =
            get_argument(mount_args, "mountflags", MS_PRIVATE as usize);
        let data = 0;

        Self {
            source,
            target,
            filesystemtype,
            mountflags,
            data,
        }
    }
}

impl SysCaller for MountCall {
    fn call(&self) -> Result<usize, Errno> {
        unsafe {
            syscalls::syscall!(
                Sysno::mount,
                self.source.as_ptr(),
                self.target.as_ptr(),
                self.filesystemtype.as_ptr(),
                self.mountflags,
                self.data
            )
        }
    }
}
