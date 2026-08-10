//! Shared Linux kernel definitions
//!
//! Architecture modules re-export shared definitions
//! when they are binary compatible and provide local
//! replacements when they are not.
//!
//! Every definition records the earliest kernel release
//! that is known to provide it with its current value.

// Match the Linux kernel's definitions
#![allow(non_camel_case_types)]
// Linux kernel defines AF_DECnet and PF_DECnet
#![allow(non_upper_case_globals)]

// Time definitions.

/// System-wide wall clock measured from the Unix epoch
/// and using Coordinated Universal Time (UTC).
///
/// Its frequency is adjusted by the kernel's timekeeping
/// mechanisms as well as NTP.  It can become discontinuous
/// due to leap seconds or a sufficiently privileged user
/// space program setting the clock.
///
/// Linux >= 2.5.63
pub const CLOCK_REALTIME: crate::ClockID = 0;

/// Monotonic time since system boot, excluding suspend.
///
/// This clock cannot be set and is not affected by
/// discontinuous changes to [`CLOCK_REALTIME`].
/// Linux may adjust its rate as part of kernel
/// timekeeping and NTP.
///
/// In a time namespace, Linux adds its monotonic offset
/// to the value returned to the calling task.
///
/// Linux >= 2.5.63
pub const CLOCK_MONOTONIC: crate::ClockID = 1;

/// Scheduler execution time consumed by the calling thread group.
///
/// Linux implements this as the `CPUCLOCK_SCHED` CPU clock
/// for the caller's thread group, so it measures scheduler
/// runtime accumulated by all tasks in that group rather
/// than elapsed wall clock time.
///
/// Linux reports one nanosecond resolution for this clock
/// because the true resolution of the underlying scheduler
/// clock is not exported.
///
/// Linux >= 2.5.63
pub const CLOCK_PROCESS_CPUTIME_ID: crate::ClockID = 2;

/// Scheduler execution time consumed by the calling task.
///
/// Linux implements this as the `CPUCLOCK_SCHED` CPU clock
/// for the calling task, so it measures scheduler runtime
/// rather than elapsed wall clock time.
///
/// Linux reports one nanosecond resolution for this clock
/// because the true resolution of the underlying scheduler
/// clock is not exported.
///
/// Linux >= 2.5.63
pub const CLOCK_THREAD_CPUTIME_ID: crate::ClockID = 3;

/// Raw monotonic time since system boot, excluding suspend.
///
/// This clock runs at the rate of the underlying hardware
/// clock source without NTP frequency corrections for clock
/// drift. It is unaffected by discontinuous changes to
/// [`CLOCK_REALTIME`].
///
/// In a time namespace, Linux adds its monotonic offset
/// to the value returned to the calling task.
///
/// Linux >= 2.6.28
pub const CLOCK_MONOTONIC_RAW: crate::ClockID = 4;

/// Coarse, low resolution version of [`CLOCK_REALTIME`].
///
/// Linux returns a cached time from the last timer tick
/// instead of reading the current hardware clock source.
/// This makes the clock cheaper to read but less current.
/// Its reported resolution is the kernel's low resolution
/// tick granularity.
///
/// Linux >= 2.6.32
pub const CLOCK_REALTIME_COARSE: crate::ClockID = 5;

/// Coarse, low resolution version of [`CLOCK_MONOTONIC`].
///
/// Linux returns a cached time from the last timer tick
/// instead of reading the current hardware clock source.
/// This makes the clock cheaper to read but less current.
/// Its reported resolution is the kernel's low resolution
/// tick granularity.
///
/// In a time namespace, Linux adds its monotonic offset
/// to the value returned to the calling task.
///
/// Linux >= 2.6.32
pub const CLOCK_MONOTONIC_COARSE: crate::ClockID = 6;

/// Monotonic time since system boot, including suspend.
///
/// This clock has the same timekeeping basis as [`CLOCK_MONOTONIC`]
/// but continues to account for time while the system is suspended.
///
/// In a time namespace, Linux adds its boot time offset
/// to the value returned to the calling task.
///
/// Linux >= 2.6.39
pub const CLOCK_BOOTTIME: crate::ClockID = 7;

