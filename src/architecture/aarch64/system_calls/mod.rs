//! The AArch64 Linux system calls.
//!
//! One file per system call. Functions perform
//! their exact system call and nothing else.

pub use crate::shared::system_calls::*;

system_calls! {
    aarch64 {
        clock_gettime,
        mmap,
    }
}
