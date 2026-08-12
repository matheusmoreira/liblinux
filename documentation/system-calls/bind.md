Bind a socket to an address.

`address` points to the exact socket address bytes Linux will receive.
`address_length` is significant. Valid address formats and lengths vary
according to the socket's address family. The `address` begins with a
16 bit address family field.

For Unix sockets, address family always equals `AF_UNIX`
and is followed by the address path which may be up to
`UNIX_PATH_MAX` bytes which is defined as 108 bytes.
Although the `sockaddr_un` structure defined by the
Linux UAPI contains a maximally sized path buffer,
this is not actually mandatory: smaller paths
could be created and passed, and they are accepted
by the `bind` system call. Passing a two byte `address`,
or, in other words, not passing a path buffer at all,
instructs Linux to autobind: the kernel automatically
chooses an abstract address.

A file system address begins with a non-NUL byte
and contains a file system path. A NUL terminator
is not actually necessary. All `UNIX_PATH_MAX`
bytes may consist of path data. Linux copies
the address into its own storage and writes
a NUL immediately after the supplied path
bytes before interpreting them as a path.

A leading NUL byte in the address path selects
the abstract Unix socket namespace, where names
are arbitrary binary data, length delimited.
Consequently, names such as `b"\0name"`
and `b"\0name\0"` are distinct.

# Safety

Rust safety and semantics do not matter to the kernel.
However, the kernel semantics matter a lot to Rust.
The caller must ensure any memory access that Linux
successfully performs through `address` does not
violate the Rust memory model or any other
invariants in Rust or anywhere else.

Linux rejects negative `address_length` values
and lengths larger than its internal socket
address storage buffer before reading through
`address`.

The address inside `address` is not actually required
to be dereferenceable by Rust. Passing unmapped user
space addresses may cause Linux to simply return `EFAULT`,
a perfectly valid result.
