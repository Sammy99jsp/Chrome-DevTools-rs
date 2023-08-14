#![feature(return_position_impl_trait_in_trait, iter_intersperse)]
//!
//! # Parsing helper
//! 
//! Essentially does all the hard work parsing and rustifying the protocol.
//! 

use proc_macro2::Span;
use util::Rustify;
mod protocol;
mod util;

#[cfg(feature = "latest")]
use reqwest::blocking::{Response, get};

#[cfg(feature = "latest")]
const SOURCES: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/SOURCES"));

///
/// Fetch the protocols, either from locally or the GitHub repo.
/// 
fn fetch_protocols() -> [String; 2] {
    #[allow(unused_mut)]
    let mut sources = [
        include_str!("../test/protocol.json").to_string(),
        include_str!("../test/js_protocol.json").to_string(),
    ];

    #[cfg(feature = "latest")]
    {
        sources = SOURCES
            .split('\n')
            .filter(|s| !s.starts_with('#'))
            .enumerate()
            .map(|(i, u)| (i, get(u)))
            .map(|(i, r)| {
                r.and_then(Response::text).unwrap_or(
                    sources
                        .get(i)
                        .cloned()
                        .expect("Too many URLs loaded in SOURCES. Fix me :( "),
                )
            })
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
    }

    sources
}

///
/// Parses and rustifies the protcols.
/// 
fn protocols_to_rust(span: impl Into<Span>, sources : impl IntoIterator<Item = String>) -> syn::File {
    let span = span.into();
    let mut protocols = sources.into_iter()
        .map(|src| serde_json::from_str::<crate::protocol::Protocol>(&src))
        .map(|protocol| protocol.expect("Error parsing protocol"))
        .map(|prototcol| prototcol.rustify(span, None))
        .collect::<Vec<_>>();

    let mut first = protocols.remove(0);

    first.items.extend(protocols.into_iter().flat_map(|f| f.items.into_iter()));

    first
}

///
/// Returns the source code for a binding file.
/// 
pub fn generate_protocol_bindings() -> String {
    let protocols = fetch_protocols();
    let file = protocols_to_rust(Span::call_site(), protocols);
    prettyplease::unparse(&file)
}