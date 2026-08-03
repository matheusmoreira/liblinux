use crate::Errno;
use crate::FileDescriptor as Fd;
use crate::definitions;

/// Read bytes from the file `descriptor` into `buffer`.
/// Returns the number of bytes read, which may be less
/// than the size of the given buffer. No retries are made.
/// Reading while at the end of the file returns zero.
///
/// # Safety
///
/// Rust safety and semantics do not matter to the kernel.
/// However, the kernel semantics matter a lot to Rust.
/// The caller must ensure any memory access that Linux
/// successfully performs through `buffer` does not
/// violate the Rust memory model or any other
/// invariants in Rust or anywhere else.
///
/// The kernel may attempt to write `buffer.len()` bytes
/// at the address contained in `buffer`, which is not
/// actually required to be dereferenceable by Rust.
/// Passing unmapped user space addresses may cause Linux
/// to simply return `EFAULT`, a perfectly valid result.
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
