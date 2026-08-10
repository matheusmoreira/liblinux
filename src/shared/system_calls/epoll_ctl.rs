use crate::Errno;
use crate::FileDescriptor as Fd;
use crate::definitions;
use crate::definitions::epoll_event;

pub unsafe fn epoll_ctl(
    epoll: Fd,
    operation: i32,
    file_descriptor: Fd,
    event: *const epoll_event,
) -> Result<(), Errno> {
    // SAFETY: `__NR_epoll_ctl` is a four argument system call
    // that takes three integers and the `event` pointer.
    let result = unsafe {
        crate::system_call!(
            definitions::__NR_epoll_ctl,
            epoll as usize,
            operation as usize,
            file_descriptor as usize,
            event as usize
        )
    };
    Errno::from_system_call(result).map(|_| ())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::definitions::{
        EPOLL_CLOEXEC,
        EPOLL_CTL_ADD,
        EPOLL_CTL_DEL,
        EPOLL_CTL_MOD,
        EPOLLIN,
        EPOLLOUT,
    };
    use crate::system_calls::{close, epoll_create1};
    use std::os::fd::AsRawFd;
    use std::os::unix::net::UnixDatagram;

    #[test]
    fn reads_the_event_before_rejecting_an_unknown_operation() {
        let epoll = epoll_create1(EPOLL_CLOEXEC).unwrap();
        let (_sender, receiver) = UnixDatagram::pair().unwrap();

        // SAFETY: the event pointer is intentionally inaccessible.
        // Linux reads an event for every operation other than
        // `EPOLL_CTL_DEL`, even an unsupported operation,
        // so the pointer fault wins over `EINVAL`.
        assert_eq!(
            unsafe {
                epoll_ctl(
                    epoll,
                    0,
                    receiver.as_raw_fd(),
                    core::ptr::null(),
                )
            },
            Err(Errno::EFAULT)
        );

        let event = epoll_event { events: 0, data: 0 };

        // SAFETY: `event` is readable for the duration of the call.
        assert_eq!(
            unsafe { epoll_ctl(epoll, 0, receiver.as_raw_fd(), &event) },
            Err(Errno::EINVAL)
        );

        // SAFETY: `epoll` remains open and owned by this test.
        assert_eq!(unsafe { close(epoll) }, Ok(()));
    }

    #[test]
    fn adds_modifies_and_removes_an_item() {
        let epoll = epoll_create1(EPOLL_CLOEXEC).unwrap();
        let (_sender, receiver) = UnixDatagram::pair().unwrap();
        let event = epoll_event {
            events: EPOLLIN | EPOLLOUT,
            data: 0x1234_5678,
        };

        // SAFETY: `event` is readable in the epoll event layout.
        assert_eq!(
            unsafe {
                epoll_ctl(
                    epoll,
                    EPOLL_CTL_ADD,
                    receiver.as_raw_fd(),
                    &event,
                )
            },
            Ok(())
        );

        // SAFETY: The item exists, and `event` remains readable.
        assert_eq!(
            unsafe {
                epoll_ctl(
                    epoll,
                    EPOLL_CTL_MOD,
                    receiver.as_raw_fd(),
                    &event,
                )
            },
            Ok(())
        );

        // SAFETY: Linux ignores the null event pointer for `EPOLL_CTL_DEL`.
        assert_eq!(
            unsafe {
                epoll_ctl(
                    epoll,
                    EPOLL_CTL_DEL,
                    receiver.as_raw_fd(),
                    core::ptr::null(),
                )
            },
            Ok(())
        );

        // SAFETY: `epoll` remains open and owned by this test.
        assert_eq!(unsafe { close(epoll) }, Ok(()));
    }
}
