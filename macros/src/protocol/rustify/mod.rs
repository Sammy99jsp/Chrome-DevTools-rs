//!
//! Handles converting out strange syntax tree into Rust code.
//!
//! See the [Rustify] trait for more info.
//!

pub mod recursive;

use std::{iter, marker::PhantomData, vec};

use convert_case::{Case, Casing};
use quote::{quote, ToTokens};
use syn::{punctuated::Punctuated, LitStr, PathSegment};

use crate::protocol::types::convention;

use super::types::{convention::Identifier, our};

///
/// The context this item is in,
/// with names for conflict resolution (mainly those darn inline enums)!.
///
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Context {
    Protocol,
    Domain(String),
    Type(String, String),
    Field(String, String, String),
}

pub trait ContextProgression {
    fn next(self, ident: String) -> Option<Context>;
}

impl ContextProgression for Option<Context> {
    fn next(self, ident: String) -> Self {
        use Context::*;
        match self {
            Some(Protocol) => Some(Domain(ident)),
            Some(Domain(a)) => Some(Type(a, ident)),
            Some(Type(a, b)) => Some(Field(a, b, ident)),
            Some(Field(_, _, _)) => None,
            None => Some(Protocol),
        }
    }
}

impl Context {
    fn into_iter(self) -> impl Iterator<Item = String> {
        match self {
            Context::Protocol => vec![],
            Context::Domain(a) => vec![a],
            Context::Type(a, b) => vec![a, b],
            Context::Field(a, b, c) => vec![a, b, c],
        }
        .into_iter()
    }
}
///
/// Convert this into valid Rust code.
///
pub trait Rustify {
    type Token;
    fn rustify(self, span: impl Into<proc_macro2::Span>, ctx: Option<Context>) -> Self::Token
    where
        Self: Sized;
}

///
/// Converts a description string into
/// a Vec of attribute macros.
///
fn to_rustdoc(
    span: impl Into<proc_macro2::Span>,
    description: impl Into<Option<String>>,
) -> Vec<syn::Attribute> {
    let d: Option<String> = description.into();
    let span = span.into();

    iter::once("".to_string())
        .chain(
            d.iter()
                .flat_map(|s| s.split('\n').map(ToString::to_string))
                .chain(iter::once("".to_string())),
        )
        .map(|ref ln| syn::Attribute {
            bracket_token: Default::default(),
            meta: syn::Meta::NameValue(syn::MetaNameValue {
                path: syn::Ident::new("doc", span).into(),
                eq_token: Default::default(),
                value: syn::Expr::Lit(syn::ExprLit {
                    attrs: Default::default(),
                    lit: LitStr::new(ln, span).into(),
                }),
            }),
            pound_token: Default::default(),
            style: syn::AttrStyle::Outer,
        })
        .collect()
}

impl Rustify for our::TypePath {
    type Token = syn::TypePath;

