//! CKB crypto util library.
//!
//! This crate keep as legacy, only used in test now.

#![no_std]
#![feature(error_in_core)]
extern crate alloc;

#[cfg(feature = "secp")]
pub mod secp;
