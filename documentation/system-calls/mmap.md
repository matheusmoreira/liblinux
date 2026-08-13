Create a memory mapping for the calling task.

This function performs the `mmap` system call exactly once.
It does not retry, issue alternative system calls, or own
the resulting mapping. Success returns the address of the
first byte of the mapping.

MMU and NOMMU kernels use different memory mapping implementations.

# MMU kernels

## Address and length

`length` must not be zero. The mapped memory can be larger than length:
Linux rounds normal mappings up to the system page size, and hugetlb
mappings up to the selected huge page size.

Without either [`MAP_FIXED`](crate::definitions::MAP_FIXED) or
[`MAP_FIXED_NOREPLACE`](crate::definitions::MAP_FIXED_NOREPLACE),
`address` is a placement hint. A null hint allows Linux to choose
the address. Linux aligns a non-null hint to a page boundary.
Security policies covering placement at low addresses
as well as architecture specific placement rules
also apply. Linux may return a different address
when the hinted range cannot be used.

[`MAP_FIXED`](crate::definitions::MAP_FIXED) requires an exact aligned address.
Linux replaces every overlapping mapping unless `mseal` has sealed one,
in which case `mmap` returns [`-EPERM`](crate::Errno::EPERM).

[`MAP_FIXED_NOREPLACE`](crate::definitions::MAP_FIXED_NOREPLACE)
also requires an exact aligned address.
Linux returns [`-EEXIST`](crate::Errno::EEXIST)
when any page overlaps an existing mapping.
This flag provides an atomic placement attempt
within the memory map. Linux holds the memory
map write lock while checking for overlap and
installing the mapping. Competing requests from
tasks that share the same memory map cannot both
claim the same free address space range.

If hugetlb is used, all exact addresses must satisfy
the alignment requirements of the selected huge page.

A successful result can be null when Linux permits an exact mapping
at address zero. For example, a null `address` with `MAP_FIXED`,
`MAP_PRIVATE`, and `MAP_ANONYMOUS` requests address zero exactly.
If low address security permits it, Linux can install the mapping
and return zero. `MAP_FIXED_NOREPLACE` can likewise return zero
when the range is free.

The capability security module normally rejects mappings below
`vm.mmap_min_addr` unless the task has `CAP_SYS_RAWIO` in the
initial user namespace. Other security modules can impose
additional rules.

Linux represents mappings as virtual memory areas (VMAs).
The returned address can lie inside a VMA formed by merging
adjacent mappings. It still begins a valid mapped range
of the rounded up length.

## Protection

`protection` is a word sized bitfield. Common bits are:

 - [`PROT_NONE`](crate::definitions::PROT_NONE)

   Requests no access permissions.

 - [`PROT_READ`](crate::definitions::PROT_READ)

   Permits reads.

 - [`PROT_WRITE`](crate::definitions::PROT_WRITE)

   Permits writes.

 - [`PROT_EXEC`](crate::definitions::PROT_EXEC)

   Permits instruction execution.

 - [`PROT_SEM`](crate::definitions::PROT_SEM)

   Marks memory as usable for atomic operations.
   No additional effect on x86_64 or AArch64.

Linux stores a `personality` word on each task for special compatibility behavior.
If its `READ_IMPLIES_EXEC` bit is set, [`PROT_READ`](crate::definitions::PROT_READ)
also adds [`PROT_EXEC`](crate::definitions::PROT_EXEC), except for a mapping backed
by a file on a no-execute mount.

Hardware can enforce a coarser permission model. Writable memory can remain
readable without [`PROT_READ`](crate::definitions::PROT_READ). Security policy
can reject executable or writable/executable mappings.

`mmap` does not generically reject unrecognized protection bits.
Bits that are not interpreted by generic or architecture code
can be ignored. Architecture-specific protection combinations
can still be rejected.

## Mapping type