    fn rustify(self, span: impl Into<proc_macro2::Span>, _: Option<Context>) -> Self::Token {
        let span = span.into();
        syn::TypePath {
            qself: None,
            path: syn::Path {
                leading_colon: None,
                segments: Punctuated::from_iter(
                    iter::empty().chain(
                        self.domain
                            .map(|s| vec![syn::Ident::new("super", span), s.rust(span)].into_iter())
                            .into_iter()
                            .flatten()
                            .chain(iter::once(self.r#type.rust(span)))
                            .map(syn::PathSegment::from),
                    ),
                ),
            },
        }
    }
}

fn wrap_type(
    span: impl Into<proc_macro2::Span>,
    option: impl Into<Option<bool>>,
    inner: syn::TypePath,
    wrapper_type: &'static str,
) -> syn::TypePath {
    let span: proc_macro2::Span = span.into();
    let option: bool = option.into().unwrap_or(false);

    if option {
        let inner: syn::Type = inner.into();
        return syn::TypePath {
            qself: None,
            path: syn::Path {
                leading_colon: None,
                segments: Punctuated::from_iter(iter::once(syn::PathSegment {
                    ident: syn::Ident::new(wrapper_type, span),
                    arguments: syn::PathArguments::AngleBracketed(
                        syn::AngleBracketedGenericArguments {
                            colon2_token: None,
                            lt_token: Default::default(),
                            gt_token: Default::default(),
                            args: Punctuated::from_iter(iter::once(syn::GenericArgument::Type(
                                inner,
                            ))),
                        },
                    ),
                })),
            },
        };
    }

    inner
}

impl Rustify for our::types::PrimitiveType {
    type Token = syn::TypePath;

    fn rustify(self, span: impl Into<proc_macro2::Span>, _: Option<Context>) -> Self::Token
    where
        Self: Sized,
    {
        use our::types::PrimitiveType::*;
        let span = span.into();
        let rust_type_ident = match self {
            Boolean => vec!["bool"],
            String => vec!["String"],
            Number => vec!["f64"],
            Any => vec!["serde_json", "Value"],
            Integer => vec!["i64"],
        };

        syn::TypePath {
            qself: None,
            path: syn::Path {
                leading_colon: None,
                segments: Punctuated::from_iter(
                    rust_type_ident
                        .into_iter()
                        .map(|s| syn::Ident::new(s, span))
                        .map(syn::PathSegment::from),
                ),
            },
        }
    }
}
pub enum OutTokens {
    Type(syn::Type),
    Enum(syn::ItemEnum),
    Struct(syn::ItemStruct),
    Item(syn::Item),
}

impl quote::ToTokens for OutTokens {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            OutTokens::Type(t) => t.to_tokens(tokens),
            OutTokens::Enum(e) => e.to_tokens(tokens),
            OutTokens::Struct(s) => s.to_tokens(tokens),
            OutTokens::Item(i) => i.to_tokens(tokens),
        }
    }
}

impl From<OutTokens> for syn::Item {
    fn from(item: OutTokens) -> Self {
        match item {
            OutTokens::Type(_) => unimplemented!(),
            OutTokens::Enum(e) => syn::Item::Enum(e),
            OutTokens::Struct(s) => syn::Item::Struct(s),
            OutTokens::Item(i) => i,
        }
    }
}

impl Rustify for our::Type {
    type Token = (OutTokens, Vec<OutTokens>);

