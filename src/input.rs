#![allow(clippy::module_inception)]
pub mod input;
pub mod input_number;

pub use input::{Input, InputSize, InputVariant};
pub use input_number::InputNumber;