`flags` must contain a valid mapping type selected by
[`MAP_TYPE`](crate::definitions::MAP_TYPE).

 - [`MAP_SHARED`](crate::definitions::MAP_SHARED)

   Updates are visible through other shared mappings of the same object.
   Updates to file-backed mappings can reach the file. Visibility does
   _not_ guarantee persistence.

 - [`MAP_PRIVATE`](crate::definitions::MAP_PRIVATE)

   Requests private copy-on-write semantics.
   Updates do not update a normal mapped file.
   Often used for memory allocation.

 - [`MAP_SHARED_VALIDATE`](crate::definitions::MAP_SHARED_VALIDATE)

   Creates a validated shared file mapping. Linux calls the historical set
   of file mapping flags `LEGACY_MAP_MASK`. [`MAP_SHARED`](crate::definitions::MAP_SHARED)
   preserves compatibility by silently discarding unsupported non-legacy flags.
   This flag instead rejects unsupported non-legacy flags by returning
   [`-EOPNOTSUPP`](crate::Errno::EOPNOTSUPP).

   [`MAP_SYNC`](crate::definitions::MAP_SYNC) is a non-legacy flag
   and requires `MAP_SHARED_VALIDATE`.

 - [`MAP_DROPPABLE`](crate::definitions::MAP_DROPPABLE)

   Creates private anonymous memory that is never swapped out.
   Linux may discard any page. A later read then observes zero.

   A fault that cannot allocate backing memory is not fatal.
   The mapping is inherited across `fork`, but its contents
   are cleared in the child. It is excluded from core dumps
   and commit reservation.

   [`MAP_ANONYMOUS`](crate::definitions::MAP_ANONYMOUS) is required.
   [`MAP_DROPPABLE`](crate::definitions::MAP_DROPPABLE) is itself
   a mapping type and cannot be combined with
   [`MAP_PRIVATE`](crate::definitions::MAP_PRIVATE) or
   [`MAP_SHARED`](crate::definitions::MAP_SHARED).
   It cannot be locked, hugetlb-backed, file-backed,
   or grow downward.

## File and offset

Without [`MAP_ANONYMOUS`](crate::definitions::MAP_ANONYMOUS), the file `descriptor`
must refer to an open object that supports memory mapping and is open for reading.
A writable shared mapping also requires write access. Files, devices, and security
modules can impose further checks.

`offset` is a byte offset. Native x86_64 and AArch64 Linux require system page
alignment. Mappings using hugetlb can have stronger alignment requirements.
The byte offset is validated before Linux considers [`MAP_ANONYMOUS`](crate::definitions::MAP_ANONYMOUS).

For normal anonymous mappings, Linux ignores the file `descriptor` and the aligned `offset`.
New memory reads as zero. Anonymous hugetlb mappings use an internal hugetlb file and retain
hugetlb offset rules. Use offset zero for them.

A successful file mapping retains the mapped object. Closing the file `descriptor`
does _not_ remove the mapping. Access to a page wholly beyond the current end of
a regular file normally delivers `SIGBUS`, so file truncation can invalidate pages
even after `mmap` succeeds.

Linux normally zero fills the part of a file's final partial
page beyond end of file and never writes that tail to the file.
A writable shared page cache mapping can nevertheless modify
those bytes. They can remain in the page cache after the
`descriptor` is closed and the mapping is removed, and a
later mapping can observe them even though they never
became file data. Calling `msync` before unmapping can
prevent later observation on some filesystems, but not
on tmpfs.

File-backed mappings can also update inode timestamps.
Generic page cache mapping setup runs the file access
time update path. When a shared file page is write faulted,
the generic file mapping path runs the modification and
status change time update before making the page writable.
Filesystem timestamp policy and custom mapping operations
can change whether and when these updates become visible.

Files and devices can define more specific
fault and writeback behavior.

## Flags

