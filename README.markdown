# Liblinux

Liblinux is a library that provides architecture-independent access to Linux system calls.

## Building

The library and all examples are freestanding and have no dependencies.
They are built with GNU Make. Currently, only GCC is supported.
Issuing the command:

    make

Will produce the `liblinux.so` shared object and place it in the build
directory.

The `makefile` implements an out-of-tree multi-architecture build using
the [prefixed path method][make.prefixed-path], where all build targets
are prefixed with their build path. While this method results in an
organized build tree that mirrors the structure of the source tree,
it also makes it harder to rebuild individual build targets.
The main build interface consists of phony targets:

- `library`

  Builds the library.

- `examples`

  Builds all library usage examples.

- `all`

  Builds the library and its usage examples.

- `clean`

  Removes the build directory tree and all build artifacts.

- `directories`

  Creates the build directory tree.

## Why?

In 2014, the [`getrandom`][getrandom] system call was introduced.
It lets applications obtain random bits without using pathnames or file descriptors.
However, [it took over 2 years][long-road-to-getrandom()-in-glibc] for `glibc` support to arrive.
The kernel's random number subsystem maintainer [wrote in an email][email.theodore-ts'o]:

> _[...]_
> maybe the kernel developers should support a `libinux.a` library
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

It is not clear to me whether this `libinux-syscalls` library exists,
[even though nobody opposed to it][email.carlos-o'donell].
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
One maintainer replied to a [`clone`-related bug][clone(CLONE_VM)-fails]:

> If you use `clone()` you're on your own.

Another supplied more implementation details:

> _[...]_
> If you use any of the standard library,
> you risk the parent and child clobbering each other's internal states.
> You also have issues like the fact that `glibc` caches the `pid`/`tid` in userspace,
> and the fact that `glibc` expects to always have a valid thread pointer
> which your call to `clone` is unable to initialize correctly
> because it does not know (and should not know) the internal implementation of threads.

In [another bug][gettid()-should-have-a-wrapper], the following is said:

> _[...]_
> most of the problems with caching `pid`/`tid`
> come from use of `clone()` (or worse, `vfork`) directly by applications,
> which should probably not be a supported use.
> With TLS being a mandatory feature in modern `glibc`
> and the thread-pointer being always-initialized for purposes like `ssp`,
> I don't think there's any way applications can safely `clone`,
> whereby "safely"
> I mean "without a risk that internal `libc` state is inconsistent afterwards".

There's also an interesting comment about the different abstractions employed by the kernel and `glibc`:

> At the kernel level,
> there is really only one kind of kernel scheduling entity (KSE) --
> commonly called a "task" in Linux parlance.
> And that one kind of KSE is identified by one kind of data type.
> Creating an artificial distinction at the `glibc` level seems illogical and confusing.
> Furthermore, the `clone(2)` system call, which creates kernel "threads",
> returns a thread ID.
> But really, this is the same for processes:
> `clone()` is equally the creator of "processes".
> And of course,
> `glibc` itself already assumes that `TID`s and `PID`s are the same thing,
> since nowadays `glibc`'s `fork()` is a wrapper around `clone()`,
> and that wrapper assumes that `clone()` returns a `PID`.

The idea of a library for Linux system calls doesn't seem to be new.
[Comments about a `liblinux`][glibc-and-the-kernel-user-space-api]
seem to go as far back as 2012:

>> It makes no sense for every tool that wants to support
>> doing things with kernel modules to do the `syscall()` thing,
>> propagating potential errors in argument signatures
>> into more than one location
>> instead of getting it right in one canonical place, `libc`.
>
> Does that canonical place have to be `libc` though?
> Why not e.g. some `liblinux` which could live in the kernel source tree?

> _[...]_
> that these functions don't belong in `LibC`,
> but instead in some separate Linux-specific library or header file (a `"liblinux"`).

> _[...]_
> It would be nifty if the kernel came with a `"liblinux"`
> that implemented things like this,
> instead of the daunting (non-starter, really)
> prospect of upgrading to a new `glibc` just to get `syncfs`.

[Posts from as far back as 2004][kernel-headers-and-user-space] mention the idea:

> _[...]_ I hate even more the proposition that a user space program
> should not include a kernel header file.
> The C library is itself a user space program,
> so the rule that a user space program has to go through the C library
> obviously isn't logical.
> And the GNU C library is an optional tool,
> not an official part of the kernel interface
> (if it were the latter, I would expect to see it packaged with the kernel).

> _[...]_
> That Linux has never had an identifiable set of interface header files
> to declare its system call interface seems to me to be a major engineering weakness.

[make.prefixed-path]: http://make.mad-scientist.net/papers/multi-architecture-builds/#explicitpath

[getrandom]: http://man7.org/linux/man-pages/man2/getrandom.2.html
[long-road-to-getrandom()-in-glibc]: https://lwn.net/Articles/711013/
[email.theodore-ts'o]: https://lwn.net/Articles/711053/

[glibc-wrappers-for-(nearly-all)-linux-system-calls]: https://lwn.net/Articles/655028/
[email.roland-mcgrath]: https://lwn.net/Articles/655034/

[email.carlos-o'donell]: https://lwn.net/Articles/655039/
[consensus]: https://sourceware.org/glibc/wiki/Consensus#WIP:_Kernel_syscalls_wrappers

[clone(CLONE_VM)-fails]: https://sourceware.org/bugzilla/show_bug.cgi?id=10311

[gettid()-should-have-a-wrapper]: https://sourceware.org/bugzilla/show_bug.cgi?id=6399

[glibc-and-the-kernel-user-space-api]: https://lwn.net/Articles/534682/

[kernel-headers-and-user-space]: https://lwn.net/Articles/113349/