/// Alarm clock using the [`CLOCK_REALTIME`] timebase.
///
/// Timers using this clock can wake a suspended system
/// through the alarm timer RTC. Reading the clock also
/// requires Linux to have selected a usable RTC that is
/// capable of system wake up.
///
/// Linux >= 3.0
pub const CLOCK_REALTIME_ALARM: crate::ClockID = 8;

/// Alarm clock using the [`CLOCK_BOOTTIME`] timebase.
///
/// Timers using this clock can wake a suspended system
/// through the alarm timer RTC. Reading the clock also
/// requires Linux to have selected a usable RTC that is
/// capable of system wake up.
///
/// In a time namespace, Linux adds its boot time offset
/// to the value returned to the calling task.
///
/// Linux >= 3.0
pub const CLOCK_BOOTTIME_ALARM: crate::ClockID = 9;

/// Reserved identifier formerly used by the removed SGI cycle clock.
///
/// Linux >= 2.6.12
pub const CLOCK_SGI_CYCLE: crate::ClockID = 10;

/// International Atomic Time derived from [`CLOCK_REALTIME`].
///
/// This clock follows Linux system timekeeping and frequency
/// steering but applies the configured TAI offset. Unlike UTC
/// real time, TAI does not jump due to leap second updates.
/// The TAI offset can be changed through Linux's time
/// adjustment interfaces.
///
/// Linux >= 3.10
pub const CLOCK_TAI: crate::ClockID = 11;

/// Boundary after the legacy clock identifier range.
///
/// Also the first auxiliary clock identifier on Linux >= 6.17.
///
/// Linux >= 2.6.12
pub const MAX_CLOCKS: crate::ClockID = 16;

/// First auxiliary clock identifier.
///
/// Auxiliary clocks are dynamically configured Linux timekeepers.
/// Each enabled clock can be steered independently of the core
/// timekeeper backing [`CLOCK_REALTIME`], [`CLOCK_MONOTONIC`]
/// and the other system clocks.
///
/// Linux >= 6.17
pub const CLOCK_AUX: crate::ClockID = MAX_CLOCKS;

/// Maximum number of auxiliary clock identifiers.
///
/// Linux defines space for eight auxiliary clocks,
/// but the number actually supported may be lower
/// because of architecture or vDSO constraints.
///
/// Linux >= 6.17
pub const MAX_AUX_CLOCKS: crate::ClockID = 8;

/// Last auxiliary clock identifier.
///
/// Linux >= 6.17
pub const CLOCK_AUX_LAST: crate::ClockID = CLOCK_AUX + MAX_AUX_CLOCKS - 1;

/// 64 bit seconds and nanoseconds time value.
///
/// The `__kernel_timespec` structure exists in the Linux UAPI
/// since Linux 4.18, but it has the same layout as the structures
/// used by the native time binary interfaces of older 64 bit kernels.
///
/// Linux >= 4.18
#[repr(C)]
pub struct __kernel_timespec {
    /// Whole seconds.
    pub tv_sec: i64,

    /// Nanosecond component.
    pub tv_nsec: i64,
}

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

// File access modes, passed in the flags argument
// to all of the open system calls.

/// Open for reading only
/// Linux >= 0.01
pub const O_RDONLY: i32 = 0;

/// Open for writing only
/// Linux >= 0.01
pub const O_WRONLY: i32 = 1;

/// Open for reading and writing
/// Linux >= 0.01
pub const O_RDWR: i32 = 2;

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
// These are the generic Linux UAPI assignments. Architecture modules
// override them when an architecture uses different error numbers.
// Several Linux architectures use different assignments.

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

/// Resource deadlock would occur
/// Linux >= 2.6.12
pub const EDEADLK: u16 = 35;

/// File name too long
/// Linux >= 2.6.12
pub const ENAMETOOLONG: u16 = 36;

/// No record locks available
/// Linux >= 2.6.12
pub const ENOLCK: u16 = 37;

/// Invalid system call number
/// Linux >= 2.6.12
pub const ENOSYS: u16 = 38;

