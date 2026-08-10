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
    // and an address output pointer plus its in/out length pointer.
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
    use crate::definitions::{AF_UNIX, MSG_PEEK, UNIX_PATH_MAX, sockaddr_un};
    use std::os::fd::AsRawFd;
    use std::os::linux::net::SocketAddrExt;
    use std::os::unix::net::{SocketAddr, UnixDatagram};

    #[repr(C, align(4))]
    struct AlignedUnixAddress(sockaddr_un);

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

    #[test]
    fn peeks_at_a_datagram_and_reports_its_sender_address() {
        let process = std::process::id();
        let sender_name = format!("liblinux-recvfrom-sender-{process}");
        let receiver_name = format!("liblinux-recvfrom-receiver-{process}");
        let sender_address =
            SocketAddr::from_abstract_name(sender_name.as_bytes()).unwrap();
        let receiver_address =
            SocketAddr::from_abstract_name(receiver_name.as_bytes()).unwrap();

        let sender = UnixDatagram::bind_addr(&sender_address).unwrap();
        let receiver = UnixDatagram::bind_addr(&receiver_address).unwrap();
        sender.set_nonblocking(true).unwrap();
        receiver.set_nonblocking(true).unwrap();

        let message = b"recvfrom";
        assert_eq!(
            sender.send_to_addr(message, &receiver_address).unwrap(),
            message.len()
        );

        const CANARY: u8 = 0xa5;
        const PREFIX_LENGTH: usize = 4;
        let mut buffer = [CANARY; 8];
        let mut address = AlignedUnixAddress(sockaddr_un {
            sun_family: 0,
            sun_path: [0; UNIX_PATH_MAX],
        });
        let mut address_length = core::mem::size_of::<sockaddr_un>() as i32;

        // SAFETY: the full datagram-sized `buffer` remains writable, although
        // only its prefix is advertised. `address` is zero-initialized,
        // four-byte aligned, and writable for `address_length` bytes.
        // The separate length is initialized, writable, and remains live.
        assert_eq!(
            unsafe {
                recvfrom(
                    receiver.as_raw_fd(),
                    buffer.as_mut_ptr(),
                    PREFIX_LENGTH,
                    MSG_PEEK,
                    core::ptr::from_mut(&mut address.0).cast::<u8>(),
                    &mut address_length,
                )
            },
            Ok(PREFIX_LENGTH)
        );

        assert_eq!(&buffer[..PREFIX_LENGTH], &message[..PREFIX_LENGTH]);
        assert_eq!(&buffer[PREFIX_LENGTH..], &[CANARY; 4]);

        let expected_address_length =
            core::mem::offset_of!(sockaddr_un, sun_path) + 1 + sender_name.len();
        assert_eq!(address_length, expected_address_length as i32);
        assert_eq!(address.0.sun_family, AF_UNIX as u16);
        assert_eq!(address.0.sun_path[0], 0);
        assert_eq!(
            &address.0.sun_path[1..1 + sender_name.len()],
            sender_name.as_bytes()
        );

        let mut complete = [CANARY; 8];
        // SAFETY: `complete` remains writable during the call.
        // A null address makes Linux ignore the null length pointer.
        assert_eq!(
            unsafe {
                recvfrom(
                    receiver.as_raw_fd(),
                    complete.as_mut_ptr(),
                    complete.len(),
                    0,
                    core::ptr::null_mut(),
                    core::ptr::null_mut(),
                )
            },
            Ok(message.len())
        );
        assert_eq!(complete, *message);
    }
}
