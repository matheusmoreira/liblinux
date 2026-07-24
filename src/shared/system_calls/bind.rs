use crate::Errno;
use crate::FileDescriptor as Fd;
use crate::definitions;

/// Bind a socket to an address.
///
/// `address` contains the exact socket address bytes
/// Linux will receive. The `address` length is significant.
/// Valid address formats and lengths vary according to the
/// socket's address family. The `address` begins with a
/// 16 bit address family field.
///
/// For Unix sockets, address family always equals `AF_UNIX`
/// and is followed by the address path which may be up to
/// `UNIX_PATH_MAX` bytes which is defined as 108 bytes.
/// Although the `sockaddr_un` structure defined by the
/// Linux UAPI contains a maximally sized path buffer,
/// this is not actually mandatory: smaller paths
/// could be created and passed, and they are accepted
/// by the `bind` system call. Passing a two byte `address`,
/// or, in other words, not passing a path buffer at all,
/// instructs Linux to autobind: the kernel automatically
/// chooses an abstract address.
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
pub unsafe fn bind(
    descriptor: Fd,
    address: *const u8,
    address_length: i32,
) -> Result<(), Errno> {
    // SAFETY: `__NR_bind` is a 3 argument system call
    // that takes the integer file `descriptor`,
    // a socket address pointer, and its signed length.
    let result = unsafe {
        crate::system_call!(
            definitions::__NR_bind,
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
    use crate::shared::definitions::{AF_UNIX, SOCK_STREAM, UNIX_PATH_MAX, sockaddr_un};
    use crate::shared::system_calls::{close, socket};

    #[test]
    fn binds_a_unix_socket_to_an_abstract_address() {
        let descriptor = socket(AF_UNIX, SOCK_STREAM, 0).unwrap();

        let name = format!("liblinux-bind-{}", std::process::id());

        let mut address = sockaddr_un {
            sun_family: AF_UNIX as u16,
            sun_path: [0; UNIX_PATH_MAX],
        };

        // address.sun_path[0] is NUL.
        // The leading NUL selects Linux's
        // abstract Unix socket namespace.
        address.sun_path[1..1 + name.len()].copy_from_slice(name.as_bytes());

        // In abstract addresses, only bytes covered
        // by the address length belong to the name.
        // Trailing NULs would be significant.
        let address_length =
            core::mem::size_of_val(&address.sun_family) + 1 + name.len();

        let address = core::ptr::from_ref(&address).cast::<u8>();

        // SAFETY: `address` points to `address_length` initialized bytes
        // that remain live and readable for the duration of the call.
        assert_eq!(
            unsafe { bind(descriptor, address, address_length as i32) },
            Ok(())
        );

        // SAFETY: `descriptor` was returned by `socket` above
        // and has not been transferred or otherwise closed.
        assert_eq!(unsafe { close(descriptor) }, Ok(()));
    }

    #[test]
    fn binds_a_unix_socket_to_an_automatic_address() {
        // autobind: pass in only the address family,
        // Linux automatically picks an abstract address name

        let family = AF_UNIX as u16;
        let address = core::ptr::from_ref(&family).cast::<u8>();
        let address_length = core::mem::size_of_val(&family) as i32;

        let descriptor = socket(AF_UNIX, SOCK_STREAM, 0).unwrap();

        // SAFETY: `address` points to the initialized `family` value,
        // which remains readable for the duration of the system call.
        assert_eq!(
            unsafe { bind(descriptor, address, address_length) },
            Ok(())
        );

        // SAFETY: `descriptor` was returned by `socket` above
        // and has not been transferred or otherwise closed.
        assert_eq!(unsafe { close(descriptor) }, Ok(()));
    }

    #[test]
    fn trailing_nul_is_part_of_an_abstract_unix_address() {
        let first = socket(AF_UNIX, SOCK_STREAM, 0).unwrap();
        let second = socket(AF_UNIX, SOCK_STREAM, 0).unwrap();

        let name = format!("liblinux-bind-nul-{}", std::process::id());

        let mut address = sockaddr_un {
            sun_family: AF_UNIX as u16,
            sun_path: [0; UNIX_PATH_MAX],
        };

        address.sun_path[1..1 + name.len()].copy_from_slice(name.as_bytes());

        let address_length =
            core::mem::size_of_val(&address.sun_family) + 1 + name.len();

        let bytes = core::ptr::from_ref(&address).cast::<u8>();

        // SAFETY: both addresses point into the initialized `address`,
        // which remains readable for the duration of each call.
        assert_eq!(
            unsafe { bind(first, bytes, address_length as i32) },
            Ok(())
        );
        assert_eq!(
            unsafe { bind(second, bytes, address_length as i32 + 1) },
            Ok(())
        );

        // SAFETY: both descriptors were returned by `socket` above
        // and have not been transferred or otherwise closed.
        assert_eq!(unsafe { close(first) }, Ok(()));
        assert_eq!(unsafe { close(second) }, Ok(()));
    }
}
