#![deny(
    missing_debug_implementations, missing_copy_implementations, trivial_casts,
    trivial_numeric_casts, unused_extern_crates, unused_import_braces, unused_qualifications,
    variant_size_differences
)]
#![allow(missing_docs)]

#[cfg(feature = "chrono")]
extern crate chrono as chrono_crate;
extern crate serde;
#[cfg(feature = "json")]
extern crate serde_json;

#[cfg(feature = "chrono")]
pub mod chrono;
#[cfg(feature = "json")]
pub mod json;
pub mod rust;
