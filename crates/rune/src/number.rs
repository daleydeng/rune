//! Shared number configuration.
//!
//! These are the canonical numeric types used throughout Rune.

/// Signed integer type used by the VM and compiler.
#[cfg(feature = "number-32")]
pub type SignedType = i32;

/// Unsigned integer type used by the VM and compiler.
#[cfg(feature = "number-32")]
pub type UnsignedType = u32;

/// Floating point type used by the VM and compiler.
#[cfg(feature = "number-32")]
pub type FloatType = f32;

/// Signed integer type used by the VM and compiler.
#[cfg(not(feature = "number-32"))]
pub type SignedType = i64;

/// Unsigned integer type used by the VM and compiler.
#[cfg(not(feature = "number-32"))]
pub type UnsignedType = u64;

/// Floating point type used by the VM and compiler.
#[cfg(not(feature = "number-32"))]
pub type FloatType = f64;
