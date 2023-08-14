//!
//! Collection of utility functions.
//!

use std::iter;

use c::NamingConvention;
use proc_macro2::Span;
use syn::{punctuated::Punctuated, PathSegment};

use crate::protocol::modular::{NamedIdentifier, AnyIdentifier, IntoAnyIdentifier};
use crate::protocol::convention as c;

///
/// > Where are we now?
///
/// Identifies where in parsing we are.
///
#[derive(Debug, Clone)]
pub enum Context {
    ///
    /// Top-level, only used when translating the protocol object itself.
    ///
    Protocol,

    ///
    /// Translating items of a domain `d`.
    ///
    Domain(AnyIdentifier),

    ///
    /// Translating fields in domain `d` of item `i`.
    ///
    Item(AnyIdentifier, AnyIdentifier),

    ///
    /// Translating a type in domain `d`, item `i`, of field `f`.
    ///
    Field(AnyIdentifier, AnyIdentifier, AnyIdentifier),
}

///
/// Utility trait for advancing the context.
///
pub trait Contextual {
    fn next(self, ident: impl IntoAnyIdentifier) -> Option<Context>;
    fn iter(&self) -> impl Iterator<Item = AnyIdentifier> + '_;
}

impl Contextual for Option<Context> {
    ///
    /// Add new context to the stack.
    ///
    fn next(self, ident: impl IntoAnyIdentifier) -> Option<Context> {
        use Context::*;

        match self {
            None => Some(Protocol),
            Some(Protocol) => Some(Domain(ident.to_any())),
            Some(Domain(p1)) => Some(Item(p1, ident.to_any())),
            Some(Item(p1, p2)) => Some(Field(p1, p2, ident.to_any())),
            Some(Field(_, _, _)) => None,
        }
    }

    ///
    /// Get context on the stack.
    ///
    fn iter(&self) -> impl Iterator<Item = AnyIdentifier> + '_ {
        use Context::*;

        match self {
            None => vec![],
            Some(Protocol) => vec![],
            Some(Domain(p1)) => vec![p1],
            Some(Item(p1, p2)) => vec![p1, p2],
            Some(Field(p1, p2, p3)) => vec![p1, p2, p3],
        }
        .into_iter()
        .cloned()
    }
}

///
/// Trait for converting the protocol definition into
/// Rust source code.
///
pub trait Rustify {
    ///
    /// Rust equivalent of the definition (with `syn`'s AST).
    ///
    type Output;

    ///
    /// Convert `self` into Rust AST.
    ///
    fn rustify(self, span: Span, ctx: Option<Context>) -> Self::Output;

    ///
    /// Provides a conversion function for multiples of `self`,
    /// used mainly in iterators:
    ///
    /// ```ignore
    /// let fields = fields
    ///     .into_iter()
    ///     .map(Rustify::rustified(span, ctx.clone()))
    ///     /// ...
    /// ```
    ///
    fn rustified<R: Rustify>(span: Span, ctx: Option<Context>) -> Box<dyn Fn(R) -> R::Output> {
        Box::new(move |item| Rustify::rustify(item, span, ctx.clone()))
    }
}

///
/// Utility for converting path segments into a [syn::Path].
///
pub trait ToPath {
    fn to_path(self) -> syn::Path;
}

impl<Segment, Iter> ToPath for Iter
where
    PathSegment: From<Segment>,
    Iter: IntoIterator<Item = Segment>,
{
    fn to_path(self) -> syn::Path {
        syn::Path {
            leading_colon: None,
            segments: Punctuated::from_iter(self.into_iter().map(PathSegment::from)),
        }
    }
}

///
/// Converts PathSegments into a [syn::TypePath].
///
pub trait ToTypedPath {
    fn to_type_path(self) -> syn::TypePath;
}

impl<C> ToTypedPath for C
where
    C: ToPath,
{
    fn to_type_path(self) -> syn::TypePath {
        syn::TypePath {
            qself: None,
            path: self.to_path(),
        }
    }
}

///
/// Helps converting an iterator of string-likes into [syn::Ident]s
///
pub fn to_ident<I: AsRef<str>>(span: Span) -> impl Fn(I) -> syn::Ident {
    move |a| syn::Ident::new(a.as_ref(), span)
}

///
/// Collection of common Rust-related identifiers,
/// and documentation conversion methods..
///
pub mod rust {
    use std::iter;

    use proc_macro2::Span;
    use quote::ToTokens;

    use super::{to_ident, ToPath, ToTypedPath};

    ///
    /// [Option]
    ///
    pub fn option(span: Span) -> syn::TypePath {
        vec!["Option"]
            .into_iter()
            .map(to_ident(span))
            .to_type_path()
    }

    ///
    /// [Box]
    ///
    pub fn box_(span: Span) -> syn::TypePath {
        vec!["Box"].into_iter().map(to_ident(span)).to_type_path()
    }