    fn rustify(self, span: impl Into<proc_macro2::Span>, ctx: Option<Context>) -> Self::Token
    where
        Self: Sized,
    {
        use our::Type::*;
        let span = span.into();
        match self {
            Reference { r#ref, optional } => (
                OutTokens::Type(syn::Type::Path(wrap_type(
                    span,
                    optional,
                    r#ref.rustify(span, ctx),
                    "Option",
                ))),
                vec![],
            ),
            Simple { r#type, optional } => (
                OutTokens::Type(syn::Type::Path(wrap_type(
                    span,
                    optional,
                    r#type.rustify(span, ctx),
                    "Option",
                ))),
                vec![],
            ),
            Enum {
                optional, r#enum, ..
            } => {
                // Make up an enum name from the rest of the context
                let enum_name = ctx
                    .clone()
                    .expect("Should not be this shallow with enum type!")
                    .into_iter()
                    .skip(1)
                    .map(|s| s.to_case(Case::Snake))
                    .intersperse("_".to_string())
                    .collect::<String>();

                let enum_ident =
                    convention::Identifier(enum_name, PhantomData::<convention::TypeIdentifier>);

                let variants = Punctuated::from_iter(
                    r#enum
                        .into_iter()
                        .map(|s| s.rust_and_original(span))
                        .map(|(ident, o)| syn::Variant {
                            attrs: vec![syn::Attribute {
                                pound_token: Default::default(),
                                style: syn::AttrStyle::Outer,
                                bracket_token: Default::default(),
                                meta: syn::Meta::List(syn::MetaList {
                                    path: syn::Path {
                                        leading_colon: None,
                                        segments: Punctuated::from_iter(vec![
                                            syn::PathSegment::from(syn::Ident::new("serde", span)),
                                        ]),
                                    },
                                    delimiter: syn::MacroDelimiter::Paren(Default::default()),
                                    tokens: quote! {rename = #o},
                                }),
                            }],
                            ident,
                            fields: syn::Fields::Unit,
                            discriminant: None,
                        }),
                );

                let is_inline_with_field = matches!(ctx, Some(Context::Field(_, _, _)));

                let [higher, field]: [_; 2] = ctx
                    .clone()
                    .into_iter()
                    .flat_map(Context::into_iter)
                    .skip(1)
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap_or(["", ""].map(ToString::to_string));

                let mut attrs = to_rustdoc(
                    span,
                    format!("🤖 Autogenerated enum for [{higher}]'s `{field}` field.",),
                );

                attrs.extend(iter::once(serde_derive(span, ["Serialize", "Deserialize"])));

                let en = syn::ItemEnum {
                    attrs,
                    vis: syn::Visibility::Public(Default::default()),
                    enum_token: Default::default(),
                    ident: enum_ident.clone().rust(span),
                    generics: Default::default(),
                    brace_token: Default::default(),
                    variants,
                };

                if is_inline_with_field {
                    let ty = OutTokens::Type(syn::Type::Path(wrap_type(
                        span,
                        optional,
                        our::TypePath {
                            domain: None,
                            r#type: enum_ident,
                        }
                        .rustify(span, ctx),
                        "Option",
                    )));

                    (ty, vec![OutTokens::Enum(en)])
                } else {
                    (OutTokens::Enum(en), vec![])
                }
            }
            Array { optional, items } => {
                let (ty, out) = items.rustify(span, ctx);

                let inner = match ty {
                    OutTokens::Type(t) => match t {
                        syn::Type::Path(p) => wrap_type(span, true, p, "Vec"),
                        _ => unimplemented!("No other syn::Type variants accepted here"),
                    },
                    _ => unimplemented!("Non-immeadiate types not supported!"),
                };

                let opt_type = wrap_type(span, optional, inner, "Option");

                (OutTokens::Type(opt_type.into()), out)
            }
            Object {
                optional,
                properties,
            } => match properties {
                Some(properties) => {
                    let (fields, ext_types) = properties
                        .into_iter()
                        .map(|f| f.rustify(span, ctx.clone()))
                        .unzip::<_, _, Vec<_>, Vec<Vec<OutTokens>>>();

                    let ext_types = ext_types.into_iter().flatten().collect();

                    let struct_def = syn::ItemStruct {
                        attrs: vec![],
                        vis: syn::Visibility::Public(Default::default()),
                        struct_token: Default::default(),
                        ident: syn::Ident::new("TEMPORARY", span),
                        generics: Default::default(),
                        fields: syn::Fields::Named(syn::FieldsNamed {
                            brace_token: Default::default(),
                            named: Punctuated::from_iter(fields),
                        }),
                        semi_token: Default::default(),
                    };

                    (OutTokens::Struct(struct_def), ext_types)
                }
                None => {
                    // If no properties, this object should be an alias for Map<String, any>
                    let ty = syn::TypePath {
                        qself: None,
                        path: syn::Path {
                            leading_colon: None,
                            segments: Punctuated::from_iter(
                                vec![syn::Ident::new("serde_json", span)]
                                    .into_iter()
                                    .map(syn::PathSegment::from)
                                    .chain(iter::once(syn::PathSegment {
                                        ident: syn::Ident::new("Map", span),
                                        arguments: syn::PathArguments::AngleBracketed(
                                            syn::AngleBracketedGenericArguments {
                                                colon2_token: None,
                                                lt_token: Default::default(),
                                                gt_token: Default::default(),
                                                args: Punctuated::from_iter(
                                                    vec![
                                                        segments_to_type_path(vec![
                                                            syn::Ident::new("String", span),
                                                        ]),
                                                        segments_to_type_path(vec![
                                                            syn::Ident::new("serde_json", span),
                                                            syn::Ident::new("Value", span),
                                                        ]),
                                                    ]
                                                    .into_iter()
                                                    .map(syn::Type::Path)
                                                    .map(syn::GenericArgument::Type),
                                                ),
                                            },
                                        ),
                                    })),
                            ),
                        },
                    };

                    (
                        OutTokens::Type(syn::Type::Path(wrap_type(span, optional, ty, "Option"))),
                        vec![],
                    )
                }
            },
        }
    }
}

fn serde_rename(
    span: proc_macro2::Span,
    ident: Identifier<convention::FieldIdentifier>,
) -> syn::Attribute {
    let (_, original) = ident.rust_and_original(span);

    syn::Attribute {
        pound_token: Default::default(),
        bracket_token: Default::default(),
        style: syn::AttrStyle::Outer,
        meta: syn::Meta::List(syn::MetaList {
            path: syn::Path {
                leading_colon: Default::default(),
                segments: Punctuated::from_iter(
                    vec![syn::Ident::new("serde", span)]
                        .into_iter()
                        .map(PathSegment::from),
                ),
            },
            delimiter: syn::MacroDelimiter::Paren(Default::default()),
            tokens: syn::ExprAssign {
                attrs: Default::default(),
                left: Box::new(syn::Expr::Path(syn::ExprPath {
                    attrs: Default::default(),
                    qself: None,
                    path: syn::Ident::new("rename", span).into(),
                })),
                eq_token: Default::default(),
                right: Box::new(syn::Expr::Lit(syn::ExprLit {
                    attrs: Default::default(),
                    lit: syn::Lit::Str(syn::LitStr::new(&original, span)),
                })),
            }
            .into_token_stream(),
        }),
    }
}

fn serde_derive(
    span: proc_macro2::Span,
    tr: impl IntoIterator<Item = &'static str>,
) -> syn::Attribute {
    syn::Attribute {
        pound_token: Default::default(),
        bracket_token: Default::default(),
        style: syn::AttrStyle::Outer,
        meta: syn::Meta::List(syn::MetaList {
            path: syn::Path {
                leading_colon: Default::default(),
                segments: Punctuated::from_iter(
                    vec![syn::Ident::new("derive", span)]
                        .into_iter()
                        .map(PathSegment::from),
                ),
            },
            delimiter: syn::MacroDelimiter::Paren(Default::default()),
            tokens: Punctuated::<_, syn::Token![,]>::from_iter(
                vec![
                    syn::Ident::new("Debug", span),
                    syn::Ident::new("Clone", span),
                ]
                .into_iter()
                .map(PathSegment::from)
                .map(|s| syn::Path {
                    leading_colon: None,
                    segments: Punctuated::from_iter(iter::once(s)),
                })
                .map(|path| syn::TypePath { qself: None, path })
                .chain(tr.into_iter().map(|t| {
                    segments_to_type_path(
                        vec!["serde", t]
                            .into_iter()
                            .map(util::ident(span))
                            .map(PathSegment::from),
                    )
                }))
                .map(syn::Type::Path),
            )
            .into_token_stream(),
        }),
    }
}

impl Rustify for our::Field {
    type Token = (syn::Field, Vec<OutTokens>);

