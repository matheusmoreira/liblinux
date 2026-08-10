Shut down one or both communication directions of a socket.

`descriptor` must be an open file descriptor that refers to a socket.

This function performs the `shutdown` system call exactly once.
It does not close `descriptor`, retry after an error, or emulate
behavior that's not supported by the socket protocol.

# Direction

`how` is a signed 32-bit selector, not a bitfield.

 - [`SHUT_RD`](crate::definitions::SHUT_RD)

   Request shutdown of the receiving direction.

 - [`SHUT_WR`](crate::definitions::SHUT_WR)

   Request shutdown of the sending direction.

 - [`SHUT_RDWR`](crate::definitions::SHUT_RDWR)

   Request shutdown of both directions.

These constants have the values zero, one, and two.
They must not be combined with bitwise operators:
`SHUT_RD | SHUT_WR` equals `SHUT_WR`, not `SHUT_RDWR`.

The generic socket layer does not validate or reinterpret `how`.
After the security hook permits the operation, Linux passes the
original value to the socket implementation. The implementation
decides which selectors it accepts and what they mean for the
socket's family, protocol, type, and current state.

# Protocol behavior

`shutdown` changes the socket object rather than merely the selected file `descriptor`.
Duplicated, inherited, or transferred file descriptors that refer to the same socket
all observe the same resulting state. `shutdown` leaves the `descriptor` open, and
`close` remains a separate operation.

Connection-oriented protocols commonly use shutdown of only the sending direction
to create a half-closed connection. On an established TCP socket, `SHUT_WR` causes
queued data to precede the FIN while the socket's receiving direction remains available.
If the connection completes normally, the peer observes end of stream only after receiving
that data.

`SHUT_RD` is not necessarily the peer-visible mirror of `SHUT_WR`.
Treatment of queued input, later packets, peer notification,
blocked operations, and readiness events is protocol specific.
`SHUT_RDWR` requests both directions through one protocol
operation, and its exact effects need not be identical
to two separate calls in every state or race.

Listening, connecting, unconnected, datagram, raw, and already shut down sockets
can all have protocol-specific behavior. Repeated requests may succeed, fail, or
have no additional effect.

# Ordering and interposition

Linux performs the generic part of `shutdown` in this order:

 1. Look up the file `descriptor`
 2. Require the file to refer to a socket
 3. Invoke the `security_socket_shutdown` hook with `how`
 4. Invoke the socket implementation's shutdown operation with `how`

Descriptor errors precede security policy and protocol validation of `how`.
A Linux security module may reject the request before the socket implementation
sees it.

The system call receives no userspace pointers and does not
directly read or write any userspace memory.

# Result and side effects

On success, Linux returns zero, and this function returns `Ok(())`.

Success does not close the descriptor or release the socket.
The socket implementation may change local state, notify a peer,
transmit protocol messages, discard or retain queued data, and
wake tasks waiting for I/O or readiness.

An error does not imply that no state changed. The Internet socket
implementation, for example, records shutdown state and wakes waiters
while returning [`-ENOTCONN`](crate::Errno::ENOTCONN) for a socket in
its closed state.

# Errors

The generic socket layer may return:

 - [`-EBADF`](crate::Errno::EBADF)

   `descriptor` is not a valid open file descriptor.

 - [`-ENOTSOCK`](crate::Errno::ENOTSOCK)

   `descriptor` is open but does not refer to a socket.

 - [`-EPERM`](crate::Errno::EPERM) or
   [`-EACCES`](crate::Errno::EACCES)

   A security policy rejected the operation.

Common socket implementation-specific results include:

 - [`-EINVAL`](crate::Errno::EINVAL)

   `how` is not an accepted selector,
   or the socket implementation rejects
   it in the socket's current state.

 - [`-ENOTCONN`](crate::Errno::ENOTCONN)

   The selected operation requires a connected socket,
   but the socket is not connected.

 - [`-EOPNOTSUPP`](crate::Errno::EOPNOTSUPP)

   The socket implementation does not support shutdown.

Socket families, protocols, and security modules
may return additional errors.

# Kernel version differences

Before Linux 3.7, the Unix socket implementation did not reject
selectors outside the range from `SHUT_RD` through `SHUT_RDWR`.
It folded the supplied value into its internal shutdown mask,
so an invalid selector could be accepted and either do nothing
or shut down an unintended direction.

Linux 3.7 and above rejects such values with
[`-EINVAL`](crate::Errno::EINVAL).

Stable and vendor kernels may have backported this change.
