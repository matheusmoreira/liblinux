//! Generic kernel definitions
//!
//! Architecture independent definitions and structures
//! that may either be re-exported or overridden
//! by the architecture specific modules.
//!
//! Every definition records the earliest kernel release
//! that is known to provide it with its current value.

// Match the Linux kernel's definitions
#![allow(non_camel_case_types)]
// Linux kernel defines AF_DECnet and PF_DECnet
#![allow(non_upper_case_globals)]

// Address families. The kernel keeps these in include/linux/socket.h,
// which is not a UAPI header and so are never exported to user space.
// Historically, libraries have simply defined these values themselves.
// They are still part of the Linux system call binary interface.
// No architecture has ever overridden them.

/// Unspecified address family
/// Linux >= 0.96a
pub const AF_UNSPEC: i32 = 0;

/// Unix domain socket
/// Local communication between processes on the same host
/// Linux >= 0.96a
pub const AF_UNIX: i32 = 1;

/// POSIX name for [`AF_UNIX`]
/// Linux >= 2.1.11
pub const AF_LOCAL: i32 = AF_UNIX;

/// Internet protocol, version 4 (IPv4)
/// Linux >= 0.96a
pub const AF_INET: i32 = 2;

/// Amateur radio AX.25
/// Linux >= 0.99.15
pub const AF_AX25: i32 = 3;

/// Novell IPX
/// Linux >= 0.99.15
pub const AF_IPX: i32 = 4;

/// AppleTalk DDP
/// Linux >= 1.1.70
pub const AF_APPLETALK: i32 = 5;

/// Amateur radio NET/ROM
/// Linux >= 1.3.0
pub const AF_NETROM: i32 = 6;

/// Multiprotocol bridge
/// Linux >= 1.3.0
pub const AF_BRIDGE: i32 = 7;

/// ATM permanent virtual circuits
/// Named `AF_AAL5` until Linux 2.1.99
/// Linux >= 2.1.99
pub const AF_ATMPVC: i32 = 8;

/// Reserved for the X.25 project
/// Linux >= 1.3.0
pub const AF_X25: i32 = 9;

/// Internet protocol, version 6 (IPv6)
/// Linux >= 1.3.21
pub const AF_INET6: i32 = 10;

/// Amateur radio X.25 PLP
/// Linux >= 2.1.9
pub const AF_ROSE: i32 = 11;

/// Reserved for the DECnet project
/// Spelled AF_DECNET until Linux 2.1.43
/// Linux >= 2.1.43
pub const AF_DECnet: i32 = 12;

/// Reserved for the 802.2 LLC project
/// Linux >= 2.1.11
pub const AF_NETBEUI: i32 = 13;

/// Security callback pseudo address family
/// Linux >= 2.1.30
pub const AF_SECURITY: i32 = 14;

/// PF_KEY key management API
/// Linux >= 2.2.11
pub const AF_KEY: i32 = 15;

/// Kernel to user space communication link
/// Linux >= 2.1.68
pub const AF_NETLINK: i32 = 16;

/// Alias of [`AF_NETLINK`] that emulates 4.4BSD
/// Linux >= 2.1.68
pub const AF_ROUTE: i32 = AF_NETLINK;

/// Device level packet interface
/// Linux >= 2.1.68
pub const AF_PACKET: i32 = 17;

/// Ash
/// Linux >= 2.1.90
pub const AF_ASH: i32 = 18;

/// Acorn Econet
/// Linux >= 2.1.96
pub const AF_ECONET: i32 = 19;

/// ATM switched virtual circuits
/// Linux >= 2.1.99
pub const AF_ATMSVC: i32 = 20;

/// Reliable datagram sockets
/// Linux >= 2.6.30
pub const AF_RDS: i32 = 21;

/// Linux SNA project
/// Linux >= 2.1.102
pub const AF_SNA: i32 = 22;

/// IrDA sockets
/// Linux >= 2.1.132
pub const AF_IRDA: i32 = 23;

/// PPP over X sockets
/// Linux >= 2.3.99-pre7
pub const AF_PPPOX: i32 = 24;

