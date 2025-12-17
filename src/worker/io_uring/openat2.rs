use std::collections::HashMap;
use std::ffi::CString;
use std::fs::File;
use std::os::fd::FromRawFd;

use io_uring::opcode::OpenAt2;
use io_uring::squeue::Entry;
use io_uring::{IoUring, types};
use syscalls::Errno;

use crate::worker::io_uring::IOUringCaller;
use crate::worker::io_uring::get_argument;

#[derive(Debug)]
#[allow(dead_code)]
pub(super) struct Openat2IOUringCall {
    openat: Entry,

    pathname: CString, // used a raw pointer from string
    openhow: Box<types::OpenHow>,
}

impl Openat2IOUringCall {
    pub fn new(openat2_args: &HashMap<String, String>) -> Self {
        let pathname = get_argument(
            openat2_args,
            "pathname",
            CString::new("/tmp").unwrap(),
        );
        let flags = get_argument(openat2_args, "flags", 0);
        let mode = get_argument(openat2_args, "mode", 0);
        let resolve = get_argument(openat2_args, "resolve", 0);
        let openhow = Box::new(
            types::OpenHow::new()
                .flags(flags)
                .mode(mode)
                .resolve(resolve),
        );

        let openat =
            OpenAt2::new(types::Fd(-1), pathname.as_ptr(), openhow.as_ref())
                .build();
        Self {
            openat,
            pathname,
            openhow,
        }
    }
}

impl IOUringCaller for Openat2IOUringCall {
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
