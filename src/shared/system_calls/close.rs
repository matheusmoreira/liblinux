use crate::Errno;
use crate::FileDescriptor as Fd;
use crate::definitions;

/// Closes a file descriptor.
///
/// Linux may report an error after releasing
/// a valid file descriptor. Do not retry the
/// system call after an error: the number may
/// have already been reused.
///
/// # Safety
///
/// If `descriptor` is open, this system call may close it
/// regardless of any Rust or foreign value referencing it.
/// The caller is responsible for ensuring that doing so
/// does not violate the invariants of any such live value.
///
/// `descriptor` does not need to identify an open file
/// descriptor. Linux returns `EBADF` for invalid inputs.
/// A valid `descriptor` must be treated as consumed after
/// this system call, regardless of the returned result.
pub unsafe fn close(descriptor: Fd) -> Result<(), Errno> {
    // SAFETY: `__NR_close` is a one argument system call that takes
    // a scalar file descriptor as its sole argument, no user space
    // pointer is passed to Linux.
    let result = unsafe {
        crate::system_call!(definitions::__NR_close, descriptor as usize)
    };
    Errno::from_system_call(result).map(|_| ())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn closing_an_invalid_descriptor_reports_a_bad_descriptor() {
        // SAFETY: -1 is outside the range of valid Linux file descriptors,
        // so it cannot identify a file descriptor owned by someone else.
        assert_eq!(unsafe { close(-1) }, Err(Errno::EBADF));
    }
}
