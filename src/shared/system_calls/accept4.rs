use crate::Errno;
use crate::FileDescriptor as Fd;
use crate::definitions;

/// Accept a pending connection on a listening socket
/// and return a new file descriptor for it.
///
/// If `address` is not null, then `address_length` must
/// point to an `i32` containing the size of `address`.
/// Linux will write the peer address to `address`,
/// and its size to `address_length`. The peer address
/// is truncated if there is not enough capacity.
/// If the value pointed to by `address_length`
/// is zero, Linux discards the peer address
/// but writes its size to `address_length`.
///
/// If `address` is null, then `address_length`
/// is completely ignored and the peer address
/// is discarded.
///
/// `flags` may contain `SOCK_CLOEXEC`, `SOCK_NONBLOCK`,
/// or both. These flags apply to the returned file descriptor.
/// They do not control whether waiting for a connection blocks,
/// that is determined by the listening socket.
///
/// # Safety
///
/// Rust safety and semantics do not matter to the kernel.
/// However, the kernel semantics matter a lot to Rust.
/// The caller must ensure any memory access that Linux
/// successfully performs through `address` or
/// `address_length` does not violate the Rust memory model
/// or any other invariants in Rust or anywhere else.
/// The fact Linux writes to the `address_length`
/// prevents the use of Rust slices.
///
/// When `address` is not null, Linux may read and write
/// the `i32` pointed to by `address_length` and may write
/// address bytes through `address`. If `address_length`
/// is null, Linux will accept the connection and fault
/// while trying to write through the null pointer,
/// consuming the connection but returning `EFAULT`.
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
