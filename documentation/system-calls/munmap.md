Remove mappings from the calling task's memory map.

This function performs the `munmap` system call exactly once.
It does not retry, track mapping ownership, or synchronize users of the mapping.

MMU and NOMMU kernels use the same two-argument system call ABI but different
memory-map implementations.

# MMU kernels

## Address and length

Linux first applies the target architecture's user-address untagging rule
to `address`. Accepted pointer tag bits are removed before validation.
Linux then treats the result as a numeric virtual address. It does not
access memory through the pointer.

The resulting address must be aligned to the system page size. `length`
must not be zero. It does not need to be page aligned. Linux rounds it up
to the system page size.

The effective range is half-open. It starts at the untagged `address`.
It ends at that address plus the rounded length. The range must fit in
the user virtual address space.

The range does not need to match an earlier `mmap` request. It can remove
a whole mapping, a prefix, a suffix, the middle of a mapping, or parts of
several mappings. Unmapped holes are ignored. A range containing no mapped
page succeeds.

Removing the middle of one virtual memory area normally divides it into
two VMAs. This increases the memory map's VMA count by one.

Special mappings can reject a boundary split. Every boundary that cuts
through a HugeTLB VMA must be aligned to that VMA's huge page size. Only
boundaries inside the HugeTLB VMA have this requirement. Transparent huge
pages use normal system-page range semantics.

## Effects

Success removes every mapped part of the effective range. Linux clears
the corresponding page table entries and makes the virtual addresses
available for immediate reuse. A later access normally faults. If another
mapping has already reused an address, the access reaches that mapping
instead.

Unmapping affects the shared memory map, not only the calling task.
Every task that shares the map through `CLONE_VM` observes the change.
A task with a copied memory map keeps its separate mapping.

Unmapping a file-backed range does not require the original file
descriptor to remain open. It releases references held only by the removed
VMA segments. Other mappings of the same object remain.

`munmap` does not truncate a mapped file or force dirty data to storage.
Dirty shared file pages can remain in the page cache and be written later.
Use the required memory, file, and storage synchronization operations when
durability matters.

Removing locked mappings also releases their locked-memory accounting.
Linux releases physical pages, swap, reservations, file references, and
other resources when no remaining mapping or kernel reference needs them.

Linux also updates userfaultfd state and completes any required unmap
notification.

Mappings are also removed when `execve` replaces the memory map or when
the last task using the memory map exits.

## Ordering and failure

Current Linux acquires the memory map write lock with a killable wait.
It then validates and rounds the range. Pending `SIGKILL` can abort the wait
with [`-EINTR`](crate::Errno::EINTR). The task normally dies before userspace
can observe that result. Lock acquisition precedes range validation.
A request that overlaps no VMA returns success.

For an overlapping range, Linux prepares every affected VMA before it
removes any selected VMA segment or page table entry. It checks memory
seals. It creates required boundary splits. It invokes mapping-specific
split operations. Linux then removes the selected VMA segments, clears
page tables, updates accounting, flushes stale address translations,
releases the lock, and completes userfaultfd notifications.

Memory management operations by tasks that share the map are serialized
by this lock. Ordinary loads, stores, and instruction fetches do not
acquire it. Userspace must provide its own synchronization.

Current Linux unmaps no page when `munmap` returns an error. A late
preparation failure can still leave an existing VMA divided into adjacent
VMAs. Such a split changes the memory map's topology. It does not itself
change the mapped bytes, permissions, or backing.

If any affected VMA is sealed by `mseal`, Linux returns
[`-EPERM`](crate::Errno::EPERM) and unmaps no page in the requested range.
Memory sealing exists since Linux 6.10. Stable and vendor kernels may have
backported it.

## Errors

