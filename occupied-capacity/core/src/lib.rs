//! Data structure measurement.

#![no_std]
#![feature(error_in_core)]

mod units;

pub use units::{Capacity, Error, IntoCapacity, Ratio, Result};
