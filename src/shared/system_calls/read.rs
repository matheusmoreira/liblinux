use crate::Errno;
use crate::FileDescriptor as Fd;
use crate::definitions;

pub unsafe fn read(
    descriptor: Fd,
    buffer: *mut u8,
    buffer_length: usize,
) -> Result<usize, Errno> {
    // SAFETY: `__NR_read` is a 3 argument system call
    // that takes the integer file `descriptor`,
    // a writable buffer pointer, and its length.
    let result = unsafe {
        crate::system_call!(
            definitions::__NR_read,
            descriptor as usize,
            buffer as usize,
            buffer_length
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
    fn read_receives_bytes_into_the_buffer() {
        // TODO: convert to pure liblinux test when more system calls are in
        let (sender, receiver) = UnixDatagram::pair().unwrap();
        let message = b"liblinux";
        assert_eq!(sender.send(message).unwrap(), message.len());

        let mut buffer = [0; 8];

        // SAFETY: `receiver` owns the open socket descriptor,
        // and `buffer` is writable for the duration of the call.
        let bytes_read = unsafe {
            read(receiver.as_raw_fd(), buffer.as_mut_ptr(), buffer.len())
        };

        assert_eq!(bytes_read, Ok(message.len()));
        assert_eq!(buffer, *message);
    }
}
