//! Linux error numbers

use core::num::NonZeroU16;

use crate::definitions;

/// A Linux error number
///
/// System call primitives normalize kernel failures
/// by returning the negated error number in the result,
/// in the `[-4095, -1]` interval. [`Errno`] holds any
/// code the kernel can produce since system calls can
/// theoretically return any of them, even those not
/// explicitly enumerated here.
///
/// There is no thread local `errno` global variable.
/// Negated error numbers from system calls can be decoded
/// via [`Errno::from_system_call`]. Error numbers obtained
/// any other way can be built with [`Errno::from_number`].
///
/// Named constants match the Linux kernel's UAPI definitions
/// and are defined in terms of the target architecture's
/// definitions, so architecture specific assignments
/// and aliases are respected.
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Errno(NonZeroU16);

impl Errno {
    /// Build from a number the caller knows is non-zero.
    /// Panic on zero to make invalid input an error,
    /// verified at compile time in case of named constant.
    pub const fn from_number(number: u16) -> Errno {
        match NonZeroU16::new(number) {
            Some(number) => Errno(number),
            None => panic!("errno cannot be zero"),
        }
    }

    /// Operation not permitted
    pub const EPERM: Errno = Errno::from_number(definitions::EPERM);

    /// No such file or directory
    pub const ENOENT: Errno = Errno::from_number(definitions::ENOENT);

    /// No such process
    pub const ESRCH: Errno = Errno::from_number(definitions::ESRCH);

    /// Interrupted system call
    pub const EINTR: Errno = Errno::from_number(definitions::EINTR);

    /// Input or output error
    pub const EIO: Errno = Errno::from_number(definitions::EIO);

    /// No such device or address
    pub const ENXIO: Errno = Errno::from_number(definitions::ENXIO);

    /// Argument list too long
    pub const E2BIG: Errno = Errno::from_number(definitions::E2BIG);

    /// Executable format error
    pub const ENOEXEC: Errno = Errno::from_number(definitions::ENOEXEC);

    /// Invalid file descriptor
    pub const EBADF: Errno = Errno::from_number(definitions::EBADF);

    /// No child processes
    pub const ECHILD: Errno = Errno::from_number(definitions::ECHILD);

    /// Resource temporarily unavailable
    /// The operation would block
    pub const EAGAIN: Errno = Errno::from_number(definitions::EAGAIN);

    /// Out of memory
    pub const ENOMEM: Errno = Errno::from_number(definitions::ENOMEM);

    /// Permission denied
    pub const EACCES: Errno = Errno::from_number(definitions::EACCES);

    /// Invalid address
    pub const EFAULT: Errno = Errno::from_number(definitions::EFAULT);

    /// Block device required
    pub const ENOTBLK: Errno = Errno::from_number(definitions::ENOTBLK);

    /// Device or resource busy
    pub const EBUSY: Errno = Errno::from_number(definitions::EBUSY);

    /// File exists
    pub const EEXIST: Errno = Errno::from_number(definitions::EEXIST);

    /// Cross device link
    pub const EXDEV: Errno = Errno::from_number(definitions::EXDEV);

    /// No such device
    pub const ENODEV: Errno = Errno::from_number(definitions::ENODEV);

    /// Not a directory
    pub const ENOTDIR: Errno = Errno::from_number(definitions::ENOTDIR);

    /// Is a directory
    pub const EISDIR: Errno = Errno::from_number(definitions::EISDIR);

    /// Invalid argument
    pub const EINVAL: Errno = Errno::from_number(definitions::EINVAL);

    /// Too many open files in the system
    pub const ENFILE: Errno = Errno::from_number(definitions::ENFILE);

    /// Too many open files in the process
    pub const EMFILE: Errno = Errno::from_number(definitions::EMFILE);

    /// Inappropriate ioctl for device
    pub const ENOTTY: Errno = Errno::from_number(definitions::ENOTTY);

    /// Text file busy
    pub const ETXTBSY: Errno = Errno::from_number(definitions::ETXTBSY);

    /// File too large
    pub const EFBIG: Errno = Errno::from_number(definitions::EFBIG);

    /// No space left on device
    pub const ENOSPC: Errno = Errno::from_number(definitions::ENOSPC);

    /// Illegal seek
    pub const ESPIPE: Errno = Errno::from_number(definitions::ESPIPE);

    /// Read only file system
    pub const EROFS: Errno = Errno::from_number(definitions::EROFS);

    /// Too many links
    pub const EMLINK: Errno = Errno::from_number(definitions::EMLINK);

    /// Broken pipe
    pub const EPIPE: Errno = Errno::from_number(definitions::EPIPE);

    /// Numerical argument out of domain
    pub const EDOM: Errno = Errno::from_number(definitions::EDOM);

