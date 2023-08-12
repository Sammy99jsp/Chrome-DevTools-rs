#![feature(iter_intersperse, associated_type_defaults, iterator_try_collect)]

mod protocol;
use crate::protocol::rustify::Rustify;

use proc_macro::{TokenStream, Span};
use quote::ToTokens;
#[cfg(feature = "latest")]
use reqwest::blocking::{get, Response};

#[cfg(feature = "latest")]
const SOURCES: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/SOURCES"));

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

fn protocols_to_rust(span: impl Into<Span>, sources : impl IntoIterator<Item = String>) -> syn::ItemMod {
    let span = span.into();
    let protocols = sources.into_iter()
        .map(|src| serde_json::from_str::<crate::protocol::Protocol>(&src))
        .map(|protocol| protocol.expect("Error parsing protocol"))
        .map(|prototcol| prototcol.rustify(span, None))
        .collect::<Vec<_>>();

    syn::ItemMod {
        attrs: protocols[0].attrs.clone(),
        vis: syn::Visibility::Public(Default::default()),
        unsafety: None,
        mod_token: Default::default(),
        ident: syn::Ident::new("protocol", span.into()),
        content: Some((Default::default(), protocols.into_iter().flat_map(|p| p.items.into_iter()).collect())),
        semi: Default::default(),
    }
}

///
/// Defines the protocol in this file.
/// 
/// For private use only!
/// 
/// ### Usage
/// ```ignore
/// #![chrome_devtools_macros::protocol]
/// 
/// // That's it!
/// ```
/// 
/// ### Requirements
/// You must have:
/// * A `crate::util` module, with:
///     * The `Command` trait.
///     * The `Nothing` struct.
///     * The `Infalible` enum.
/// 
#[proc_macro_attribute]
pub fn protocol(_: TokenStream, _: TokenStream) -> TokenStream {
    let sources = fetch_protocols();
    let rust_code = protocols_to_rust(Span::call_site(), sources);

    rust_code.into_token_stream().into()
}