`flags` is a word sized bitfield. Common flags are:

 - [`MAP_ANONYMOUS`](crate::definitions::MAP_ANONYMOUS)

   Selects a mapping that is not backed by the supplied file `descriptor`.
   Linux can still create internal backing objects. Shared anonymous mappings
   use shmem, and anonymous hugetlb mappings use an internal hugetlbfs file.

 - [`MAP_FIXED`](crate::definitions::MAP_FIXED)

   Selects destructive exact placement as described above.

 - [`MAP_FIXED_NOREPLACE`](crate::definitions::MAP_FIXED_NOREPLACE)

   Selects non-destructive exact placement as described above.

 - [`MAP_GROWSDOWN`](crate::definitions::MAP_GROWSDOWN)

   Marks a mapping that can expand toward lower addresses.

   Linux keeps an unmapped `stack_guard_gap` between a growing
   mapping and neighboring accessible mappings. Growth stops
   rather than consuming that gap. The current default is 256
   system pages and can be changed with the `stack_guard_gap`
   kernel parameter. Growth is also subject to address space,
   stack size, and locked memory limits.

   Linux accepts this flag only for anonymous
   [`MAP_PRIVATE`](crate::definitions::MAP_PRIVATE) mappings.
   File-backed, shared, and droppable mappings reject it
   with [`-EINVAL`](crate::Errno::EINVAL).

 - [`MAP_DENYWRITE`](crate::definitions::MAP_DENYWRITE) and
   [`MAP_EXECUTABLE`](crate::definitions::MAP_EXECUTABLE)

   Legacy flags that are accepted with no effect.

 - [`MAP_LOCKED`](crate::definitions::MAP_LOCKED)

   Locks the mapping against reclamation when permissions
   and the locked memory limit both allow it. Linux attempts
   to populate the range after creating it. Population failures
   are not returned as `mmap` errors.

 - [`MAP_NORESERVE`](crate::definitions::MAP_NORESERVE)

   Suppresses normal swap or commit reservation when the mapping and overcommit
   policy permit it. A later write can fail through a signal or OOM handling
   when backing memory is unavailable.

 - [`MAP_POPULATE`](crate::definitions::MAP_POPULATE)

   Requests page table population and file readahead after installation
   of the mapping. Population failures are not returned as `mmap` errors.

 - [`MAP_NONBLOCK`](crate::definitions::MAP_NONBLOCK)

   Modifies [`MAP_POPULATE`](crate::definitions::MAP_POPULATE).
   Linux suppresses the population work instead of performing
   non-blocking work.

 - [`MAP_STACK`](crate::definitions::MAP_STACK)

   Marks the mapping as suitable for a process or thread stack. It does not
   imply [`MAP_GROWSDOWN`](crate::definitions::MAP_GROWSDOWN). When transparent
   huge pages are configured, current Linux excludes the mapping from them.

 - [`MAP_HUGETLB`](crate::definitions::MAP_HUGETLB)

   Uses the hugetlb subsystem. Combine using bitwise OR with one of the `MAP_HUGE_*`
   values into `flags` to request a specific huge page size. Zero size bits select
   the system default.

   The encoded value is the base two logarithm of the size in bytes.
   It is found by shifting right by [`MAP_HUGE_SHIFT`](crate::definitions::MAP_HUGE_SHIFT)
   and masking with [`MAP_HUGE_MASK`](crate::definitions::MAP_HUGE_MASK). The requested size
   must be supported by the system in order to be used.

 - [`MAP_SYNC`](crate::definitions::MAP_SYNC)

   Requests synchronous page fault behavior from a supporting file. It requires
   [`MAP_SHARED_VALIDATE`](crate::definitions::MAP_SHARED_VALIDATE). Direct access
   persistent memory files are the usual consumers. The flag does not make every
   CPU store durable by itself.

 - [`MAP_UNINITIALIZED`](crate::definitions::MAP_UNINITIALIZED)

   Has no effect on MMU kernels. NOMMU behavior is described below.

 - [`MAP_FILE`](crate::definitions::MAP_FILE)

   Is zero and has no effect.

## Result and lifetime

Success creates the aligned memory range in the caller's address space.
It does not guarantee that physical pages are present. Most normal mappings
are faulted in on demand. A later access can still deliver `SIGSEGV` or `SIGBUS`
because of protection, file truncation, storage errors, memory exhaustion,
or mapping-specific rules.

The mapping remains until Linux removes or replaces it. Common causes include
`munmap`, `mremap`, an exact replacement mapping, `execve`, and destruction
of the last task sharing the memory map. `fork` normally copies the mapping
into the child's memory map. Tasks sharing one memory map observe changes
immediately.

Shared visibility is not synchronization or persistence.
Programs must still use the atomic, volatile, cache management,
file synchronization, and protocol operations as required.

## Ordering and interposition

The architecture-specific system call entry can perform argument handling
before Linux enters the generic mmap path. On x86_64 and AArch64, the entry
point rejects a byte `offset` that is not system page aligned, converts the
byte offset to a page offset, and then enters the generic path. The offset
error therefore precedes file `descriptor` lookup on both interfaces.

The generic `mmap` code path performs the main work in this order:

 1. Look up a file `descriptor`, or create an anonymous hugetlb object.
 2. Apply mmap security and file notification checks.
 3. Acquire the memory map write lock with a killable wait.
 4. Validate and round up the length. Check range and mapping count limits.
 5. Derive VMA permissions and select or enforce the address.
 6. Validate overlap, locking, type, flags, file access, and the backing object.
 7. Prepare, install, or merge the mapping. Destructive exact placement checks
    for sealed overlaps before gathering and clearing overlapping mappings.
 8. Release the lock and complete userfaultfd unmap notifications.
 9. Perform any requested population work.

