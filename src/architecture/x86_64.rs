//! x86_64 Linux system call interface.

/// x86_64 Linux kernel definitions.
/// Symbols match those in UAPI headers.
pub mod definitions {
    #![allow(non_upper_case_globals)]

    pub const __NR_read: usize = 0;
    pub const __NR_write: usize = 1;
    pub const __NR_open: usize = 2;
    pub const __NR_close: usize = 3;
    pub const __NR_mmap: usize = 9;
    pub const __NR_munmap: usize = 11;
    pub const __NR_access: usize = 21;
    pub const __NR_mremap: usize = 25;
    pub const __NR_dup: usize = 32;
    pub const __NR_getpid: usize = 39;
    pub const __NR_setsockopt: usize = 54;
    pub const __NR_faccessat: usize = 269;
    pub const __NR_prlimit64: usize = 302;
    pub const __NR_renameat2: usize = 316;
    pub const __NR_faccessat2: usize = 439;
}

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