/// Wanpipe API sockets
/// Linux >= 2.4.4
pub const AF_WANPIPE: i32 = 25;

/// Linux LLC
/// Linux >= 2.4.17
pub const AF_LLC: i32 = 26;

/// Native InfiniBand address
/// Linux >= 3.11
pub const AF_IB: i32 = 27;

/// Multiprotocol label switching
/// Linux >= 4.1
pub const AF_MPLS: i32 = 28;

/// Controller area network
/// Linux >= 2.6.25
pub const AF_CAN: i32 = 29;

/// TIPC sockets
/// Linux >= 2.6.16
pub const AF_TIPC: i32 = 30;

/// Bluetooth sockets
/// Linux >= 2.4.6
pub const AF_BLUETOOTH: i32 = 31;

/// IUCV sockets
/// Linux >= 2.6.21
pub const AF_IUCV: i32 = 32;

/// RxRPC sockets
/// Linux >= 2.6.22
pub const AF_RXRPC: i32 = 33;

/// mISDN sockets
/// Linux >= 2.6.27
pub const AF_ISDN: i32 = 34;

/// Phonet sockets
/// Linux >= 2.6.28
pub const AF_PHONET: i32 = 35;

/// IEEE 802.15.4 sockets
/// Linux >= 2.6.31
pub const AF_IEEE802154: i32 = 36;

/// CAIF sockets
/// Linux >= 2.6.35
pub const AF_CAIF: i32 = 37;

/// Kernel crypto algorithm sockets
/// Linux >= 2.6.38
pub const AF_ALG: i32 = 38;

/// Near field communication sockets
/// Linux >= 3.1
pub const AF_NFC: i32 = 39;

/// Virtual machine sockets
/// Linux >= 3.9
pub const AF_VSOCK: i32 = 40;

/// Kernel connection multiplexor
/// Linux >= 4.6
pub const AF_KCM: i32 = 41;

/// Qualcomm IPC router
/// Linux >= 4.7
pub const AF_QIPCRTR: i32 = 42;

/// Shared memory communication sockets
/// Reserved for the protocol family
/// that reuses the [`AF_INET`] addresses
/// Linux >= 4.11
pub const AF_SMC: i32 = 43;

/// Express data path sockets
/// Linux >= 4.18
pub const AF_XDP: i32 = 44;

/// Management component transport protocol
/// Linux >= 5.15
pub const AF_MCTP: i32 = 45;

/// One past the highest assigned address family:
/// a moving bound that rises as families are added
/// Linux >= 5.15
pub const AF_MAX: i32 = 46;

// Protocol families. Berkeley sockets meant to address a protocol family
// with a family of addresses, so every `AF_` has a matching `PF_`.
// Linux never distinguished them.

/// Alias of [`AF_UNSPEC`]
/// Linux >= 1.1.23
pub const PF_UNSPEC: i32 = AF_UNSPEC;

/// Alias of [`AF_UNIX`]
/// Linux >= 0.96a
pub const PF_UNIX: i32 = AF_UNIX;

/// Alias of [`AF_LOCAL`]
/// Linux >= 2.1.11
pub const PF_LOCAL: i32 = AF_LOCAL;

/// Alias of [`AF_INET`]
/// Linux >= 0.96a
pub const PF_INET: i32 = AF_INET;

/// Alias of [`AF_AX25`]
/// Linux >= 0.99.15
pub const PF_AX25: i32 = AF_AX25;

/// Alias of [`AF_IPX`]
/// Linux >= 0.99.15
pub const PF_IPX: i32 = AF_IPX;

/// Alias of [`AF_APPLETALK`]
/// Linux >= 1.1.70
pub const PF_APPLETALK: i32 = AF_APPLETALK;

/// Alias of [`AF_NETROM`]
/// Linux >= 1.3.0
pub const PF_NETROM: i32 = AF_NETROM;

/// Alias of [`AF_BRIDGE`]
/// Linux >= 1.3.0
pub const PF_BRIDGE: i32 = AF_BRIDGE;

