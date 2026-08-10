Read the clock identified by `clock` and, if successful,
write its value to `time` as whole seconds and nanoseconds.

This function invokes the system call directly
instead of using the vDSO.

# Clock identifiers

Linux clock identifiers are signed 32-bit integers.
They are an open namespace, not a closed enumeration.

Zero or positive clock identifiers select from
a table of Linux clocks that includes:

 - [`CLOCK_REALTIME`](crate::definitions::CLOCK_REALTIME)
 - [`CLOCK_MONOTONIC`](crate::definitions::CLOCK_MONOTONIC)
 - [`CLOCK_MONOTONIC_RAW`](crate::definitions::CLOCK_MONOTONIC_RAW)
 - [`CLOCK_PROCESS_CPUTIME_ID`](crate::definitions::CLOCK_PROCESS_CPUTIME_ID)
 - [`CLOCK_THREAD_CPUTIME_ID`](crate::definitions::CLOCK_THREAD_CPUTIME_ID)
 - [`CLOCK_REALTIME_COARSE`](crate::definitions::CLOCK_REALTIME_COARSE)
   and [`CLOCK_MONOTONIC_COARSE`](crate::definitions::CLOCK_MONOTONIC_COARSE)
 - [`CLOCK_BOOTTIME`](crate::definitions::CLOCK_BOOTTIME)
 - [`CLOCK_REALTIME_ALARM`](crate::definitions::CLOCK_REALTIME_ALARM)
   and [`CLOCK_BOOTTIME_ALARM`](crate::definitions::CLOCK_BOOTTIME_ALARM)
 - [`CLOCK_TAI`](crate::definitions::CLOCK_TAI)
 - Auxiliary clocks beginning at
   [`CLOCK_AUX`](crate::definitions::CLOCK_AUX)

Negative clock identifiers encode a file descriptor
or task identifier by complementing it and shifting
it left three bits. Bits 1 and 0 select combined user
and system CPU time, user CPU time, scheduler execution
time, or a dynamic clock backed by a file descriptor.
For CPU clocks, bit 2 selects whether the identifier
names one task or a thread group. Setting all three
low bits produces an invalid clock identifier.

Task identifiers are interpreted in the context of
the caller's PID namespace. A task identifier value
of zero selects the caller for a task CPU clock
and the caller's thread group for a thread group
CPU clock. A non-zero task CPU clock may only
identify a task in the caller's thread group.
A non-zero thread group CPU clock normally
identifies a thread group by its leader's
TGID, numerically the task ID of its leader.
The calling task's own TID is accepted
for a thread group CPU clock and resolves
to the caller's thread group.

Negative CPU clocks may select:

 - User CPU time
 - Combined user and system CPU time
 - Scheduler execution time

Dynamic POSIX clock identifiers encode open file descriptors.
Linux requires them to refer to registered POSIX clock devices,
which are often provided by PTP hardware clocks.

Encoded clock identifiers do not retain references
to the tasks or file descriptors they identify.
Their targets may disappear and the associated numbers
may be reused independently of the clock identifier.

Linux normalizes the nanoseconds value of its builtin
and CPU clocks to the `[0, 999_999_999]` interval.
However, Linux does _not_ validate or normalize
any values returned by dynamic POSIX clocks,
it copies them to user space unchanged.

# Time namespaces

Linux applies the caller's time namespace monotonic offset
to the monotonic, raw monotonic and coarse monotonic clocks.
It also applies the namespace's boot time offset to the boot
time and boot time alarm clocks.

Linux time namespaces do not shift the real time, coarse real time,
real time alarm, TAI, CPU, dynamic POSIX or auxiliary clocks.

# Errors

Linux may return:

 - [`Errno::EINVAL`](crate::Errno::EINVAL)

   `clock` does not identify a readable clock.
   This includes unassigned or unavailable
   positive clock identifiers, invalid CPU
   clock encodings or targets, invalid or
   non-clock file descriptors, and alarm
   clocks lacking a usable alarm timer RTC
   device.

 - [`Errno::ENODEV`](crate::Errno::ENODEV)

   A dynamic POSIX clock or auxiliary clock
   is or has become unavailable.

 - [`EOPNOTSUPP`](crate::Errno::EOPNOTSUPP)

   A dynamic POSIX clock exists
   but does not implement clock reading.

 - [`Errno::EFAULT`](crate::Errno::EFAULT)

   Linux read the clock successfully
   but could not write the result
   through `time`.

Dynamic POSIX clock implementations may return
additional device specific errors.

Linux resolves and reads the clock before attempting
to access `time`. An error while selecting or reading
the clock occurs before accessing the output pointer.

Linux does not write `time` transactionally.
If the copy faults, some bytes may already
have been written by the time Linux returns
`EFAULT`.

# Safety

The caller must ensure any memory access that Linux
successfully performs through `time` does not violate
the Rust memory model or any other invariants in Rust
or anywhere else.

Linux does not read through `time`. After successfully
reading the selected clock, it may write a complete
or partial time value through the pointer.

The pointer does not need to be dereferenceable by Rust
for the system call to be issued. Passing an inaccessible
user space address may cause Linux to return `EFAULT`,
a perfectly valid result.

# Example

Read the monotonic clock:

```rust
use linux::definitions::{CLOCK_MONOTONIC, __kernel_timespec};
use linux::system_calls::clock_gettime;

let mut time = __kernel_timespec {
    tv_sec: 0,
    tv_nsec: 0,
};

// SAFETY: `time` remains writable
// for the duration of the system call.
unsafe {
    clock_gettime(CLOCK_MONOTONIC, &mut time).unwrap();
}
```
