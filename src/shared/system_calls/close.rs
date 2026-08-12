use crate::Errno;
use crate::FileDescriptor as Fd;
use crate::definitions;

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
