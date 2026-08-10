Receive data from a socket. Optionally, return its source address.

`descriptor` must be an open file descriptor that refers to a socket.

This function performs the `recvfrom` system call exactly once.
It does not retry after interruption, continue after a short
result, or otherwise add policy around the kernel operation.

# Data

`buffer` supplies the userspace address of the output region.
`buffer_length` specifies the maximum number of data bytes
Linux should write there.

Before looking up the file `descriptor`, the generic socket layer
clamps `buffer_length` to `MAX_RW_COUNT` and validates that the
resulting region is an allowed userspace range. This preliminary
validation does not fault in pages or verify that every byte is
mapped and writable. The socket protocol may still fail when it
later writes the data.

Linux writes only within this clamped region of `buffer`.
Normally, a successful result is the number of data bytes
written there. Byte stream protocols may return fewer bytes
than requested even when more data could arrive later.

Message-oriented protocols preserve message boundaries.
For example, UDP and Unix datagram sockets copy only
the prefix that fits when the message is larger than
the clamped output region. They may consume the message
and discard the suffix unless [`MSG_PEEK`](crate::definitions::MSG_PEEK)
is present.

Those protocols also record an output
[`MSG_TRUNC`](crate::definitions::MSG_TRUNC)
status when truncation occurs, but `recvfrom`
has no output message flags field through which
to expose it. Passing [`MSG_TRUNC`](crate::definitions::MSG_TRUNC)
in `flags` allows supported protocols to return
the full message length. The result may be greater
than `buffer_length`, even though Linux never write
 beyond the clamped output region.

A zero `buffer_length` may be a meaningful value.
Message-oriented protocols may consume a queued
message while writing no data and returning zero.
With input [`MSG_TRUNC`](crate::definitions::MSG_TRUNC),
they may instead return the full message length.

# Source address

When `address` is null, Linux does not provide source address storage
to the socket protocol and completely ignores `address_length`.

When `address` is not null, Linux gives the protocol storage
for a source address. After the protocol reports successful
reception, Linux copies that address and its length through
`address` and `address_length`. The representation and meaning
of the address are determined by the socket's address family
and protocol. If the protocol returns an error, the generic
socket layer does not access either source address output.

Before the call, `address_length` must point to a signed
32-bit integer containing the capacity in bytes of the region
at `address`. The generic address copy accepts every non-negative
capacity. Linux copies the lesser of that capacity and the actual
address length and replaces the capacity with the actual length.
The reported length may be greater than the number of address
bytes copied. Truncation of the source address is not an error
and does not change the data result.

A zero capacity suppresses the address copy but still reports
the actual length through `address_length`. A negative capacity
causes Linux to return [`-EINVAL`](crate::Errno::EINVAL) without
copying the address or replacing the negative value.

Linux does not validate either source address output pointer before
receiving the data. If `recvfrom` fails while reading or writing
`address_length`, auditing the address, or copying it through
`address`, it returns an error instead of a length result even
though the data may already have been written to the `buffer`,
and the protocol may already have consumed the message.
On Linux 6.18 and above, the actual address length may already
have been written before a later address audit or copy fails.

Linux does not retain either source address pointer
after the system call returns.

# Flags

`flags` is a 32-bit bitfield. Linux passes it to the receive
security hook and socket protocol after adding
[`MSG_DONTWAIT`](crate::definitions::MSG_DONTWAIT)
when `O_NONBLOCK` is set in the socket file's flags.

Common receive flags include:

 - [`MSG_DONTWAIT`](crate::definitions::MSG_DONTWAIT)

   Requests non-blocking behavior for this operation
   without changing the socket file's flags.

 - [`MSG_PEEK`](crate::definitions::MSG_PEEK)

   Requests observing data without consuming it.

 - [`MSG_TRUNC`](crate::definitions::MSG_TRUNC)

   Requests the full message length as the result even when
   only a prefix fits in `buffer`. Without this flag, a result
   equal to the buffer region length does not distinguish an exact
   fit from truncation.

 - [`MSG_WAITALL`](crate::definitions::MSG_WAITALL)

   Requests waiting for the clamped amount,
   although signals, errors, record boundaries,
   and protocol rules may still produce a shorter
   result.

 - [`MSG_OOB`](crate::definitions::MSG_OOB)

   Requests urgent or out-of-band data.

 - [`MSG_ERRQUEUE`](crate::definitions::MSG_ERRQUEUE)

   Selects the socket's asynchronous error queue.
   Extended error records are returned as ancillary
   data and are unavailable through `recvfrom`.

Support and exact semantics are protocol specific.
A protocol may use, ignore, or reject flag bits
that reach it.

`recvfrom` provides no ancillary data region and no output message flags
field. Control messages and output status bits produced by the protocol
are discarded rather than returned through this system call.

# Ordering and interposition

Linux performs the generic part of `recvfrom` in this order:

 1. Clamp and preliminarily validate the data output region
 2. Look up the file `descriptor`
 3. Require the file `descriptor` to refer to a socket
 4. Add `MSG_DONTWAIT` when the socket file has `O_NONBLOCK`
 5. Invoke the `security_socket_recvmsg` hook
 6. Invoke the socket protocol's receive operation
 7. After successful reception, export a requested source address

Preliminary rejection of an impossible data range can occur before
a file descriptor is rejected with [`-EBADF`](crate::Errno::EBADF).
Source address errors occur only after Linux has found a valid socket
and the protocol has reported success.