A bad file `descriptor` can be reported before a zero `length` is checked.
A pending `SIGKILL` can interrupt the memory map lock wait and return
[`-EINTR`](crate::Errno::EINTR). The task normally terminates before
userspace observes this result.

Address selection can invoke architecture-specific placement logic.
Mapping setup can invoke filesystem and device operations.

Linux security modules, filesystem operations, device operations, hugetlb,
memory policy, resource limits, and userfaultfd can interpose on this path
and can return additional errors.

A failing [`MAP_FIXED`](crate::definitions::MAP_FIXED) call does not imply
that the old mappings survived. Linux can clear overlapping mappings before
a file or device mapping finishes. A later failure can leave a gap.
[`MAP_FIXED_NOREPLACE`](crate::definitions::MAP_FIXED_NOREPLACE)
avoids this destructive overlap behavior.

Population occurs after successful installation. Failure during
[`MAP_POPULATE`](crate::definitions::MAP_POPULATE) or the population
phase of [`MAP_LOCKED`](crate::definitions::MAP_LOCKED) does not change
the successful `mmap` result.

## Errors

Generic errors include:

 - [`-EINVAL`](crate::Errno::EINVAL)

   `length` is zero. The byte offset or an exact address is misaligned.
   The mapping type, flags, or any combination thereof is invalid.
   Architecture-specific protection requirements and hugetlb alignment
   or size requirements can also fail this way.

 - [`-ENOMEM`](crate::Errno::ENOMEM)

   No suitable virtual address space range, mapping slot, reservation,
   or other required memory resource is available. Rounding up `length`
   can also wrap the native address space. A net expansion of the memory
   map can exceed `RLIMIT_AS`. A writable mapping that is neither shared
   nor a stack can exceed `RLIMIT_DATA`. The `ignore_rlimit_data` kernel
   parameter can turn the latter rejection into a warning.

 - [`-EOVERFLOW`](crate::Errno::EOVERFLOW)

   The page offset and rounded length overflow,
   or the requested file range is not representable
   by the mapped object.

 - [`-EBADF`](crate::Errno::EBADF)

   A non-anonymous mapping names a file `descriptor` that is not open.

 - [`-EACCES`](crate::Errno::EACCES)

   The file is not readable, a writable shared mapping lacks file write access,
   a shared mapping uses an append-only file opened for writing, or Linux has
   rejected the mapping due to a memory-deny-write-execute policy.

 - [`-EPERM`](crate::Errno::EPERM)

   [`PROT_EXEC`](crate::definitions::PROT_EXEC) is requested for a file on a
   no-execute mount, [`MAP_LOCKED`](crate::definitions::MAP_LOCKED) is requested
   while `RLIMIT_MEMLOCK` is zero and the task lacks `CAP_IPC_LOCK`, the capability
   low address check rejects exact placement below `vm.mmap_min_addr`, `mseal` protects
   an overlapping VMA from destructive exact placement, or either `F_SEAL_WRITE` or
   `F_SEAL_FUTURE_WRITE` rejects a new writable shared mapping of a sealed memfd.

 - [`-EEXIST`](crate::Errno::EEXIST)

   [`MAP_FIXED_NOREPLACE`](crate::definitions::MAP_FIXED_NOREPLACE)
   overlaps an existing mapping.

 - [`-EAGAIN`](crate::Errno::EAGAIN)

   [`MAP_LOCKED`](crate::definitions::MAP_LOCKED)
   exceeds the caller's locked memory limit.

 - [`-EOPNOTSUPP`](crate::Errno::EOPNOTSUPP)

   A validated shared mapping contains an unsupported non-legacy flag,
   the selected object does not support [`MAP_SYNC`](crate::definitions::MAP_SYNC),
   or the executing kernel cannot represent [`MAP_DROPPABLE`](crate::definitions::MAP_DROPPABLE).

 - [`-ENODEV`](crate::Errno::ENODEV)

   The selected file or device does not support memory mapping.

 - [`-ENOENT`](crate::Errno::ENOENT)

   Anonymous hugetlb setup selected a huge page size
   whose internal hugetlbfs mount is unavailable.

 - [`-ENFILE`](crate::Errno::ENFILE)

   The system-wide file structure limit prevents Linux from creating
   the internal file used by an anonymous hugetlb mapping.

 - [`-ENOSPC`](crate::Errno::ENOSPC)

   Linux cannot allocate the internal hugetlbfs inode
   required by an anonymous hugetlb mapping.

 - [`-ETXTBSY`](crate::Errno::ETXTBSY)

   A writable shared mapping targets an active swap file.

 - [`-EINTR`](crate::Errno::EINTR)

   A pending `SIGKILL` interrupts the killable wait
   for the memory map write lock. The task normally
   terminates before userspace observes this result.