    /// Numerical result out of range
    pub const ERANGE: Errno = Errno::from_number(definitions::ERANGE);

    /// Resource deadlock would occur
    pub const EDEADLK: Errno = Errno::from_number(definitions::EDEADLK);

    /// File name too long
    pub const ENAMETOOLONG: Errno = Errno::from_number(definitions::ENAMETOOLONG);

    /// No record locks available
    pub const ENOLCK: Errno = Errno::from_number(definitions::ENOLCK);

    /// Invalid system call number
    pub const ENOSYS: Errno = Errno::from_number(definitions::ENOSYS);

    /// Directory not empty
    pub const ENOTEMPTY: Errno = Errno::from_number(definitions::ENOTEMPTY);

    /// Too many symbolic links encountered
    pub const ELOOP: Errno = Errno::from_number(definitions::ELOOP);

    /// Operation would block
    pub const EWOULDBLOCK: Errno = Errno::from_number(definitions::EWOULDBLOCK);

    /// No message of desired type
    pub const ENOMSG: Errno = Errno::from_number(definitions::ENOMSG);

    /// Identifier removed
    pub const EIDRM: Errno = Errno::from_number(definitions::EIDRM);

    /// Channel number out of range
    pub const ECHRNG: Errno = Errno::from_number(definitions::ECHRNG);

    /// Level 2 not synchronized
    pub const EL2NSYNC: Errno = Errno::from_number(definitions::EL2NSYNC);

    /// Level 3 halted
    pub const EL3HLT: Errno = Errno::from_number(definitions::EL3HLT);

    /// Level 3 reset
    pub const EL3RST: Errno = Errno::from_number(definitions::EL3RST);

    /// Link number out of range
    pub const ELNRNG: Errno = Errno::from_number(definitions::ELNRNG);

    /// Protocol driver not attached
    pub const EUNATCH: Errno = Errno::from_number(definitions::EUNATCH);

    /// No CSI structure available
    pub const ENOCSI: Errno = Errno::from_number(definitions::ENOCSI);

    /// Level 2 halted
    pub const EL2HLT: Errno = Errno::from_number(definitions::EL2HLT);

    /// Invalid exchange
    pub const EBADE: Errno = Errno::from_number(definitions::EBADE);

    /// Invalid request descriptor
    pub const EBADR: Errno = Errno::from_number(definitions::EBADR);

    /// Exchange full
    pub const EXFULL: Errno = Errno::from_number(definitions::EXFULL);

    /// No anode
    pub const ENOANO: Errno = Errno::from_number(definitions::ENOANO);

    /// Invalid request code
    pub const EBADRQC: Errno = Errno::from_number(definitions::EBADRQC);

    /// Invalid slot
    pub const EBADSLT: Errno = Errno::from_number(definitions::EBADSLT);

    /// Resource deadlock would occur
    pub const EDEADLOCK: Errno = Errno::from_number(definitions::EDEADLOCK);

    /// Invalid font file format
    pub const EBFONT: Errno = Errno::from_number(definitions::EBFONT);

    /// Device not a stream
    pub const ENOSTR: Errno = Errno::from_number(definitions::ENOSTR);

    /// No data available
    pub const ENODATA: Errno = Errno::from_number(definitions::ENODATA);

    /// Timer expired
    pub const ETIME: Errno = Errno::from_number(definitions::ETIME);

    /// Out of streams resources
    pub const ENOSR: Errno = Errno::from_number(definitions::ENOSR);

    /// Machine is not on the network
    pub const ENONET: Errno = Errno::from_number(definitions::ENONET);

    /// Package not installed
    pub const ENOPKG: Errno = Errno::from_number(definitions::ENOPKG);

    /// Object is remote
    pub const EREMOTE: Errno = Errno::from_number(definitions::EREMOTE);

    /// Link has been severed
    pub const ENOLINK: Errno = Errno::from_number(definitions::ENOLINK);

    /// Advertise error
    pub const EADV: Errno = Errno::from_number(definitions::EADV);

    /// Srmount error
    pub const ESRMNT: Errno = Errno::from_number(definitions::ESRMNT);

    /// Communication error on send
    pub const ECOMM: Errno = Errno::from_number(definitions::ECOMM);

    /// Protocol error
    pub const EPROTO: Errno = Errno::from_number(definitions::EPROTO);

    /// Multihop attempted
    pub const EMULTIHOP: Errno = Errno::from_number(definitions::EMULTIHOP);

    /// RFS specific error
    pub const EDOTDOT: Errno = Errno::from_number(definitions::EDOTDOT);

    /// Not a data message
    pub const EBADMSG: Errno = Errno::from_number(definitions::EBADMSG);

    /// Invalid CRC detected
    pub const EFSBADCRC: Errno = Errno::from_number(definitions::EFSBADCRC);