    fn rustify(self, span: impl Into<proc_macro2::Span>, ctx: Option<Context>) -> Self::Token
    where
        Self: Sized,
    {
        let span = span.into();

        let mut attrs = to_rustdoc(span, self.description.clone());
        let og_name = serde_rename(span, self.name.clone());
        attrs.push(og_name);

        let ident = self.name.clone().rust(span);
        let ctx = ctx.next(ident.to_string());
        let (ty, ext_types) = self.typedef.rustify(span, ctx);

        let ty = match ty {
            OutTokens::Type(t) => t,
            _ => unimplemented!("Enum/Struct here should not be in this position."),
        };

        (
            syn::Field {
                attrs,
                vis: syn::Visibility::Public(Default::default()),
                mutability: syn::FieldMutability::None,
                ident: Some(ident),
                colon_token: Default::default(),
                ty,
            },
            ext_types,
        )
    }
}

impl Rustify for our::TypeDeclaration {
    type Token = (syn::Item, Vec<OutTokens>);

    fn rustify(self, span: impl Into<proc_macro2::Span>, ctx: Option<Context>) -> Self::Token
    where
        Self: Sized,
    {
        let our::TypeDeclaration {
            id,
            description,
            typedef,
        } = self;
        let span = span.into();
        let ident = id.rust(span);
        let ctx = ctx.next(ident.to_string());

        let mut attrs = to_rustdoc(span, description);

        let (ty, ext_types) = typedef.rustify(span, ctx);

        let ty = match ty {
            OutTokens::Type(t) => syn::Item::Type(syn::ItemType {
                attrs,
                vis: syn::Visibility::Public(Default::default()),
                type_token: Default::default(),
                ident,
                generics: syn::Generics {
                    lt_token: None,
                    params: Default::default(),
                    gt_token: None,
                    where_clause: None,
                },
                eq_token: Default::default(),
                ty: Box::new(t),
                semi_token: Default::default(),
            }),
            OutTokens::Enum(mut e) => {
                attrs.extend(iter::once(serde_derive(span, ["Serialize", "Deserialize"])));

                e.attrs = attrs;
                e.ident = ident;
                syn::Item::Enum(e)
            }
            OutTokens::Struct(mut s) => {
                attrs.extend(iter::once(serde_derive(span, ["Serialize", "Deserialize"])));

                s.attrs = attrs;
                s.ident = ident;
                syn::Item::Struct(s)
            }
            _ => unimplemented!(),
        };
        (ty, ext_types)
    }
}

///
/// Somewhat mirrors the `util` mod
/// in the main crate.
///
mod util {
    use std::iter;

