use crate::Errno;
use crate::FileDescriptor as Fd;
use crate::definitions;

pub unsafe fn accept4(
    descriptor: Fd,
    address: *mut u8,
    address_length: *mut i32,
    flags: i32,
) -> Result<Fd, Errno> {
    // SAFETY: `__NR_accept4` is a 4 argument system call
    // that takes the scalar `descriptor` and `flags`,
    // in addition to the userspace pointers `address` and
    // `address_length`. When `address` is null, Linux ignores
    // both pointers. Otherwise, it may write through `address`
    // and may attempt to read from and write to `address_length`
    // even if `address_length` is null.
    let result = unsafe {
        crate::system_call!(
            definitions::__NR_accept4,
            descriptor as usize,
            address as usize,
            address_length as usize,
            flags as usize
        )
    };

    Errno::from_system_call(result).map(|descriptor| descriptor as Fd)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::definitions::{AF_UNIX, SOCK_NONBLOCK, SOCK_STREAM};
    use crate::system_calls::{bind, close, listen, socket};

    #[test]
    fn nonblocking_listener_with_no_pending_connection_reports_eagain() {
        let listener =
            socket(AF_UNIX, SOCK_STREAM | SOCK_NONBLOCK, 0).unwrap();

        // Autobind an abstract Unix socket name.
        let family = AF_UNIX as u16;
        let address = core::ptr::from_ref(&family).cast::<u8>();
        let address_length = core::mem::size_of_val(&family) as i32;

        // SAFETY: `address` points to the initialized `family` value,
        // which remains readable for the duration of the system call.
        assert_eq!(
            unsafe { bind(listener, address, address_length) },
            Ok(())
        );

        assert_eq!(listen(listener, 1), Ok(()));

        // SAFETY: both optional output pointers are null,
        // so Linux doesn't write the peer address and size.
        assert_eq!(
            unsafe {
                accept4(
                    listener,
                    core::ptr::null_mut(),
                    core::ptr::null_mut(),
                    0,
                )
            },
            Err(Errno::EAGAIN)
        );

        // SAFETY: `listener` was returned
        // by `socket` and remains open.
        assert_eq!(unsafe { close(listener) }, Ok(()));
    }
}