/// Directory not empty
/// Linux >= 2.6.12
pub const ENOTEMPTY: u16 = 39;

/// Too many symbolic links encountered
/// Linux >= 2.6.12
pub const ELOOP: u16 = 40;

/// Operation would block
/// Alias of [`EAGAIN`]
/// Linux >= 2.6.12
pub const EWOULDBLOCK: u16 = EAGAIN;

/// No message of desired type
/// Linux >= 2.6.12
pub const ENOMSG: u16 = 42;

/// Identifier removed
/// Linux >= 2.6.12
pub const EIDRM: u16 = 43;

/// Channel number out of range
/// Linux >= 2.6.12
pub const ECHRNG: u16 = 44;

/// Level 2 not synchronized
/// Linux >= 2.6.12
pub const EL2NSYNC: u16 = 45;

/// Level 3 halted
/// Linux >= 2.6.12
pub const EL3HLT: u16 = 46;

/// Level 3 reset
/// Linux >= 2.6.12
pub const EL3RST: u16 = 47;

/// Link number out of range
/// Linux >= 2.6.12
pub const ELNRNG: u16 = 48;

/// Protocol driver not attached
/// Linux >= 2.6.12
pub const EUNATCH: u16 = 49;

/// No CSI structure available
/// Linux >= 2.6.12
pub const ENOCSI: u16 = 50;

/// Level 2 halted
/// Linux >= 2.6.12
pub const EL2HLT: u16 = 51;

/// Invalid exchange
/// Linux >= 2.6.12
pub const EBADE: u16 = 52;

/// Invalid request descriptor
/// Linux >= 2.6.12
pub const EBADR: u16 = 53;

/// Exchange full
/// Linux >= 2.6.12
pub const EXFULL: u16 = 54;

/// No anode
/// Linux >= 2.6.12
pub const ENOANO: u16 = 55;

/// Invalid request code
/// Linux >= 2.6.12
pub const EBADRQC: u16 = 56;

/// Invalid slot
/// Linux >= 2.6.12
pub const EBADSLT: u16 = 57;

/// Resource deadlock would occur
/// Alias of [`EDEADLK`]
/// Linux >= 2.6.12
pub const EDEADLOCK: u16 = EDEADLK;

/// Invalid font file format
/// Linux >= 2.6.12
pub const EBFONT: u16 = 59;

/// Device not a stream
/// Linux >= 2.6.12
pub const ENOSTR: u16 = 60;

/// No data available
/// Linux >= 2.6.12
pub const ENODATA: u16 = 61;

/// Timer expired
/// Linux >= 2.6.12
pub const ETIME: u16 = 62;

/// Out of streams resources
/// Linux >= 2.6.12
pub const ENOSR: u16 = 63;

/// Machine is not on the network
/// Linux >= 2.6.12
pub const ENONET: u16 = 64;

/// Package not installed
/// Linux >= 2.6.12
pub const ENOPKG: u16 = 65;

/// Object is remote
/// Linux >= 2.6.12
pub const EREMOTE: u16 = 66;

/// Link has been severed
/// Linux >= 2.6.12
pub const ENOLINK: u16 = 67;

/// Advertise error
/// Linux >= 2.6.12
pub const EADV: u16 = 68;

/// Srmount error
/// Linux >= 2.6.12
pub const ESRMNT: u16 = 69;

/// Communication error on send
/// Linux >= 2.6.12
pub const ECOMM: u16 = 70;

/// Protocol error
/// Linux >= 2.6.12
pub const EPROTO: u16 = 71;

/// Multihop attempted
/// Linux >= 2.6.12
pub const EMULTIHOP: u16 = 72;

/// RFS specific error
/// Linux >= 2.6.12
pub const EDOTDOT: u16 = 73;

/// Not a data message
/// Linux >= 2.6.12
pub const EBADMSG: u16 = 74;

/// Invalid CRC detected
/// Alias of [`EBADMSG`]
/// Linux >= 7.0-rc1
pub const EFSBADCRC: u16 = EBADMSG;

/// Value too large for defined data type
/// Linux >= 2.6.12
pub const EOVERFLOW: u16 = 75;

