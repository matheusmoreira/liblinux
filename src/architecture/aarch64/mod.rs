//! AArch64 Linux system call interface.

/// AArch64 Linux kernel definitions.
/// Symbols match those in UAPI headers.
///
/// System call numbers are architecture specific.
/// All other definitions are either re-exports
/// of the shared Linux kernel definitions
/// or architecture specific overrides.
pub mod definitions {
    #![allow(non_upper_case_globals)]

    /// System call number for `epoll_create1`.
    /// Linux >= 3.7
    pub const __NR_epoll_create1: usize = 20;
    /// System call number for `epoll_ctl`.
    /// Linux >= 3.7
    pub const __NR_epoll_ctl: usize = 21;
    /// System call number for `dup`.
    /// Linux >= 3.7
    pub const __NR_dup: usize = 23;
    /// System call number for `faccessat`.
    /// Linux >= 3.7
    pub const __NR_faccessat: usize = 48;
    /// System call number for `close`.
    /// Linux >= 3.7
    pub const __NR_close: usize = 57;
    /// System call number for `read`.
    /// Linux >= 3.7
    pub const __NR_read: usize = 63;
    /// System call number for `write`.
    /// Linux >= 3.7
    pub const __NR_write: usize = 64;
    /// System call number for `exit_group`.
    /// Linux >= 3.7
    pub const __NR_exit_group: usize = 94;
    /// System call number for `clock_gettime`.
    /// Linux >= 3.7
    pub const __NR_clock_gettime: usize = 113;
    /// System call number for `getpid`.
    /// Linux >= 3.7
    pub const __NR_getpid: usize = 172;
    /// System call number for `socket`.
    /// Linux >= 3.7
    pub const __NR_socket: usize = 198;
    /// System call number for `bind`.
    /// Linux >= 3.7
    pub const __NR_bind: usize = 200;
    /// System call number for `listen`.
    /// Linux >= 3.7
    pub const __NR_listen: usize = 201;
    /// System call number for `connect`.
    /// Linux >= 3.7
    pub const __NR_connect: usize = 203;
    /// System call number for `setsockopt`.
    /// Linux >= 3.7
    pub const __NR_setsockopt: usize = 208;
    /// System call number for `munmap`.
    /// Linux >= 3.7
    pub const __NR_munmap: usize = 215;
    /// System call number for `mremap`.
    /// Linux >= 3.7
    pub const __NR_mremap: usize = 216;
    /// System call number for `mmap`.
    /// Linux >= 3.7
    pub const __NR_mmap: usize = 222;
    /// System call number for `accept4`.
    /// Linux >= 3.7
    pub const __NR_accept4: usize = 242;
    /// System call number for `prlimit64`.
    /// Linux >= 3.7
    pub const __NR_prlimit64: usize = 261;
    /// System call number for `renameat2`.
    /// Linux >= 3.15
    pub const __NR_renameat2: usize = 276;
    /// System call number for `faccessat2`.
    /// Linux >= 5.8
    pub const __NR_faccessat2: usize = 439;

    /// An event exchanged with an epoll instance.
    #[repr(C)]
    pub struct epoll_event {
        /// Events that occurred or are being requested.
        pub events: u32,

        /// Caller-owned data returned with the event.
        pub data: u64,
    }

    pub use crate::shared::definitions::*;
}

/// The AArch64 Linux system calls.
#[cfg(all(target_arch = "aarch64", target_pointer_width = "64"))]
pub mod system_calls;

/// Perform an AArch64 Linux system call with 0 arguments.
///
/// # Safety
///
/// System calls can do anything the kernel allows.
/// They may end the process or read and write memory
/// through pointer arguments.
///
/// The caller must ensure `number` refers
/// to a system call that takes no arguments.
#[inline]
#[cfg(all(target_arch = "aarch64", target_pointer_width = "64"))]
pub unsafe fn system_call_0(number: usize) -> isize {
    let result: isize;

    unsafe {
        core::arch::asm!(
            "svc 0",
            in("x8") number,                // system call number
            lateout("x0") result,           // return value
            options(nostack),                // deliberately omitted: nomem, preserves_flags
        );
    }

    result
}

/// Perform an AArch64 Linux system call with 1 argument.
///
/// # Safety
///
/// System calls can do anything the kernel allows.
/// They may end the process or read and write memory
/// through pointer arguments.
///
/// The caller must ensure `number` refers
/// to a system call that takes one argument.
#[inline]
#[cfg(all(target_arch = "aarch64", target_pointer_width = "64"))]
pub unsafe fn system_call_1(number: usize, argument_1: usize) -> isize {
    let result: isize;

    unsafe {
        core::arch::asm!(
            "svc 0",
            in("x8") number,                        // system call number
            inlateout("x0") argument_1 => result,   // argument 1, return value
            options(nostack),                        // deliberately omitted: nomem, preserves_flags
        );
    }

    result
}

