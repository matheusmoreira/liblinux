use crate::Errno;
use crate::FileDescriptor as Fd;
use crate::definitions;

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