    pub fn ident(span: proc_macro2::Span) -> impl for<'a> Fn(&'a str) -> syn::Ident {
        move |s| syn::Ident::new(s, span)
    }

    fn path_to_util(span: proc_macro2::Span) -> impl Iterator<Item = syn::PathSegment> {
        vec!["crate", "util"]
            .into_iter()
            .map(ident(span))
            .map(Into::into)
    }

    ///
    /// The alias of {},
    /// the "Nothing" struct.
    ///
    /// For commands that don't send/receive any structured data.
    ///
    #[allow(non_snake_case)]
    pub fn Nothing(span: proc_macro2::Span) -> impl Iterator<Item = syn::PathSegment> {
        path_to_util(span).chain(iter::once(ident(span)("Nothing").into()))
    }

    ///
    /// For commands that cannot fail,
    /// for now the default
    ///
    #[allow(non_snake_case)]
    pub fn Error(span: proc_macro2::Span) -> impl Iterator<Item = syn::PathSegment> {
        path_to_util(span).chain(iter::once(ident(span)("Infallible").into()))
    }

    ///
    /// Things associated with the command trait.
    ///
    #[allow(non_snake_case)]
    pub mod Command {
        use std::iter;

        use syn::{punctuated::Punctuated, PathSegment};

        use crate::protocol::rustify::segments_to_type_path;

        ///
        /// Own path
        ///
        #[allow(non_snake_case)]
        fn self_path(span: proc_macro2::Span) -> impl Iterator<Item = syn::PathSegment> {
            super::path_to_util(span).chain(iter::once(super::ident(span)("Command").into()))
        }

        ///
        /// Generate impl block for the "Command" trait for
        /// a struct.
        ///
        pub fn gen_impl(
            span: proc_macro2::Span,
            struct_ident: syn::Ident,
            original_name: String,
            params_type: syn::Type,
            returns_type: syn::Type,
            error_type: syn::Type,
        ) -> syn::ItemImpl {
            let assoc_types = vec!["Parameters", "Returns", "Error"]
                .into_iter()
                .map(super::ident(span));
            let items = vec![params_type, returns_type, error_type]
                .into_iter()
                .zip(assoc_types)
                .map(|(ty, ident)| {
                    syn::ImplItem::Type(syn::ImplItemType {
                        attrs: Default::default(),
                        vis: syn::Visibility::Inherited,
                        defaultness: None,
                        type_token: Default::default(),
                        ident,
                        generics: Default::default(),
                        eq_token: Default::default(),
                        ty,
                        semi_token: Default::default(),
                    })
                })
                .chain(iter::once(syn::ImplItem::Fn(syn::ImplItemFn {
                    attrs: vec![],
                    vis: syn::Visibility::Inherited,
                    defaultness: None,
                    sig: syn::Signature {
                        constness: None,
                        asyncness: None,
                        unsafety: None,
                        abi: None,
                        fn_token: Default::default(),
                        ident: syn::Ident::new("id", span),
                        generics: Default::default(),
                        paren_token: Default::default(),
                        inputs: Punctuated::from_iter(iter::empty::<syn::FnArg>()),
                        variadic: None,
                        output: syn::ReturnType::Type(
                            Default::default(),
                            Box::new(syn::Type::Reference(syn::TypeReference {
                                and_token: Default::default(),
                                lifetime: Some(syn::Lifetime::new("'static", span)),
                                mutability: None,
                                elem: Box::new(syn::Type::Path(segments_to_type_path(
                                    iter::once(super::ident(span)("str")).map(PathSegment::from),
                                ))),
                            })),
                        ),
                    },
                    block: syn::Block {
                        brace_token: Default::default(),
                        stmts: vec![syn::Stmt::Expr(
                            syn::Expr::Lit(syn::ExprLit {
                                attrs: vec![],
                                lit: syn::Lit::Str(syn::LitStr::new(&original_name, span)),
                            }),
                            None,
                        )],
                    },
                })))
                .collect();

            syn::ItemImpl {
                attrs: vec![],
                defaultness: None,
                unsafety: None,
                impl_token: Default::default(),
                generics: syn::Generics::default(),
                trait_: Some((
                    None,
                    syn::Path {
                        leading_colon: None,
                        segments: Punctuated::from_iter(self_path(span)),
                    },
                    syn::token::For::default(),
                )),
                self_ty: Box::new(syn::Type::Path(syn::TypePath {
                    qself: None,
                    path: syn::Path {
                        leading_colon: None,
                        segments: Punctuated::from_iter(iter::once::<PathSegment>(
                            struct_ident.into(),
                        )),
                    },
                })),
                brace_token: Default::default(),
                items,
            }
        }
    }
}

