use std::collections::HashMap;
use std::ffi::CString;
use std::fs::File;
use std::os::fd::FromRawFd;

use io_uring::opcode::OpenAt;
use io_uring::squeue::Entry;
use io_uring::{IoUring, types};
use syscalls::Errno;

use crate::worker::io_uring::IOUringCaller;
use crate::worker::io_uring::get_argument;

#[derive(Debug)]
#[allow(dead_code)]
pub(super) struct OpenatIOUringCall {
    openat: Entry,

    pathname: CString, // used a raw pointer from string
}

impl OpenatIOUringCall {
    pub fn new(openat_args: &HashMap<String, String>) -> Self {
        let pathname = get_argument(
            openat_args,
            "pathname",
            CString::new("/tmp").unwrap(),
        );
        let flags = get_argument(openat_args, "flags", 0);
        let mode = get_argument(openat_args, "mode", 0);
        let openat = OpenAt::new(types::Fd(-1), pathname.as_ptr())
            .flags(flags)
            .mode(mode)
            .build();
        Self { openat, pathname }
    }
}

impl IOUringCaller for OpenatIOUringCall {
    fn submit(&self, ring: &mut IoUring) -> Result<usize, Errno> {
        unsafe {
            ring.submission()
                .push(&self.openat)
                .expect("submission queue is full");
        }
        ring.submit_and_wait(1).expect("submission failed");

        let cqe = ring.completion().next().expect("completion queue is empty");
        if cqe.result() > -1 {
            // Close file descriptor
            unsafe { File::from_raw_fd(cqe.result()) };
            return Ok(cqe.result() as usize);
        }
        Err(Errno::new(-cqe.result()))
    }
}
