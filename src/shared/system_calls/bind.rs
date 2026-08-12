use crate::Errno;
use crate::FileDescriptor as Fd;
use crate::definitions;

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
