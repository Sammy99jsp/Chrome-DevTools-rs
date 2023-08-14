//!
//! A collection of naming conventions.
//!
//! These provide an interface to translate
//! from the original conventions to the
//! [official Rust](https://rust-lang.github.io/api-guidelines/naming.html)
//! naming convention using [convert_case](https://crates.io/crates/convert_case).
//!

use convert_case::{Case, Casing};

///
/// Naming convention for identifiers.
///
pub trait NamingConvention: std::fmt::Debug + Copy + Clone {
    const CASE: Case;

    fn convert(src: String) -> String 
        where Self: Sized
    {
        src.to_case(Self::CASE)
    }
}


///
/// Naming convention for a Domain (a `mod` in Rust).
///
#[derive(Debug, Clone, Copy)]
pub struct Domain;

impl NamingConvention for Domain {
    const CASE: Case = Case::Snake;
}

///
/// Naming convention for a type alias (`type` in Rust).
///
#[derive(Debug, Clone, Copy)]
pub struct Type;

impl NamingConvention for Type {
    const CASE: Case = Case::Pascal;
}

///
/// Naming convention for a command (`struct` in Rust).
///
#[derive(Debug, Clone, Copy)]
pub struct Command;

impl NamingConvention for Command {
    const CASE: Case = Case::Pascal;
}

///
/// Naming convention for a field.
///
#[derive(Debug, Clone, Copy)]
pub struct Field;

impl NamingConvention for Field {
    const CASE: Case = Case::Snake;
}

///
/// Naming convention for an event
/// (a `struct` in Rust, with "Event" added to the end)
///
#[derive(Debug, Clone, Copy)]
pub struct Event;

impl NamingConvention for Event {
    const CASE: Case = Case::Pascal;

    fn convert(src: String) -> String {
        format!("{}Event", src.to_case(Self::CASE))
    }
}