fn segments_to_type_path(
    iter: impl IntoIterator<Item = impl Into<syn::PathSegment>>,
) -> syn::TypePath {
    syn::TypePath {
        qself: None,
        path: syn::Path {
            leading_colon: None,
            segments: Punctuated::from_iter(iter.into_iter().map(Into::into)),
        },
    }
}

impl Rustify for super::Command {
    type Token = Vec<OutTokens>;

    fn rustify(self, span: impl Into<proc_macro2::Span>, ctx: Option<Context>) -> Self::Token
    where
        Self: Sized,
    {
        let span = span.into();
        let (ident, original_name) = self.name.rust_and_original(span);

        let ctx = ctx.next(ident.to_string());

        let (params, ext_tokens): (Vec<syn::Field>, Vec<Vec<OutTokens>>) = self
            .parameters
            .map(IntoIterator::into_iter)
            .map(|params| params.map(|f| f.rustify(span, ctx.clone())))
            .map(|r| r.unzip())
            .unwrap_or((vec![], vec![]));

        let ext_tokens = ext_tokens.into_iter().flatten();

        let mut attrs = to_rustdoc(span, format!("Parameters for [{ident}]"));
        attrs.extend(iter::once(serde_derive(span, ["Serialize", "Deserialize"])));

        let param_struct: Option<syn::ItemStruct> = (!params.is_empty()).then(|| syn::ItemStruct {
            attrs,
            vis: syn::Visibility::Public(Default::default()),
            struct_token: Default::default(),
            ident: syn::Ident::new(&format!("{ident}Params"), span),
            generics: syn::Generics::default(),
            fields: syn::Fields::Named(syn::FieldsNamed {
                brace_token: Default::default(),
                named: Punctuated::from_iter(params),
            }),
            semi_token: Default::default(),
        });

        // What to put in the impl block for type Parameters = <..>
        let param_type = param_struct
            .clone()
            .map(|ref i| syn::Type::Path(segments_to_type_path(iter::once(i.ident.clone()))))
            .unwrap_or(syn::Type::Path(segments_to_type_path(util::Nothing(span))));

        let (returns, ext_tokens_2): (Vec<syn::Field>, Vec<Vec<OutTokens>>) = self
            .returns
            .map(IntoIterator::into_iter)
            .map(|params| params.map(|f| f.rustify(span, ctx.clone())))
            .map(|r| r.unzip())
            .unwrap_or((vec![], vec![]));

        let ext_tokens_2 = ext_tokens_2.into_iter().flatten();

        let mut attrs = to_rustdoc(span, format!("Return value for [{ident}]"));
        attrs.extend(iter::once(serde_derive(span, ["Serialize", "Deserialize"])));

        let returns_struct: Option<syn::ItemStruct> =
            (!returns.is_empty()).then(|| syn::ItemStruct {
                attrs,
                vis: syn::Visibility::Public(Default::default()),
                struct_token: Default::default(),
                ident: syn::Ident::new(&format!("{ident}Returns"), span),
                generics: syn::Generics::default(),
                fields: syn::Fields::Named(syn::FieldsNamed {
                    brace_token: Default::default(),
                    named: Punctuated::from_iter(returns),
                }),
                semi_token: Default::default(),
            });

        // What to put in the impl block for type Returns
        let returns_type = returns_struct
            .clone()
            .map(|ref i| syn::Type::Path(segments_to_type_path(iter::once(i.ident.clone()))))
            .unwrap_or(syn::Type::Path(segments_to_type_path(util::Nothing(span))));

        // What to put in impl for type Error = <...>
        let error_type = syn::Type::Path(segments_to_type_path(util::Error(span)));

        let attrs = to_rustdoc(span, self.description);

        let command_struct = syn::ItemStruct {
            attrs,
            vis: syn::Visibility::Public(Default::default()),
            struct_token: Default::default(),
            ident: ident.clone(),
            generics: Default::default(),
            fields: syn::Fields::Unit,
            semi_token: Default::default(),
        };

        let impl_block = util::Command::gen_impl(
            span,
            command_struct.ident.clone(),
            ctx.map(Context::into_iter)
                .into_iter()
                .flatten()
                .next()
                .into_iter()
                .map(|s| s.to_case(Case::Pascal))
                .chain(iter::once(original_name.to_case(Case::Camel)))
                .intersperse(".".to_string())
                .collect(),
            param_type,
            returns_type,
            error_type,
        );

        let command_items = iter::empty()
            .chain(param_struct)
            .chain(returns_struct)
            .chain(iter::once(command_struct))
            .map(syn::Item::Struct)
            .chain(iter::once(syn::Item::Impl(impl_block)))
            .map(OutTokens::Item);

        ext_tokens
            .chain(ext_tokens_2)
            .chain(command_items)
            .collect()
    }
}

