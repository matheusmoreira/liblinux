Open the `path` and return its file descriptor.

On success, the caller owns the returned
file descriptor and must eventually close it.

The original `open` system call.
Not all architectures have it.
`openat` is the modern version.

# Safety

Rust safety and semantics do not matter to the kernel.
However, the kernel semantics matter a lot to Rust.

The kernel may read bytes beginning at `path`
in order to resolve the location named by it.
`path` is not required to be dereferenceable
by Rust. Unmapped user space addresses may
cause Linux to return `EFAULT`.

The caller must ensure that any memory access
Linux successfully performs through `path`
does not violate Rust's memory model or
any other invariants of any live values.
