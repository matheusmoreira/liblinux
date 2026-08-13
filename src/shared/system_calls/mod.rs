//! The system calls present in every Linux kernel.
//!
//! One file per system call. Functions perform
//! their exact system call and nothing else.
//! Architecture modules glob import this,
//! add their own system calls on top,
//! and may or may not override symbols.

system_calls! {
    accept4,
    bind,
    close,
    connect,
    epoll_create1,
    epoll_ctl,
    epoll_pwait,
    exit_group,
    listen,
    read,
    socket,
    write,
}
