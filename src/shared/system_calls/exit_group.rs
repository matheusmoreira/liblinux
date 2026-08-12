use crate::Errno;
use crate::definitions;

pub fn exit_group(status: i32) -> Result<(), Errno> {
    // SAFETY: `__NR_exit_group` is a one argument system call
    // that takes an integer status as its sole argument
    // and does not dereference any pointers.
    let result = unsafe {
        crate::system_call!(definitions::__NR_exit_group, status as usize)
    };
    Errno::from_system_call(result).map(|_| ())
}