File systems, devices, security modules, hugetlb,
and architecture code can return additional errors.

# NOMMU kernels

NOMMU uses a separate mapping implementation. There is no virtual address
translation or ordinary demand paging. Linux still records each mapping
as a VMA and associates it with a memory region.

## Address and length

`length` must not be zero. Linux rounds it up to the system page size.
Linux returns [`-ENOMEM`](crate::Errno::ENOMEM) if the rounding wraps
or if the rounded length exceeds the task address space.

[`MAP_FIXED`](crate::definitions::MAP_FIXED)
is rejected with [`-EINVAL`](crate::Errno::EINVAL).

Without `MAP_FIXED`, `address` is not an MMU-style placement hint.
Linux passes its numeric value through the low address security
check, then discards it before selecting the mapping address.
A non-null value can affect security policy without requesting
that placement.

[`MAP_FIXED_NOREPLACE`](crate::definitions::MAP_FIXED_NOREPLACE)
is not promoted to `MAP_FIXED` on this path. It does not provide
atomic exact placement or overlap rejection.

Anonymous and copied mappings are page aligned.
A direct file or device mapping can return an
address with a different alignment when the
backing object requires it.

## Protection and mapping type

Only [`MAP_PRIVATE`](crate::definitions::MAP_PRIVATE) and
[`MAP_SHARED`](crate::definitions::MAP_SHARED) are accepted.
[`MAP_SHARED_VALIDATE`](crate::definitions::MAP_SHARED_VALIDATE) and
[`MAP_DROPPABLE`](crate::definitions::MAP_DROPPABLE) return
[`-EINVAL`](crate::Errno::EINVAL).

The requested protections do not create ordinary page table permissions.
They still record access intent and constrain direct mappings. A private
mapping can fall back to a copy when the backing object cannot provide
the requested access directly. A shared mapping fails instead.

[`PROT_EXEC`](crate::definitions::PROT_EXEC) on a file from a no-execute mount
returns [`-EPERM`](crate::Errno::EPERM). `READ_IMPLIES_EXEC` can add executable
intent when the backing object supports it. Anonymous mappings apply the same
personality rule without a backing file capability check.

Anonymous `MAP_SHARED` uses the same internal mapping behavior as `MAP_PRIVATE`.
NOMMU Linux does not provide `fork`. Tasks created with `clone` must share the
memory map.

[`MAP_GROWSDOWN`](crate::definitions::MAP_GROWSDOWN) can mark the VMA,
but NOMMU Linux does not support expanding it across the mapping boundary.

## Backing and file mappings

Anonymous mappings allocate backing memory before success.
Linux normally clears the whole mapping before return.
[`MAP_UNINITIALIZED`](crate::definitions::MAP_UNINITIALIZED)
suppresses this only when the kernel was built with the
`CONFIG_MMAP_ALLOW_UNINITIALIZED` option enabled.
Otherwise Linux ignores the flag.

The MMU behaviors of [`MAP_NORESERVE`](crate::definitions::MAP_NORESERVE),
[`MAP_POPULATE`](crate::definitions::MAP_POPULATE), and
[`MAP_NONBLOCK`](crate::definitions::MAP_NONBLOCK)
do not apply to these eagerly backed mappings.
The generic NOMMU path also does not select
anonymous hugetlb backing for
[`MAP_HUGETLB`](crate::definitions::MAP_HUGETLB).

A file mapping requires an open readable file that supports memory mapping.
The file or device determines whether Linux can map its storage directly,
copy it into allocated memory, or both.

A private file mapping requires copy support. A non-writable private mapping
can use the backing storage directly when the object permits it. Otherwise
Linux allocates memory and copies the file data. A writable private mapping
is forced onto the copy path. A short read clears the remainder of the mapping.