/// Name not unique on network
/// Linux >= 2.6.12
pub const ENOTUNIQ: u16 = 76;

/// File descriptor in invalid state
/// Linux >= 2.6.12
pub const EBADFD: u16 = 77;

/// Remote address changed
/// Linux >= 2.6.12
pub const EREMCHG: u16 = 78;

/// Cannot access a needed shared library
/// Linux >= 2.6.12
pub const ELIBACC: u16 = 79;

/// Accessing a corrupted shared library
/// Linux >= 2.6.12
pub const ELIBBAD: u16 = 80;

/// `.lib` section in a.out corrupted
/// Linux >= 2.6.12
pub const ELIBSCN: u16 = 81;

/// Attempting to link in too many shared libraries
/// Linux >= 2.6.12
pub const ELIBMAX: u16 = 82;

/// Cannot execute a shared library directly
/// Linux >= 2.6.12
pub const ELIBEXEC: u16 = 83;

/// Illegal byte sequence
/// Linux >= 2.6.12
pub const EILSEQ: u16 = 84;

/// Interrupted system call should be restarted
/// Linux >= 2.6.12
pub const ERESTART: u16 = 85;

/// Streams pipe error
/// Linux >= 2.6.12
pub const ESTRPIPE: u16 = 86;

/// Too many users
/// Linux >= 2.6.12
pub const EUSERS: u16 = 87;

/// Socket operation on non-socket
/// Linux >= 2.6.12
pub const ENOTSOCK: u16 = 88;

/// Destination address required
/// Linux >= 2.6.12
pub const EDESTADDRREQ: u16 = 89;

/// Message too long
/// Linux >= 2.6.12
pub const EMSGSIZE: u16 = 90;

/// Protocol wrong type for socket
/// Linux >= 2.6.12
pub const EPROTOTYPE: u16 = 91;

/// Protocol not available
/// Linux >= 2.6.12
pub const ENOPROTOOPT: u16 = 92;

/// Protocol not supported
/// Linux >= 2.6.12
pub const EPROTONOSUPPORT: u16 = 93;

/// Socket type not supported
/// Linux >= 2.6.12
pub const ESOCKTNOSUPPORT: u16 = 94;

/// Operation not supported on transport endpoint
/// Linux >= 2.6.12
pub const EOPNOTSUPP: u16 = 95;

/// Protocol family not supported
/// Linux >= 2.6.12
pub const EPFNOSUPPORT: u16 = 96;

/// Address family not supported by protocol
/// Linux >= 2.6.12
pub const EAFNOSUPPORT: u16 = 97;

/// Address already in use
/// Linux >= 2.6.12
pub const EADDRINUSE: u16 = 98;

/// Cannot assign requested address
/// Linux >= 2.6.12
pub const EADDRNOTAVAIL: u16 = 99;

/// Network is down
/// Linux >= 2.6.12
pub const ENETDOWN: u16 = 100;

/// Network is unreachable
/// Linux >= 2.6.12
pub const ENETUNREACH: u16 = 101;

/// Network dropped connection because of reset
/// Linux >= 2.6.12
pub const ENETRESET: u16 = 102;

/// Software caused connection abort
/// Linux >= 2.6.12
pub const ECONNABORTED: u16 = 103;

/// Connection reset by peer
/// Linux >= 2.6.12
pub const ECONNRESET: u16 = 104;

/// No buffer space available
/// Linux >= 2.6.12
pub const ENOBUFS: u16 = 105;

/// Transport endpoint is already connected
/// Linux >= 2.6.12
pub const EISCONN: u16 = 106;

/// Transport endpoint is not connected
/// Linux >= 2.6.12
pub const ENOTCONN: u16 = 107;

/// Cannot send after transport endpoint shutdown
/// Linux >= 2.6.12
pub const ESHUTDOWN: u16 = 108;

/// Too many references: cannot splice
/// Linux >= 2.6.12
pub const ETOOMANYREFS: u16 = 109;

/// Connection timed out
/// Linux >= 2.6.12
pub const ETIMEDOUT: u16 = 110;

