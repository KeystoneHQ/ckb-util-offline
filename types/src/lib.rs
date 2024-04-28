//! # The Core Types Library
//!
//! This Library provides the essential types for CKB.


#![cfg_attr(feature = "alloc", no_std)]
#![cfg_attr(feature = "alloc", feature(error_in_core))]

#[cfg(feature = "alloc")]
extern crate alloc;

#[macro_use]
pub mod prelude;

pub use bytes;
pub use ckb_fixed_hash::{h160, h256, H160, H256};
pub use ckb_gen_types::packed;
pub use molecule::{self, error};
pub use numext_fixed_uint::{u256, U128, U256};

pub mod core;
pub mod global;

pub mod constants;
mod conversion;
mod extension;
pub mod utilities;

// #[cfg(test)]
// mod tests;
