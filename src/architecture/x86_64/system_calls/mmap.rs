use crate::Errno;
use crate::FileDescriptor as Fd;
use crate::definitions;

pub unsafe fn mmap(
    address: *mut u8,
    length: usize,
    protection: usize,
    flags: usize,
    descriptor: Fd,
    offset: usize,
) -> Result<*mut u8, Errno> {
    // SAFETY: `__NR_mmap` is a six argument system call.
    // Its arguments match the native LP64 binary interface.
    // The caller owns the memory map safety obligations.
    let result = unsafe {
        crate::system_call!(
            definitions::__NR_mmap,
            address as usize,
            length,
            protection,
            flags,
            descriptor as usize,
            offset
        )
    };
    Errno::from_system_call(result).map(|address| address as *mut u8)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::definitions::{
        MAP_ANONYMOUS,
        MAP_PRIVATE,
        PROT_READ,
        PROT_WRITE,
    };

    #[test]
    fn maps_writable_anonymous_memory() {
        const MAPPING_SIZE: usize = 1;

        // SAFETY: This requests a new private anonymous mapping
        // without replacing an existing address. Linux rounds up
        // the non-zero length to the next page boundary. The test
        // accesses only the first byte. Linux destroys the mapping
        // when the process exits.
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

        // SAFETY: `mapping` names a writable mapping containing
        // at least one byte. Volatile accesses ensure the test
        // actually touches the page.
        unsafe {
            mapping.write_volatile(0x5a);
            assert_eq!(mapping.read_volatile(), 0x5a);
        }
    }

    #[test]
    fn rejects_an_empty_mapping() {
        // SAFETY: No mapping can be created for zero length,
        // and the null hint does not designate existing storage.
        assert_eq!(
            unsafe {
                mmap(
                    core::ptr::null_mut(),
                    0,
                    PROT_READ,
                    MAP_PRIVATE | MAP_ANONYMOUS,
                    -1,
                    0,
                )
            },
            Err(Errno::EINVAL)
        );
    }

    #[test]
    fn rejects_an_unaligned_offset() {
        // SAFETY: The null address is only a placement hint.
        // Linux rejects the unaligned byte offset before it
        // can create a mapping.
        assert_eq!(
            unsafe {
                mmap(
                    core::ptr::null_mut(),
                    1,
                    PROT_READ,
                    MAP_PRIVATE | MAP_ANONYMOUS,
                    -1,
                    1,
                )
            },
            Err(Errno::EINVAL)
        );
    }
}
