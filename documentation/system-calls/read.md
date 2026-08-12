Read up to `buffer_length` bytes from the file `descriptor` into `buffer`.
Returns the number of bytes read, which may be less
than `buffer_length`. No retries are made.
Reading while at the end of the file
returns zero.

# Safety

Rust safety and semantics do not matter to the kernel.
However, the kernel semantics matter a lot to Rust.
The caller must ensure any memory access that Linux
successfully performs through `buffer` does not
violate the Rust memory model or any other
invariants in Rust or anywhere else.

The kernel may attempt to write `buffer_length` bytes
at the address contained in `buffer`, which is not
actually required to be dereferenceable by Rust.
Passing unmapped user space addresses may cause Linux
to simply return `EFAULT`, a perfectly valid result.
