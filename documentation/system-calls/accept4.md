Accept a pending connection on a listening socket
and return a new file descriptor for it.

If `address` is not null, it points to a userspace output
region and `address_length` must point to an `i32` containing
that region's capacity. Linux writes the peer address through
`address` and replaces the value through `address_length`
with the peer address's size. The peer address is truncated
if there is not enough capacity.
If the value pointed to by `address_length`
is zero, Linux discards the peer address
but writes its size to `address_length`.

If `address` is null, then `address_length`
is completely ignored and the peer address
is discarded.

`flags` may contain `SOCK_CLOEXEC`, `SOCK_NONBLOCK`,
or both. These flags apply to the returned file descriptor.
They do not control whether waiting for a connection blocks,
that is determined by the listening socket.

# Safety

Rust safety and semantics do not matter to the kernel.
However, the kernel semantics matter a lot to Rust.
The caller must ensure any memory access that Linux
successfully performs through `address` or
`address_length` does not violate the Rust memory model
or any other invariants in Rust or anywhere else.

When `address` is not null, Linux may read and write
the `i32` pointed to by `address_length` and may write
address bytes through `address`. If `address_length`
is null, Linux will accept the connection and fault
while trying to write through the null pointer,
consuming the connection but returning `EFAULT`.
