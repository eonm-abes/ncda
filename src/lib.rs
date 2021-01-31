#![feature(test)]
//! # NCDA (Noid Check Digit Algorithm)
//!
//!
//!

extern crate test;

mod bindings;
mod errors;
mod ncda;
mod consts;

pub use bindings::*;
pub use errors::*;