    ///
    /// [Vec]
    ///
    pub fn vec(span: Span) -> syn::TypePath {
        vec!["Vec"].into_iter().map(to_ident(span)).to_type_path()
    }

    ///
    /// ```ignore
    /// super
    /// ```
    ///
    pub fn super_(span: Span) -> syn::Ident {
        to_ident(span)("super")
    }

    ///
    /// ```ignore
    /// crate
    /// ```
    ///
    pub fn crate_(span: Span) -> syn::Ident {
        to_ident(span)("crate")
    }

    ///
    /// Generates a `#[doc = "..."]` macro with the providied string.
    ///
    pub fn doc(contents: &str, span: Span) -> syn::Attribute {
        syn::Attribute {
            pound_token: Default::default(),
            style: syn::AttrStyle::Outer,
            bracket_token: Default::default(),
            meta: syn::Meta::NameValue(syn::MetaNameValue {
                path: iter::once("doc").map(to_ident(span)).to_path(),
                eq_token: Default::default(),
                value: syn::Expr::Lit(syn::ExprLit {
                    attrs: Default::default(),
                    lit: syn::Lit::Str(syn::LitStr::new(contents, span)),
                }),
            }),
        }
    }

    ///
    /// Generates multi-line documentation from a multi-line string.
    ///
    pub fn rustdoc(src: &str, span: Span) -> impl Iterator<Item = syn::Attribute> + '_ {
        iter::once("")
            .chain(src.split('\n'))
            .chain(iter::once(""))
            .map(|s| format!(" {s}"))
            .map(move |ln| doc(&ln, span))
    }

    ///
    /// Generates an `allow` macro for linters.
    /// ```ignore
    /// #![allow(...)]
    /// ```
    /// 
    pub fn allow(span: Span, path: syn::Path) -> Vec<syn::Attribute> {
        let attr = syn::Attribute {
            pound_token: Default::default(),
            bracket_token: Default::default(),
            style: syn::AttrStyle::Inner(Default::default()),
            meta: syn::Meta::List(syn::MetaList {
                path: ["allow"].map(to_ident(span)).to_path(),
                delimiter: syn::MacroDelimiter::Paren(Default::default()),
                tokens: path.into_token_stream(),
            }),
        };

        vec![attr]
    }

    pub fn default_variant(span: Span) -> Vec<syn::Attribute> {
        let attr = syn::Attribute {
            pound_token: Default::default(),
            bracket_token: Default::default(),
            style: syn::AttrStyle::Outer,
            meta: syn::Meta::Path(["default"].map(to_ident(span)).to_path()),
        };

        vec![attr]
    }

    ///
    /// Generates
    /// ```ignore
    /// &'static str
    /// ```
    pub fn static_str(span: Span) -> syn::Type {
        syn::Type::Reference(syn::TypeReference {
            and_token: Default::default(),
            lifetime: Some(syn::Lifetime::new("'static", span)),
            mutability: None,
            elem: Box::new(syn::Type::Path(["str"].map(to_ident(span)).to_type_path())),
        })
    }

    ///
    /// Path for the `Command` trait, which
    /// associates a command with its parameters
    /// and return types.
    ///
    #[allow(non_snake_case)]
    pub fn Command(span: Span) -> syn::Path {
        ["crate", "util", "Command"].map(to_ident(span)).to_path()
    }

    ///
    /// Path for the `Event` trait.
    ///
    #[allow(non_snake_case)]
    pub fn Event(span: Span) -> syn::Path {
        ["crate", "util", "Event"].map(to_ident(span)).to_path()
    }

    ///
    /// Path to the `Nothing` struct
    /// (no parameters/return type).
    ///
    #[allow(non_snake_case)]
    pub fn Nothing(span: Span) -> syn::Path {
        ["crate", "util", "Nothing"].map(to_ident(span)).to_path()
    }

    ///
    /// Path to the `Infalible` enum
    /// (no possible errors).
    ///
    #[allow(non_snake_case)]
    pub fn Infallible(span: Span) -> syn::Path {
        ["crate", "util", "Infallible"]
            .map(to_ident(span))
            .to_path()
    }
}

///
/// Utilities to generate common [serde] macros.
///
pub mod serde {
    use std::iter;

    use proc_macro2::Span;
    use quote::ToTokens;
    use syn::punctuated::Punctuated;

    use crate::protocol::{convention as conv, modular::{self, Identifier}};

    use super::{to_ident, ToPath};