/// Alias of [`AF_ATMPVC`]
/// Linux >= 2.1.99
pub const PF_ATMPVC: i32 = AF_ATMPVC;

/// Alias of [`AF_X25`]
/// Linux >= 1.3.0
pub const PF_X25: i32 = AF_X25;

/// Alias of [`AF_INET6`]
/// Linux >= 1.3.21
pub const PF_INET6: i32 = AF_INET6;

/// Alias of [`AF_ROSE`]
/// Linux >= 2.1.22
pub const PF_ROSE: i32 = AF_ROSE;

/// Alias of [`AF_DECnet`]
/// Linux >= 2.1.43
pub const PF_DECnet: i32 = AF_DECnet;

/// Alias of [`AF_NETBEUI`]
/// Linux >= 2.1.11
pub const PF_NETBEUI: i32 = AF_NETBEUI;

/// Alias of [`AF_SECURITY`]
/// Linux >= 2.1.30
pub const PF_SECURITY: i32 = AF_SECURITY;

/// Alias of [`AF_KEY`]
/// Reserved until Linux 2.2.11
/// Linux >= 2.2.11
pub const PF_KEY: i32 = AF_KEY;

/// Alias of [`AF_NETLINK`]
/// Linux >= 2.1.68
pub const PF_NETLINK: i32 = AF_NETLINK;

/// Alias of [`AF_ROUTE`]
/// Linux >= 2.1.68
pub const PF_ROUTE: i32 = AF_ROUTE;

/// Alias of [`AF_PACKET`]
/// Linux >= 2.1.68
pub const PF_PACKET: i32 = AF_PACKET;

/// Alias of [`AF_ASH`]
/// Linux >= 2.1.90
pub const PF_ASH: i32 = AF_ASH;

/// Alias of [`AF_ECONET`]
/// Linux >= 2.1.112
pub const PF_ECONET: i32 = AF_ECONET;

/// Alias of [`AF_ATMSVC`]
/// Linux >= 2.1.99
pub const PF_ATMSVC: i32 = AF_ATMSVC;

/// Alias of [`AF_RDS`]
/// Linux >= 2.6.30
pub const PF_RDS: i32 = AF_RDS;

/// Alias of [`AF_SNA`]
/// Linux >= 2.1.102
pub const PF_SNA: i32 = AF_SNA;

/// Alias of [`AF_IRDA`]
/// Linux >= 2.1.132
pub const PF_IRDA: i32 = AF_IRDA;

/// Alias of [`AF_PPPOX`]
/// Linux >= 2.3.99-pre7
pub const PF_PPPOX: i32 = AF_PPPOX;

/// Alias of [`AF_WANPIPE`]
/// Linux >= 2.4.4
pub const PF_WANPIPE: i32 = AF_WANPIPE;

/// Alias of [`AF_LLC`]
/// Linux >= 2.4.17
pub const PF_LLC: i32 = AF_LLC;

/// Alias of [`AF_IB`]
/// Linux >= 3.11
pub const PF_IB: i32 = AF_IB;

/// Alias of [`AF_MPLS`]
/// Linux >= 4.1
pub const PF_MPLS: i32 = AF_MPLS;

/// Alias of [`AF_CAN`]
/// Linux >= 2.6.25
pub const PF_CAN: i32 = AF_CAN;

/// Alias of [`AF_TIPC`]
/// Linux >= 2.6.16
pub const PF_TIPC: i32 = AF_TIPC;

/// Alias of [`AF_BLUETOOTH`]
/// Linux >= 2.4.6
pub const PF_BLUETOOTH: i32 = AF_BLUETOOTH;

/// Alias of [`AF_IUCV`]
/// Linux >= 2.6.21
pub const PF_IUCV: i32 = AF_IUCV;

/// Alias of [`AF_RXRPC`]
/// Linux >= 2.6.22
pub const PF_RXRPC: i32 = AF_RXRPC;

/// Alias of [`AF_ISDN`]
/// Linux >= 2.6.27
pub const PF_ISDN: i32 = AF_ISDN;

/// Alias of [`AF_PHONET`]
/// Linux >= 2.6.28
pub const PF_PHONET: i32 = AF_PHONET;

