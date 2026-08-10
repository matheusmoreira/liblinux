use crate::Errno;
use crate::FileDescriptor as Fd;
use crate::definitions;

pub fn shutdown(descriptor: Fd, how: i32) -> Result<(), Errno> {
    // SAFETY: `__NR_shutdown` is a 2 argument system call
    // that takes only integer arguments and dereferences
    // no pointers.
    let result = unsafe {
        crate::system_call!(
            definitions::__NR_shutdown,
            descriptor as usize,
            how as usize
        )
    };
    Errno::from_system_call(result).map(|_| ())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::definitions::{
        __kernel_timespec, AF_UNIX, CLOCK_MONOTONIC, SHUT_WR, SOCK_STREAM,
        UNIX_PATH_MAX, sockaddr_un,
    };
    use crate::system_calls::{
        accept4, bind, clock_gettime, close, connect, listen, read, socket,
        write,
    };

    #[test]
    fn shuts_down_only_the_sending_direction() {
        let listener = socket(AF_UNIX, SOCK_STREAM, 0).unwrap();

        let mut address = sockaddr_un {
            sun_family: AF_UNIX as u16,
            sun_path: [0; UNIX_PATH_MAX],
        };

        let mut time = __kernel_timespec {
            tv_sec: 0,
            tv_nsec: 0,
        };

        // SAFETY: `time` remains writable for the duration  of the system call.
        assert_eq!(
            unsafe { clock_gettime(CLOCK_MONOTONIC, &mut time) },
            Ok(())
        );

        // Construct an abstract Unix socket name
        // from the current monotonic time.
        let mut name_length = 1;

        let seconds = time.tv_sec.to_ne_bytes();
        address.sun_path[name_length..name_length + seconds.len()]
            .copy_from_slice(&seconds);
        name_length += seconds.len();

        let nanoseconds = time.tv_nsec.to_ne_bytes();
        address.sun_path[name_length..name_length + nanoseconds.len()]
            .copy_from_slice(&nanoseconds);
        name_length += nanoseconds.len();

        let identity =
            (core::ptr::from_ref(&address) as usize).to_ne_bytes();
        address.sun_path[name_length..name_length + identity.len()]
            .copy_from_slice(&identity);
        name_length += identity.len();

        let address_length =
            core::mem::size_of_val(&address.sun_family) + name_length;
        let address = core::ptr::from_ref(&address).cast::<u8>();

        // SAFETY: `address` points to `address_length` initialized
        // bytes that remain readable for the duration of the call.
        assert_eq!(
            unsafe { bind(listener, address, address_length as i32) },
            Ok(())
        );
        assert_eq!(listen(listener, 1), Ok(()));

        let left = socket(AF_UNIX, SOCK_STREAM, 0).unwrap();

        // SAFETY: `address` points to `address_length` initialized
        // bytes that remain readable for the duration of the call.
        assert_eq!(
            unsafe { connect(left, address, address_length as i32) },
            Ok(())
        );

        // SAFETY: both optional address output pointers are null.
        let right = unsafe {
            accept4(
                listener,
                core::ptr::null_mut(),
                core::ptr::null_mut(),
                0,
            )
        }
        .unwrap();

        // SAFETY: `listener` remains open and owned by this test.
        assert_eq!(unsafe { close(listener) }, Ok(()));

        let before = b"before";

        // SAFETY: `before` remains readable for the duration of the call.
        assert_eq!(
            unsafe { write(left, before.as_ptr(), before.len()) },
            Ok(before.len())
        );

        assert_eq!(shutdown(left, SHUT_WR), Ok(()));

        let mut received = [0; 6];

        // SAFETY: `received` remains writable for the duration of the call.
        assert_eq!(
            unsafe { read(right, received.as_mut_ptr(), received.len()) },
            Ok(received.len())
        );
        assert_eq!(received, *before);

        let mut eof = [0; 1];

        // SAFETY: `eof` remains writable for the duration of the call.
        assert_eq!(
            unsafe { read(right, eof.as_mut_ptr(), eof.len()) },
            Ok(0)
        );

        let reverse = b"reverse";

        // SAFETY: `reverse` remains readable for the duration of the call.
        assert_eq!(
            unsafe { write(right, reverse.as_ptr(), reverse.len()) },
            Ok(reverse.len())
        );

        let mut received = [0; 7];

        // SAFETY: `received` remains writable for the duration of the call.
        assert_eq!(
            unsafe { read(left, received.as_mut_ptr(), received.len()) },
            Ok(received.len())
        );
        assert_eq!(received, *reverse);

        // SAFETY: both descriptors remain open and owned by this test.
        assert_eq!(unsafe { close(left) }, Ok(()));
        assert_eq!(unsafe { close(right) }, Ok(()));
    }
}
