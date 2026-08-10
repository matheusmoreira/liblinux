# Time ABI

On this architecture, `clock_gettime` uses the native
64-bit time binary interface. There is no separate
`clock_gettime64` system call.

The `time` argument points to a
[`__kernel_timespec`](crate::definitions::__kernel_timespec)
whose `tv_sec` and `tv_nsec` fields are signed 64-bit integers.
