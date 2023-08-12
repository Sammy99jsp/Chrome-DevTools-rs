//!
//! # Chrome DevTools Protocol
//! This trait ports the [Chrome DevTools Protocol](https://chromedevtools.github.io/devtools-protocol/)
//! into Rust, using an auto-generated implementation using proc-macros.
//! 
//! ## Feature Flags
//! ### `latest`
//! By default, this crate uses the `/macros/test` protocol files as a source.
//! You can optionally try to fetch the latest tip-of-tree protcol from the [DevTools GitHub](https://github.com/ChromeDevTools/devtools-protocol/tree/master)
//! this is not guaranteed to build however.
//! 
//! ## Usage
//! It's pretty much [`serde`](https://docs.rs/serde/1.0.183/serde/) and [`serde_json`](https://docs.rs/serde_json/1.0.104/serde_json/) all the way down.
//! 
//! ### Helpers
//! All `Command`s (aka `Method`s) implement the [util::Command] trait,
//! which link their associated parameters and return types.
//! ```
//! use chrome_devtools_api::util::Command;
//! ```
//! 

#![feature(associated_type_defaults)]
pub mod util;
pub use util::*;

#[chrome_devtools_macros::protocol]
pub mod protocol {}