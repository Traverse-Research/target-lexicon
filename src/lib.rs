//! Target triple support.

#![deny(missing_docs, trivial_numeric_casts, unused_extern_crates)]
#![warn(unused_import_braces)]
#![cfg_attr(
    feature = "cargo-clippy",
    warn(
        clippy::float_arithmetic,
        clippy::mut_mut,
        clippy::nonminimal_bool,
        clippy::option_map_unwrap_or,
        clippy::option_map_unwrap_or_else,
        clippy::print_stdout,
        clippy::unicode_not_nfc,
        clippy::use_self
    )
)]
#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

mod data_model;
mod host;
mod parse_error;
mod targets;
#[macro_use]
mod triple;

pub use self::data_model::{CDataModel, Size};
pub use self::host::HOST;
pub use self::parse_error::ParseError;
pub use self::targets::{
    Aarch64Architecture, Architecture, ArmArchitecture, BinaryFormat, CustomVendor, Environment,
    OperatingSystem, Vendor,
};
pub use self::triple::{CallingConvention, Endianness, PointerWidth, Triple};

/// A simple wrapper around `Triple` that provides an implementation of
/// `Default` which defaults to `Triple::host()`.
pub struct DefaultToHost(pub Triple);

impl Default for DefaultToHost {
    fn default() -> Self {
        Self(Triple::host())
    }
}
