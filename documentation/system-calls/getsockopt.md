Read an option from a socket.

`descriptor` must be an open file descriptor
that refers to a socket.

# Socket option level

`level` selects the layer that owns `option`.
[`SOL_SOCKET`](crate::definitions::SOL_SOCKET)
selects Linux's generic socket layer. Other levels
are interpreted by the socket's protocol implementation.

The representation of the option value and the meaning
of its length are defined by the selected `level` and
`option`. The value may be an integer, a structure,
or an arbitrary sequence of bytes.

This function performs the `getsockopt` system call
exactly once. It does not retry if interrupted.

# Socket option value

`value` points to storage through which
Linux may return the option value.

`value_length` points to a signed 32-bit integer.
It is an input and output argument.

On entry, the value read through `value_length`
describes the amount of storage available in
the memory pointed to by `value`. On return,
Linux may replace it with a length defined
by the selected option.

For ordinary fixed size [`SOL_SOCKET`](crate::definitions::SOL_SOCKET)
options, Linux first reads the length through `value_length`
and rejects a negative length with [`-EINVAL`](crate::Errno::EINVAL).
It then limits the size of the output value that is copied
to user space through `value` to either the supplied length
or the size of the option itself, whichever is smallest.
Linux then writes the option's value through `value`
and the number of bytes copied through `value_length`.
A buffer larger than the option is accepted, and the
length is overwritten with the size of the option value.

Other options, including variable sized `SOL_SOCKET`
options and protocol specific options, may interpret
the length differently. Some options report a required
buffer size through `value_length` and return an error
such as [`-ERANGE`](crate::Errno::ERANGE). So Linux may
modify `value_length` even when `getsockopt` fails,
and its returned value must not be assumed to equal
the number of bytes successfully written through `value`.

# SO_ERROR

[`SO_ERROR`](crate::definitions::SO_ERROR) at the
[`SOL_SOCKET`](crate::definitions::SOL_SOCKET) level
returns a Linux integer error number through `value`.

Linux atomically reads and clears the socket's pending error.
If no pending error is present, Linux reads and clears
the socket's soft error instead. Zero means that neither
error is present.

Linux clears this error state before copying the option value
to userspace. A non-negative length smaller than 4 bytes still
consumes the complete pending error while returning only part
of its integer representation. If either output copy fails,
Linux may return [`-EFAULT`](crate::Errno::EFAULT) _after_
the relevant socket error has already been cleared.

The system call's own result is distinct from the socket error
returned through `value`. It may succeed while `SO_ERROR` reports
an earlier socket error. It may also fail after modifying `value`,
`value_length`, or the socket's error state.

# Interposition

After validating `descriptor`, Linux invokes the
`security_socket_getsockopt` security hook _before_
reading `value_length` or retrieving the option.
A security module may reject the request.

On Linux 5.3 and above, when the calling task is not using
a compatibility system call ABI, Linux runs any attached
`BPF_CGROUP_GETSOCKOPT` programs after the socket or protocol
option handler. These programs may inspect and replace the bytes
returned through `value`, reduce `value_length` within the caller's
original capacity, replace the handler's error with success, or reject
the system call with [`-EPERM`](crate::Errno::EPERM).

These programs observe side effects already performed
by the option handler. In particular, `SO_ERROR` may
already have consumed the socket's pending error before
a BPF program changes the final value, length or result
returned to userspace.

# Errors

`getsockopt` may return:

 - [`-EBADF`](crate::Errno::EBADF)

   `descriptor` is not a valid open file descriptor.

 - [`-ENOTSOCK`](crate::Errno::ENOTSOCK)

   `descriptor` is open but does not refer to a socket.

 - [`-EFAULT`](crate::Errno::EFAULT)

   Linux could not perform the userspace memory access
   required by the selected option, or a cgroup BPF
   program produced an invalid result or output length.

 - [`-EINVAL`](crate::Errno::EINVAL)

   For generic `SOL_SOCKET` options, the value read
   through `value_length` is negative. Other option
   levels may use this error for other invalid values.

 - [`-ENOPROTOOPT`](crate::Errno::ENOPROTOOPT)

   The selected socket option is not available.

 - [`-EOPNOTSUPP`](crate::Errno::EOPNOTSUPP)

   The socket's protocol does not implement
   a `getsockopt` operation for the selected
   option level.

 - [`-EPERM`](crate::Errno::EPERM)

   A security hook or `BPF_CGROUP_GETSOCKOPT`
   program rejected the request.

 - [`-ERANGE`](crate::Errno::ERANGE)

   Some variable sized options use this error
   when the supplied buffer is too small.
   Such an option may write its required
   size through `value_length` before
   returning the error.

Security modules, cgroup BPF programs,
socket protocols and individual options
may return additional errors that are
specific to them.

Linux validates `descriptor` before retrieving
the socket option, so an invalid descriptor
or a non-socket descriptor takes precedence
over errors that would arise while processing
the option.

# Safety

The caller must ensure any memory access that Linux
successfully performs through `value` or `value_length`
does not violate the Rust memory model or any other
invariants in Rust or anywhere else.

`value_length` is an input and output pointer.
Any call that reaches an option implementation
may read through it, and some options may write
through it even when the system call fails and
returns an error.

A failing system call may also have partially written
through `value` before encountering an inaccessible
address. The caller must not assume that an error
left memory untouched.

Neither pointer is actually required to be dereferenceable
by Rust for the system call to be issued. Passing inaccessible
userspace addresses to the system call may cause Linux to return
[`-EFAULT`](crate::Errno::EFAULT), a perfectly valid result.

Linux does not retain any pointer after the system call returns.
