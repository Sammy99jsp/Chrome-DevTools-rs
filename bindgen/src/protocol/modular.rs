//!
//! Contains reocurring patterns used while rustify-ing.
//!

use std::{iter, marker::PhantomData};

use proc_macro2::Span;
use syn::{punctuated::Punctuated, PathSegment};

use crate::util::{self, Rustify, ToPath};

use super::convention::{self as conv, NamingConvention};

///
/// Escapes an otherwise unsafe identifier
/// with an underscore (`_`).
///
fn escape_identifier(unsfe: &String) -> String {
    format!("{unsfe}_")
}

///
/// Identifier of a Domain, Item, Command, Event or Field.
///
/// Has its own naming convention ([NamingConvention]).
///
#[derive(Debug, Clone)]
pub struct Identifier<NC: NamingConvention>(String, PhantomData<NC>);

impl<NC: NamingConvention> Identifier<NC> {
    pub fn new(s: impl ToString) -> Self {
        Self(s.to_string(), PhantomData::<NC>)
    }

    ///
    /// Original identifier, as parsed.
    ///
    pub fn original(&self) -> &String {
        &self.0
    }

    ///
    /// Convert to convention.
    /// 
    pub fn convert(self) -> String {
        NC::convert(self.0)
    }
}

impl<NC: NamingConvention> Rustify for Identifier<NC> {
    type Output = syn::Ident;

    fn rustify(self, _: Span, _: Option<crate::util::Context>) -> Self::Output {
        let s = self.convert();
        syn::parse_str(&s).unwrap_or_else(|_| {
            syn::parse_str(&escape_identifier(&s)).expect("Could not escape identifier")
        })
    }
}

impl<NC: NamingConvention> ToString for Identifier<NC> {
    fn to_string(&self) -> String {
        NC::convert(self.0.clone())
    }
}

///
/// Type path in the format `[Domain].<Type>`
///
#[derive(Debug, Clone)]
pub struct TypePath(pub(crate) Option<Identifier<conv::Domain>>, pub(crate) Identifier<conv::Type>);

impl Rustify for TypePath {
    type Output = syn::TypePath;

    fn rustify(self, span: Span, ctx: Option<crate::util::Context>) -> Self::Output {
        
        

        let mut segments = self
            .0
            .map(|s| s.rustify(span, ctx.clone()))
            .into_iter()
            .chain(iter::once(self.1.rustify(span, ctx)))
            .map(PathSegment::from)
            .collect::<Vec<_>>();

        if segments.len() == 2 {
            let root = ["crate", "protocol"]
                .map(util::to_ident(span))
                .map(PathSegment::from)
                .into_iter();

            segments = root.chain(segments).collect();
        }

        syn::TypePath {
            qself: None,
            path: syn::Path {
                leading_colon: None,
                segments: Punctuated::from_iter(segments),
            },
        }
    }
}

///
/// Experimental flag.
///
/// For now, using a custom `#[experimental]` attr macro that
/// for now will just add stuff to the end of the rustdoc
///
#[derive(Debug, Clone, Copy)]
pub struct Experimental;

impl Rustify for Experimental {
    type Output = syn::Attribute;

    fn rustify(self, span: Span, _: Option<util::Context>) -> Self::Output {
        syn::Attribute {
            pound_token: Default::default(),
            bracket_token: Default::default(),
            style: syn::AttrStyle::Outer,
            meta: syn::Meta::Path(
                vec!["util", "experimental"].into_iter()
                    .map(util::to_ident(span))
                    .to_path(),
            ),
        }
    }
}

///
/// Deprecated flag.
///
/// Uses the in-built #[deprecated] flag.
/// Fixes Issue #1
///
#[derive(Debug, Clone, Copy)]
pub struct Deperecated;

impl Rustify for Deperecated {
    type Output = syn::Attribute;

    fn rustify(self, span: Span, _: Option<util::Context>) -> Self::Output {
        syn::Attribute {
            pound_token: Default::default(),
            bracket_token: Default::default(),
            style: syn::AttrStyle::Outer,
            meta: syn::Meta::Path(iter::once("deprecated").map(util::to_ident(span)).to_path()),
        }
    }
}

///
/// Documentation for an element,
/// split by lines.
/// 
/// Converted into `#[doc = "..."]`
///
#[derive(Debug, Clone)]
pub struct Documentation(pub Vec<String>);

impl Rustify for Documentation {
    type Output = Vec<syn::Attribute>;

    fn rustify(self, span: Span, _: Option<crate::util::Context>) -> Self::Output {
        self.0
            .into_iter()
            .map(|ref ln| util::rust::doc(ln, span))
            .collect()
    }
}
