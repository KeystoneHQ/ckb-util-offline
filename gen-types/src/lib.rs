//! # The Generated Types Library
//!
//! This Library provides the generated types for CKB.

#![no_std]

extern crate alloc;

mod conversion;
pub mod core;
mod extension;
mod generated;
pub mod prelude;
pub use generated::packed;

//re-exports
pub use molecule::bytes;

cfg_if::cfg_if! {
    if #[cfg(feature = "std")] {
        use std::{vec, borrow};
    } else {
        use alloc::{vec, borrow};
    }
}
