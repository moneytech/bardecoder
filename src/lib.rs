//! Barcode Decoder

#![allow(unknown_lints)]
#![allow(new_without_default_derive)]
#![warn(missing_docs)]

#[macro_use]
extern crate log;

#[macro_use]
extern crate failure_derive;

mod decoder;

pub mod decode;
pub mod detect;
pub mod extract;
pub mod prepare;
pub mod util;

pub use crate::decoder::{default_builder, default_decoder};
pub use crate::decoder::{Decoder, DecoderBuilder};