/// Alias of [`AF_IEEE802154`]
/// Linux >= 2.6.31
pub const PF_IEEE802154: i32 = AF_IEEE802154;

/// Alias of [`AF_CAIF`]
/// Linux >= 2.6.35
pub const PF_CAIF: i32 = AF_CAIF;

/// Alias of [`AF_ALG`]
/// Linux >= 2.6.38
pub const PF_ALG: i32 = AF_ALG;

/// Alias of [`AF_NFC`]
/// Linux >= 3.1
pub const PF_NFC: i32 = AF_NFC;

/// Alias of [`AF_VSOCK`]
/// Linux >= 3.9
pub const PF_VSOCK: i32 = AF_VSOCK;

/// Alias of [`AF_KCM`]
/// Linux >= 4.6
pub const PF_KCM: i32 = AF_KCM;

/// Alias of [`AF_QIPCRTR`]
/// Linux >= 4.7
pub const PF_QIPCRTR: i32 = AF_QIPCRTR;

/// Alias of [`AF_SMC`]
/// Linux >= 4.11
pub const PF_SMC: i32 = AF_SMC;

/// Alias of [`AF_XDP`]
/// Linux >= 4.18
pub const PF_XDP: i32 = AF_XDP;

/// Alias of [`AF_MCTP`]
/// Linux >= 5.15
pub const PF_MCTP: i32 = AF_MCTP;

/// Alias of [`AF_MAX`]
/// Linux >= 5.15
pub const PF_MAX: i32 = AF_MAX;

// Socket types. The kernel keeps these definitions in include/linux/net.h,
// which is not a UAPI header and so are never exported to user space.
// Historically, libraries have simply defined these values themselves.
// They are still part of the Linux system call binary interface.
// The definitions are guarded by ARCH_HAS_SOCKET_TYPES.
// Only MIPS defines that and renumbers the socket types.
// The generic definitions below date back to fhe first release.
// The original numbering had SOCK_SEQPACKET = 3 and SOCK_RAW = 4.
// They were renumbered to the Berkeley assignment around version 0.98.

/// Sequenced, reliable, two-way, connection-based byte stream
/// Linux >= 0.96a
pub const SOCK_STREAM: i32 = 1;

/// Connectionless, unreliable datagrams of fixed maximum length
/// Linux >= 0.96a
pub const SOCK_DGRAM: i32 = 2;

/// Raw access to the protocol below the transport layer
/// Linux >= 0.99
pub const SOCK_RAW: i32 = 3;

/// Reliably delivered datagrams that may arrive out of order
/// Linux >= 0.99
pub const SOCK_RDM: i32 = 4;

/// Sequenced, reliable, two-way, connection-based datagrams
/// Linux >= 0.99
pub const SOCK_SEQPACKET: i32 = 5;

/// Datagram congestion control protocol
/// Retired in Linux 6.16 but the constant remains
/// On Linux >= 6.16 creating such a socket fails
/// with ESOCKTNOSUPPORT
/// Linux >= 2.6.14
pub const SOCK_DCCP: i32 = 6;

/// Obsolete Linux specific way of receiving raw packets
/// Use [`AF_PACKET`]
/// Linux >= 0.99
pub const SOCK_PACKET: i32 = 10;

/// One past the highest assigned socket type
/// Linux >= 2.4.6
pub const SOCK_MAX: i32 = SOCK_PACKET + 1;

/// The bits of the type argument that hold the socket type
/// rather than the flags that can be combined into it
/// Linux >= 2.6.27
pub const SOCK_TYPE_MASK: i32 = 0xF;

// Socket flags, combined via bitwise OR into the type argument
// of the `socket`, `socketpair` and `accept4` system calls.
// Derived from their corresponding open flags, so an architecture
// that redefines those also redefines the socket flags.
// The alpha and parisc architectures both override SOCK_NONBLOCK
// to avoid a collision with the socket type bits, and sparc, alpha
// and parisc all override O_CLOEXEC.

/// Atomically set new socket file descriptor close-on-exec
/// Linux >= 2.6.27
pub const SOCK_CLOEXEC: i32 = 0o2000000;

