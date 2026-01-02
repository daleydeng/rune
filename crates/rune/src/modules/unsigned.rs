//! Integers.

use core::cmp::Ordering;
use core::num::ParseIntError;

use crate as rune;
use crate::alloc::string::TryToString;
use crate::{ContextError, Module};

/// Unsigned integers.
///
/// This provides methods for computing over and parsing 64-bit unsigned integers.
#[cfg(not(feature = "number-32"))]
#[rune::module(::std::u64)]
pub fn module() -> Result<Module, ContextError> {
    let mut m = Module::from_meta(self::module__meta)?;
    unsigned!(m, u64);
    Ok(m)
}

#[cfg(not(feature = "number-32"))]
unsigned_fns!(u64);

/// Unsigned integers.
///
/// This provides methods for computing over and parsing 32-bit unsigned integers.
#[cfg(feature = "number-32")]
#[rune::module(::std::u32)]
pub fn module() -> Result<Module, ContextError> {
    let mut m = Module::from_meta(self::module__meta)?;
    unsigned!(m, u32);
    Ok(m)
}

#[cfg(feature = "number-32")]
unsigned_fns!(u32);
