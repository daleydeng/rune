//! Integers.

use core::cmp::Ordering;
use core::num::ParseIntError;

use crate as rune;
use crate::alloc::string::TryToString;
use crate::{ContextError, Module};

/// Signed integers.
///
/// This provides methods for computing over and parsing 64-bit signed integers.
#[cfg(not(feature = "number-32"))]
#[rune::module(::std::i64)]
pub fn module() -> Result<Module, ContextError> {
    let mut m = Module::from_meta(self::module__meta)?;
    signed!(m, i64);
    Ok(m)
}

#[cfg(not(feature = "number-32"))]
signed_fns!(i64);

/// Signed integers.
///
/// This provides methods for computing over and parsing 32-bit signed integers.
#[cfg(feature = "number-32")]
#[rune::module(::std::i32)]
pub fn module() -> Result<Module, ContextError> {
    let mut m = Module::from_meta(self::module__meta)?;
    signed!(m, i32);
    Ok(m)
}

#[cfg(feature = "number-32")]
signed_fns!(i32);
