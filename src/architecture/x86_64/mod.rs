//! x86_64 Linux system call interface.

/// x86_64 Linux kernel definitions.
/// Symbols match those in UAPI headers.
///
/// Linux system call numbers are architecture specific.
/// Definitions that use the default values are re-exported
/// from the shared module. Architecture specific values
/// are defined here instead.
pub mod definitions {
    #![allow(non_upper_case_globals)]

    /// Make native automatic placement
    /// use the low 2 GiB address range.
    ///
    /// Linux >= 2.5.5
    pub const MAP_32BIT: usize = 0x40;

    /// Make native automatic top-down
    /// placement begin at 4 GiB.
    ///
    /// Linux >= 6.6
    pub const MAP_ABOVE4G: usize = 0x80;

    /// System call number for `read`.
    /// Linux >= 2.5.5
    pub const __NR_read: usize = 0;
    /// System call number for `write`.
    /// Linux >= 2.5.5
    pub const __NR_write: usize = 1;
    /// System call number for `open`.
    /// Linux >= 2.5.5
    pub const __NR_open: usize = 2;
    /// System call number for `close`.
    /// Linux >= 2.5.5
    pub const __NR_close: usize = 3;
    /// System call number for `mmap`.
    /// Linux >= 2.5.5
    pub const __NR_mmap: usize = 9;
    /// System call number for `munmap`.
    /// Linux >= 2.5.5
    pub const __NR_munmap: usize = 11;
    /// System call number for `access`.
    /// Linux >= 2.5.5
    pub const __NR_access: usize = 21;
    /// System call number for `mremap`.
    /// Linux >= 2.5.5
    pub const __NR_mremap: usize = 25;
    /// System call number for `dup`.
    /// Linux >= 2.5.5
    pub const __NR_dup: usize = 32;
    /// System call number for `getpid`.
    /// Linux >= 2.5.5
    pub const __NR_getpid: usize = 39;
    /// System call number for `socket`.
    /// Linux >= 2.5.5
    pub const __NR_socket: usize = 41;
    /// System call number for `connect`.
    /// Linux >= 2.5.5
    pub const __NR_connect: usize = 42;
    /// System call number for `sendto`.
    /// Linux >= 2.5.5
    pub const __NR_sendto: usize = 44;
    /// System call number for `recvfrom`.
    /// Linux >= 2.5.5
    pub const __NR_recvfrom: usize = 45;
    /// System call number for `shutdown`.
    /// Linux >= 2.5.5
    pub const __NR_shutdown: usize = 48;
    /// System call number for `bind`.
    /// Linux >= 2.5.5
    pub const __NR_bind: usize = 49;
    /// System call number for `listen`.
    /// Linux >= 2.5.5
    pub const __NR_listen: usize = 50;
    /// System call number for `setsockopt`.
    /// Linux >= 2.5.5
    pub const __NR_setsockopt: usize = 54;
    /// System call number for `getsockopt`.
    /// Linux >= 2.5.5
    pub const __NR_getsockopt: usize = 55;
    /// System call number for `clock_gettime`.
    /// Linux >= 2.5.63
    pub const __NR_clock_gettime: usize = 228;
    /// System call number for `exit_group`.
    /// Linux >= 2.5.67
    pub const __NR_exit_group: usize = 231;
    /// System call number for `epoll_ctl`.
    ///
    /// x86_64 keeps the original slot 214 as `epoll_ctl_old`.
    /// The current `epoll_ctl` interface uses number 233.
    ///
    /// Linux >= 2.5.74
    pub const __NR_epoll_ctl: usize = 233;
    /// System call number for `faccessat`.
    /// Linux >= 2.6.16
    pub const __NR_faccessat: usize = 269;
    /// System call number for `epoll_pwait`.
    /// Linux >= 2.6.22
    pub const __NR_epoll_pwait: usize = 281;
    /// System call number for `accept4`.
    /// Linux >= 2.6.28
    pub const __NR_accept4: usize = 288;
    /// System call number for `epoll_create1`.
    /// Linux >= 2.6.27
    pub const __NR_epoll_create1: usize = 291;
    /// System call number for `prlimit64`.
    /// Linux >= 2.6.36
    pub const __NR_prlimit64: usize = 302;
    /// System call number for `renameat2`.
    /// Linux >= 3.15
    pub const __NR_renameat2: usize = 316;
    /// System call number for `faccessat2`.
    /// Linux >= 5.8
    pub const __NR_faccessat2: usize = 439;
    /// System call number for `epoll_pwait2`.
    /// Linux >= 5.11
    pub const __NR_epoll_pwait2: usize = 441;

