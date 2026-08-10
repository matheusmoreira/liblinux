use crate::ClockID;
use crate::Errno;
use crate::definitions;
use crate::definitions::__kernel_timespec;

pub unsafe fn clock_gettime(
    clock: ClockID,
    time: *mut __kernel_timespec,
) -> Result<(), Errno> {
    // SAFETY: `__NR_clock_gettime` is a two argument system call
    // that takes a clock identifier and a user space pointer.
    // Linux may write a complete or partial time value through
    // that pointer after successfully reading the clock.
    let result = unsafe {
        crate::system_call!(
            definitions::__NR_clock_gettime,
            clock as usize,
            time as usize
        )
    };

    Errno::from_system_call(result).map(|_| ())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::definitions::CLOCK_MONOTONIC;

    #[test]
    fn kernel_timespec_has_the_native_64_bit_layout() {
        assert_eq!(core::mem::size_of::<__kernel_timespec>(), 16);
        assert_eq!(core::mem::align_of::<__kernel_timespec>(), 8);
        assert_eq!(core::mem::offset_of!(__kernel_timespec, tv_sec), 0);
        assert_eq!(core::mem::offset_of!(__kernel_timespec, tv_nsec), 8);
    }

    #[test]
    fn monotonic_time_is_normalized_and_does_not_go_backwards() {
        let mut before = __kernel_timespec {
            tv_sec: -1,
            tv_nsec: -1,
        };
        let mut after = __kernel_timespec {
            tv_sec: -1,
            tv_nsec: -1,
        };

        // SAFETY: Both pointers refer values that are writable
        // for the duration of their respective system calls.
        assert_eq!(
            unsafe { clock_gettime(CLOCK_MONOTONIC, &mut before) },
            Ok(())
        );
        assert_eq!(
            unsafe { clock_gettime(CLOCK_MONOTONIC, &mut after) },
            Ok(())
        );

        assert!(before.tv_sec >= 0);
        assert!((0..1_000_000_000).contains(&before.tv_nsec));
        assert!(after.tv_sec >= 0);
        assert!((0..1_000_000_000).contains(&after.tv_nsec));
        assert!(
            (after.tv_sec, after.tv_nsec)
                >= (before.tv_sec, before.tv_nsec)
        );
    }

    #[test]
    fn malformed_clock_is_rejected_before_output_pointer() {
        // SAFETY: The clock identifier is deliberately malformed
        // and the output pointer is deliberately inaccessible.
        // Linux validates the clock before attempting to write
        // through the output pointer.
        assert_eq!(
            unsafe { clock_gettime(-1, core::ptr::null_mut()) },
            Err(Errno::EINVAL)
        );
    }

    #[test]
    fn inaccessible_output_reports_efault() {
        // SAFETY: A null pointer is deliberately passed
        // to exercise Linux's `EFAULT` behavior.
        // It is not dereferenced from Rust.
        assert_eq!(
            unsafe {
                clock_gettime(CLOCK_MONOTONIC, core::ptr::null_mut())
            },
            Err(Errno::EFAULT)
        );
    }
}
