use crate::Errno;
use crate::definitions;

pub unsafe fn munmap(address: *mut u8, length: usize) -> Result<(), Errno> {
    // SAFETY: `__NR_munmap` is a two-argument system call.
    // Its arguments are an address and a scalar byte length.
    // The caller owns every memory map safety obligation.
    let result = unsafe {
        crate::system_call!(
            definitions::__NR_munmap,
            address as usize,
            length
        )
    };
    Errno::from_system_call(result).map(|_| ())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::definitions::{
        MAP_ANONYMOUS,
        MAP_FIXED_NOREPLACE,
        MAP_PRIVATE,
        PROT_READ,
        PROT_WRITE,
    };
    use crate::system_calls::mmap;

    #[test]
    fn removes_a_subrange_with_an_unaligned_length() {
        // 64 KiB is aligned to every supported architecture's page size.
        const ALIGNED_SIZE: usize = 64 * 1024;
        const MAPPING_SIZE: usize = ALIGNED_SIZE * 3;
        const UNMAP_LENGTH: usize = 1;

        // SAFETY: this creates a fresh private anonymous mapping without
        // replacing any existing address.
        let mapping = unsafe {
            mmap(
                core::ptr::null_mut(),
                MAPPING_SIZE,
                PROT_READ | PROT_WRITE,
                MAP_PRIVATE | MAP_ANONYMOUS,
                -1,
                0,
            )
        }
        .unwrap();

        let mapping_address = mapping as usize;
        let unmapped_address =
            (mapping_address + ALIGNED_SIZE) as *mut u8;
        let left_address = (unmapped_address as usize - 1) as *mut u8;
        let right_address =
            (unmapped_address as usize + ALIGNED_SIZE) as *mut u8;

        // SAFETY: all three pointers name writable bytes in the mapping.
        unsafe {
            left_address.write_volatile(0x11);
            unmapped_address.write_volatile(0x22);
            right_address.write_volatile(0x33);
        }

        // SAFETY: `unmapped_address` is page aligned and lies within the
        // live mapping. No pointer into the selected page is dereferenced
        // after this call.
        assert_eq!(
            unsafe { munmap(unmapped_address, UNMAP_LENGTH) },
            Ok(())
        );

        // SAFETY: `unmapped_address` is page aligned. MAP_FIXED_NOREPLACE
        // prevents this call from replacing any mapping that might exist.
        let replacement = unsafe {
            mmap(
                unmapped_address,
                UNMAP_LENGTH,
                PROT_READ | PROT_WRITE,
                MAP_PRIVATE | MAP_ANONYMOUS | MAP_FIXED_NOREPLACE,
                -1,
                0,
            )
        }
        .unwrap();

        // Exact non-replacing placement proves the selected page was removed.
        assert_eq!(replacement as usize, unmapped_address as usize);

        // SAFETY: `replacement` names a new zero-filled writable page.
        // The outer bytes remained mapped throughout the operation.
        unsafe {
            assert_eq!(replacement.read_volatile(), 0);
            assert_eq!(left_address.read_volatile(), 0x11);
            assert_eq!(right_address.read_volatile(), 0x33);
        }

        // SAFETY: the complete original range is mapped again. No pointer
        // into it is used after this call.
        assert_eq!(unsafe { munmap(mapping, MAPPING_SIZE) }, Ok(()));
    }

    #[test]
    fn rejects_an_unaligned_address() {
        const MAPPING_SIZE: usize = 1;

        // SAFETY: this creates a fresh private anonymous mapping without
        // replacing any existing address.
        let mapping = unsafe {
            mmap(
                core::ptr::null_mut(),
                MAPPING_SIZE,
                PROT_READ | PROT_WRITE,
                MAP_PRIVATE | MAP_ANONYMOUS,
                -1,
                0,
            )
        }
        .unwrap();

        let unaligned_address = (mapping as usize + 1) as *mut u8;

        // SAFETY: Linux rejects the unaligned address before changing the
        // live mapping.
        assert_eq!(
            unsafe { munmap(unaligned_address, MAPPING_SIZE) },
            Err(Errno::EINVAL)
        );

        // SAFETY: the failed call left the writable mapping intact.
        unsafe {
            mapping.write_volatile(0xa5);
            assert_eq!(mapping.read_volatile(), 0xa5);
        }

        // SAFETY: `mapping` is the start of the live mapping, and no
        // pointer into it is used after this call.
        assert_eq!(unsafe { munmap(mapping, MAPPING_SIZE) }, Ok(()));
    }

    #[test]
    fn rejects_an_empty_range() {
        // SAFETY: zero length prevents Linux from unmapping any address.
        assert_eq!(
            unsafe { munmap(core::ptr::null_mut(), 0) },
            Err(Errno::EINVAL)
        );
    }
}
