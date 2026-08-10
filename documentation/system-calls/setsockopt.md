Set an option on a socket.

`descriptor` must be an open file descriptor
that refers to a socket.

# Socket option level

`level` selects the layer that owns `option`.
[`SOL_SOCKET`](crate::definitions::SOL_SOCKET)
normally selects Linux's generic socket layer
for `setsockopt`, but some socket implementations
provide custom handling for options at this level.
Other levels are interpreted by the socket's protocol
implementation.

The representation of the option value and the meaning
of its length are defined by the selected `level` and
`option`. The value may be an integer, a structure,
an arbitrary sequence of bytes, or no bytes at all.

This function performs the `setsockopt` system call
exactly once. It does not retry if interrupted.

# Socket option value

`value` points to memory containing the option value
that Linux may read.

`value_length` is a signed 32-bit integer passed by value.
It describes the size of the option value stored at `value`.

Linux rejects a negative `value_length` with
[`-EINVAL`](crate::Errno::EINVAL) _before_
invoking a security hook, cgroup BPF program,
or socket option handler.

There is no minimum length or required representation
for the memory addressed by `value`. The selected option
decides which bytes are meaningful and how many bytes
must be available.

Most scalar [`SOL_SOCKET`](crate::definitions::SOL_SOCKET)
options require at least four bytes in order to read
a 32-bit signed integer value at the address of `value`.
Other options use structures, exact lengths, byte strings
of variable size, or special length rules. For example,
`SO_BINDTODEVICE` is handled specially by the generic
socket layer instead of using its ordinary integer path.

A null or otherwise inaccessible `value` does not by itself
make the system call invalid. It matters only if Linux actually
tries to read through the pointer. An option that does not
consume any bytes may accept a zero length without accessing
`value`.

# Interposition

After validating `descriptor` and rejecting a negative `value_length`,
Linux invokes the `security_socket_setsockopt` security hook using
the requested `level` and `option`. A security module may reject
the request.

On Linux 5.3 and above, when the calling task is not using
a compatibility system call ABI, Linux runs any attached
`BPF_CGROUP_SETSOCKOPT` programs before the socket option
handler. These programs may inspect the supplied option
value, reject the request, change `level`, `option`,
`value_length` or the bytes delivered to the option
handler, or report success while bypassing the underlying
option handler entirely.

The option handler may receive arguments different from those
originally supplied by userspace. The security hook runs before
this BPF interposition and therefore observes the original `level`
and `option`.

# Errors

`setsockopt` may return:

 - [`-EBADF`](crate::Errno::EBADF)

   `descriptor` is not a valid open file descriptor.

 - [`-ENOTSOCK`](crate::Errno::ENOTSOCK)

   `descriptor` is open but does not refer to a socket.

 - [`-EINVAL`](crate::Errno::EINVAL)

   `value_length` is negative, or the selected option
   rejects its supplied length or value.

 - [`-EFAULT`](crate::Errno::EFAULT)

   Linux could not read required bytes through `value`,
   or a cgroup BPF program produced an invalid option
   length.

 - [`-ENOMEM`](crate::Errno::ENOMEM)

   A cgroup BPF program or the selected option required
   kernel memory that could not be allocated.

 - [`-ENOPROTOOPT`](crate::Errno::ENOPROTOOPT)

   The selected socket option is not available.

 - [`-EOPNOTSUPP`](crate::Errno::EOPNOTSUPP)

   The socket's protocol does not implement
   a `setsockopt` operation for the selected
   option level.

 - [`-EPERM`](crate::Errno::EPERM)

   A security hook, `BPF_CGROUP_SETSOCKOPT` program,
   or the selected option rejected the operation
   for permission reasons.

Security modules, cgroup BPF programs,
socket protocols and individual options
may return additional errors that are
specific to them.

`descriptor` validation and rejection of a negative `value_length`
both precede the security hook, but their order relative to each
other depends on the Linux version. The security hook runs before
cgroup BPF, and cgroup BPF runs before the selected option handler.

# Safety

The caller must ensure any memory access that Linux
successfully performs through `value` does not violate
the Rust memory model or any other invariants in Rust
or anywhere else.

Normally, the selected option defines how much of `value` Linux reads.
An attached `BPF_CGROUP_SETSOCKOPT` program can change that access
pattern: before running such a program, Linux may copy substantially
more of the advertised region than the selected option would ordinarily
read. The caller must account for those possible accesses rather than
only the option handler's usual read size.

`value` does not need to be dereferenceable by Rust for the system call
to be performed. Passing an inaccessible userspace address may cause
Linux to return [`-EFAULT`](crate::Errno::EFAULT), a perfectly valid
result. It may also succeed when the selected path never actually
accesses that address.

`value_length` is passed by value, not through a pointer.
Linux does not return an option value or length through
either argument, nor does it retain `value` after the
system call returns.

# Kernel version differences

Linux up to version 6.6 checks `value_length` first.
Linux 6.7 and above validate `descriptor` first.
