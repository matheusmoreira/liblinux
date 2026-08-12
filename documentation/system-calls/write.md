Write up to `buffer_length` bytes from `buffer` to a file descriptor.
Returns the number of bytes written, which may be less than `buffer_length`.
No automatic retries are made.

# Safety

The caller must ensure any memory access that Linux
successfully performs through `buffer` does not
violate the Rust memory model or any other
invariants in Rust or anywhere else.

The kernel may attempt to read `buffer_length` bytes
at the address contained in `buffer`, which is not
actually required to be dereferenceable by Rust.
Passing unmapped user space addresses may cause Linux
to simply return `EFAULT`, a perfectly valid result.
