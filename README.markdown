# Liblinux

Liblinux is a C library that provides architecture-independent access to
Linux system calls.

## Building

The library and all examples are freestanding and have no dependencies.
They are built with GNU Make.
Running `make` without arguments will create both static and shared libraries.
Optional features available:

- Process startup code

  Code that serves as the process entry point and performs initialization before
  passing control to application code.
  The current implementation facilitates access to the argument,
  environment and auxiliary vectors.
  GCC calls these objects `startfiles`.

- `liblinux.specs` file for GCC

  This file tells GCC to use liblinux `startfiles`.

- `liblinux-gcc` wrapper script

  This script makes GCC use the `liblinux.specs` file.
  It was inspired by and works just like [`musl-gcc`][musl-gcc].

- Example code

  Examples that demonstrate use of many system calls are provided.
  They are compiled with `liblinux-gcc` and can use both the static and dynamic
  liblinux.

Currently, the only supported architecture and compiler is `x86_64` and `GCC`,
respectively.

The `makefile` implements an out-of-tree multi-architecture build using
the [prefixed path method][make.prefixed-path], where all build targets
are prefixed with their build path. While this method results in an
organized build tree that mirrors the structure of the source tree,
it also makes it harder to rebuild individual build targets.
The main build interface consists of phony targets:

- `static-library`

  Builds a static library.

- `dynamic-library`

  Builds a dynamic library.

- `libraries`

  Builds static and dynamic libraries.

- `startfiles`

  Builds the process startup object files.

- `static-examples`

  Builds statically linked executables of all library usage examples.

- `dynamic-examples`

  Builds dynamically linked executables of all library usage examples.

- `examples`

  Builds both statically and dynamically linked versions of all examples.

- `all`

  Builds the `libraries`, `startfiles` and `examples`.

- `clean`

  Removes the build directory tree and all build artifacts.

- `directories`

  Creates the build directory tree.

- `run-hello-world`

  Builds and runs the `hello-world` example.
  Similar rules are automatically generated for every example.
  By default, examples are statically linked.

- `run-static-hello-world`

  Builds and runs the statically linked `hello-world` example.
  Similar rules are automatically generated for every example.

- `run-dynamic-hello-world`

  Builds and runs the dynamically linked `hello-world` example.
  Similar rules are automatically generated for every example.

- `checkpatch`

  Runs the Linux kernel's `checkpatch.pl` script on the liblinux source code.

- `system-calls.available`

  Computes a list of available system calls on the system.

- `system-calls.implemented`

  Computes a list of system calls currently implemented by liblinux.

- `system-calls.missing`

  Computes a list of available system calls that are not present in liblinux.

## Project structure

    liblinux
    ├── examples
    ├── include
    │   └── liblinux
    │       └── system_calls
    ├── make
    │   └── compilers
    ├── scripts
    │   └── linux
    ├── source
    │   ├── arch
    │   └── system_calls
    └── start

### `examples`

Library usage examples.
Each `.c` file represents one example.
Make automatically discovers all examples.
All examples can be built with `make examples`.
Specific examples can be compiled and executed with `make run-$example`.

### `include`

Library header files.
Added to the compiler's search directories list.

### `include/liblinux`

All headers are inside the `liblinux` directory.
Prevents name clashes since headers are meant to be copied to `/usr/include`
as part of the installation process.

### `include/liblinux/system_calls`

All system call prototypes are declared here.
One header per function.
Each header includes the Linux user space headers required by the function.

### `make`

Makefiles that are included by the top-level `makefile` in a specific order.
They are named according to the purpose of their definitions.

### `make/compilers`

Makefiles that define compiler-specific variables and macros.
They are named after the compiler they provide support for.

### `scripts`

Scripts invoked during the build process.
They generate a GCC wrapper that uses liblinux's startup files when compiling.

### `scripts/linux`

Linux kernel scripts integrated with the build process.
They check source code for compliance with the
[Linux kernel coding style][coding-style].
`make checkpatch` will automatically download `checkpatch.pl`
and run it on the liblinux source code.

### `source`

Library source code.
Make automatically discovers all `.c` files and includes them in the build.

### `source/arch`

Architecture-specific source code.
Contains the implementation of the `system_call` function,
which is used by all system call wrapper functions.
Make will select which architecture directory to include in the build
depending on the target platform.

### `source/system_calls`

System call wrapper function definitions.
One translation unit per function.

### `start`

Architecture-specific program startup code.
Provides the `_start` symbol, which is the default ELF entry point.
This code ultimately calls the program's `start` function.

## Why?