impl Rustify for super::Event {
    type Token = Vec<OutTokens>;

    fn rustify(self, span: impl Into<proc_macro2::Span>, ctx: Option<Context>) -> Self::Token
    where
        Self: Sized,
    {
        let span = span.into();
        let ident = self.name.rust(span);
        let ctx = ctx.next(ident.to_string());

        let mut attrs = to_rustdoc(span, self.description);
        attrs.extend(iter::once(serde_derive(span, ["Serialize"])));

        let (params, ext_tokens): (Vec<_>, Vec<_>) = self
            .parameters
            .into_iter()
            .flat_map(IntoIterator::into_iter)
            .map(|f| f.rustify(span, ctx.clone()))
            .unzip();

        let ext_tokens = ext_tokens.into_iter().flat_map(IntoIterator::into_iter);

        if params.is_empty() {
            return ext_tokens.collect();
        }

        let event_struct = syn::ItemStruct {
            attrs,
            vis: syn::Visibility::Public(Default::default()),
            struct_token: Default::default(),
            ident,
            generics: Default::default(),
            fields: syn::Fields::Named(syn::FieldsNamed {
                brace_token: Default::default(),
                named: Punctuated::from_iter(params),
            }),
            semi_token: Default::default(),
        };

        ext_tokens
            .chain(iter::once(OutTokens::Struct(event_struct)))
            .collect()
    }
}

impl Rustify for super::Domain {
    type Token = syn::ItemMod;

    fn rustify(self, span: impl Into<proc_macro2::Span>, ctx: Option<Context>) -> Self::Token
    where
        Self: Sized,
    {
        let span = span.into();
        let ident = self.domain.rust(span);
        let ctx = ctx.next(ident.to_string());

        let typedefs = self.types.into_iter().flat_map(|ty| {
            let (i, tkns) = ty.rustify(span, ctx.clone());
            tkns.into_iter().chain(iter::once(OutTokens::Item(i)))
        });

        let uses = self
            .dependencies
            .into_iter()
            .map(|ident| {
                let ident = ident.rust(span);

                syn::Item::Use(syn::ItemUse {
                    attrs: Default::default(),
                    vis: syn::Visibility::Public(Default::default()),
                    use_token: Default::default(),
                    leading_colon: None,
                    tree: syn::UseTree::Path(syn::UsePath {
                        ident: syn::Ident::new("super", span),
                        colon2_token: Default::default(),
                        tree: Box::new(syn::UseTree::Name(syn::UseName { ident })),
                    }),
                    semi_token: Default::default(),
                })
            })
            .map(OutTokens::Item);

        let commands = self
            .commands
            .into_iter()
            .flat_map(|cmd| cmd.rustify(span, ctx.clone()).into_iter());
        let events = self
            .events
            .into_iter()
            .flat_map(|evt| evt.rustify(span, ctx.clone()));

        let items = iter::empty()
            .chain(uses)
            .chain(typedefs)
            .chain(commands)
            .chain(events)
            .map(syn::Item::from)
            .collect();

        syn::ItemMod {
            attrs: Default::default(),
            vis: syn::Visibility::Public(Default::default()),
            unsafety: None,
            mod_token: Default::default(),
            ident,
            content: Some((Default::default(), items)),
            semi: Default::default(),
        }
    }
}