    ///
    /// Generate a #[serde(rename = "")]
    /// attribute macro for a field.
    ///
    pub fn rename(to: &str, span: Span) -> [syn::Attribute; 1] {
        [syn::Attribute {
            pound_token: Default::default(),
            bracket_token: Default::default(),
            style: syn::AttrStyle::Outer,
            meta: syn::Meta::List(syn::MetaList {
                path: iter::once("serde").map(to_ident(span)).to_path(),
                delimiter: syn::MacroDelimiter::Paren(Default::default()),
                tokens: syn::ExprAssign {
                    attrs: Default::default(),
                    eq_token: Default::default(),
                    left: Box::new(syn::Expr::Path(syn::ExprPath {
                        attrs: Default::default(),
                        qself: Default::default(),
                        path: iter::once("rename").map(to_ident(span)).to_path(),
                    })),
                    right: Box::new(syn::Expr::Lit(syn::ExprLit {
                        attrs: Default::default(),
                        lit: syn::Lit::Str(syn::LitStr::new(to, span)),
                    })),
                }
                .into_token_stream(),
            }),
        }]
    }

    ///
    /// Generates a derive macro
    /// ```ignore
    /// #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
    /// ```
    ///
    pub fn derive_macro(span: Span) -> impl Iterator<Item = syn::Attribute> {
        let attr = syn::Attribute {
            pound_token: Default::default(),
            bracket_token: Default::default(),
            style: syn::AttrStyle::Outer,
            meta: syn::Meta::List(syn::MetaList {
                delimiter: syn::MacroDelimiter::Paren(Default::default()),
                path: ["derive"].map(to_ident(span)).to_path(),
                tokens: Punctuated::<_, syn::Token![,]>::from_iter(
                    [
                        vec!["Debug"],
                        vec!["Clone"],
                        vec!["Default"],
                        vec!["serde", "Serialize"],
                        vec!["serde", "Deserialize"],
                    ]
                    .map(|p| p.into_iter().map(to_ident(span)).to_path()),
                )
                .into_token_stream(),
            }),
        };

        iter::once(attr)
    }

    ///
    /// Utility trait to generate a `#[serde(rename = "...")]` macro
    /// for a fields' identifier.
    ///
    pub trait GenRenameMacro {
        ///
        /// Generate the `#[serde(rename = "...")]` for this identifier's original name.
        ///
        fn serde_rename(&self, span: Span) -> [syn::Attribute; 1];
    }

    impl<NC: conv::NamingConvention> GenRenameMacro for modular::NamedIdentifier<NC> {
        fn serde_rename(&self, span: Span) -> [syn::Attribute; 1] {
            rename(self.original(), span)
        }
    }
}

///
/// Common strings to use when generating documentation.
///
pub mod info {
    use proc_macro2::Span;

    use super::{Context, Contextual};

    ///
    /// "Autogenerated" notice.
    ///
    pub const AUTOGENERATED: &str = "🤖 Autogenerated";

    ///
    /// Documentation for an autogenerated enum (from an inline enum declaration).
    ///
    pub fn autogenned_enum(ctx: &Option<Context>, span: Span) -> Vec<syn::Attribute> {
        // Context should be Field(_, _, _), so we should be confident in this `.expect(...)`
        let [strct, field] = Contextual::iter(ctx)
            .skip(1)
            .collect::<Vec<_>>()
            .try_into()
            .expect("Insufficient context for inline enum doc autogeneration");

        super::rust::rustdoc(
            &format!("Enum for [{}]'s `{}`\n---\n{AUTOGENERATED}", strct.to_string(), field.to_string()),
            span,
        )
        .collect()
    }

    ///
    /// Documentation for an autogenerated struct
    /// (typically the `Parameters` and `Returns` types for a [Command](crate::protocol::Command)).
    ///
    pub fn autogenned_struct(t: &String, original: &String, span: Span) -> Vec<syn::Attribute> {
        super::rust::rustdoc(&format!("{t} value for [{original}]."), span).collect()
    }

    ///
    ///Documentation for the entire [Protocol Definition](crate::protocol::Protocol) itself.
    ///
    #[allow(unused)]
    pub fn protocol_docs(version: String, span: Span) -> Vec<syn::Attribute> {
        #[cfg(feature = "latest")]
        let version = "latest (tip-of-tree) [{version}]".to_string();

        #[cfg(not(feature = "latest"))]
        let version = format!("V{version}");

        super::rust::rustdoc(
            &format!(
                "# Chrome DevTools Protocol \n\
            Version: `{}`\n\n\
            Autogenerated Rust bindings for the Chrome DevTools Protocol. \
            ",
                version
            ),
            span,
        )
        .map(|mut attr| {
            attr.style = syn::AttrStyle::Inner(Default::default());
            attr
        })
        .collect()
    }
}

///
/// Wrap type with another type (type path).
///
pub fn wrap_type(mut outer: syn::TypePath) -> impl FnOnce(syn::Type) -> syn::Type + 'static {
    move |inner| {
        if let Some(segment) = outer.path.segments.last_mut() {
            segment.arguments =
                syn::PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments {
                    colon2_token: Default::default(),
                    lt_token: Default::default(),
                    gt_token: Default::default(),
                    args: Punctuated::from_iter(iter::once(syn::GenericArgument::Type(inner))),
                })
        }

        syn::Type::Path(outer)
    }
}