In 2014, the [`getrandom`][getrandom] system call was introduced.
It lets applications obtain random bits
without using pathnames or file descriptors.
However, [it took over 2 years][long-road-to-getrandom()-in-glibc]
for `glibc` support to arrive.
The kernel's random number subsystem maintainer
[wrote in an email][email.theodore-ts'o]:

> _[...]_
> maybe the kernel developers should support a `libinux.a` library
> that would allow us to bypass `glibc` when they are being non-helpful.

Other system calls
[are also unsupported][glibc-wrappers-for-linux-system-calls].
Apparently, `glibc` does not see itself
as a wrapper for Linux kernel functionality.
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
[The documented consensus][consensus]
regarding Linux-specific system calls makes no mention of any library.
However, one bullet point is quite interesting:

> If a syscall cannot meaningfully be used behind `glibc`'s back,
> or is not useful in the `glibc` context
> except for in the ways in which it is used by `glibc`,
> but can only be used directly by `glibc`,
> there is no need to add a wrapper to `glibc`
> (this probably applies to `set_thread_area`, for example).

Due to global and thread-local state maintained by `glibc`,
it also applies to fundamental system calls such as `clone`.
A maintainer replied to a [`clone`-related bug][clone(CLONE_VM)-fails]:

> If you use `clone()` you're on your own.

Another supplied more implementation details:

> _[...]_
> If you use any of the standard library,
> you risk the parent and child clobbering each other's internal states.
> You also have issues like the fact that `glibc` caches the `pid`/`tid`
> in userspace, and the fact that `glibc` expects to always have
> a valid thread pointer which your call to `clone` is unable to
> initialize correctly because it does not know (and should not know)
> the internal implementation of threads.

In [another bug][gettid()-should-have-a-wrapper], the following is said:

> _[...]_
> most of the problems with caching `pid`/`tid`
> come from use of `clone()` (or worse, `vfork`) directly
> by applications, which should probably not be a supported use.
> With TLS being a mandatory feature in modern `glibc`
> and the thread-pointer being always-initialized
> for purposes like `ssp`, I don't think there's any way applications
> can safely `clone`, whereby "safely" I mean "without a risk
> that internal `libc` state is inconsistent afterwards".

There's also an interesting comment about the different abstractions
employed by the kernel and `glibc`:

> At the kernel level,
> there is really only one kind of kernel scheduling entity (KSE) --
> commonly called a "task" in Linux parlance.
> And that one kind of KSE is identified by one kind of data type.
> Creating an artificial distinction at the `glibc` level seems
> illogical and confusing. Furthermore, the `clone(2)` system call,
> which creates kernel "threads", returns a thread ID.
> But really, this is the same for processes:
> `clone()` is equally the creator of "processes".
> And of course, `glibc` itself already assumes that `TID`s and `PID`s
> are the same thing, since nowadays `glibc`'s `fork()` is a wrapper
> around `clone()`, and that wrapper assumes that `clone()`
> returns a `PID`.

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
> Why not e.g. some `liblinux`
> which could live in the kernel source tree?

> _[...]_
> that these functions don't belong in `LibC`,
> but instead in some separate Linux-specific library or header file
> (a `"liblinux"`).

> _[...]_
> It would be nifty if the kernel came with a `"liblinux"`
> that implemented things like this,
> instead of the daunting (non-starter, really)
> prospect of upgrading to a new `glibc` just to get `syncfs`.

[Posts from as far back as 2004][kernel-headers-and-user-space]
mention the idea:

> _[...]_ I hate even more the proposition that a user space program
> should not include a kernel header file.
> The C library is itself a user space program,
> so the rule that a user space program has to go through the C library
> obviously isn't logical.
> And the GNU C library is an optional tool,
> not an official part of the kernel interface
> (if it were the latter,
> I would expect to see it packaged with the kernel).

> _[...]_
> That Linux has never had an identifiable set of interface header files
> to declare its system call interface seems to me to be a major
> engineering weakness.

[musl-gcc]: https://www.musl-libc.org/how.html

[make.prefixed-path]: http://make.mad-scientist.net/papers/multi-architecture-builds/#explicitpath

[coding-style]: https://www.kernel.org/doc/html/latest/process/coding-style.html

[getrandom]: http://man7.org/linux/man-pages/man2/getrandom.2.html
[long-road-to-getrandom()-in-glibc]: https://lwn.net/Articles/711013/
[email.theodore-ts'o]: https://lwn.net/Articles/711053/

[glibc-wrappers-for-linux-system-calls]: https://lwn.net/Articles/655028/
[email.roland-mcgrath]: https://lwn.net/Articles/655034/

[email.carlos-o'donell]: https://lwn.net/Articles/655039/
[consensus]: https://sourceware.org/glibc/wiki/Consensus#WIP:_Kernel_syscalls_wrappers

[clone(CLONE_VM)-fails]: https://sourceware.org/bugzilla/show_bug.cgi?id=10311

[gettid()-should-have-a-wrapper]: https://sourceware.org/bugzilla/show_bug.cgi?id=6399

[glibc-and-the-kernel-user-space-api]: https://lwn.net/Articles/534682/

[kernel-headers-and-user-space]: https://lwn.net/Articles/113349/
