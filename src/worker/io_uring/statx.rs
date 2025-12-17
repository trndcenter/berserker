use std::collections::HashMap;
use std::ffi::CString;

use io_uring::opcode::Statx;
use io_uring::squeue::Entry;
use io_uring::{IoUring, types};
use syscalls::Errno;

use crate::worker::io_uring::IOUringCaller;
use crate::worker::io_uring::get_argument;

#[derive(Debug)]
#[allow(dead_code)]
pub(super) struct StatxIOUringCall {
    statx: Entry,

    pathname: CString, // used a raw pointer from string
    statx_struct: Box<libc::statx>, // used as a mutable raw pointer
}

impl StatxIOUringCall {
    pub fn new(openat_args: &HashMap<String, String>) -> Self {
        let pathname = get_argument(
            openat_args,
            "pathname",
            CString::new("/tmp").unwrap(),
        );
        let flags = get_argument(openat_args, "flags", 0);
        let mask = get_argument(openat_args, "mask", 0);
        let mut statx_struct: Box<libc::statx> =
            Box::new(unsafe { std::mem::zeroed() });

        let statx = Statx::new(
            types::Fd(-1),
            pathname.as_ptr(),
            statx_struct.as_mut() as *mut libc::statx as *mut _,
        )
        .flags(flags)
        .mask(mask)
        .build();
        Self {
            statx,
            pathname,
            statx_struct,
        }
    }
}

impl IOUringCaller for StatxIOUringCall {
    fn submit(&self, ring: &mut IoUring) -> Result<usize, Errno> {
        unsafe {
            ring.submission()
                .push(&self.statx)
                .expect("submission queue is full");
        }
        ring.submit_and_wait(1).expect("submission failed");

        let cqe = ring.completion().next().expect("completion queue is empty");

        if cqe.result() == 0 {
            Ok(0)
        } else {
            Err(Errno::new(-cqe.result()))
        }
    }
}
