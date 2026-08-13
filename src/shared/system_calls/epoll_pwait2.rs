use crate::Errno;
use crate::FileDescriptor as Fd;
use crate::definitions;
use crate::definitions::__kernel_timespec;
use crate::definitions::epoll_event;

pub unsafe fn epoll_pwait2(
    epoll: Fd,
    events: *mut epoll_event,
    max_events: i32,
    timeout: *const __kernel_timespec,
    signal_mask: *const u8,
    signal_mask_size: usize,
) -> Result<usize, Errno> {
    // SAFETY: `__NR_epoll_pwait2` is a six argument system call
    // with three integers, an output event pointer, and two
    // optional input pointers.
    let result = unsafe {
        crate::system_call!(
            definitions::__NR_epoll_pwait2,
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
    use crate::definitions::EPOLL_CLOEXEC;
    use crate::system_calls::{close, epoll_create1};

    #[test]
    fn accepts_a_zero_nanosecond_timeout() {
        let timeout = __kernel_timespec { tv_sec: 0, tv_nsec: 0 };
        let mut event = epoll_event { events: 0, data: 0 };

        let epoll = epoll_create1(EPOLL_CLOEXEC).unwrap();

        // SAFETY: `event` can hold one event and is writable,
        // `timeout` is readable, and the null signal mask
        // makes Linux ignore its zero size.
        assert_eq!(
            unsafe {
                epoll_pwait2(
                    epoll,
                    &mut event,
                    1,
                    &timeout,
                    core::ptr::null(),
                    0,
                )
            },
            Ok(0)
        );

        // SAFETY: `epoll` remains open and owned by this test.
        assert_eq!(unsafe { close(epoll) }, Ok(()));
    }
}
