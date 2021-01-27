#![feature(test)]
//! # NCDA (Noid Check Digit Algorithm)
//! 
//! 
//! 

extern crate test;

mod ncda;
mod bindings;
mod errors;

pub use bindings::*;
pub use errors::*;
