Receive data from a socket. Optionally, return a related socket address
such as the source address.

`descriptor` must be an open file descriptor that refers to a socket.

If an interruption is returned to userspace, this function does not retry it
or continue after a short result. Linux may transparently restart a blocked
system call according to its signal restart rules.

# Data

`buffer` supplies the userspace address of the output region.
`buffer_length` specifies the maximum number of data bytes
Linux should write.

Before looking up the file `descriptor`, the generic socket layer clamps
`buffer_length` to `MAX_RW_COUNT` and validates that the clamped region
is an allowed userspace range. This check does not fault in the pages
or verify that every byte is mapped and writable. The socket protocol
may still fail when it later accesses the region.

Linux then asks the socket protocol to receive data into the clamped region.
A successful result is the non-negative value returned by that protocol.
It is commonly the number of bytes written, but byte stream protocols
may return a shorter result.

Message-oriented protocols commonly preserve message boundaries.
For example, UDP with GRO disabled and Unix datagram sockets
both copy only the prefix that fits when a message is larger
than the output region. The message may be consumed and the
remaining bytes discarded unless the protocol honors
[`MSG_PEEK`](crate::definitions::MSG_PEEK).

These protocols may also record output [`MSG_TRUNC`](crate::definitions::MSG_TRUNC),
but `recvfrom` has no output message flags field through which to return it.

Input [`MSG_TRUNC`](crate::definitions::MSG_TRUNC) has protocol-specific
semantics. Supported message protocols may return the full message length,
so the result can exceed `buffer_length` even though Linux never writes
beyond the clamped region. TCP data reception may instead consume and
count data without copying it. [`MSG_PEEK`](crate::definitions::MSG_PEEK)
can prevent consumption when the protocol honors that combination.
[`MSG_WAITALL`](crate::definitions::MSG_WAITALL) changes the TCP
wait target but does not enable copying. Successful results do not
always specify how many bytes in `buffer` were initialized.

A zero `buffer_length` is protocol specific. Message-oriented protocols may
consume a queued message while writing no data and return zero, or with input
[`MSG_TRUNC`](crate::definitions::MSG_TRUNC), return the full message length.
On a byte stream with an empty receive queue and an open receiving direction,
a zero-length request does not force immediate success: a blocking call may
wait and a non-blocking call may return [`-EAGAIN`](crate::Errno::EAGAIN).

A zero result is also protocol specific. On a byte stream with a non-zero
request, it may indicate that the peer shut down its sending direction.

# Address

When `address` is null, Linux gives the protocol no address output storage
and completely ignores `address_length`.

When `address` is not null, Linux gives the protocol storage for a returned
address. Its representation and meaning are protocol specific. It usually
identifies the sender. An IPv4 receive from the error queue, selected by
[`MSG_ERRQUEUE`](crate::definitions::MSG_ERRQUEUE), may instead return the
original destination address of the datagram associated with the queued error.

Linux exports the address only after the protocol reports successful reception.
`address_length` must then identify readable and writable storage for a signed
32-bit capacity. For a non-negative capacity, Linux copies at most that many
address bytes and replaces the capacity with the actual address length.
The reported length may be greater than the number of bytes copied.
Truncation does not cause the system call to return an error.

A zero capacity suppresses the address copy but still reports the actual length.
A negative capacity returns [`-EINVAL`](crate::Errno::EINVAL) without copying
the address or replacing the length value.

Linux does not validate the address output pointers before receiving data.
Failure while reading or writing `address_length`, auditing the address,
or copying through `address` can replace a successful protocol result
with an error after data was written or consumed. Address output memory
may also have been modified before the error.

Linux does not retain either address output pointer
after the system call returns.

# Flags

`flags` is a 32-bit bitfield. Linux adds
[`MSG_DONTWAIT`](crate::definitions::MSG_DONTWAIT)
when `O_NONBLOCK` is set in the socket
file's flags, then passes the flags to
the receive security hook and socket
protocol.

Common receive flags include:

 - [`MSG_DONTWAIT`](crate::definitions::MSG_DONTWAIT)

   Requests non-blocking behavior for this operation
   without changing the socket file's flags.

 - [`MSG_PEEK`](crate::definitions::MSG_PEEK)

   Requests observation without consumption. Exact semantics are specific
   to the protocol and flag combination. IPv4 UDP error queue reception
   may still dequeue an entry.

 - [`MSG_TRUNC`](crate::definitions::MSG_TRUNC)

   Requests protocol-specific truncation behavior.
   Semantics are described above.

 - [`MSG_WAITALL`](crate::definitions::MSG_WAITALL)

   Requests waiting for the full receive target. Signals, errors,
   record boundaries, and protocol rules may produce short results.

 - [`MSG_OOB`](crate::definitions::MSG_OOB)

   Requests urgent or out-of-band data.

 - [`MSG_ERRQUEUE`](crate::definitions::MSG_ERRQUEUE)

   Selects the socket's asynchronous error queue.

Support and exact semantics are specific to each protocol.
A protocol may use, ignore, or reject bits that reach it.

`recvfrom` provides no ancillary data region and no output message flags field.
Output status such as [`MSG_TRUNC`](crate::definitions::MSG_TRUNC) and control
records such as extended errors selected by [`MSG_ERRQUEUE`](crate::definitions::MSG_ERRQUEUE)
cannot be returned through this interface. Protocol features that require control
records may fail or return a shorter result. TCP device memory reception can report
internal [`-ETOOSMALL`](crate::Errno::ETOOSMALL) when required control records have
nowhere to go. UDP GRO requires `recvmsg` to recover the segment size control message
and individual datagram boundaries.