/// Perform an AArch64 Linux system call with 2 arguments.
///
/// # Safety
///
/// System calls can do anything the kernel allows.
/// They may end the process or read and write memory
/// through pointer arguments.
///
/// The caller must ensure `number` refers
/// to a system call that takes two arguments.
#[inline]
#[cfg(all(target_arch = "aarch64", target_pointer_width = "64"))]
pub unsafe fn system_call_2(number: usize, argument_1: usize, argument_2: usize) -> isize {
    let result: isize;

    unsafe {
        core::arch::asm!(
            "svc 0",
            in("x8") number,                        // system call number
            inlateout("x0") argument_1 => result,   // argument 1, return value
            in("x1") argument_2,                    // system call argument 2
            options(nostack),                        // deliberately omitted: nomem, preserves_flags
        );
    }

    result
}

/// Perform an AArch64 Linux system call with 3 arguments.
///
/// # Safety
///
/// System calls can do anything the kernel allows.
/// They may end the process or read and write memory
/// through pointer arguments.
///
/// The caller must ensure `number` refers
/// to a system call that takes three arguments.
#[inline]
#[cfg(all(target_arch = "aarch64", target_pointer_width = "64"))]
pub unsafe fn system_call_3(
    number: usize,
    argument_1: usize,
    argument_2: usize,
    argument_3: usize,
) -> isize {
    let result: isize;

    unsafe {
        core::arch::asm!(
            "svc 0",
            in("x8") number,                        // system call number
            inlateout("x0") argument_1 => result,   // argument 1, return value
            in("x1") argument_2,                    // system call argument 2
            in("x2") argument_3,                    // system call argument 3
            options(nostack),                        // deliberately omitted: nomem, preserves_flags
        );
    }

    result
}

/// Perform an AArch64 Linux system call with 4 arguments.
///
/// # Safety
///
/// System calls can do anything the kernel allows.
/// They may end the process or read and write memory
/// through pointer arguments.
///
/// The caller must ensure `number` refers
/// to a system call that takes four arguments.
#[inline]
#[cfg(all(target_arch = "aarch64", target_pointer_width = "64"))]
pub unsafe fn system_call_4(
    number: usize,
    argument_1: usize,
    argument_2: usize,
    argument_3: usize,
    argument_4: usize,
) -> isize {
    let result: isize;

    unsafe {
        core::arch::asm!(
            "svc 0",
            in("x8") number,                        // system call number
            inlateout("x0") argument_1 => result,   // argument 1, return value
            in("x1") argument_2,                    // system call argument 2
            in("x2") argument_3,                    // system call argument 3
            in("x3") argument_4,                    // system call argument 4
            options(nostack),                        // deliberately omitted: nomem, preserves_flags
        );
    }

    result
}

/// Perform an AArch64 Linux system call with 5 arguments.
///
/// # Safety
///
/// System calls can do anything the kernel allows.
/// They may end the process or read and write memory
/// through pointer arguments.
///
/// The caller must ensure `number` refers
/// to a system call that takes five arguments.
#[inline]
#[cfg(all(target_arch = "aarch64", target_pointer_width = "64"))]
pub unsafe fn system_call_5(
    number: usize,
    argument_1: usize,
    argument_2: usize,
    argument_3: usize,
    argument_4: usize,
    argument_5: usize,
) -> isize {
    let result: isize;

    unsafe {
        core::arch::asm!(
            "svc 0",
            in("x8") number,                        // system call number
            inlateout("x0") argument_1 => result,   // argument 1, return value
            in("x1") argument_2,                    // system call argument 2
            in("x2") argument_3,                    // system call argument 3
            in("x3") argument_4,                    // system call argument 4
            in("x4") argument_5,                    // system call argument 5
            options(nostack),                        // deliberately omitted: nomem, preserves_flags
        );
    }

    result
}

/// Perform an AArch64 Linux system call with 6 arguments.
///
/// # Safety
///
/// System calls can do anything the kernel allows.
/// They may end the process or read and write memory
/// through pointer arguments.
///
/// The caller must ensure `number` refers
/// to a system call that takes six arguments.
#[inline]
#[cfg(all(target_arch = "aarch64", target_pointer_width = "64"))]
pub unsafe fn system_call_6(
    number: usize,
    argument_1: usize,
    argument_2: usize,
    argument_3: usize,
    argument_4: usize,
    argument_5: usize,
    argument_6: usize,
) -> isize {
    let result: isize;

    unsafe {
        core::arch::asm!(
            "svc 0",
            in("x8") number,                        // system call number
            inlateout("x0") argument_1 => result,   // argument 1, return value
            in("x1") argument_2,                    // system call argument 2
            in("x2") argument_3,                    // system call argument 3
            in("x3") argument_4,                    // system call argument 4
            in("x4") argument_5,                    // system call argument 5
            in("x5") argument_6,                    // system call argument 6
            options(nostack),                        // deliberately omitted: nomem, preserves_flags
        );
    }

    result
}

