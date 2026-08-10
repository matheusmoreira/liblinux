use crate::Errno;
use crate::FileDescriptor as Fd;
use crate::definitions;

pub unsafe fn getsockopt(
    descriptor: Fd,
    level: i32,
    option: i32,
    value: *mut u8,
    value_length: *mut i32,
) -> Result<(), Errno> {
    // SAFETY: `__NR_getsockopt` is a 5 argument system call
    // with three integer arguments, `descriptor`, `level`
    // and `option`, as well as the userspace output pointer
    // `value` and input/output pointer `value_length`.
    let result = unsafe {
        crate::system_call!(
            definitions::__NR_getsockopt,
            descriptor as usize,
            level as usize,
            option as usize,
            value as usize,
            value_length as usize
        )
    };
    Errno::from_system_call(result).map(|_| ())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::definitions::{AF_UNIX, SOCK_STREAM, SOL_SOCKET, SO_ERROR};
    use crate::system_calls::{close, socket};

    #[test]
    fn reads_a_scalar_socket_option() {
        let mut value = [-1i32; 2];
        let mut value_length = core::mem::size_of_val(&value) as i32;

        let descriptor = socket(AF_UNIX, SOCK_STREAM, 0).unwrap();

        // SAFETY: `value` provides `value_length` writable bytes,
        // and `value_length` itself is initialized and writable.
        assert_eq!(
            unsafe {
                getsockopt(
                    descriptor,
                    SOL_SOCKET,
                    SO_ERROR,
                    core::ptr::from_mut(&mut value).cast::<u8>(),
                    &mut value_length,
                )
            },
            Ok(())
        );

        assert_eq!(value[0], 0);
        assert_eq!(value[1], -1);
        assert_eq!(value_length, core::mem::size_of::<i32>() as i32);

        // SAFETY: `descriptor` remains open and owned by this test.
        assert_eq!(unsafe { close(descriptor) }, Ok(()));
    }
}
