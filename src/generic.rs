//! Architecture independent kernel definitions
//!
//! This module mirrors the definitions shared by
//! every architecture: the asm-generic UAPI headers
//! and the socket constants the kernel keeps to itself.
//! It is not an architecture that gets compiled for
//! and so is a sibling of the architecture module.
//! Architectures either re-export generic definitions
//! or override them with their own definitions.
//! This encodes the generic/overridden knowledge
//! structurally. Concrete code always resolves
//! definitions via architecture::target.

pub mod definitions;