/// Atomically set new socket file descriptor non-blocking
/// Linux >= 2.6.27
pub const SOCK_NONBLOCK: i32 = 0o0004000;

// Error numbers. The kernel returns these negated in the result register.
// These are generic and shared by every architecture, with only one exception:
// for some reason, alpha swaps EAGAIN with EDEADLK. All other values come from
// MINIX and have never changed.

/// Operation not permitted
/// Linux >= 0.01
pub const EPERM: u16 = 1;

/// No such file or directory
/// Linux >= 0.01
pub const ENOENT: u16 = 2;

/// No such process
/// Linux >= 0.01
pub const ESRCH: u16 = 3;

/// Interrupted system call
/// Linux >= 0.01
pub const EINTR: u16 = 4;

/// Input or output error
/// Linux >= 0.01
pub const EIO: u16 = 5;

/// No such device or address
/// Linux >= 0.01
pub const ENXIO: u16 = 6;

/// Argument list too long
/// Linux >= 0.01
pub const E2BIG: u16 = 7;

/// Executable format error
/// Linux >= 0.01
pub const ENOEXEC: u16 = 8;

/// Invalid file descriptor
/// Linux >= 0.01
pub const EBADF: u16 = 9;

/// No child processes
/// Linux >= 0.01
pub const ECHILD: u16 = 10;

/// Resource temporarily unavailable
/// The operation would block
/// Linux >= 0.01
pub const EAGAIN: u16 = 11;

/// Out of memory
/// Linux >= 0.01
pub const ENOMEM: u16 = 12;

/// Permission denied
/// Linux >= 0.01
pub const EACCES: u16 = 13;

/// Invalid address
/// Linux >= 0.01
pub const EFAULT: u16 = 14;

/// Block device required
/// Linux >= 0.01
pub const ENOTBLK: u16 = 15;

/// Device or resource busy
/// Linux >= 0.01
pub const EBUSY: u16 = 16;

/// File exists
/// Linux >= 0.01
pub const EEXIST: u16 = 17;

/// Cross device link
/// Linux >= 0.01
pub const EXDEV: u16 = 18;

/// No such device
/// Linux >= 0.01
pub const ENODEV: u16 = 19;

/// Not a directory
/// Linux >= 0.01
pub const ENOTDIR: u16 = 20;

/// Is a directory
/// Linux >= 0.01
pub const EISDIR: u16 = 21;

/// Invalid argument
/// Linux >= 0.01
pub const EINVAL: u16 = 22;

/// Too many open files in the system
/// Linux >= 0.01
pub const ENFILE: u16 = 23;

/// Too many open files in the process
/// Linux >= 0.01
pub const EMFILE: u16 = 24;

/// Inappropriate ioctl for device
/// Linux >= 0.01
pub const ENOTTY: u16 = 25;

/// Text file busy
/// Linux >= 0.01
pub const ETXTBSY: u16 = 26;

/// File too large
/// Linux >= 0.01
pub const EFBIG: u16 = 27;

/// No space left on device
/// Linux >= 0.01
pub const ENOSPC: u16 = 28;

/// Illegal seek
/// Linux >= 0.01
pub const ESPIPE: u16 = 29;

/// Read only file system
/// Linux >= 0.01
pub const EROFS: u16 = 30;

/// Too many links
/// Linux >= 0.01
pub const EMLINK: u16 = 31;

/// Broken pipe
/// Linux >= 0.01
pub const EPIPE: u16 = 32;

/// Numerical argument out of domain
/// Linux >= 0.01
pub const EDOM: u16 = 33;

/// Numerical result out of range
/// Linux >= 0.01
pub const ERANGE: u16 = 34;

/// A Unix domain socket address: a family tag and a 108 byte path
#[repr(C)]
pub struct sockaddr_un {
    /// Always [`AF_UNIX`]
    pub sun_family: u16,

    /// The socket path:
    /// NUL-terminated file system path, or
    /// leading NUL for the abstract name space
    pub sun_path: [u8; 108],
}
