use crate::Errno;
use crate::FileDescriptor as Fd;
use crate::definitions;

pub unsafe fn setsockopt(
    descriptor: Fd,
    level: i32,
    option: i32,
    value: *const u8,
    value_length: i32,
) -> Result<(), Errno> {
    // SAFETY: `__NR_setsockopt` is a 5 argument system call
    // with the four integer arguments `descriptor`, `level`,
    // `option` and `value_length`, plus the address `value`.
    let result = unsafe {
        crate::system_call!(
            definitions::__NR_setsockopt,
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
    use crate::definitions::{
        AF_UNIX,
        SOCK_STREAM,
        SOL_SOCKET,
        SO_REUSEADDR,
    };
    use crate::system_calls::{close, getsockopt, socket};

    #[test]
    fn sets_a_scalar_socket_option() {
        let enabled = 1i32;
        let descriptor = socket(AF_UNIX, SOCK_STREAM, 0).unwrap();

        // SAFETY: `enabled` provides a readable integer option value
        // whose size is supplied through `value_length`.
        assert_eq!(
            unsafe {
                setsockopt(
                    descriptor,
                    SOL_SOCKET,
                    SO_REUSEADDR,
                    core::ptr::from_ref(&enabled).cast::<u8>(),
                    core::mem::size_of_val(&enabled) as i32,
                )
            },
            Ok(())
        );

        let mut returned = 0i32;
        let mut returned_length = core::mem::size_of_val(&returned) as i32;

        // SAFETY: `returned` is a writable output integer, and
        // `returned_length` is an initialized, writable input/output
        // integer describing its capacity.
        assert_eq!(
            unsafe {
                getsockopt(
                    descriptor,
                    SOL_SOCKET,
                    SO_REUSEADDR,
                    core::ptr::from_mut(&mut returned).cast::<u8>(),
                    &mut returned_length,
                )
            },
            Ok(())
        );

        assert_eq!(returned, 1);
        assert_eq!(returned_length, core::mem::size_of::<i32>() as i32);

        // SAFETY: `descriptor` remains open and owned by this test.
        assert_eq!(unsafe { close(descriptor) }, Ok(()));
    }

    #[test]
    fn rejects_a_negative_value_length() {
        let descriptor = socket(AF_UNIX, SOCK_STREAM, 0).unwrap();

        // SAFETY: a negative `value_length` is rejected
        // before Linux attempts to read through `value`.
        assert_eq!(
            unsafe {
                setsockopt(
                    descriptor,
                    SOL_SOCKET,
                    SO_REUSEADDR,
                    core::ptr::null(),
                    -1,
                )
            },
            Err(Errno::EINVAL)
        );

        // SAFETY: `descriptor` remains open and owned by this test.
        assert_eq!(unsafe { close(descriptor) }, Ok(()));
    }
}
