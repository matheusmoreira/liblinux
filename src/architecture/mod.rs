//! Per-architecture system call interfaces.
//!
//! Each architecture supplies the definitions its kernel uses,
//! the `system_call_0` through `system_call_6` primitives that
//! issue system calls, and the set of system calls it can make.
//!
//! # Negated errno returns
//!
//! `system_call_*` functions must return the kernel's result
//! such that an error is a negated error number that falls
//! within the `[-4095, -1]` interval. Every layer above
//! depends on this: [`crate::Errno::from_system_call`]
//! tests exactly that range and has no other way
//! to distinguish an error from a normal result.
//!
//! On most architectures the kernel already returns errors
//! that way and the primitives need not adapt. However,
//! some architectures instead signal failure separately
//! from the result:
//!
//! - `mips` signals failure in `a3`,
//!   with the error number in `v0`
//! - `powerpc` signals failure in the
//!   summary overflow bit of `cr0`,
//!   with the error number in `r3`
//! - `sparc` signals failure in the carry bit,
//!   with the error number in `o0`
//!
//! On those architectures the primitive must fold that signal
//! into the sign of its return value. The kernel's own nolibc
//! library does this normalization at the system call primitive
//! boundary. Omitting it can silently turn failed system calls
//! into successful results.

pub mod aarch64;
pub mod x86_64;

#[cfg(all(target_arch = "aarch64", target_pointer_width = "64"))]
pub use self::aarch64 as target;

#[cfg(all(target_arch = "x86_64", target_pointer_width = "64"))]
pub use self::x86_64 as target;
