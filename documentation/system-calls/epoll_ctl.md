Add `file_descriptor` to `epoll`,
modify its event settings,
or remove it.

`file_descriptor` identifies the file to watch.
The file must support polling. Sockets, pipes
and other event-producing files normally do,
while regular files and directories normally
do not. An epoll instance may watch another
epoll instance, subject to Linux's cycle,
nesting and wakeup path limits.

# Operations

`operation` is a signed 32-bit integer.
Linux owns this namespace, so it is not
a closed Rust enumeration. The running
kernel determines which values are
accepted. Newer kernels may define
additional values, which older
kernels reject.

Linux currently defines:

 - [`EPOLL_CTL_ADD`](crate::definitions::EPOLL_CTL_ADD)

   Add `file_descriptor` to the interest set of `epoll`.
   Linux creates an item linking the watched file
   and its descriptor number to the epoll instance,
   then copies the settings from `event` into it.

 - [`EPOLL_CTL_MOD`](crate::definitions::EPOLL_CTL_MOD)

   Replace the event interest mask and user data
   stored in the existing item for `file_descriptor`.
   This operation also rearms an item disabled by
   [`EPOLLONESHOT`](crate::definitions::EPOLLONESHOT)
   after an event is delivered.

 - [`EPOLL_CTL_DEL`](crate::definitions::EPOLL_CTL_DEL)

   Remove the existing item for `file_descriptor`
   from the interest set of `epoll`. Current Linux
   kernels completely ignore `event` during this
   operation.

Unsupported operation values are rejected with
[`-EINVAL`](crate::Errno::EINVAL).

# Item identity and lifetime

Linux identifies an epoll interest set item
by the watched file and its corresponding
file descriptor number.

If the interest set of `epoll` already contains an item
for the same watched file and file descriptor number,
Linux returns [`-EEXIST`](crate::Errno::EEXIST).

A file descriptor returned by `dup`, `dup2`, `dup3`,
or `fcntl` with `F_DUPFD` or `F_DUPFD_CLOEXEC`
may refer to the same open file description
but have a completely different numeric value.
Because the number differs, the duplicate
identifies a different item and can be added
to `epoll` with a different event mask
and user data.

Closing `file_descriptor` does not necessarily remove
its item from `epoll` immediately. Should another file
descriptor still refer to the same open file description,
Linux may continue reporting events for the item. Only
when the last file descriptor referring to the watched
file is closed does Linux remove any remaining items
for that file from epoll instances. If deterministic
removal matters, remove the item _before_ closing
the file descriptor that was used to add it.

# Event

`event` points to an instance of the target architecture's
[`epoll_event`](crate::definitions::epoll_event) structure.

Linux copies the event structure during the system call
and does not retain the userspace pointer after it returns.

The `events` field is a bit mask. Linux owns this namespace,
so it is not a closed Rust enumeration. The running kernel
determines which bits are defined and their behavior.

Current Linux does not generally reject undefined bits.
It preserves them in the item's event interest mask
and can return them if the watched file reports
matching bits. Some restrictions apply when
`EPOLLEXCLUSIVE` is set, as described below.

Linux currently defines the following event bits:

 - [`EPOLLIN`](crate::definitions::EPOLLIN)
   for read readiness

 - [`EPOLLPRI`](crate::definitions::EPOLLPRI)
   for urgent or exceptional conditions

 - [`EPOLLOUT`](crate::definitions::EPOLLOUT)
   for write readiness

 - [`EPOLLERR`](crate::definitions::EPOLLERR)
   for error conditions

 - [`EPOLLHUP`](crate::definitions::EPOLLHUP)
   for hangups

 - [`EPOLLNVAL`](crate::definitions::EPOLLNVAL)
   for invalid polling requests

 - [`EPOLLRDNORM`](crate::definitions::EPOLLRDNORM)
   when normal priority data can be read

 - [`EPOLLRDBAND`](crate::definitions::EPOLLRDBAND)
   when priority band data can be read

 - [`EPOLLWRNORM`](crate::definitions::EPOLLWRNORM)
   when normal priority data can be written

 - [`EPOLLWRBAND`](crate::definitions::EPOLLWRBAND)
   when priority band data can be written

 - [`EPOLLMSG`](crate::definitions::EPOLLMSG)
   for message events

 - [`EPOLLRDHUP`](crate::definitions::EPOLLRDHUP)
   when a stream peer closes or shuts down its writing half

Linux also defines
[`EPOLL_URING_WAKE`](crate::definitions::EPOLL_URING_WAKE),
which Linux describes as an internal wakeup bit generated
by `io_uring`. The bit is propagated through epoll wakeups
to detect recursion back into the `io_uring` poll handler.

An event mask of zero is entirely valid.
When adding or modifying an item, Linux
adds `EPOLLERR` and `EPOLLHUP` whether
or not they were requested.

Linux groups the following as epoll private bits
inside the event mask:

 - [`EPOLLET`](crate::definitions::EPOLLET)

   Select edge-triggered notification
   instead of the level-triggered default.

 - [`EPOLLONESHOT`](crate::definitions::EPOLLONESHOT)

   Disable event delivery after delivering
   exactly one event for the item, which
   can be rearmed via `EPOLL_CTL_MOD`.

 - [`EPOLLWAKEUP`](crate::definitions::EPOLLWAKEUP)

   Request inhibition of system suspend
   while an event is pending or being
   processed.

 - [`EPOLLEXCLUSIVE`](crate::definitions::EPOLLEXCLUSIVE)

   Request exclusive wakeups
   when several epoll instances
   watch the same target.

