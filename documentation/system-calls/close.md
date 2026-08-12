Closes a file descriptor.

Linux may report an error after releasing
a valid file descriptor. Do not retry the
system call after an error: the number may
have already been reused.

# Safety

If `descriptor` is open, this system call may close it
regardless of any Rust or foreign value referencing it.
The caller is responsible for ensuring that doing so
does not violate the invariants of any such live value.

`descriptor` does not need to identify an open file
descriptor. Linux returns `EBADF` for invalid inputs.
A valid `descriptor` must be treated as consumed after
this system call, regardless of the returned result.
