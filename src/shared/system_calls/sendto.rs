use crate::Errno;
use crate::FileDescriptor as Fd;
use crate::definitions;

pub unsafe fn sendto(
    descriptor: Fd,
    buffer: *const u8,
    buffer_length: usize,
    flags: i32,
    address: *const u8,
    address_length: i32,
) -> Result<usize, Errno> {
    // SAFETY: `__NR_sendto` is a system call that takes six arguments:
    // the file `descriptor`, an integer `flags`, as well as the two
    // userspace pointers `buffer` and `address` plus their associated
    // lengths `buffer_length` and `address_length`.
    let result = unsafe {
        crate::system_call!(
            definitions::__NR_sendto,
            descriptor as usize,
            buffer as usize,
            buffer_length,
            flags as usize,
            address as usize,
            address_length as usize
        )
    };
    Errno::from_system_call(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::definitions::{
        AF_UNIX,
        MSG_NOSIGNAL,
        SOCK_DGRAM,
        UNIX_PATH_MAX,
        sockaddr_un,
    };
    use crate::system_calls::{bind, socket};
    use std::os::fd::{AsRawFd, FromRawFd};
    use std::os::unix::net::UnixDatagram;

    #[test]
    fn sends_on_a_connected_socket_without_a_destination() {
        let (sender, receiver) = UnixDatagram::pair().unwrap();
        let message = b"sendto";

        // SAFETY: `message` remains readable during the call.
        // The connected socket requires no destination.
        // Linux ignores the length entirely when the
        // destination pointer is null.
        assert_eq!(
            unsafe {
                sendto(
                    sender.as_raw_fd(),
                    message.as_ptr(),
                    message.len(),
                    MSG_NOSIGNAL,
                    core::ptr::null(),
                    -1,
                )
            },
            Ok(message.len())
        );

        let mut received = [0; 6];
        assert_eq!(receiver.recv(&mut received).unwrap(), message.len());
        assert_eq!(received, *message);
    }

    #[test]
    fn sends_to_an_explicit_destination() {
        let receiver = socket(AF_UNIX, SOCK_DGRAM, 0).unwrap();

        let name = format!("liblinux-sendto-{}", std::process::id());
        let mut address_storage = sockaddr_un {
            sun_family: AF_UNIX as u16,
            sun_path: [0; UNIX_PATH_MAX],
        };

        // A leading NUL selects the abstract namespace.
        address_storage.sun_path[1..1 + name.len()]
            .copy_from_slice(name.as_bytes());

        let address_length =
            core::mem::size_of_val(&address_storage.sun_family) + 1 + name.len();

        let address = core::ptr::from_ref(&address_storage).cast::<u8>();

        // SAFETY: `address` points to initialized bytes that
        // remain readable for the duration of the system call.
        assert_eq!(
            unsafe { bind(receiver, address, address_length as i32) },
            Ok(())
        );

        // SAFETY: `receiver` is an open descriptor owned by this test.
        // Ownership is transferred to the `UnixDatagram`.
        let receiver = unsafe { UnixDatagram::from_raw_fd(receiver) };
        let sender = UnixDatagram::unbound().unwrap();
        let message = b"sendto";

        // SAFETY: `message` and the explicit destination address
        // remain readable for the duration of the system call.
        assert_eq!(
            unsafe {
                sendto(
                    sender.as_raw_fd(),
                    message.as_ptr(),
                    message.len(),
                    MSG_NOSIGNAL,
                    address,
                    address_length as i32,
                )
            },
            Ok(message.len())
        );

        let mut received = [0; 6];
        assert_eq!(receiver.recv(&mut received).unwrap(), message.len());
        assert_eq!(received, *message);
    }
}
