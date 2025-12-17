use std::collections::HashMap;
use std::ffi::CString;

use io_uring::opcode::UnlinkAt;
use io_uring::squeue::Entry;
use io_uring::{IoUring, types};
use syscalls::Errno;

use crate::worker::io_uring::IOUringCaller;
use crate::worker::io_uring::get_argument;

#[derive(Debug)]
#[allow(dead_code)]
pub(super) struct UnlinkatIOUringCall {
    unlinkat: Entry,

    pathname: CString, // used a raw pointer from string
}

impl UnlinkatIOUringCall {
    pub fn new(unlinkat_args: &HashMap<String, String>) -> Self {
        let pathname = get_argument(
            unlinkat_args,
            "pathname",
            CString::new("/not_existing_file").unwrap(),
        );
        let flags = get_argument(unlinkat_args, "flags", 0);

        let unlinkat = UnlinkAt::new(types::Fd(-1), pathname.as_ptr())
            .flags(flags)
            .build();
        Self { unlinkat, pathname }
    }
}

impl IOUringCaller for UnlinkatIOUringCall {
    fn submit(&self, ring: &mut IoUring) -> Result<usize, Errno> {
        unsafe {
            ring.submission()
                .push(&self.unlinkat)
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
