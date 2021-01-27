#[cfg(target_arch = "wasm32")]
mod wasm;
#[cfg(target_arch = "wasm32")]
pub use wasm::*;

#[cfg(not(target_arch = "wasm32"))]
pub use rust::*;
#[cfg(not(target_arch = "wasm32"))]
mod rust;

mod python;