    /// Value too large for defined data type
    pub const EOVERFLOW: Errno = Errno::from_number(definitions::EOVERFLOW);

    /// Name not unique on network
    pub const ENOTUNIQ: Errno = Errno::from_number(definitions::ENOTUNIQ);

    /// File descriptor in invalid state
    pub const EBADFD: Errno = Errno::from_number(definitions::EBADFD);

    /// Remote address changed
    pub const EREMCHG: Errno = Errno::from_number(definitions::EREMCHG);

    /// Cannot access a needed shared library
    pub const ELIBACC: Errno = Errno::from_number(definitions::ELIBACC);

    /// Accessing a corrupted shared library
    pub const ELIBBAD: Errno = Errno::from_number(definitions::ELIBBAD);

    /// `.lib` section in a.out corrupted
    pub const ELIBSCN: Errno = Errno::from_number(definitions::ELIBSCN);

    /// Attempting to link in too many shared libraries
    pub const ELIBMAX: Errno = Errno::from_number(definitions::ELIBMAX);

    /// Cannot execute a shared library directly
    pub const ELIBEXEC: Errno = Errno::from_number(definitions::ELIBEXEC);

    /// Illegal byte sequence
    pub const EILSEQ: Errno = Errno::from_number(definitions::EILSEQ);

    /// Interrupted system call should be restarted
    pub const ERESTART: Errno = Errno::from_number(definitions::ERESTART);

    /// Streams pipe error
    pub const ESTRPIPE: Errno = Errno::from_number(definitions::ESTRPIPE);

    /// Too many users
    pub const EUSERS: Errno = Errno::from_number(definitions::EUSERS);

    /// Socket operation on non-socket
    pub const ENOTSOCK: Errno = Errno::from_number(definitions::ENOTSOCK);

    /// Destination address required
    pub const EDESTADDRREQ: Errno = Errno::from_number(definitions::EDESTADDRREQ);

    /// Message too long
    pub const EMSGSIZE: Errno = Errno::from_number(definitions::EMSGSIZE);

    /// Protocol wrong type for socket
    pub const EPROTOTYPE: Errno = Errno::from_number(definitions::EPROTOTYPE);

    /// Protocol not available
    pub const ENOPROTOOPT: Errno = Errno::from_number(definitions::ENOPROTOOPT);

    /// Protocol not supported
    pub const EPROTONOSUPPORT: Errno = Errno::from_number(definitions::EPROTONOSUPPORT);

    /// Socket type not supported
    pub const ESOCKTNOSUPPORT: Errno = Errno::from_number(definitions::ESOCKTNOSUPPORT);

    /// Operation not supported on transport endpoint
    pub const EOPNOTSUPP: Errno = Errno::from_number(definitions::EOPNOTSUPP);

    /// Protocol family not supported
    pub const EPFNOSUPPORT: Errno = Errno::from_number(definitions::EPFNOSUPPORT);

    /// Address family not supported by protocol
    pub const EAFNOSUPPORT: Errno = Errno::from_number(definitions::EAFNOSUPPORT);

    /// Address already in use
    pub const EADDRINUSE: Errno = Errno::from_number(definitions::EADDRINUSE);

    /// Cannot assign requested address
    pub const EADDRNOTAVAIL: Errno = Errno::from_number(definitions::EADDRNOTAVAIL);

    /// Network is down
    pub const ENETDOWN: Errno = Errno::from_number(definitions::ENETDOWN);

    /// Network is unreachable
    pub const ENETUNREACH: Errno = Errno::from_number(definitions::ENETUNREACH);

    /// Network dropped connection because of reset
    pub const ENETRESET: Errno = Errno::from_number(definitions::ENETRESET);

    /// Software caused connection abort
    pub const ECONNABORTED: Errno = Errno::from_number(definitions::ECONNABORTED);

    /// Connection reset by peer
    pub const ECONNRESET: Errno = Errno::from_number(definitions::ECONNRESET);

    /// No buffer space available
    pub const ENOBUFS: Errno = Errno::from_number(definitions::ENOBUFS);

    /// Transport endpoint is already connected
    pub const EISCONN: Errno = Errno::from_number(definitions::EISCONN);

    /// Transport endpoint is not connected
    pub const ENOTCONN: Errno = Errno::from_number(definitions::ENOTCONN);

    /// Cannot send after transport endpoint shutdown
    pub const ESHUTDOWN: Errno = Errno::from_number(definitions::ESHUTDOWN);

    /// Too many references: cannot splice
    pub const ETOOMANYREFS: Errno = Errno::from_number(definitions::ETOOMANYREFS);

    /// Connection timed out
    pub const ETIMEDOUT: Errno = Errno::from_number(definitions::ETIMEDOUT);

    /// Connection refused
    pub const ECONNREFUSED: Errno = Errno::from_number(definitions::ECONNREFUSED);