These bits control epoll behavior rather than
describe readiness of the watched file.

The `data` field is arbitrary `u64` user data.
Linux does not interpret the value in any way.
It stores the data unmodified in the item and
returns it unchanged when `epoll_wait` delivers
an event for that item.

`EPOLL_CTL_MOD` replaces both `events` and `data`.

# Exclusive wakeups

`EPOLLEXCLUSIVE` may only be supplied
while adding `file_descriptor` to `epoll`
with `EPOLL_CTL_ADD`. An item added with
`EPOLLEXCLUSIVE` cannot be changed later
with `EPOLL_CTL_MOD`. It must be removed
and added again.

Current Linux allows it to be combined with
`EPOLLIN`, `EPOLLOUT`, `EPOLLERR`, `EPOLLHUP`,
`EPOLLWAKEUP`, and `EPOLLET`. Other event bits
are rejected.

`EPOLLEXCLUSIVE` cannot be used when
`file_descriptor` refers to another
epoll instance.

Linux installs exclusive poll wait queue entries
for an item using `EPOLLEXCLUSIVE`. When the watched
file wakes its wait queues, Linux may wake one or more
epoll instances containing exclusive items. Nonexclusive
wait queue entries for the same watched file are _also_
woken up.

# System suspend inhibition

When Linux is built without `CONFIG_PM_SLEEP`,
it clears `EPOLLWAKEUP` from the copied event.
No error is returned or signaled.

When Linux is built with `CONFIG_PM_SLEEP`
but the caller lacks `CAP_BLOCK_SUSPEND`,
Linux also clears the bit without
returning or signaling an error.

Should the bit survive, Linux attempts to use wakeup sources
to inhibit system suspend while matching events are pending
or being processed.

Should wakeup source allocation fail
during `EPOLL_CTL_ADD`, Linux removes
the new item and returns
[`-ENOMEM`](crate::Errno::ENOMEM).

During `EPOLL_CTL_MOD`, current Linux ignores failure
to create a newly requested wakeup source and still
returns success. The item retains `EPOLLWAKEUP`
but has no wakeup source.

For a level-triggered item, `EPOLLWAKEUP` keeps
the item's wakeup source active while the event
remains on the ready list. After delivering the
event, Linux requeues the item and reactivates
its wakeup source, inhibiting suspend until
a later `epoll_wait` processes it again.

With `EPOLLET` or `EPOLLONESHOT`, Linux does not
requeue the item for that delivery. It relaxes
the item's wakeup source during delivery and keeps
the epoll instance awake until the current event
scan completes. A later readiness notification
may requeue an edge-triggered item and reactivate
its wakeup source. A one-shot item must first be
rearmed with `EPOLL_CTL_MOD`.

# Errors

Linux may return:

 - [`-EBADF`](crate::Errno::EBADF)

   `epoll` or `file_descriptor` is not
   a valid open file descriptor.

 - [`-EEXIST`](crate::Errno::EEXIST)

   `operation` is `EPOLL_CTL_ADD`, but the
   epoll instance already contains an item
   with the same watched file and descriptor
   number.

 - [`-EFAULT`](crate::Errno::EFAULT)

   `operation` is not `EPOLL_CTL_DEL`
   and Linux could not read a complete event
   through `event`.

   Linux performs this read before validating
   `epoll`, `file_descriptor` or `operation`,
   so `-EFAULT` can take precedence over
   errors such as `-EBADF` and `-EINVAL`.

 - [`-EINVAL`](crate::Errno::EINVAL)

   `epoll` is open but does not refer to an epoll instance,
   `file_descriptor` refers to the same epoll instance,
   `operation` is unsupported, an `EPOLLEXCLUSIVE`
   restriction was violated, or an `EPOLL_CTL_ADD`
   operation would create more nested wakeup paths
   than Linux permits.

 - [`-ELOOP`](crate::Errno::ELOOP)

   `operation` is `EPOLL_CTL_ADD`, `file_descriptor`
   refers to another epoll instance, and adding it
   would create a cycle or exceed Linux's epoll
   nesting limit.

 - [`-ENOENT`](crate::Errno::ENOENT)

   `operation` is `EPOLL_CTL_MOD` or `EPOLL_CTL_DEL`,
   but the epoll instance contains no item matching
   the watched file and descriptor number identified
   by `file_descriptor`.

 - [`-ENOMEM`](crate::Errno::ENOMEM)

   Linux could not allocate memory required
   to perform the requested operation.

 - [`-ENOSPC`](crate::Errno::ENOSPC)

   `operation` is `EPOLL_CTL_ADD`
   and adding the item would exceed
   `/proc/sys/fs/epoll/max_user_watches`.

 - [`-EPERM`](crate::Errno::EPERM)

   `file_descriptor` refers to a file
   that does not support polling.

# Safety

For every operation other than `EPOLL_CTL_DEL`,
Linux attempts to read one complete event through
`event`. The caller must ensure any memory access
Linux successfully performs through that pointer
does not violate the Rust memory model or any other
invariants in Rust or anywhere else.

The event structure `event` points to must remain readable
for the duration of the system call. Linux makes a copy of
its contents and does not retain the pointer.

The pointer does not need to be dereferenceable by Rust
for the system call to be issued. Passing an inaccessible
userspace address may cause Linux to return `-EFAULT`,
a perfectly valid result.

Since Linux 2.6.9, `EPOLL_CTL_DEL` does not read `event`,
so the pointer may be null for that operation. Older kernels
still require a non-null pointer even though they do not use
the event it points to.
