#![doc(html_root_url = "https://docs.rs/prost/0.2.3")]

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "std")]
extern crate core;

#[cfg(not(feature = "std"))]
#[macro_use]
extern crate alloc;

extern crate bytes;

#[cfg(test)]
#[macro_use]
extern crate quickcheck;

mod error;
mod message;
mod types;

#[doc(hidden)]
pub mod encoding;

pub use message::Message;
pub use error::{DecodeError, EncodeError};
