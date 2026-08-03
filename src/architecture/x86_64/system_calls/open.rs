use crate::Errno;
use crate::FileDescriptor as Fd;
use crate::architecture::x86_64::{definitions, system_call_3};
use core::ffi::c_char;

/// Open the `path` and return its file descriptor.
///
/// On success, the caller owns the returned
/// file descriptor and must eventually close it.
///
/// The original `open` system call.
/// Not all architectures have it.
/// `openat` is the modern version.
///
/// # Safety
///
/// Rust safety and semantics do not matter to the kernel.
/// However, the kernel semantics matter a lot to Rust.
///
/// The kernel may read bytes beginning at `path`
/// in order to resolve the location named by it.
/// `path` is not required to be dereferenceable
/// by Rust. Unmapped user space addresses may
/// cause Linux to return `EFAULT`.
///
/// The caller must ensure that any memory access
/// Linux successfully performs through `path`
/// does not violate Rust's memory model or
/// any other invariants of any live values.
pub unsafe fn open(
    path: *const c_char,
    flags: i32,
    mode: u32,
) -> Result<Fd, Errno> {
    // SAFETY: `__NR_open` is a three argument system call on x86_64.
    // `path` is passed to Linux as a user space address, and the caller
    // accepts it will be read by Linux as described by this function's
    // safety contract. Linux will handle invalid inputs in its own way,
    // likely by returning `EFAULT`.
    let result = unsafe {
        system_call_3(
            definitions::__NR_open,
            path as usize,
            flags as usize,
            mode as usize,
        )
    };
    Errno::from_system_call(result).map(|descriptor| descriptor as Fd)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn opening_an_empty_path_reports_no_such_file() {
        assert_eq!(
            // SAFETY: Rust C string literals construct
            // valid NUL-terminated paths that are readable
            // for the duration of the open system call.
            unsafe { open(c"".as_ptr(), definitions::O_RDONLY, 0) },
            Err(Errno::ENOENT)
        );
    }

    #[test]
    fn opening_a_directory_for_writing_reports_is_a_directory() {
        assert_eq!(
            // SAFETY: Rust C string literals construct
            // valid NUL-terminated paths that are readable
            // for the duration of the open system call.
            unsafe { open(c"/".as_ptr(), definitions::O_WRONLY, 0) },
            Err(Errno::EISDIR)
        );
    }
}
