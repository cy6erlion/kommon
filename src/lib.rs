//! kommon

#![warn(missing_docs, unreachable_pub, future_incompatible, rust_2018_idioms)]

pub use environment::Environment;
pub use error::Error;
pub use gender::Gender;

mod environment;
mod error;
#[cfg(feature = "ffi")]
pub mod ffi;
mod gender;

/// Entity unique identifier
pub type ID = i32;

/// Schema version
pub type VERSION = i16;
