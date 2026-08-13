Wait for ready events on `epoll`
and write them through `events`,
while optionally replacing the
calling task's blocked signal mask
for the duration of the wait.

`epoll` must be an open file descriptor
that refers to an epoll instance.

This function performs the `epoll_pwait` system call
exactly once. It does not retry when Linux returns
[`-EINTR`](crate::Errno::EINTR).

# Event buffer

`events` points to an array of the target architecture's
[`epoll_event`](crate::definitions::epoll_event) structures.
Linux does not retain `events` after the system call returns.

`max_events` is the maximum number of events Linux may write
through `events`. It must be positive. Linux also limits it
to the internal `EP_MAX_EVENTS` constant, defined as:

```c
INT_MAX / sizeof(struct epoll_event)
```

Before waiting, Linux checks whether the virtual address range
beginning at `events` and spanning `max_events` elements lies
in userspace. This does not establish that the memory is mapped
or writable. Individual event copies may still fault.

On success, the return value is the number of events
that Linux has actually written through `events`,
ranging from zero through `max_events`.

For each item delivered, Linux polls the watched file
again and writes the resulting event mask along with
the arbitrary `data` value stored in that item by
[`epoll_ctl`](crate::system_calls::epoll_ctl).
An item on the ready list may produce no event
if the watched file no longer reports any
requested event when Linux scans it.

If Linux writes `max_events` before finishing the scan,
items it did not process remain available and may be
returned by later waits.

Should copying an event fail after one or more complete
events have already been written, Linux returns the number
already written rather than [`-EFAULT`](crate::Errno::EFAULT).
Linux requeues the item whose copy failed for another attempt.
If Linux fails to copy the first event,
it returns [`-EFAULT`](crate::Errno::EFAULT).

A failed copy may have modified part of the
`epoll_event` structure Linux was writing
when the fault occurred.

# Timeout

`timeout` is a signed 32-bit number.

Every negative value waits indefinitely.
A value of zero checks the ready list
without blocking. Positive values specify
a timeout duration in milliseconds.

The wait continues until Linux finds a ready event,
the calling task is interrupted, or the positive
timeout expires.

Linux adds positive timeouts to the current
monotonic time to form an absolute deadline.
Linux computes timer slack for the wait and
may allow its high-resolution timer to expire
after that deadline so timer wakeups can be
coalesced. Scheduling may further delay the
resumption of the calling task.

# Signal mask

When `signal_mask` is null, Linux leaves
the calling task's blocked signal mask
unchanged. In this case, Linux completely
ignores `signal_mask_size`.

When `signal_mask` is not null, it points to
the target architecture's native kernel `sigset_t`.
Linux requires `signal_mask_size` to exactly equal
`sizeof(sigset_t)`. The size can vary by architecture
because Linux architectures may define different
numbers of signals.

Linux copies the new mask, saves the caller's
current blocked signal mask, removes `SIGKILL`
and `SIGSTOP` because they cannot be blocked,
and installs the new blocked signal mask before
entering the epoll wait. There is no return to
userspace between changing the mask and waiting.

If the wait finishes without `-EINTR`,
Linux restores the saved blocked signal
mask before returning from the system call.

When the wait returns `-EINTR`, Linux deliberately leaves
restoration of the saved blocked signal mask pending while
processing signals and other work before returning to userspace.
If Linux returns to userspace without entering a signal handler,
it restores the saved mask first.

If Linux invokes a signal handler, the temporary mask becomes
the basis of the handler's blocked signal mask. Linux adds
the signal action's `sa_mask` and, unless `SA_NODEFER`
is set, the delivered signal. The signal stack frame
contains the saved mask for a later `rt_sigreturn`
to restore.

Linux does not retain `signal_mask`
after the system call returns.

# Errors

Linux may return:

 - [`-EBADF`](crate::Errno::EBADF)

   `epoll` is not a valid open file descriptor.

 - [`-EFAULT`](crate::Errno::EFAULT)

   A non-null `signal_mask` could not be read,
   the event buffer does not fit entirely in userspace,
   or Linux could not copy the first complete event
   through `events`.

 - [`-EINTR`](crate::Errno::EINTR)

   Linux interrupted the wait before delivering an event.
   A pending signal is often the cause, but not always.
   For example, `TIF_NOTIFY_SIGNAL` is not a signal but
   it is treated like one internally so Linux can break
   interruptible waits and process pending kernel work.

   This function returns the error
   directly and does not retry.

 - [`-EINVAL`](crate::Errno::EINVAL)

   `signal_mask` is non-null and `signal_mask_size`
   does not equal `sizeof(sigset_t)`,
   `max_events` is not positive
   or exceeds `EP_MAX_EVENTS`,
   or `epoll` is open but does not
   refer to an epoll instance.

Linux processes a non-null signal mask
before examining any epoll wait argument.
It validates `signal_mask_size` before
reading through `signal_mask`, so `-EINVAL`
takes precedence over `-EFAULT` when both
are invalid. Signal mask errors take
precedence over errors in `epoll`,
`events`, or `max_events`.

After accepting the signal mask, Linux looks up
`epoll`, validates `max_events`, checks the complete
event buffer range, and finally checks whether
the open file is an epoll instance. When several
arguments are invalid, an earlier check takes
precedence over a later one.

# Safety

The caller must ensure every memory access
Linux successfully performs through `events`
or `signal_mask` respects the Rust memory model
and all other invariants in Rust or anywhere else.

Any `epoll_event` Linux successfully writes
must be writable at the time of the write
and must not be subject to incompatible
aliasing.

Linux reads a non-null `signal_mask` only while
copying it before the wait. It is not accessed
after that point.

A failing system call may have written through `events`.
Linux may partially write an event before returning `-EFAULT`,
or return a positive event count after a later event copy faults.

Neither pointer needs to be dereferenceable
by Rust merely for the system call to be issued.
Passing an inaccessible userspace address may
cause Linux to return `-EFAULT`,
a perfectly valid result.
