use crate::Errno;
use crate::FileDescriptor as Fd;
use crate::definitions;

pub fn socket(
    domain: i32,
    r#type: i32,
    protocol: i32,
) -> Result<Fd, Errno> {
    // SAFETY: `__NR_socket` is a 3 argument system call
    // that takes scalar arguments and dereferences
    // no pointers, so it is memory safe.
    let result = unsafe {
        crate::system_call!(
            definitions::__NR_socket,
            domain as usize,
            r#type as usize,
            protocol as usize
        )
    };
    Errno::from_system_call(result).map(|descriptor| descriptor as i32)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shared::definitions::{AF_UNIX, SOCK_STREAM};
    use crate::shared::system_calls::close;

    #[test]
    fn opens_and_closes_a_unix_stream_socket() {
        let descriptor = socket(AF_UNIX, SOCK_STREAM, 0).unwrap();
        assert!(descriptor >= 0);

        // SAFETY: `descriptor` was returned by `socket` above
        // and has not been transferred or otherwise closed.
        assert_eq!(unsafe { close(descriptor) }, Ok(()));
    }
}