    /// An event exchanged with an epoll instance.
    ///
    /// Linux packs this structure to make the 64-bit layout
    /// match the 32-bit layout, simplifying 32-bit emulation.
    #[repr(C, packed)]
    pub struct epoll_event {
        /// Events that occurred or are being requested.
        pub events: u32,

        /// Caller-owned data returned with the event.
        pub data: u64,
    }

    pub use crate::shared::definitions::*;
}

/// The x86_64 Linux system calls.
/// All shared system calls,
/// plus the x86_64 exclusives.
#[cfg(all(target_arch = "x86_64", target_pointer_width = "64"))]
pub mod system_calls;

/// Perform a x86_64 Linux system call with 0 arguments.
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
#[cfg(all(target_arch = "x86_64", target_pointer_width = "64"))]
pub unsafe fn system_call_0(number: usize) -> isize {
    let result: isize;

    unsafe {
        core::arch::asm!(
            "syscall",
            inlateout("rax") number => result,    // system call number, return value
            out("rcx") _,                         // saved return address
            out("r11") _,                         // saved flags
            options(nostack),                     // deliberately omitted: nomem, preserves flags
        );
    }

    result
}

/// Perform a x86_64 Linux system call with 1 argument.
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
#[cfg(all(target_arch = "x86_64", target_pointer_width = "64"))]
pub unsafe fn system_call_1(number: usize, argument_1: usize) -> isize {
    let result: isize;

    unsafe {
        core::arch::asm!(
            "syscall",
            inlateout("rax") number => result,    // system call number, return value
            in("rdi") argument_1,                 // system call argument 1
            out("rcx") _,                         // saved return address
            out("r11") _,                         // saved flags
            options(nostack),                     // deliberately omitted: nomem, preserves flags
        );
    }

    result
}

/// Perform a x86_64 Linux system call with 2 arguments.
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
#[cfg(all(target_arch = "x86_64", target_pointer_width = "64"))]
pub unsafe fn system_call_2(number: usize, argument_1: usize, argument_2: usize) -> isize {
    let result: isize;

    unsafe {
        core::arch::asm!(
            "syscall",
            inlateout("rax") number => result,    // system call number, return value
            in("rdi") argument_1,                 // system call argument 1
            in("rsi") argument_2,                 // system call argument 2
            out("rcx") _,                         // saved return address
            out("r11") _,                         // saved flags
            options(nostack),                     // deliberately omitted: nomem, preserves flags
        );
    }

    result
}

/// Perform a x86_64 Linux system call with 3 arguments.
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
#[cfg(all(target_arch = "x86_64", target_pointer_width = "64"))]
pub unsafe fn system_call_3(
    number: usize,
    argument_1: usize,
    argument_2: usize,
    argument_3: usize,
) -> isize {
    let result: isize;

    unsafe {
        core::arch::asm!(
            "syscall",
            inlateout("rax") number => result,    // system call number, return value
            in("rdi") argument_1,                 // system call argument 1
            in("rsi") argument_2,                 // system call argument 2
            in("rdx") argument_3,                 // system call argument 3
            out("rcx") _,                         // saved return address
            out("r11") _,                         // saved flags
            options(nostack),                     // deliberately omitted: nomem, preserves flags
        );
    }

    result
}

