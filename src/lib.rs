#![cfg(target_os = "linux")]
#![cfg_attr(not(test), no_std)]
#![warn(missing_docs)]
#![deny(rustdoc::broken_intra_doc_links)]
#![deny(rustdoc::private_intra_doc_links)]

//! liblinux - freestanding Linux system calls for Rust
//!
//! Definitions, types and system call functions
//! for all supported architectures. Matches the
//! Linux UAPI headers.
//!
//! The crate is normally freestanding and compiled with no_std.
//! The standard library is linked only in test configuration.

/// Computes paths relative to liblinux's Cargo manifest directory.
macro_rules! manifest_path {
    ($($part:expr),+ $(,)?) => {
        concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/",
            $($part),+
        )
    };
}

/// Computes the path to the specified system call documentation file.
macro_rules! system_call_documentation_path {
    ($name:ident) => {
        manifest_path!("documentation/system-calls/", stringify!($name), ".md")
    };

    ($name:ident, $architecture:ident) => {
        manifest_path!(
            "documentation/system-calls/",
            stringify!($name),
            ".",
            stringify!($architecture),
            ".md"
        )
    };
}

/// Declare one module per system call and re-export the system call.
/// The module remains private. This enables one system call per file
/// while maintaining ergonomics: `system_calls::open` rather than
/// `system_calls::open::open`. Automatically attaches documentation.
macro_rules! system_calls {
    (
        $($name:ident),* $(,)?
    ) => {
        $(
            // Public syscall documentation is attached
            // via the re-exported function below.
            #[allow(missing_docs)]
            #[allow(clippy::missing_safety_doc)]
            mod $name;

            #[doc = include_str!(system_call_documentation_path!($name))]
            pub use $name::$name;
        )*
    };

    (
        $architecture:ident {
            $($name:ident),* $(,)?
        }
    ) => {
        $(
            // Public syscall documentation is attached
            // via the re-exported function below.
            #[allow(missing_docs)]
            #[allow(clippy::missing_safety_doc)]
            mod $name;

            #[doc = include_str!(system_call_documentation_path!($name))]
            #[doc = include_str!(system_call_documentation_path!($name, $architecture))]
            pub use $name::$name;
        )*
    };
}

pub mod architecture;
pub mod shared;

// The errno type is surfaced at the crate root as linux::Errno
mod errno;
pub use errno::Errno;

/// Linux clock identifier.
///
/// Clock identifiers are signed 32-bit integers.
/// Linux clock identifiers are an open namespace.
/// This type does not encode validity, ownership or lifetime.
pub type ClockID = i32;

/// Linux file descriptor.
///
/// File descriptors fall within the range: [0, INT_MAX].
/// This type does not encode validity, ownership or openness.
pub type FileDescriptor = i32;

// The definitions of the configured target architecture's Linux kernel
// are re-exported as linux::definitions::* for convenience and readability.
// Each architecture module re-exports compatible shared defaults and provides
// local replacements for definitions whose values differ for that architecture.
// Symbol names match those of the Linux UAPI headers.
pub use architecture::target::definitions;

// The system calls of the configured target architecture's Linux kernel
// are re-exported as linux::system_calls::* for convenience and readability.
// Definitions are sourced from architecture specific modules which either
// re-export shared definitions or override them for just that architecture.
// Symbol names match Linux system call definitions exactly.
pub use architecture::target::system_calls;

/// Perform a Linux system call for the target architecture
/// with up to six arguments.
///
/// `system_call!(number)` calls `system_call_0(number),
/// `system_call!(number, x)` calls `system_call_1(number, x)`,
/// and so on, all the way up to six arguments.
///
/// # Safety
///
/// System calls can do anything the kernel allows.
/// They may end the process or read and write memory
/// through pointer arguments. This macro can perform
/// arbitrary system calls and is therefore inherently
/// unsafe.
///
/// The caller must ensure `number` refers
/// to a system call that takes the same
/// number of arguments passed to this macro.
#[macro_export]
macro_rules! system_call {
    ($number:expr) => {
        $crate::architecture::target::system_call_0($number)
    };
    ($number:expr, $argument_1:expr) => {
        $crate::architecture::target::system_call_1($number, $argument_1)
    };
    ($number:expr, $argument_1:expr, $argument_2:expr) => {
        $crate::architecture::target::system_call_2($number, $argument_1, $argument_2)
    };
    ($number:expr, $argument_1:expr, $argument_2:expr, $argument_3:expr) => {
        $crate::architecture::target::system_call_3($number, $argument_1, $argument_2, $argument_3)
    };
    ($number:expr, $argument_1:expr, $argument_2:expr, $argument_3:expr, $argument_4:expr) => {
        $crate::architecture::target::system_call_4(
            $number,
            $argument_1,
            $argument_2,
            $argument_3,
            $argument_4,
        )
    };
    ($number:expr, $argument_1:expr, $argument_2:expr, $argument_3:expr, $argument_4:expr, $argument_5:expr) => {
        $crate::architecture::target::system_call_5(
            $number,
            $argument_1,
            $argument_2,
            $argument_3,
            $argument_4,
            $argument_5,
        )
    };
    ($number:expr, $argument_1:expr, $argument_2:expr, $argument_3:expr, $argument_4:expr, $argument_5:expr, $argument_6:expr) => {
        $crate::architecture::target::system_call_6(
            $number,
            $argument_1,
            $argument_2,
            $argument_3,
            $argument_4,
            $argument_5,
            $argument_6,
        )
    };
}

#[cfg(test)]
mod tests {
    use crate::definitions;

    #[test]
    fn the_macro_selects_correct_primitive() {
        let pid = unsafe { system_call!(definitions::__NR_getpid) };
        assert_eq!(pid, std::process::id() as isize);

        const F_OK: usize = 0;
        const AT_FDCWD: isize = -100;
        let path = c".";
        let result = unsafe {
            system_call!(
                definitions::__NR_faccessat,
                AT_FDCWD as usize,
                path.as_ptr() as usize,
                F_OK
            )
        };
        assert_eq!(result, 0);
    }
}
