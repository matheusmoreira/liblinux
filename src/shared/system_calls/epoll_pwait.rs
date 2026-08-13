use crate::Errno;
use crate::FileDescriptor as Fd;
use crate::definitions;
use crate::definitions::epoll_event;

pub unsafe fn epoll_pwait(
    epoll: Fd,
    events: *mut epoll_event,
    max_events: i32,
    timeout: i32,
    signal_mask: *const u8,
    signal_mask_size: usize,
) -> Result<usize, Errno> {
    // SAFETY: `__NR_epoll_pwait` is a six argument system call
    // that takes the integers `epoll`, `max_events`, `timeout`,
    // and `signal_mask_size`, in addition to the output `events`
    // pointer and optional input `signal_mask` pointer.
    let result = unsafe {
        crate::system_call!(
            definitions::__NR_epoll_pwait,
            epoll as usize,
            events as usize,
            max_events as usize,
            timeout as usize,
            signal_mask as usize,
            signal_mask_size
        )
    };
    Errno::from_system_call(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::definitions::{EPOLL_CLOEXEC, EPOLL_CTL_ADD, EPOLLIN};
    use crate::system_calls::{close, epoll_create1, epoll_ctl};
    use std::os::fd::AsRawFd;
    use std::os::unix::net::UnixDatagram;

    #[test]
    fn returns_a_ready_event_without_blocking() {
        let epoll = epoll_create1(EPOLL_CLOEXEC).unwrap();
        let (sender, receiver) = UnixDatagram::pair().unwrap();
        let event = epoll_event {
            events: EPOLLIN,
            data: 0x1234_5678,
        };

        // SAFETY: `event` remains readable for the system call.
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

        assert_eq!(sender.send(b"ready").unwrap(), 5);

        let mut returned_event = epoll_event { events: 0, data: 0 };
        // SAFETY: `returned_event` has room for one `epoll_event`.
        // A null signal mask pointer makes Linux ignore its size.
        assert_eq!(
            unsafe {
                epoll_pwait(
                    epoll,
                    &mut returned_event,
                    1,
                    0,
                    core::ptr::null(),
                    usize::MAX,
                )
            },
            Ok(1)
        );

        // SAFETY: Linux has initialized both fields.
        // Unaligned reads work for all structure layouts,
        // including the packed x86_64 layout.
        let returned_events = unsafe {
            core::ptr::addr_of!(returned_event.events).read_unaligned()
        };
        let returned_data = unsafe {
            core::ptr::addr_of!(returned_event.data).read_unaligned()
        };

        assert_ne!(returned_events & EPOLLIN, 0);
        assert_eq!(returned_data, 0x1234_5678);

        // SAFETY: `epoll` remains open and owned by this test.
        assert_eq!(unsafe { close(epoll) }, Ok(()));
    }

    #[test]
    fn validates_a_non_null_signal_mask_before_wait_arguments() {
        let signal_mask = [0_u8; 8];

        // SAFETY: `signal_mask` remains readable for the system call.
        // Its size is deliberately incorrect, and Linux rejects it
        // before it examines the invalid file descriptor,
        // event pointer, or `max_events`.
        assert_eq!(
            unsafe {
                epoll_pwait(
                    -1,
                    core::ptr::null_mut(),
                    0,
                    0,
                    signal_mask.as_ptr(),
                    0,
                )
            },
            Err(Errno::EINVAL)
        );
    }

    #[test]
    fn rejects_a_zero_event_count_before_checking_the_event_pointer() {
        let epoll = epoll_create1(EPOLL_CLOEXEC).unwrap();

        // SAFETY: The event pointer is intentionally inaccessible.
        // Linux rejects `max_events` before dereferencing the pointer.
        assert_eq!(
            unsafe {
                epoll_pwait(
                    epoll,
                    core::ptr::null_mut(),
                    0,
                    0,
                    core::ptr::null(),
                    0,
                )
            },
            Err(Errno::EINVAL)
        );

        // SAFETY: `epoll` remains open and owned by this test.
        assert_eq!(unsafe { close(epoll) }, Ok(()));
    }
}