A shared file mapping requires direct mapping support. Linux does not fall back
to a private copy. A writable shared mapping also requires write access.
An append-only file that is open for writing cannot be mapped shared.

Linux can reuse an existing compatible direct region, including one created
for another task. Filesystems and device drivers can define more specific
mapping behavior and errors. A successful file mapping retains its file
references, so closing the supplied `descriptor` does not remove it.

The validated shared mapping interface is unavailable because
`MAP_SHARED_VALIDATE` is rejected. Passing [`MAP_SYNC`](crate::definitions::MAP_SYNC)
with plain `MAP_SHARED` does not reproduce the MMU mapping
validation behavior described above.

## Result and lifetime

On success, Linux returns the VMA address. Anonymous and copied mappings
already have backing memory. Direct mappings refer to storage supplied
by a file, filesystem, or device. The MMU lazy fault model does not apply.

NOMMU Linux does not provide `fork`. `clone` must share the memory map.
Mappings remain until an operation such as `munmap`, `mremap`, or `execve`
changes them, or until the memory map is destroyed. NOMMU `munmap` and
`mremap` have stricter range rules than their MMU implementations.

## Ordering and interposition

The common entry code path first looks up a non-anonymous file `descriptor`.
It then runs mmap security and file-notification checks and acquires the
memory map write lock with a killable wait.

The NOMMU code path then:

 1. Validates the mapping type, fixed placement, length, offset range,
    file access, and direct or copied mapping capabilities.
 2. Applies executable file rules and the low address security check.
 3. Checks the VMA count limit, discards the caller's address hint,
    and rounds the length up to the system page size.
 4. Allocates VMA and region metadata.
 5. Acquires the NOMMU region lock. It reuses a compatible region,
    requests a direct mapping, or allocates and copies backing memory.
 6. Clears new anonymous memory unless `MAP_UNINITIALIZED` is active.
 7. Registers the mapping and flushes the instruction cache when required.
 8. Releases the NOMMU region lock and then the memory map write lock.

Pending `SIGKILL` can interrupt the killable wait and produce
[`-EINTR`](crate::Errno::EINTR). The task normally terminates
before userspace observes this result. The NOMMU region lock
acquisition cannot be interrupted.

A bad file `descriptor` can be reported before a zero `length` is checked.
Security and file-notification failures also precede NOMMU argument validation.

## Errors

Generic NOMMU errors include:

 - [`-EINVAL`](crate::Errno::EINVAL)

   `MAP_FIXED` is present, the mapping type is not `MAP_PRIVATE` or `MAP_SHARED`,
   `length` is zero, the file type is unsupported, a shared mapping cannot provide
   the requested direct access, or Linux finds an incompatible region sharing request.

 - [`-ENOMEM`](crate::Errno::ENOMEM)

   The rounded length wraps or exceeds the task address space,
   the VMA count limit is reached, or Linux cannot allocate
   mapping metadata or backing memory.

 - [`-EOVERFLOW`](crate::Errno::EOVERFLOW)

   The page offset plus the rounded mapping length overflows.

 - [`-EBADF`](crate::Errno::EBADF)

   A non-anonymous mapping names a file `descriptor` that is not open.

 - [`-EACCES`](crate::Errno::EACCES)

   The file is not readable, a writable shared mapping lacks write access,
   or a shared mapping uses an append-only file that is open for writing.

 - [`-ENODEV`](crate::Errno::ENODEV)

   The file cannot be memory mapped or cannot provide
   the direct/copied mapping required by the request.

 - [`-EPERM`](crate::Errno::EPERM)

   [`PROT_EXEC`](crate::definitions::PROT_EXEC) is requested for a file
   on a no-execute mount, or the capability low address check rejects `address`.

 - [`-EINTR`](crate::Errno::EINTR)

   Pending `SIGKILL` interrupts the killable wait for the memory map write
   lock. The task normally terminates before userspace observes this result.

Filesystems, devices, security modules, and architecture code
can return additional errors.

# Safety

The caller must preserve the Rust memory model and every other invariant
for every mapping change Linux can commit. This obligation applies regardless
of success or error.

Linux interprets `address` as a number. Merely issuing the system call
does not dereference the pointer, so it needs not designate live or
dereferenceable Rust storage.

On MMU kernels, exact placement can remove live Rust allocations,
references, stacks, thread-local storage, executable code, or runtime data.
It can make return from the system call impossible. Destructive exact
placement requires every possible overlap to be disposable. An error from
[`MAP_FIXED`](crate::definitions::MAP_FIXED) is _not_ a rollback guarantee.

