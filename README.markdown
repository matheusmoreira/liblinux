# Liblinux

Liblinux is a library that provides architecture-independent access to Linux system calls.

## Why?

In 2014, the [getrandom] system call was introduced.
It lets applications obtain random bits without using pathnames or file descriptors.
However, [it took over 2 years][long-road-to-getrandom()-in-glibc] for `glibc` support to arrive.
The kernel's random number subsystem maintainer [wrote in an email][email.theodore-ts'o]:

> _[...]_
> maybe the kernel developers should support a libinux.a library
> that would allow us to bypass `glibc` when they are being non-helpful.

[Other system calls are also unsupported][glibc-wrappers-for-(nearly-all)-linux-system-calls].
Apparently, `glibc` does not see itself as a wrapper for Linux kernel functionality.
One of the proposed solutions was to
[put them in a separate library][email.roland-mcgrath]:

> We could provide OS-specific ABIs in an OS-specific shared library,
> e.g. `libinux-syscalls.so.N`.
> _[...]_
> If this
> library contains nothing but syscall wrappers or equivalently trivial
> code (importantly, stateless code that doesn't need any data objects
> of permanent extent), then it won't be a practical problem to have
> multiple versions of the library loaded in the same process at the
> same time--so all the usual issues that make changing SONAMEs very
> hard don't really apply.
>
> My preference would be that we not put such OS-specific ABIs into the
> common link-time API either.  That is, programs would be required to
> link explicitly with `-linux-syscalls`.
> _[...]_

It is not clear to me whether this `libinux-syscalls` library exists, [even though nobody opposed to it][email.carlos-o'donell].
I could not find traces of it in a `glibc` repository.
[The documented consensus regarding Linux-specific system calls][consensus]
makes no mention of any library.
However, one bullet point is quite interesting:

> If a syscall cannot meaningfully be used behind `glibc`'s back,
> or is not useful in the `glibc` context
> except for in the ways in which it is used by `glibc`,
> but can only be used directly by `glibc`,
> there is no need to add a wrapper to `glibc`
> (this probably applies to `set_thread_area`, for example).

Due to global and thread-local state maintained by `glibc`,
it also applies to fundamental system calls such as `clone`.
One maintainer replied to a [`clone`-related bug][bug]:

> If you use `clone()` you're on your own.

Another supplied more implementation details:

> _[...]_
> If you use any of the standard library,
> you risk the parent and child clobbering each other's internal states.
> You also have issues like the fact that `glibc` caches the `pid`/`tid` in userspace,
> and the fact that `glibc` expects to always have a valid thread pointer
> which your call to `clone` is unable to initialize correctly
> because it does not know (and should not know) the internal implementation of threads.

[getrandom]: http://man7.org/linux/man-pages/man2/getrandom.2.html
[long-road-to-getrandom()-in-glibc]: https://lwn.net/Articles/711013/
[email.theodore-ts'o]: https://lwn.net/Articles/711053/

[glibc-wrappers-for-(nearly-all)-linux-system-calls]: https://lwn.net/Articles/655028/
[email.roland-mcgrath]: https://lwn.net/Articles/655034/

[email.carlos-o'donell]: https://lwn.net/Articles/655039/
[consensus]: https://sourceware.org/glibc/wiki/Consensus#WIP:_Kernel_syscalls_wrappers

[bug]: https://sourceware.org/bugzilla/show_bug.cgi?id=10311
