use crate::Errno;
use crate::FileDescriptor as Fd;
use crate::definitions;

pub unsafe fn recvfrom(
    descriptor: Fd,
    buffer: *mut u8,
    buffer_length: usize,
    flags: i32,
    address: *mut u8,
    address_length: *mut i32,
) -> Result<usize, Errno> {
    // SAFETY: `__NR_recvfrom` is a 6 argument system call
    // with three integer arguments, a writable buffer pointer,
    // and two optional sender address output pointers.
    let result = unsafe {
        crate::system_call!(
            definitions::__NR_recvfrom,
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
    use std::os::fd::AsRawFd;
    use std::os::unix::net::UnixDatagram;

    #[test]
    fn receives_while_discarding_the_sender_address() {
        let (sender, receiver) = UnixDatagram::pair().unwrap();
        let message = b"recvfrom";
        assert_eq!(sender.send(message).unwrap(), message.len());

        let mut buffer = [0; 8];
        // SAFETY: `buffer` remains writable during the call.
        // A null address discards sender metadata and makes
        // Linux ignore the null length pointer.
        assert_eq!(
            unsafe {
                recvfrom(
                    receiver.as_raw_fd(),
                    buffer.as_mut_ptr(),
                    buffer.len(),
                    0,
                    core::ptr::null_mut(),
                    core::ptr::null_mut(),
                )
            },
            Ok(message.len())
        );
        assert_eq!(buffer, *message);
    }
}