Generic errors include:

 - [`-EINVAL`](crate::Errno::EINVAL)

   `length` is zero. The untagged `address` is not aligned to the system
   page size. The range overflows or extends beyond the user virtual
   address space. A mapping-specific split rule can also return this
   error. HugeTLB boundary misalignment is a common example.

 - [`-ENOMEM`](crate::Errno::ENOMEM)

   Linux cannot allocate required VMA, memory-map tree, or userfaultfd
   notification metadata. Removing the middle of a VMA can also exceed
   the limit configured by `/proc/sys/vm/max_map_count`.

 - [`-EPERM`](crate::Errno::EPERM)

   The range intersects a VMA sealed by `mseal`.

 - [`-EINTR`](crate::Errno::EINTR)

   Pending `SIGKILL` interrupts the killable wait for the memory map write
   lock. The task normally terminates before userspace observes this result.

Special mappings can return additional errors from their split operations.

An unmapped address is not itself an error. Linux does not return
[`-EFAULT`](crate::Errno::EFAULT) merely because the range contains a
hole.

# NOMMU kernels

The NOMMU syscall uses a separate implementation. The MMU rules above do not
apply to it.

Unlike the MMU implementation, the NOMMU syscall does not call
`untagged_addr`. Linux rounds `length` up to the system page size and rejects
a length that becomes zero. It does not impose the MMU path's blanket
page-alignment check on `address`.

A range with no overlapping VMA returns
[`-EINVAL`](crate::Errno::EINVAL). NOMMU does not provide the MMU rule that
unmapped holes and arbitrary parts of several VMAs are silently accepted.

Anonymous mappings can be removed wholly or partially. A partial range must
stay within one VMA. Every boundary inside that VMA must be page aligned.
Removing the middle can split the VMA.

File-backed VMAs cannot be split by `munmap`. A request that would remove only
part of one returns [`-EINVAL`](crate::Errno::EINVAL). A successful request
removes the selected file-backed VMA as a whole.

Linux takes the memory map write lock non-interruptibly on this path. A signal
while waiting for that lock does not itself produce
[`-EINTR`](crate::Errno::EINTR).

Success removes or shrinks the selected VMA and updates its backing-region
references. Linux releases backing memory, file references, and other
resources when no remaining reference needs them. There are no user page
tables to clear or stale address translations to flush.

A failed request can still change VMA topology. An anonymous VMA can remain
split if a later memory-map tree update fails.

## Errors

Generic NOMMU errors include:

 - [`-EINVAL`](crate::Errno::EINVAL)

   `length` becomes zero after page rounding, no VMA overlaps the range, a
   file-backed VMA would need to be split, or an anonymous subrange crosses
   its VMA boundary or has an unaligned internal boundary.

 - [`-ENOMEM`](crate::Errno::ENOMEM)

   Linux cannot allocate or update VMA or memory-map metadata needed to split,
   shrink, or remove the mapping.

VMA operations can return additional errors.

# Result

On success, Linux returns zero, and this function returns `Ok(())`.

On failure, Linux returns a negated error number, and this function
returns the corresponding [`Errno`](crate::Errno).

# Safety

`address` does not need to be dereferenceable by Rust. Linux consumes its
numeric value only. This does not make unmapping live Rust storage safe.

The caller must ensure every address Linux can remove may be unmapped now.
No Rust reference may remain live to removed storage. No pointer may be
dereferenced through the old mapping. This includes access by other tasks,
allocators, runtimes, and foreign code.

On NOMMU kernels, stale access need not fault. The absence of a hardware fault
does not make access through a removed mapping valid.

The range must not contain code that can still execute, an active stack,
thread-local storage, a signal frame, unwind data, allocator metadata, or
other process state that remains in use.

Every task that can access the range must be synchronized before
unmapping. The kernel's memory map lock does not provide Rust or
application synchronization.

A later mapping at the same numeric address does not by itself make old
Rust pointers valid again. Rust provenance and every higher-level
invariant still apply.

The caller must preserve the Rust memory model and every process invariant
for the rounded range and every mapping change Linux can commit.
