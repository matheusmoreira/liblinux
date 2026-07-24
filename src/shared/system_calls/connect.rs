use crate::Errno;
use crate::FileDescriptor as Fd;
use crate::definitions;

/// Connect a socket to an address.
///
/// `address` contains the exact socket address bytes
/// Linux will receive. The `address` length is significant.
/// Valid address formats and lengths vary according to the
/// socket's address family. The `address` begins with a
/// 16 bit address family field.
///
/// For Unix sockets, an `AF_UNIX` address is followed
/// by the address path which may be up to `UNIX_PATH_MAX`
/// bytes which is defined as 108 bytes. Although the
/// `sockaddr_un` structure defined by the Linux UAPI
/// contains a maximally sized path buffer, this is not
/// actually mandatory: smaller paths could be created
/// and passed, and they are accepted by the `connect`
/// system call.
///
/// A file system address begins with a non-NUL byte
/// and contains a file system path. A NUL terminator
/// is not actually necessary. All `UNIX_PATH_MAX`
/// bytes may consist of path data. Linux copies
/// the address into its own storage and writes
/// a NUL immediately after the supplied path
/// bytes before interpreting them as a path.
///
/// A leading NUL byte in the address path selects
/// the abstract Unix socket namespace, where names
/// are arbitrary binary data, length delimited.
/// Consequently, names such as `b"\0name"`
/// and `b"\0name\0"` are distinct.
///
/// For Unix datagram sockets, an `AF_UNSPEC` address
/// disconnects the socket from its current peer.
///
/// # Safety
///
/// Rust safety and semantics do not matter to the kernel.
/// However, the kernel semantics matter a lot to Rust.
/// The caller must ensure any memory access that Linux
/// successfully performs through `address` does not
/// violate the Rust memory model or any other
/// invariants in Rust or anywhere else.
///
/// Linux may attempt to read the number of bytes it interprets
/// as the address length from the address contained in `address`.
/// Linux's address length argument is an `int`, but `address.len()`
/// is a `usize`. Slice lengths greater than `i32::MAX` may therefore
/// be interpreted by Linux as a different numeric value.
///
/// After interpreting the argument as an `int`, Linux rejects
/// negative address lengths and lengths larger than its internal
/// socket address storage buffer before reading the address.
///
/// The address inside `address` is not actually required
/// to be dereferenceable by Rust. Passing unmapped user
/// space addresses may cause Linux to simply return `EFAULT`,
/// a perfectly valid result.
pub unsafe fn connect(
    descriptor: Fd,
    address: *const u8,
    address_length: i32,
) -> Result<(), Errno> {
    // SAFETY: `__NR_connect` is a 3 argument system call
    // that takes the integer file `descriptor`,
    // a socket address pointer, and its signed length.
    let result = unsafe {
        crate::system_call!(
            definitions::__NR_connect,
            descriptor as usize,
            address as usize,
            address_length as usize
        )
    };

    Errno::from_system_call(result).map(|_| ())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::system_calls::{bind, close, listen, socket};
    use crate::definitions::{
        AF_UNIX,
        SOCK_STREAM,
        UNIX_PATH_MAX,
        sockaddr_un,
    };

    #[test]
    fn connects_to_a_listening_unix_socket() {
        let server = socket(AF_UNIX, SOCK_STREAM, 0).unwrap();

        let name = format!("liblinux-connect-{}", std::process::id());

        let mut address = sockaddr_un {
            sun_family: AF_UNIX as u16,
            sun_path: [0; UNIX_PATH_MAX],
        };

        // A leading NUL selects the abstract namespace.
        address.sun_path[1..1 + name.len()].copy_from_slice(name.as_bytes());

        let address_length =
            core::mem::size_of_val(&address.sun_family) + 1 + name.len();

        let address = core::ptr::from_ref(&address).cast::<u8>();

        // SAFETY: `address` points to initialized bytes that remain
        // readable for the duration of the system call.
        assert_eq!(
            unsafe { bind(server, address, address_length as i32) },
            Ok(())
        );

        assert_eq!(listen(server, 1), Ok(()));

        let client = socket(AF_UNIX, SOCK_STREAM, 0).unwrap();

        // SAFETY: `address` points to initialized bytes that remain
        // readable for the duration of the system call.
        assert_eq!(
            unsafe { connect(client, address, address_length as i32) },
            Ok(())
        );

        // SAFETY: both descriptors were returned by `socket`
        // and have not been transferred or otherwise closed.
        assert_eq!(unsafe { close(client) }, Ok(()));
        assert_eq!(unsafe { close(server) }, Ok(()));
    }
}