# Ordering and interposition

Linux performs the generic part of `recvfrom` in this order:

 1. Clamp and preliminarily validate the data output region
 2. Look up the file `descriptor`
 3. Require the file `descriptor` to refer to a socket
 4. Add `MSG_DONTWAIT` when the socket file has `O_NONBLOCK`
 5. Invoke the `security_socket_recvmsg` hook
 6. Invoke the socket protocol's receive operation
 7. After successful reception, write the `address` if requested

Preliminary rejection of an impossible data range can occur before
[`-EBADF`](crate::Errno::EBADF). Address output errors occur only
after Linux has found a valid socket and the protocol has reported
success.

A Linux security module may reject the operation before the protocol sees it.
Protocol implementations may apply further security, BPF, filtering, queueing,
accounting, and transport-specific rules. They may also change socket or
protocol state before the final result is known. An error does not imply
that no data was written, no message was consumed, or no state was changed.

# Errors

The generic socket layer may return:

 - [`-EFAULT`](crate::Errno::EFAULT)

   The clamped data region failed preliminary userspace validation,
   the protocol could not perform a required data write, Linux could
   not access `address_length`, or Linux could not copy the address.

 - [`-EBADF`](crate::Errno::EBADF)

   `descriptor` is not a valid open file descriptor.

 - [`-ENOTSOCK`](crate::Errno::ENOTSOCK)

   `descriptor` is open but does not refer to a socket.

 - [`-EINVAL`](crate::Errno::EINVAL)

   The address capacity is negative, or the selected protocol
   rejected an argument or flag combination.

 - [`-ENOMEM`](crate::Errno::ENOMEM)

   Address auditing or a protocol operation could not allocate
   required kernel memory.

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

   The socket protocol does not support the requested operation or flags.

 - [`-ENOTCONN`](crate::Errno::ENOTCONN)

   The socket protocol requires a connection, but the socket is not connected.

 - [`-ECONNREFUSED`](crate::Errno::ECONNREFUSED),
   [`-ECONNRESET`](crate::Errno::ECONNRESET), or
   [`-ETIMEDOUT`](crate::Errno::ETIMEDOUT)

   The connection or network path reported a pending failure.

 - [`-ENOBUFS`](crate::Errno::ENOBUFS) or [`-ENOMEM`](crate::Errno::ENOMEM)

   Kernel or protocol resources required for the receive were unavailable.

Socket families, protocols, security modules, BPF programs,
and network devices may produce additional errors.

# Safety

The caller must ensure every userspace memory access Linux successfully performs
through `buffer`, `address`, or `address_length` respects allocation lifetimes,
mutability, Rust aliasing and concurrency rules, and every other invariant.
This obligation applies even when `recvfrom` returns an error.

The selected protocol may attempt to write any byte within the clamped
`buffer` region. The protocol and flags determine which bytes become
initialized. The caller must derive these initialized bytes from those
semantics or initialize the destination before the call.

When `address` is non-null and protocol reception succeeds, Linux may read
and write through `address_length`. If the read succeeds, the storage must
contain an initialized `i32`. For a non-negative capacity, Linux may write
up to that many bytes at `address`. When `address` is null, Linux does not
access `address_length`.

The pointers need not be dereferenceable by Rust merely to issue
the system call. Inaccessible userspace addresses may instead cause
Linux to return [`-EFAULT`](crate::Errno::EFAULT).

Linux completes these memory accesses before returning
and does not retain the pointers.

# Kernel version differences

The following data-buffer transitions describe upstream Linux releases:

 - Before Linux 2.6.37, the generic path passes the full native `buffer_length`
   to the protocol without a smaller generic clamp and looks up `descriptor`
   before any preliminary data range validation.

 - Linux 2.6.37 through 3.19 clamps the request to `INT_MAX` and still performs
   `descriptor` lookup before the later data access checks.

 - Linux 4.0 performs preliminary range validation before `descriptor` lookup
   but retains the `INT_MAX` limit.

 - Linux 4.1 and above clamps to `MAX_RW_COUNT` instead of `INT_MAX`
   and validates the clamped region before `descriptor` lookup.

Stable and vendor kernels can backport these transitions.
The `INT_MAX` clamp appears in stable Linux 2.6.36.2 and 2.6.32.26.
Preliminary validation appears in stable Linux 3.19.3.

Before upstream Linux 3.13, `recvfrom` supplied the protocol
with internal address storage even when `address` was null
and initialized the address length to that storage's size.
It still discarded the address and ignored `address_length`
when the userspace `address` was null. With a non-null
userspace address, a deficient protocol handler could
leave the inherited length too large and expose
uninitialized internal bytes.

Linux 3.13 and above supplies no protocol address storage for a null `address`,
and for a non-null `address` it initializes the length to zero and requires the
protocol to report the actual length.

Before Linux 6.18, the generic address helper copies any requested
non-empty address prefix before replacing the caller's capacity with
the actual address length. Linux 2.6.17 through 6.17 additionally audits
the complete returned address before copying the requested prefix. An audit
or copy failure leaves the length unchanged, while writing the length can still
fail after address bytes were copied.

Linux 6.18 and above writes the actual address length before auditing the
returned address or copying any non-empty requested prefix. A later `-ENOMEM`
or `-EFAULT` can leave the length changed even though `recvfrom` returns an
error.
