use crate::Errno;
use crate::definitions;

/// Terminate the caller's thread group with the specified `status`.
///
/// This system call requests that Linux terminate every thread
/// in the caller's thread group, including the caller. Thread
/// group membership alone determines which tasks are terminated
/// by this system call. Resource sharing has no bearing whatsoever:
/// tasks in other thread groups will _normally_ remain alive even
/// if they share virtual memory or file descriptor tables with
/// the caller.
///
/// Should the last thread of a PID namespace's child reaper exit,
/// however, Linux will send `SIGKILL` to all remaining tasks in
/// that PID namespace regardless of thread group membership.
/// So it is actually possible for this system call to end up
/// terminating threads outside of the caller's thread group.
/// In that case, no new processes can be created in that PID
/// namespace, and attempts fail with `-ENOMEM`. If the initial
/// PID namespace's child reaper exits, Linux panics.
///
/// When this call initiates the thread group exit, the low eight
/// bits of `status` become the thread group's exit status. Things
/// other than this system call can initiate thread group exits,
/// fatal signals being the prime example. If they race to start
/// the thread group exit, the first one determines the result
/// that a parent receives when it waits for the thread group.
/// It is therefore possible for the `status` passed to this
/// function to be completely ignored. Races are also possible
/// when a thread attempts to `exec`. If the kernel is already
/// in the middle of replacing the process image for `exec`,
/// this system call does not initiate thread group exit,
/// only the calling thread exits and `status` is zeroed.
/// If the exit is already in progress, then it is `exec`
/// that fails with `-EAGAIN`, even though the thread group
/// is exiting and it would be impossible to try again.
///
/// If Linux executes the request, this function does not return.
/// Linux might not actually execute the request: seccomp or tracers
/// can intercept the system call and force it to return something
/// instead of terminating the thread group. This function therefore
/// decodes its result normally instead of being declared divergent.
pub fn exit_group(status: i32) -> Result<(), Errno> {
    // SAFETY: `__NR_exit_group` is a one argument system call
    // that takes an integer status as its sole argument
    // and does not dereference any pointers.
    let result = unsafe {
        crate::system_call!(definitions::__NR_exit_group, status as usize)
    };
    Errno::from_system_call(result).map(|_| ())
}