A Linux security module may reject the operation before the protocol
sees it. Protocol implementations may apply further security, BPF,
filtering, queueing, accounting, and transport-specific rules.

# Result and side effects

On success, `recvfrom` returns the non-negative result produced by
the socket protocol. This is commonly the number of data bytes copied,
but protocol semantics such as input
[`MSG_TRUNC`](crate::definitions::MSG_TRUNC)
can make it larger than the output region.

A zero result is protocol dependent. It can describe a zero length request
or message. On a byte stream with a non-zero request, it may indicate that
the peer has shut down its sending direction.

Unless a flag such as [`MSG_PEEK`](crate::definitions::MSG_PEEK)
prevents consumption, successful reception normally removes data
from the socket's receive queue. A later source address error can
cause this system call to return an error number even in spite of
data copy, queue consumption, or other protocol side effects.

The operation may change socket and protocol state even when it returns
an error. Security hooks and protocol implementations may also have
observable side effects before deciding the final result.

# Errors

The generic socket layer may return:

 - [`-EFAULT`](crate::Errno::EFAULT)

   The clamped data region failed preliminary userspace validation,
   the protocol could not write required data through the `buffer`,
   Linux could not access `address_length`, or Linux could not copy
   a requested source address. A source address fault may occur
   after data was received.

 - [`-EBADF`](crate::Errno::EBADF)

   The file `descriptor` is not a valid open file descriptor.

 - [`-ENOTSOCK`](crate::Errno::ENOTSOCK)

   The file `descriptor` is open but does not refer to a socket.

 - [`-EINVAL`](crate::Errno::EINVAL)

   A requested source address region has a negative capacity,
   or the selected protocol rejects an argument or flag combination.

 - [`-ENOMEM`](crate::Errno::ENOMEM)

   Source address auditing or a protocol operation
   could not allocate the required kernel memory.

 - [`-EPERM`](crate::Errno::EPERM) or
   [`-EACCES`](crate::Errno::EACCES)

   A security policy or protocol-specific permission check
   rejected the operation.

Common protocol-specific results include:

 - [`-EAGAIN`](crate::Errno::EAGAIN)

   No suitable data was available while non-blocking behavior
   was active, or a receive timeout expired.

 - [`-EINTR`](crate::Errno::EINTR)

   A blocking receive was interrupted before returning
   a successful partial result.

 - [`-EOPNOTSUPP`](crate::Errno::EOPNOTSUPP)

   The socket protocol does not support
   the requested operation or flags.

 - [`-ENOTCONN`](crate::Errno::ENOTCONN)

   The socket protocol requires a connection,
   but the socket is not connected.

 - [`-ECONNREFUSED`](crate::Errno::ECONNREFUSED),
   [`-ECONNRESET`](crate::Errno::ECONNRESET), or
   [`-ETIMEDOUT`](crate::Errno::ETIMEDOUT)

   The connection or network path reported a pending failure.

 - [`-ENOBUFS`](crate::Errno::ENOBUFS) or
   [`-ENOMEM`](crate::Errno::ENOMEM)

   Kernel or protocol resources required
   for the receive were unavailable.

Socket families, protocols, security modules, BPF programs,
and network devices may produce additional errors.

# Safety

The caller must ensure every memory access Linux successfully performs
through the regions described by `buffer` and `buffer_length` or by
`address` and `address_length` respects the Rust memory model and
every other relevant invariant.

Linux may write as many as the clamped number of bytes starting
at `buffer`. It may write only part of that region. It may write
even if there is an error. In particular, a source address error
can be returned after the payload was successfully copied.
Concurrent access must not create a Rust data race
or invalidate assumptions made by the protocol
or other code.

When `address` is not null and the protocol reports success,
Linux may read and write the `i32` through `address_length`.
For a non-negative input capacity, it may write the prefix
of the source address through `address`. The address region
may be partly modified, and `address_length` may already
have been replaced, even when the final result is an error.

The pointers do not need to be dereferenceable by Rust
merely to issue the system call. Inaccessible userspace
addresses may cause Linux to return [`-EFAULT`](crate::Errno::EFAULT),
a valid kernel result. Earlier failures may prevent Linux
from accessing some or all of the pointers. When `address`
is null, `address_length` is never accessed.

`recvfrom` completes these userspace memory accesses before returning
and does not access or otherwise retain the pointers afterward.

# Kernel version differences

Generic data buffer setup and error precedence
have changed Gacross Linux releases.

Linux 3.19 and below clamps `buffer_length` to `INT_MAX`
and looks up the file `descriptor` without first performing
the preliminary data range validation used by later kernels.

Linux 4.0 performs preliminary range validation before
looking up the file `descriptor` while retaining
the `INT_MAX` limit.

Linux 4.1 and above clamps to `MAX_RW_COUNT`
and validates the clamped region before
looking up the file `descriptor`.

Linux 6.17 and below audits the source address and copies
any requested non-empty prefix before replacing the caller's
capacity with the actual address length. An address audit
or copy failure leaves the length unchanged. A failure
writing the length may still occur after address bytes
were copied.

Linux 6.18 and above writes the actual address length before
auditing the source address or copying any non-empty prefix.
A later `-ENOMEM` or `-EFAULT` can leave the length changed
even though `recvfrom` returns an error.