/// Perform a x86_64 Linux system call with 4 arguments.
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
#[cfg(all(target_arch = "x86_64", target_pointer_width = "64"))]
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
            "syscall",
            inlateout("rax") number => result,    // system call number, return value
            in("rdi") argument_1,                 // system call argument 1
            in("rsi") argument_2,                 // system call argument 2
            in("rdx") argument_3,                 // system call argument 3
            in("r10") argument_4,                 // system call argument 4
            out("rcx") _,                         // saved return address
            out("r11") _,                         // saved flags
            options(nostack),                     // deliberately omitted: nomem, preserves flags
        );
    }

    result
}

/// Perform a x86_64 Linux system call with 5 arguments.
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
#[cfg(all(target_arch = "x86_64", target_pointer_width = "64"))]
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
            "syscall",
            inlateout("rax") number => result,    // system call number, return value
            in("rdi") argument_1,                 // system call argument 1
            in("rsi") argument_2,                 // system call argument 2
            in("rdx") argument_3,                 // system call argument 3
            in("r10") argument_4,                 // system call argument 4
            in("r8")  argument_5,                 // system call argument 5
            out("rcx") _,                         // saved return address
            out("r11") _,                         // saved flags
            options(nostack),                     // deliberately omitted: nomem, preserves flags
        );
    }

    result
}

/// Perform a x86_64 Linux system call with 6 arguments.
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
#[cfg(all(target_arch = "x86_64", target_pointer_width = "64"))]
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
            "syscall",
            inlateout("rax") number => result,    // system call number, return value
            in("rdi") argument_1,                 // system call argument 1
            in("rsi") argument_2,                 // system call argument 2
            in("rdx") argument_3,                 // system call argument 3
            in("r10") argument_4,                 // system call argument 4
            in("r8")  argument_5,                 // system call argument 5
            in("r9")  argument_6,                 // system call argument 6
            out("rcx") _,                         // saved return address
            out("r11") _,                         // saved flags
            options(nostack),                     // deliberately omitted: nomem, preserves flags
        );
    }

    result
}

#[cfg(all(test, target_arch = "x86_64", target_pointer_width = "64",))]
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
    fn access_reports_that_root_exists() {
        const F_OK: usize = 0;

        // File system root always exists
        let path = c"/";
        let result =
            unsafe { system_call_2(definitions::__NR_access, path.as_ptr() as usize, F_OK) };

        assert_eq!(result, 0);
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

        // the only check that a failure comes back as a negative number
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
        const PROT_READ_WRITE: usize =
            definitions::PROT_READ | definitions::PROT_WRITE;
        const MAP_PRIVATE_ANONYMOUS: usize =
            definitions::MAP_PRIVATE | definitions::MAP_ANONYMOUS;
        const MREMAP_MAYMOVE: usize = 0x1;
        const MREMAP_FIXED: usize = 0x2;
        const PAGE: usize = 4096;

        let no_fd: usize = usize::MAX;
        let offset: usize = 0;

        // one page to move, and a two page window to move it into
        let mapped = unsafe {
            system_call_6(
                definitions::__NR_mmap,
                0,
                PAGE,
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
                PAGE * 2,
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
                PAGE,
                PAGE * 2,
                MREMAP_MAYMOVE | MREMAP_FIXED,
                window as usize,
            )
        };

        assert_eq!(remapped, window);

        let freed = unsafe { system_call_2(definitions::__NR_munmap, remapped as usize, PAGE * 2) };

        assert_eq!(freed, 0);
    }

    #[test]
    fn epoll_event_has_the_x86_64_layout() {
        assert_eq!(core::mem::size_of::<epoll_event>(), 12);
        assert_eq!(core::mem::align_of::<epoll_event>(), 1);
        assert_eq!(core::mem::offset_of!(epoll_event, events), 0);
        assert_eq!(core::mem::offset_of!(epoll_event, data), 4);
    }
}
