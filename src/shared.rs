//! Linux kernel interfaces shared across architectures
//!
//! Definitions and system call implementations
//! that can be reused across architectures.
//! Each architecture re-exports compatible
//! symbols and overrides incompatible ones
//! with architecture specific replacements.
//!
//! Constants are defined here and reused
//! wherever they fit. Architectures need
//! only specify what's different by adding
//! new constants or overriding the shared
//! ones. System calls aren't overridden,
//! only those present on every architecture
//! supported by Linux are shared in this
//! module. Architectures always implement
//! their exclusive system calls directly.
//!
//! Symbols flow from the shared modules
//! to the architecture specific modules
//! and finally to the target architecture
//! module which is what gets compiled.
//! This module is not an architecture,
//! so it's a sibling to the architecture
//! module.

pub mod definitions;