#[cfg(all(test, target_arch = "aarch64", target_pointer_width = "64"))]
mod tests {
    use super::*;
    use definitions::epoll_event;

    #[test]
    fn getpid_matches_std() {
        let std_pid = std::process::id() as isize;

        // getpid takes no arguments and cannot fail
        let liblinux_pid = unsafe { system_call_0(definitions::__NR_getpid) };

        assert_eq!(liblinux_pid, std_pid);
    }

    #[test]
    fn dup_then_close_the_duplicate() {
        let standard_input: usize = 0;

        // duplicate the standard input file descriptor then close it
        let duplicate = unsafe { system_call_1(definitions::__NR_dup, standard_input) };

        assert!(duplicate > 2);

        let closed = unsafe { system_call_1(definitions::__NR_close, duplicate as usize) };

        assert_eq!(closed, 0);
    }

    #[test]
    fn faccessat_resolves_a_relative_path() {
        const F_OK: usize = 0;
        const AT_FDCWD: isize = -100;
        const EBADF: isize = 9;

        // use a relative path so the kernel doesn't ignore the descriptor
        let path = c".";
        let result = unsafe {
            system_call_3(
                definitions::__NR_faccessat,
                AT_FDCWD as usize,
                path.as_ptr() as usize,
                F_OK,
            )
        };

        assert_eq!(result, 0);

        // also verify that the kernel's negative errno is returned unchanged
        let invalid = unsafe {
            system_call_3(
                definitions::__NR_faccessat,
                usize::MAX,
                path.as_ptr() as usize,
                F_OK,
            )
        };

        assert_eq!(invalid, -EBADF);
    }

    #[test]
    fn prlimit_reads_the_descriptor_limit() {
        const RLIMIT_NOFILE: usize = 7;

        let pid_self: usize = 0;
        let null_new: usize = 0;

        // struct rlimit is two u64 values: soft, hard
        let mut old = [0u64; 2];

        // query own limits
        let result = unsafe {
            system_call_4(
                definitions::__NR_prlimit64,
                pid_self,
                RLIMIT_NOFILE,
                null_new,
                old.as_mut_ptr() as usize,
            )
        };

        assert_eq!(result, 0);
        assert!(old[0] > 0); // a real soft limit would be positive
    }

    #[test]
    fn mmap_mremap_munmap() {
        const PROT_READ_WRITE: usize = 0x1 | 0x2;
        const MAP_PRIVATE_ANONYMOUS: usize = 0x2 | 0x20;
        const MREMAP_MAYMOVE: usize = 0x1;
        const MREMAP_FIXED: usize = 0x2;
        // 64 KiB is aligned to every AArch64 Linux page size
        const MAPPING_SIZE: usize = 64 * 1024;
        const EINVAL: isize = 22;

        let no_fd: usize = usize::MAX;
        let offset: usize = 0;

        // mmap rejects a byte offset that is not page aligned
        // provoke failure to exercise the sixth argument
        let invalid = unsafe {
            system_call_6(
                definitions::__NR_mmap,
                0,
                MAPPING_SIZE,
                PROT_READ_WRITE,
                MAP_PRIVATE_ANONYMOUS,
                no_fd,
                1,
            )
        };

        assert_eq!(invalid, -EINVAL);

        // one unit to move, and a two unit window to move it into
        let mapped = unsafe {
            system_call_6(
                definitions::__NR_mmap,
                0,
                MAPPING_SIZE,
                PROT_READ_WRITE,
                MAP_PRIVATE_ANONYMOUS,
                no_fd,
                offset,
            )
        };

        assert!(mapped > 0);

        let window = unsafe {
            system_call_6(
                definitions::__NR_mmap,
                0,
                MAPPING_SIZE * 2,
                PROT_READ_WRITE,
                MAP_PRIVATE_ANONYMOUS,
                no_fd,
                offset,
            )
        };

        assert!(window > 0);

        // kernel ignores new address without MREMAP_FIXED
        let remapped = unsafe {
            system_call_5(
                definitions::__NR_mremap,
                mapped as usize,
                MAPPING_SIZE,
                MAPPING_SIZE * 2,
                MREMAP_MAYMOVE | MREMAP_FIXED,
                window as usize,
            )
        };

        assert_eq!(remapped, window);

        let freed = unsafe {
            system_call_2(
                definitions::__NR_munmap,
                remapped as usize,
                MAPPING_SIZE * 2,
            )
        };

        assert_eq!(freed, 0);
    }

    #[test]
    fn epoll_event_has_the_aarch64_layout() {
        assert_eq!(core::mem::size_of::<epoll_event>(), 16);
        assert_eq!(core::mem::align_of::<epoll_event>(), 8);
        assert_eq!(core::mem::offset_of!(epoll_event, events), 0);
        assert_eq!(core::mem::offset_of!(epoll_event, data), 8);
    }
}
