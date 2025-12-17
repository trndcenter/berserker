use libc::{MAP_ANONYMOUS, MAP_PRIVATE, PROT_EXEC, PROT_READ, PROT_WRITE};
use std::collections::HashMap;
use std::{fs::File, os::fd::FromRawFd};

use super::{SysCaller, get_argument};
use syscalls::{Errno, Sysno, syscall};

#[derive(Debug)]
pub struct MmapCall {
    pub address: usize,
    pub length: usize,
    pub prot: usize,
    pub flags: usize,
    pub fd: usize,
    pub offset: usize,
}

impl MmapCall {
    pub fn new(mmap_args: &HashMap<String, String>) -> Self {
        let address = 0;
        let length = get_argument(mmap_args, "length", 8);
        let prot = get_argument(
            mmap_args,
            "prot",
            (PROT_READ | PROT_WRITE | PROT_EXEC) as usize,
        );
        let flags = get_argument(
            mmap_args,
            "flags",
            (MAP_PRIVATE | MAP_ANONYMOUS) as usize,
        );
        let fd = get_argument(mmap_args, "fd", usize::MAX); // -1
        let offset = get_argument(mmap_args, "offset", 0);

        Self {
            address,
            length,
            prot,
            flags,
            fd,
            offset,
        }
    }
}

impl Drop for MmapCall {
    fn drop(&mut self) {
        unsafe {
            File::from_raw_fd(self.fd as i32);
        }
    }
}

impl SysCaller for MmapCall {
    fn call(&self) -> Result<usize, Errno> {
        let res = unsafe {
            syscall!(
                Sysno::mmap,
                self.address,
                self.length,
                self.prot,
                self.flags,
                self.fd,
                self.offset
            )
        };

        if let Ok(addr) = res {
            // Unmap memory
            unsafe { syscall!(Sysno::munmap, addr, self.length)? };
        }

        res
    }
}
