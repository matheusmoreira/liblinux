Send bytes through a socket.

`descriptor` must be an open file descriptor that refers to a socket.

This function performs the `sendto` system call exactly once.
It does not retry after interruption, continue after a short
result, or otherwise add policy around the kernel operation.

# Data

`buffer` supplies the userspace address of the data to send.
`buffer_length` specifies how many bytes Linux is supposed
to read from the beginning of `buffer`.

Before looking up `descriptor`, the generic socket layer clamps
`buffer_length` to `MAX_RW_COUNT` and validates that the resulting
range is an allowed userspace range. This initial validation does
not fault in pages or verify that all bytes are mapped and readable.
The socket protocol may still fail when it later accesses the data.

A successful result is the number of bytes accepted from the clamped
input range. It is never greater than `MAX_RW_COUNT`, and due to the
clamping it may be smaller than `buffer_length` when `buffer_length`
is greater than `MAX_RW_COUNT`.

Whether a shorter successful result is otherwise possible
depends on the socket protocol. Byte stream protocols may
accept only part of the data. Message oriented protocols
may instead require one atomic message and fail when it
cannot be sent as such.

A zero `buffer_length` may be a meaningful value.
A message oriented protocol may transmit an empty
message and return zero.

# Destination

`address` is either null or points to `address_length` bytes
containing a socket address. The representation of the address
and its meaning are both selected by the socket's address family
and protocol.

When `address` is null, Linux does not supply a destination
address to the protocol and completely ignores `address_length`.
A connected socket can be instructed to use its existing peer
by passing a null address. A protocol that requires a destination
and has no connected peer may reject the operation instead.

When `address` is not null, Linux interprets `address_length`
as a signed 32-bit integer. A negative length or one greater
than 128 bytes, the size of `struct __kernel_sockaddr_storage`,
is rejected with [`-EINVAL`](crate::Errno::EINVAL). A zero length
is accepted without reading through `address`. For a positive accepted
length, Linux copies exactly that many bytes into kernel storage before
invoking the security hook or socket protocol.

The protocol decides whether the supplied bytes form a valid
address and whether an explicit destination is meaningful
for the socket's current state. A connected protocol may
use, ignore, or reject a supplied address.

Linux does not retain the userspace `address` pointer
after the system call returns.

# Flags

`flags` is a 32-bit bitfield.

Linux clears message bits reserved for its own internal use before
the socket protocol sees them. The exact internal mask is a kernel
implementation detail and has changed many times across releases.

When `O_NONBLOCK` is set in the socket file's flags,
Linux adds `MSG_DONTWAIT` regardless of whether that
bit was present in `flags`. An individual call may
also request non-blocking behavior by explicitly
passing `MSG_DONTWAIT`.

[`MSG_NOSIGNAL`](crate::definitions::MSG_NOSIGNAL) suppresses
`SIGPIPE` when a connection-oriented send would otherwise finish
with [`-EPIPE`](crate::Errno::EPIPE). It does not suppress the
error itself: the system call still returns `-EPIPE`.

Other flag meanings and accepted combinations are protocol specific.
A protocol may use, ignore, or reject bits that pass through
the generic socket layer.

# Ordering and interposition

Linux performs the generic part of `sendto` in this order:

 1. Clamp and validate the buffer range
 2. Look up the file `descriptor`
 3. Require the file `descriptor` to refer to a socket
 4. Process a non-null destination by copying
    and auditing it when its length is positive
 5. Clear internal flags and add non-blocking state
 6. Invoke the `security_socket_sendmsg` hook
 7. Invoke the socket protocol's send operation

Consequently, an address error occurs only after Linux has found
a valid socket. Initial rejection of an impossible buffer range
can occur before [`-EBADF`](crate::Errno::EBADF), but an unmapped
but superficially valid range might not fault until the protocol
reads it later.

A Linux security module may reject the operation before the protocol
sees it. Protocol implementations may apply further security, BPF,
routing, filtering, accounting, and transport-specific rules.

# Result and side effects

On success, `sendto` returns the non-negative number of bytes
consumed from the clamped input range.

A successful return does not mean that a peer has received the data.
Depending on the protocol, it may mean only that Linux has accepted
the data for later transmission.

For stream protocols, Linux may replace an `EPIPE` condition
with a pending socket error. `SIGPIPE` is generated only when
the final error remains `EPIPE` and `MSG_NOSIGNAL` was not
supplied.