impl Rustify for super::Protocol {
    type Token = syn::File;

    fn rustify(self, span: impl Into<proc_macro2::Span>, _: Option<Context>) -> Self::Token
    where
        Self: Sized,
    {
        let span = span.into();
        let ctx = Some(Context::Protocol);

        #[cfg(not(feature = "latest"))]
        let version = format!("{}.{}", self.version.major, self.version.minor);
        
        #[cfg(feature = "latest")]
        let version = "`latest` (tip-of-tree)".to_string();


        let attrs = format!(
            "# Chrome DevTools Protocol\n🤖 Autogenerated Rust types for version ***{}*** .",
            version
        )
        .split('\n')
        .map(|ln| syn::Attribute {
            pound_token: Default::default(),
            style: syn::AttrStyle::Inner(Default::default()),
            bracket_token: Default::default(),
            meta: syn::Meta::NameValue(syn::MetaNameValue {
                path: syn::Ident::new("doc", span).into(),
                eq_token: Default::default(),
                value: syn::Expr::Lit(syn::ExprLit {
                    attrs: vec![],
                    lit: syn::Lit::Str(syn::LitStr::new(ln, span)),
                }),
            }),
        })
        // Disable clippy's linting warnings for the enums.
        // It's not my fault, blame Google !
        .chain(iter::once(syn::Attribute {
            pound_token: Default::default(),
            style: syn::AttrStyle::Inner(Default::default()),
            bracket_token: Default::default(),
            meta: syn::Meta::List(syn::MetaList {
                path: segments_to_type_path(
                    iter::once(syn::Ident::new("allow", span)).map(PathSegment::from),
                )
                .path,
                delimiter: syn::MacroDelimiter::Paren(Default::default()),
                tokens: segments_to_type_path(
                    vec!["clippy", "enum_variant_names"]
                        .into_iter()
                        .map(util::ident(span))
                        .map(PathSegment::from),
                )
                .path
                .into_token_stream(),
            }),
        }))
        .collect();

        let mut items: Vec<syn::Item> = self
            .domains
            .into_iter()
            .map(|r| r.rustify(span, ctx.clone()))
            .map(syn::Item::Mod)
            .collect();

        // Post-processing for recursion
        // Box<...> -es up self-referential fields in structs.

        items
            .iter_mut()
            .filter_map(|i| match i {
                syn::Item::Mod(m) => Some(m),
                _ => None,
            })
            .flat_map(|m| {
                m.content
                    .as_mut()
                    .map(|(_, c)| c.iter_mut())
                    .into_iter()
                    .flatten()
            })
            .filter_map(|i| match i {
                syn::Item::Struct(m) => Some(m),
                _ => None,
            })
            .for_each(recursive::boxify_struct_recursion);

        syn::File {
            shebang: None,
            attrs,
            items,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::protocol::rustify::{Context, Rustify};
    use prettyplease::unparse;
    use std::{fs::File, io::Write};
    #[test]
    fn raw_ident() {
        let _iden: syn::Ident = syn::parse_str("r#type").expect("Valid type!");
    }

    #[test]
    fn rustify() {
        let mut f: crate::protocol::Protocol =
            serde_json::from_str(include_str!("../../../test/protocol.json")).expect("Valid parse");

        let j: crate::protocol::Protocol =
            serde_json::from_str(include_str!("../../../test/js_protocol.json"))
                .expect("Valid parse");

        f.domains.extend(j.domains);

        let c = f.rustify(proc_macro2::Span::mixed_site(), Some(Context::Protocol));

        let pretty = unparse(&c);

        File::create("./test/protocol_test.rs")
            .expect("Should be able to make file")
            .write_all(pretty.as_bytes())
            .expect("Should be writabele");
    }
}