A new mapping can alias another mapping of the same memory.
File-backed, shared, device, and droppable mappings can also
change without a Rust write through the returned pointer.
The caller must enforce Rust aliasing and concurrency rules
across every alias and participant.

The returned pointer has no Rust lifetime, ownership, alignment beyond
the mapping alignment, or initialized value guarantees. The caller must
track the memory range and its lifetime. A typed read requires initialized
bytes that form a valid value of that type. Zero-filled memory is not valid
for every Rust type.

Requested protections do not replace Rust synchronization or validity
requirements. Device memory and memory changed outside Rust can require
volatile access. Shared memory protocols can require atomics and fences.
Executable mappings can require architecture-specific instruction cache
and code publication rules.

On NOMMU kernels, requested protections do not provide ordinary MMU isolation.
Direct mappings can expose memory controlled by a file, device, or another task.
`MAP_UNINITIALIZED` can expose uncleared bytes. Rust validity and aliasing rules
still apply even when the hardware permits an access.

On MMU kernels, a successful call does not guarantee every later access
will succeed. Code that can reach a lazily faulted, file-backed, overcommitted,
hugetlb, or droppable page must account for all possible signals and content
changes that its memory mapping permits.

# Kernel version differences

The following transitions describe upstream Linux.
Unless stated otherwise, they concern the MMU implementation:

 - Before Linux 2.4.10, file-backed
   [`MAP_GROWSDOWN`](crate::definitions::MAP_GROWSDOWN)
   mappings were accepted. Downward growth subtracted from the file page offset
   without checking for underflow, so the offset could wrap. Linux 2.4.10 began
   rejecting file-backed growing mappings with [`-EINVAL`](crate::Errno::EINVAL).

 - Before Linux 2.6, [`MAP_NORESERVE`](crate::definitions::MAP_NORESERVE)
   affected only private writable mappings.

 - Before Linux 2.6.12, a zero `length` could succeed without creating a
   mapping. Linux 2.6.12 and above return [`-EINVAL`](crate::Errno::EINVAL).

 - [`MAP_POPULATE`](crate::definitions::MAP_POPULATE) and
   [`MAP_NONBLOCK`](crate::definitions::MAP_NONBLOCK) exist
   since Linux 2.5.46. Before Linux 2.6.7, `MAP_POPULATE`
   had effect only with `PROT_NONE`. Linux 2.6.23 added
   `MAP_POPULATE` support for private mappings and changed
   `MAP_NONBLOCK` to suppress population work.

 - [`MAP_STACK`](crate::definitions::MAP_STACK) exists since Linux 2.6.27.
   [`MAP_HUGETLB`](crate::definitions::MAP_HUGETLB) exists since Linux 2.6.32.
   Hugetlb page size encoding exists since Linux 3.8.

 - NOMMU [`MAP_UNINITIALIZED`](crate::definitions::MAP_UNINITIALIZED)
   exists since Linux 2.6.33. Linux honors it only when built with
   `CONFIG_MMAP_ALLOW_UNINITIALIZED`.

 - Linux 2.6.39 made downward VMA growth reject an expansion
   that would underflow its page offset.

 - Linux 4.7 enabled `RLIMIT_DATA` enforcement by default for writable mappings
   that are neither shared nor stacks. The `ignore_rlimit_data` kernel parameter
   can still disable rejection.

 - [`MAP_SHARED_VALIDATE`](crate::definitions::MAP_SHARED_VALIDATE) and
   [`MAP_SYNC`](crate::definitions::MAP_SYNC) exist since Linux 4.15.

 - [`MAP_FIXED_NOREPLACE`](crate::definitions::MAP_FIXED_NOREPLACE) exists
   since Linux 4.17. Older kernels can ignore the unknown flag and treat
   `address` as a hint. Code supporting those kernels must compare the
   returned address with the requested address.

 - Linux 6.10 introduced `mseal`.
   A VMA sealed by `mseal` rejects
   replacement with an overlapping
   [`MAP_FIXED`](crate::definitions::MAP_FIXED)
   request with [`-EPERM`](crate::Errno::EPERM).

 - [`MAP_DROPPABLE`](crate::definitions::MAP_DROPPABLE) exists since Linux 6.11.

Stable and vendor kernels can backport these changes.