    /// Host is down
    pub const EHOSTDOWN: Errno = Errno::from_number(definitions::EHOSTDOWN);

    /// No route to host
    pub const EHOSTUNREACH: Errno = Errno::from_number(definitions::EHOSTUNREACH);

    /// Operation already in progress
    pub const EALREADY: Errno = Errno::from_number(definitions::EALREADY);

    /// Operation now in progress
    pub const EINPROGRESS: Errno = Errno::from_number(definitions::EINPROGRESS);

    /// Stale file handle
    pub const ESTALE: Errno = Errno::from_number(definitions::ESTALE);

    /// Structure needs cleaning
    pub const EUCLEAN: Errno = Errno::from_number(definitions::EUCLEAN);

    /// Filesystem is corrupted
    pub const EFSCORRUPTED: Errno = Errno::from_number(definitions::EFSCORRUPTED);

    /// Not a XENIX named type file
    pub const ENOTNAM: Errno = Errno::from_number(definitions::ENOTNAM);

    /// No XENIX semaphores available
    pub const ENAVAIL: Errno = Errno::from_number(definitions::ENAVAIL);

    /// Is a named type file
    pub const EISNAM: Errno = Errno::from_number(definitions::EISNAM);

    /// Remote I/O error
    pub const EREMOTEIO: Errno = Errno::from_number(definitions::EREMOTEIO);

    /// Quota exceeded
    pub const EDQUOT: Errno = Errno::from_number(definitions::EDQUOT);

    /// No medium found
    pub const ENOMEDIUM: Errno = Errno::from_number(definitions::ENOMEDIUM);

    /// Wrong medium type
    pub const EMEDIUMTYPE: Errno = Errno::from_number(definitions::EMEDIUMTYPE);

    /// Operation canceled
    pub const ECANCELED: Errno = Errno::from_number(definitions::ECANCELED);

    /// Required key not available
    pub const ENOKEY: Errno = Errno::from_number(definitions::ENOKEY);

    /// Key has expired
    pub const EKEYEXPIRED: Errno = Errno::from_number(definitions::EKEYEXPIRED);

    /// Key has been revoked
    pub const EKEYREVOKED: Errno = Errno::from_number(definitions::EKEYREVOKED);

    /// Key was rejected by service
    pub const EKEYREJECTED: Errno = Errno::from_number(definitions::EKEYREJECTED);

    /// Owner died
    pub const EOWNERDEAD: Errno = Errno::from_number(definitions::EOWNERDEAD);

    /// State not recoverable
    pub const ENOTRECOVERABLE: Errno = Errno::from_number(definitions::ENOTRECOVERABLE);

    /// Operation not possible due to RF-kill
    pub const ERFKILL: Errno = Errno::from_number(definitions::ERFKILL);

    /// Memory page has hardware error
    pub const EHWPOISON: Errno = Errno::from_number(definitions::EHWPOISON);

    /// Wrong file type for the intended operation
    pub const EFTYPE: Errno = Errno::from_number(definitions::EFTYPE);

    /// Buffer or request is too small
    ///
    /// This is an internal Linux error that's defined outside the Linux UAPI.
    /// However, commit `18282100d7040614b553f1cad737cb689c04e2b9` deliberately
    /// exposed it to userspace by preserving TCP device memory receive errors.
    pub const ETOOSMALL: Errno = Errno::from_number(definitions::ETOOSMALL);

    /// Decode the normalized result of a system call primitive.
    ///
    /// The architecture primitives represent failures as negated error
    /// numbers in the `[-4095, -1]` interval. Every other value is a
    /// success: zero, a byte count, a descriptor, or even a high address
    /// that looks negative when read as signed.
    pub fn from_system_call(result: isize) -> Result<usize, Errno> {
        if (-4095..=-1).contains(&result) {
            Err(Errno::from_number((-result) as u16))
        } else {
            Ok(result as usize)
        }
    }

    /// The underlying error number
    pub const fn number(self) -> u16 {
        self.0.get()
    }
}

impl core::ops::Neg for Errno {
    type Output = isize;

    fn neg(self) -> Self::Output {
        -(self.number() as isize)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_system_call_decodes_the_errno_band() {
        // the negated error number in [-4095, -1] is a failure
        assert_eq!(Errno::from_system_call(-9), Err(Errno::EBADF));

        // an unnamed errno is still representable
        assert_eq!(Errno::from_system_call(-4095).unwrap_err().number(), 4095);

        // zero, a byte count, a descriptor are successes
        assert_eq!(Errno::from_system_call(0), Ok(0));
        assert_eq!(Errno::from_system_call(5), Ok(5));

        // a value just outside the error range is a success
        assert_eq!(Errno::from_system_call(-4096), Ok((-4096_isize) as usize));
    }
}
