use crate::Errno;
use crate::FileDescriptor as Fd;
use crate::definitions;

pub fn epoll_create1(flags: i32) -> Result<Fd, Errno> {
    // SAFETY: `__NR_epoll_create1` is a one argument system call
    // that takes integer `flags` and dereferences no pointers.
    let result = unsafe {
        crate::system_call!(definitions::__NR_epoll_create1, flags as usize)
    };
    Errno::from_system_call(result).map(|descriptor| descriptor as Fd)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::definitions::EPOLL_CLOEXEC;
    use crate::system_calls::close;

    #[test]
    fn creates_and_closes_an_epoll_instance() {
        let descriptor = epoll_create1(EPOLL_CLOEXEC).unwrap();
        assert!(descriptor >= 0);

        // SAFETY: `descriptor` was just returned
        // by `epoll_create1` and remains open.
        assert_eq!(unsafe { close(descriptor) }, Ok(()));
    }

    #[test]
    fn rejects_unknown_flags() {
        assert_eq!(epoll_create1(-1), Err(Errno::EINVAL));
    }
}
