use crate::Errno;
use crate::FileDescriptor as Fd;
use crate::definitions;

/// Mark a socket as listening for connections.
pub fn listen(descriptor: Fd, backlog: i32) -> Result<(), Errno> {
    // SAFETY: `__NR_listen` is a 2 argument system call
    // that takes scalar arguments and dereferences
    // no pointers, so it is memory safe.
    let result = unsafe {
        crate::system_call!(
            definitions::__NR_listen,
            descriptor as usize,
            backlog as usize
        )
    };
    Errno::from_system_call(result).map(|_| ())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shared::definitions::{AF_UNIX, SOCK_STREAM};
    use crate::shared::system_calls::{bind, close, socket};

    #[test]
    fn listens_on_a_bound_unix_socket() {
        let descriptor = socket(AF_UNIX, SOCK_STREAM, 0).unwrap();

        // Autobind an abstract Unix socket name.
        let family = AF_UNIX as u16;
        let address = core::ptr::from_ref(&family).cast::<u8>();
        let address_length = core::mem::size_of_val(&family) as i32;

        // SAFETY: `address` points to the initialized `family` value,
        // which remains readable for the duration of the system call.
        assert_eq!(
            unsafe { bind(descriptor, address, address_length) },
            Ok(())
        );

        assert_eq!(listen(descriptor, 1), Ok(()));

        // SAFETY: `descriptor` was returned by `socket` above
        // and has not been transferred or otherwise closed.
        assert_eq!(unsafe { close(descriptor) }, Ok(()));
    }
}
