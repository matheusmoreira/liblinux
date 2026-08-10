Create a new epoll instance and return a file descriptor for it.

The returned file descriptor can be passed to
the other epoll system calls in order to
add watched file descriptors and wait for
readiness events.

The epoll instance in the kernel remains alive
for as long as a file descriptor refers to it.
Closing the last file descriptor referencing it
will destroy it and release its kernel resources.

# Flags

`flags` is a bit mask.

A value of zero creates the file descriptor
without any additional behavior.

[`EPOLL_CLOEXEC`](crate::definitions::EPOLL_CLOEXEC)
atomically sets the close-on-exec flag on the new
file descriptor.

Only flags known to the executing Linux kernel
are accepted, all other bits are rejected with
`-EINVAL`. Currently, only these flag bits are
defined:

 - [`EPOLL_CLOEXEC`](crate::definitions::EPOLL_CLOEXEC)

# Errors

Linux may return:

 - [`Errno::EINVAL`](crate::Errno::EINVAL)

   `flags` contains unknown flag bits.

 - [`Errno::EMFILE`](crate::Errno::EMFILE)

   The calling task has reached its file descriptor limit.

 - [`Errno::ENFILE`](crate::Errno::ENFILE)

   Linux cannot allocate another open file
   because a system-wide limit was reached.

 - [`Errno::ENOMEM`](crate::Errno::ENOMEM)

   Linux could not allocate memory required
   for the epoll instance or its file object.

# Example

Create an epoll instance whose descriptor will be closed by `execve`:

```rust
use linux::definitions::EPOLL_CLOEXEC;
use linux::system_calls::{close, epoll_create1};

let descriptor = epoll_create1(EPOLL_CLOEXEC).unwrap();

// SAFETY: `descriptor` is the open file descriptor
// returned by the above `epoll_create1` system call.
unsafe {
    close(descriptor).unwrap();
}
```