/// Connection refused
/// Linux >= 2.6.12
pub const ECONNREFUSED: u16 = 111;

/// Host is down
/// Linux >= 2.6.12
pub const EHOSTDOWN: u16 = 112;

/// No route to host
/// Linux >= 2.6.12
pub const EHOSTUNREACH: u16 = 113;

/// Operation already in progress
/// Linux >= 2.6.12
pub const EALREADY: u16 = 114;

/// Operation now in progress
/// Linux >= 2.6.12
pub const EINPROGRESS: u16 = 115;

/// Stale file handle
/// Linux >= 2.6.12
pub const ESTALE: u16 = 116;

/// Structure needs cleaning
/// Linux >= 2.6.12
pub const EUCLEAN: u16 = 117;

/// File system is corrupted
/// Alias of [`EUCLEAN`]
/// Linux >= 7.0-rc1
pub const EFSCORRUPTED: u16 = EUCLEAN;

/// Not a XENIX named type file
/// Linux >= 2.6.12
pub const ENOTNAM: u16 = 118;

/// No XENIX semaphores available
/// Linux >= 2.6.12
pub const ENAVAIL: u16 = 119;

/// Is a named type file
/// Linux >= 2.6.12
pub const EISNAM: u16 = 120;

/// Remote I/O error
/// Linux >= 2.6.12
pub const EREMOTEIO: u16 = 121;

/// Quota exceeded
/// Linux >= 2.6.12
pub const EDQUOT: u16 = 122;

/// No medium found
/// Linux >= 2.6.12
pub const ENOMEDIUM: u16 = 123;

/// Wrong medium type
/// Linux >= 2.6.12
pub const EMEDIUMTYPE: u16 = 124;

/// Operation canceled
/// Linux >= 2.6.12
pub const ECANCELED: u16 = 125;

/// Required key not available
/// Linux >= 2.6.12
pub const ENOKEY: u16 = 126;

/// Key has expired
/// Linux >= 2.6.12
pub const EKEYEXPIRED: u16 = 127;

/// Key has been revoked
/// Linux >= 2.6.12
pub const EKEYREVOKED: u16 = 128;

/// Key was rejected by service
/// Linux >= 2.6.12
pub const EKEYREJECTED: u16 = 129;

/// Owner died
/// Linux >= 2.6.12
pub const EOWNERDEAD: u16 = 130;

/// State not recoverable
/// Linux >= 2.6.12
pub const ENOTRECOVERABLE: u16 = 131;

/// Operation not possible due to RF-kill
/// Linux >= 2.6.31
pub const ERFKILL: u16 = 132;

/// Memory page has hardware error
/// Linux >= 2.6.39
pub const EHWPOISON: u16 = 133;

/// Wrong file type for the intended operation
/// Linux >= 7.2-rc1
pub const EFTYPE: u16 = 134;

// Socket address structures.

/// Size of [`sockaddr_un::sun_path`] in bytes.
/// TODO: pin down Linux version
pub const UNIX_PATH_MAX: usize = 108;

/// A Unix domain socket address:
/// family tag and 108 byte path.
#[repr(C)]
pub struct sockaddr_un {
    /// Always [`AF_UNIX`].
    pub sun_family: u16,

    /// The socket name.
    ///
    /// A file system address begins with a non-NUL byte
    /// and contains the file system path. Linux accepts
    /// the pathname with or without a terminating NUL byte
    /// and appends one internally when absent.
    ///
    /// A Linux abstract address *begins* with a NUL byte.
    /// Every subsequent byte included in the socket address
    /// length is part of the socket's abstract name.
    /// NUL bytes have no special meaning within it,
    /// the abstract name could be arbitrary binary data.
    ///
    /// Although this variable has `UNIX_PATH_MAX` bytes,
    /// Linux accepts smaller path buffers, and passing
    /// zero length `sun_path` to bind makes it autobind.
    /// Only bytes covered by the socket address length
    /// are significant, and those bytes could encompass
    /// just [`sockaddr_un::sun_family`].
    pub sun_path: [u8; UNIX_PATH_MAX],
}