The operation may change socket and protocol state
even when it returns an error. Security hooks and
protocol implementations may also have observable
side effects before deciding the final result.

# Errors

The generic socket layer may return:

 - [`-EFAULT`](crate::Errno::EFAULT)

   The clamped buffer range failed the preliminary userspace range
   validation, Linux could not read required bytes from `buffer`,
   or Linux could not copy a non-null destination address.

 - [`-EBADF`](crate::Errno::EBADF)

   `descriptor` is not a valid open file descriptor.

 - [`-ENOTSOCK`](crate::Errno::ENOTSOCK)

   `descriptor` is open but does not refer to a socket.

 - [`-EINVAL`](crate::Errno::EINVAL)

   A non-null destination has a negative or excessively large
   `address_length`, or the selected protocol rejects an argument.

 - [`-EPERM`](crate::Errno::EPERM) or
   [`-EACCES`](crate::Errno::EACCES)

   A security policy or protocol-specific permission check
   rejected the operation.

Common protocol-specific results include:

 - [`-EAGAIN`](crate::Errno::EAGAIN)

   The operation would block and non-blocking behavior is active.

 - [`-EINTR`](crate::Errno::EINTR)

   A blocking operation was interrupted.

 - [`-EDESTADDRREQ`](crate::Errno::EDESTADDRREQ)

   The protocol requires a destination
   but no explicit address or connected
   peer was available.

 - [`-EMSGSIZE`](crate::Errno::EMSGSIZE)

   The requested message is too large
   for an atomic protocol operation.

 - [`-ENOBUFS`](crate::Errno::ENOBUFS) or
   [`-ENOMEM`](crate::Errno::ENOMEM)

   Destination auditing, kernel allocation, or protocol resources
   required for the send were unavailable.

 - [`-EOPNOTSUPP`](crate::Errno::EOPNOTSUPP)

   The socket protocol does not support
   the requested operation or flags.

 - [`-ENOTCONN`](crate::Errno::ENOTCONN) or
   [`-EISCONN`](crate::Errno::EISCONN)

   The socket's connection state conflicts
   with the requested destination semantics.

 - [`-EPIPE`](crate::Errno::EPIPE)

   A connection-oriented socket can no longer send.
   Linux generates `SIGPIPE` unless `MSG_NOSIGNAL`
   was supplied. Linux then handles the generated
   signal according to its configured action.

Socket families, protocols, security modules, BPF programs,
routing, and network devices may produce additional errors.

# Safety

The caller must ensure every memory access Linux successfully
performs through the memory regions described by `buffer` and
`buffer_length` or by `address` and `address_length` respects
the Rust memory model and any other relevant invariant.

Linux treats both pointers as input. It may read only part of the region
described by `buffer` and `buffer_length`, may read it after other socket
work has occurred, and may encounter a fault after partially consuming the
input. Concurrent mutation must not create a Rust data race or invalidate
any assumptions made by the kernel, the protocol, or other code.

Ordinarily Linux finishes reading the data before the system call returns.
`MSG_ZEROCOPY` is an important exception: when `SO_ZEROCOPY` is enabled on
a socket and `MSG_ZEROCOPY` is supplied, Linux may retain references to the
`buffer` pages after the system call returns. The caller must keep those
pages alive and must not modify the data until receiving the corresponding
zero copy completion notification. That notification means the pages may
be reused. It does not mean transmission or delivery has completed.

A non-null positive length `address` must remain readable for the address
copy. Linux copies it before the security hook and protocol operation, so
that userspace address memory need not remain valid after return.

Neither pointer is required to be dereferenceable by Rust in order to
perform the system call. Passing inaccessible userspace addresses may
cause Linux to return [`-EFAULT`](crate::Errno::EFAULT), a valid kernel
result. Early validation errors may also prevent Linux from accessing
one or both pointers at all.

# Kernel version differences

Generic setup and error precedence have changed across Linux releases.

Linux 3.19 and below clamps long requests to `INT_MAX` and looks up
the file `descriptor` before performing the generic userspace range
validation on the data buffer.

Linux 4.0 performs that preliminary range validation before
file `descriptor` lookup while retaining the `INT_MAX` limit.

Linux 4.1 and above clamps to `MAX_RW_COUNT` and validates
the clamped range before file `descriptor` lookup.

Older implementations did not clear `MSG_INTERNAL_SENDMSG_FLAGS` bits.
Both that mask and the meaning of its bits are implementation details
and may change again.
