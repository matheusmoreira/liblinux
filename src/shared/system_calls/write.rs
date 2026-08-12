use crate::Errno;
use crate::FileDescriptor as Fd;
use crate::definitions;

pub unsafe fn write(
    descriptor: Fd,
    buffer: *const u8,
    buffer_length: usize,
) -> Result<usize, Errno> {
    // SAFETY: `__NR_write` is a 3 argument system call
    // that takes the integer file `descriptor`,
    // a readable buffer pointer, and its length.
    let result = unsafe {
        crate::system_call!(
            definitions::__NR_write,
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
    use std::io::Read;
    use std::os::fd::AsRawFd;
    use std::os::unix::net::UnixStream;

    #[test]
    fn write_sends_the_buffer_bytes() {
        // TODO: convert to pure liblinux test when more system calls are in
        let (writer, mut reader) = UnixStream::pair().unwrap();
        let message = b"liblinux";

        // SAFETY: `writer` owns the open socket descriptor,
        // and `message` is readable for the duration of the call.
        let result = unsafe {
            write(writer.as_raw_fd(), message.as_ptr(), message.len())
        };

        assert_eq!(result, Ok(message.len()));

        let mut received = [0; 8];
        reader.read_exact(&mut received).unwrap();
        assert_eq!(received, *message);
    }
}
