//!
//! Implementations of [Rustify] for the protocol
//! types.
//!

use std::iter;

use super::{convention as conv, modular as m, post_ast};
use super::{
    Command, Domain, DomainDependency, Event, Field, Primitive, Protocol, Type, TypeDeclaration,
};
use convert_case::{Case, Casing};
use proc_macro2::Span;
use syn::punctuated::Punctuated;

use crate::util::ToPath;
use crate::util::{
    self,
    info::{self, protocol_docs},
    serde::GenRenameMacro,
    Contextual, Rustify, ToTypedPath,
};

impl Rustify for Primitive {
    type Output = syn::TypePath;

    fn rustify(self, span: proc_macro2::Span, _: Option<crate::util::Context>) -> Self::Output {
        use Primitive::*;
        match self {
            Boolean => vec!["bool"],
            Number => vec!["f64"],
            Integer => vec!["i64"],
            String => vec!["String"],
            Any => vec!["serde_json", "Value"],
        }
        .into_iter()
        .map(util::to_ident(span))
        .to_type_path()
    }
}

impl Type {
    pub fn is_optional(&self) -> bool {
        use Type::*;

        *match self {
            Primitive { optional, .. } => optional,
            Reference { optional, .. } => optional,
            Array { optional, .. } => optional,
            Object { optional, .. } => optional,
            Enum { optional, .. } => optional,
        }
    }
}

pub enum ItemOrType {
    Item(syn::Item),
    Type(syn::Type),
}

impl ItemOrType {
    pub fn try_into_type(self) -> Option<syn::Type> {
        match self {
            ItemOrType::Item(_) => None,
            ItemOrType::Type(t) => Some(t),
        }
    }
}

trait IntoItemOrType {
    fn to_item_or_type(self) -> ItemOrType;
}

impl IntoItemOrType for syn::Item {
    fn to_item_or_type(self) -> ItemOrType {
        ItemOrType::Item(self)
    }
}

impl IntoItemOrType for syn::Type {
    fn to_item_or_type(self) -> ItemOrType {
        ItemOrType::Type(self)
    }
}

impl Rustify for Type {
    type Output = (ItemOrType, Vec<syn::Item>);

    fn rustify(self, span: Span, ctx: Option<util::Context>) -> Self::Output {
        use Type::*;

        let optional = self.is_optional();

        // Wrap in option if `optional` is true.
        let optionalize = |t: syn::Type| {
            optional
                .then(|| util::wrap_type(util::rust::option(span))(t.clone()))
                .unwrap_or(t)
        };

        match self {
            Primitive { ty, .. } => (
                optionalize(ty.rustify(span, ctx).into()).to_item_or_type(),
                vec![],
            ),
            Reference { path, .. } => (
                optionalize(path.rustify(span, ctx).into()).to_item_or_type(),
                vec![],
            ),
            Array { item_type, .. } => {
                let (ty, additional) = item_type.rustify(span, ctx);
                let vectorize = util::wrap_type(util::rust::vec(span));
                (
                    optionalize(vectorize(
                        ty.try_into_type()
                            .expect("Nested complex object inside array type"),
                    ))
                    .to_item_or_type(),
                    additional,
                )
            }
            Object { fields, .. } => {
                // If no fields, we are an alias for `serde_json::Map<String, serde_json::Value>`

                if fields.is_none() {
                    let [mut map, key, value]: [_; 3] = vec![
                        vec!["serde_json", "Map"],
                        vec!["String"],
                        vec!["serde_json", "Value"],
                    ]
                    .into_iter()
                    .map(IntoIterator::into_iter)
                    .map(|i| i.map(util::to_ident(span)))
                    .map(ToTypedPath::to_type_path)
                    .collect::<Vec<_>>()
                    .try_into()
                    .ok()
                    .unwrap();

                    let m = map.path.segments.last_mut().unwrap();
                    m.arguments =
                        syn::PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments {
                            colon2_token: Default::default(),
                            gt_token: Default::default(),
                            lt_token: Default::default(),
                            args: Punctuated::from_iter(
                                [key, value]
                                    .map(syn::Type::Path)
                                    .map(syn::GenericArgument::Type),
                            ),
                        });

                    return (optionalize(syn::Type::Path(map)).to_item_or_type(), vec![]);
                }

                let fields = fields.unwrap();

                // An object type here is equivalent to an anonymous struct
                // (which don't exist in Rust :[ ).

                let (fields, additional): (Vec<_>, Vec<_>) =
                    fields.into_iter().map(Field::rustified(span, ctx)).unzip();

                let additional = additional.into_iter().flatten().collect();

                let skeleton_struct = syn::ItemStruct {
                    attrs: Default::default(),
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

                (
                    syn::Item::Struct(skeleton_struct).to_item_or_type(),
                    additional,
                )
            }
            Enum { values, .. } => {
                // Special case where there are ocassionally *very*
                // lazy inline enum definitions (equivalent to union types in TS).
                // We in Rust however don't have this luxury and must declare them
                // in their own item.
                let ctx2 = ctx.clone();
                let variants = values.into_iter().map(|v| syn::Variant {
                    attrs: v.clone().serde_rename(span).to_vec(),
                    ident: v.rustify(span, ctx2.clone()),
                    fields: syn::Fields::Unit,
                    discriminant: None,
                });

                // If an inline enum...
                if matches!(ctx, Some(util::Context::Field(_, _, _))) {
                    // Generate a plausible-sounding name based off the parent struct and field
                    // names from the context (`ctx`): "Badger" + "HungerLevel" => "BadgerHungerLevel"

                    let ident = Contextual::iter(&ctx)
                        .skip(1)
                        .map(|s| s.to_case(Case::Snake))
                        .intersperse("_".to_string())
                        .collect::<String>();

                    let ident = m::Identifier::<conv::Type>::new(ident).rustify(span, ctx.clone());

                    let attrs = util::info::autogenned_enum(&ctx, span);
                    let (ty, _) = Type::Reference {
                        optional,
                        path: m::TypePath(
                            None,
                            m::Identifier::<conv::Type>::new(ident.to_string()),
                        ),
                    }
                    .rustify(span, ctx);

                    (
                        optionalize(ty.try_into_type().unwrap()).to_item_or_type(),
                        vec![syn::Item::Enum(syn::ItemEnum {
                            attrs,
                            vis: syn::Visibility::Public(Default::default()),
                            enum_token: Default::default(),
                            ident,
                            generics: Default::default(),
                            brace_token: Default::default(),
                            variants: Punctuated::from_iter(variants),
                        })],
                    )
                } else {
                    (
                        syn::Item::Enum(syn::ItemEnum {
                            attrs: Default::default(),
                            vis: syn::Visibility::Public(Default::default()),
                            enum_token: Default::default(),
                            ident: syn::Ident::new("TEMPORARY", span),
                            generics: Default::default(),
                            brace_token: Default::default(),
                            variants: Punctuated::from_iter(variants),
                        })
                        .to_item_or_type(),
                        vec![],
                    )
                }
            }
        }
    }
}

fn deprecated_docs_experimental(
    ctx: Option<util::Context>,
    span: Span,
    deprecated: Option<m::Deperecated>,
    description: Option<m::Documentation>,
    experimental: Option<m::Experimental>,
) -> impl Iterator<Item = syn::Attribute> + 'static {
    description
        .into_iter()
        .flat_map(m::Documentation::rustified(span, ctx.clone()))
        .chain(
            deprecated
                .into_iter()
                .map(m::Deperecated::rustified(span, ctx.clone())),
        )
    // .chain(
    //     experimental
    //         .into_iter()
    //         .map(m::Experimental::rustified(span, ctx.clone())),
    // )
}

impl Rustify for Field {
    type Output = (syn::Field, Vec<syn::Item>);

    fn rustify(self, span: Span, ctx: Option<crate::util::Context>) -> Self::Output {
        let ident = self.name.clone().rustify(span, ctx.clone());

        // Update context for inner nested structure.
        let ctx = ctx.next(ident.clone());

        let attrs = deprecated_docs_experimental(
            ctx.clone(),
            span,
            self.deprecated,
            self.description,
            self.experimental,
        )
        // Ensure that the #[serde(rename = "...")] is present, since many fields are escaped.
        .chain(self.name.serde_rename(span))
        .collect();

        let (ty, additional_item) = self.ty.rustify(span, ctx.clone());

        (
            syn::Field {
                attrs,
                ident: ident.into(),
                ty: ty
                    .try_into_type()
                    .expect("Nested structure in a field's type declaration"),
                colon_token: Default::default(),
                mutability: syn::FieldMutability::None,
                vis: syn::Visibility::Public(Default::default()),
            },
            additional_item,
        )
    }
}

impl Rustify for TypeDeclaration {
    type Output = Vec<syn::Item>;

    fn rustify(self, span: Span, ctx: Option<util::Context>) -> Self::Output {
        let ident = self.id.rustify(span, ctx.clone());
        let ctx = ctx.next(&ident);

        let attrs = deprecated_docs_experimental(
            ctx.clone(),
            span,
            self.deprecated,
            self.description,
            self.experimental,
        )
        .collect();

        let (ty, mut additional) = self.ty.rustify(span, ctx);

        let main_type = match ty {
            ItemOrType::Item(i) => {
                match i {
                    syn::Item::Enum(mut e) => {
                        // Enum declaration (not an inline enum).
                        e.ident = ident;
                        e.attrs = attrs;
                        syn::Item::Enum(e)
                    }
                    syn::Item::Struct(mut s) => {
                        s.ident = ident;
                        s.attrs = attrs;
                        syn::Item::Struct(s)
                    }
                    _ => {
                        unimplemented!("Only struct, enum, or type alias items passed from fields.")
                    }
                }
            }
            ItemOrType::Type(t) => {
                // Type alias
                syn::Item::Type(syn::ItemType {
                    attrs,
                    vis: syn::Visibility::Public(Default::default()),
                    type_token: Default::default(),
                    ident,
                    generics: Default::default(),
                    eq_token: Default::default(),
                    ty: Box::new(t),
                    semi_token: Default::default(),
                })
            }
        };

        additional.push(main_type);

        additional
    }
}

impl Command {
    fn gen_util_struct(
        span: Span,
        ctx: Option<util::Context>,
        fields: Option<Vec<Field>>,
        original_name: String,
        deriv_name: String,
        msg: &'static str,
    ) -> Vec<syn::Item> {
        let ident = syn::Ident::new(&deriv_name, span);

        let fields = fields.into_iter().flatten().collect::<Vec<_>>();

        if fields.is_empty() {
            return vec![];
        }

        let (fields, additional): (Vec<_>, Vec<_>) = fields
            .into_iter()
            .map(Field::rustified(span, ctx.clone()))
            .unzip();

        let additional = additional.into_iter().flatten();
        let attrs = info::autogenned_struct(&msg.to_string(), &original_name, span);

        let def = syn::ItemStruct {
            attrs,
            vis: syn::Visibility::Public(Default::default()),
            struct_token: Default::default(),
            ident,
            generics: Default::default(),
            fields: syn::Fields::Named(syn::FieldsNamed {
                brace_token: Default::default(),
                named: Punctuated::from_iter(fields),
            }),
            semi_token: Default::default(),
        };

        additional
            .chain(iter::once(syn::Item::Struct(def)))
            .collect()
    }

    fn gen_command_impl(&self, span: Span, original: &str, ident: &syn::Ident) -> syn::ItemImpl {
        let trait_path = util::rust::Command(span);

        let assoc_type = |ident, path| syn::ImplItemType {
            attrs: Default::default(),
            defaultness: Default::default(),
            type_token: Default::default(),
            generics: Default::default(),
            eq_token: Default::default(),
            semi_token: Default::default(),
            vis: syn::Visibility::Inherited,
            ident: syn::Ident::new(ident, span),
            ty: syn::Type::Path(syn::TypePath { qself: None, path }),
        };

        let if_def = |def: bool, yes: &str, no| {
            if def {
                [format!("{}{}", ident, yes)]
                    .map(util::to_ident(span))
                    .to_path()
            } else {
                no
            }
        };

        let params = assoc_type(
            "Parameters",
            if_def(
                self.parameters
                    .as_ref()
                    .map(|i| i.is_empty())
                    .unwrap_or(false),
                "Params",
                util::rust::Nothing(span),
            ),
        );

        let returns = assoc_type(
            "Returns",
            if_def(
                self.parameters
                    .as_ref()
                    .map(|i| i.is_empty())
                    .unwrap_or(false),
                "Returns",
                util::rust::Nothing(span),
            ),
        );

        let error = assoc_type("Error", util::rust::Infallible(span));

        let stmt = syn::Stmt::Expr(
            syn::Expr::Lit(syn::ExprLit {
                attrs: Default::default(),
                lit: syn::Lit::Str(syn::LitStr::new(original, span)),
            }),
            None,
        );

        let id_fn = syn::ImplItem::Fn(syn::ImplItemFn {
            attrs: Default::default(),
            defaultness: Default::default(),
            vis: syn::Visibility::Inherited,
            sig: syn::Signature {
                constness: Default::default(),
                asyncness: Default::default(),
                unsafety: Default::default(),
                abi: Default::default(),
                fn_token: Default::default(),
                generics: Default::default(),
                paren_token: Default::default(),
                variadic: Default::default(),
                ident: syn::Ident::new("id", span),
                inputs: Punctuated::from_iter(iter::empty::<syn::FnArg>()),
                output: syn::ReturnType::Type(
                    Default::default(),
                    Box::new(util::rust::static_str(span)),
                ),
            },
            block: syn::Block {
                brace_token: Default::default(),
                stmts: vec![stmt],
            },
        });

        syn::ItemImpl {
            attrs: Default::default(),
            defaultness: Default::default(),
            unsafety: Default::default(),
            impl_token: Default::default(),
            generics: Default::default(),
            trait_: Some((None, trait_path, Default::default())),
            self_ty: Box::new([ident.clone()].to_type_path().into()),
            brace_token: Default::default(),
            items: [params, returns, error]
                .map(syn::ImplItem::Type)
                .into_iter()
                .chain(iter::once(id_fn))
                .collect(),
        }
    }
}

impl Rustify for Command {
    type Output = Vec<syn::Item>;

    fn rustify(self, span: Span, ctx: Option<util::Context>) -> Self::Output {
        let original = self.name.original().to_string();
        let ident = self.name.clone().rustify(span, ctx.clone());
        let ctx = ctx.next(&ident);

        let impl_block = syn::Item::Impl(self.gen_command_impl(span, &original, &ident));

        let attrs = deprecated_docs_experimental(
            ctx.clone(),
            span,
            self.deprecated,
            self.description.map(|mut v| {
                v.0.extend(
                    iter::once("---".to_string())
                        .chain(
                            self.parameters
                                .is_some()
                                .then_some(format!("Parameter Type: [{}Params]", ident)),
                        )
                        .chain(
                            self.returns
                                .is_some()
                                .then_some(format!("Return Type: [{}Returns]", ident)),
                        ),
                );
                v
            }),
            self.experimental,
        )
        .collect();

        let params = Self::gen_util_struct(
            span,
            ctx.clone(),
            self.parameters,
            ident.to_string(),
            format!("{}Params", ident),
            "Parameter",
        );
        let returns = Self::gen_util_struct(
            span,
            ctx.clone(),
            self.returns,
            ident.to_string(),
            format!("{}Returns", ident),
            "Return",
        );
        let self_struct = syn::Item::Struct(syn::ItemStruct {
            attrs,
            vis: syn::Visibility::Public(Default::default()),
            struct_token: Default::default(),
            ident,
            generics: Default::default(),
            fields: syn::Fields::Unit,
            semi_token: Default::default(),
        });

        params
            .into_iter()
            .chain(returns)
            .chain(iter::once(self_struct))
            .chain(iter::once(impl_block))
            .collect()
    }
}

impl Rustify for Event {
    type Output = Vec<syn::Item>;

    fn rustify(self, span: Span, ctx: Option<util::Context>) -> Self::Output {
        let ident = self.name.rustify(span, ctx.clone());
        let ctx = ctx.next(ident.clone());

        let attrs = deprecated_docs_experimental(
            ctx.clone(),
            span,
            self.deprecated,
            self.description,
            self.experimental,
        )
        .collect();

        let (fields, additional): (Vec<_>, Vec<_>) = self
            .parameters
            .into_iter()
            .flatten()
            .map(Field::rustified(span, ctx.clone()))
            .unzip();

        let additional = additional.into_iter().flatten();

        let strct = syn::Item::Struct(syn::ItemStruct {
            vis: syn::Visibility::Public(Default::default()),
            struct_token: Default::default(),
            semi_token: Default::default(),
            generics: Default::default(),
            attrs,
            ident,
            fields: syn::Fields::Named(syn::FieldsNamed {
                brace_token: Default::default(),
                named: Punctuated::from_iter(fields),
            }),
        });

        additional.chain(iter::once(strct)).collect()
    }
}

impl Rustify for DomainDependency {
    type Output = syn::ItemUse;

    fn rustify(self, span: Span, ctx: Option<util::Context>) -> Self::Output {
        syn::ItemUse {
            attrs: Default::default(),
            use_token: Default::default(),
            semi_token: Default::default(),
            vis: syn::Visibility::Inherited,
            leading_colon: None,
            tree: syn::UseTree::Path(syn::UsePath {
                colon2_token: Default::default(),
                ident: util::rust::super_(span),
                tree: Box::new(syn::UseTree::Name(syn::UseName {
                    ident: self.0.rustify(span, ctx),
                })),
            }),
        }
    }
}

impl Domain {
    fn add_derive_attr(span: Span) -> impl Fn(syn::Item) -> syn::Item {
        move |mut item| {
            match &mut item {
                syn::Item::Enum(ref mut s) => {
                    s.attrs.extend(util::serde::derive_macro(span));
                }
                syn::Item::Struct(ref mut e) => {
                    e.attrs.extend(util::serde::derive_macro(span));
                }
                _ => {}
            }

            item
        }
    }
}

impl Rustify for Domain {
    type Output = syn::ItemMod;

    fn rustify(self, span: Span, ctx: Option<util::Context>) -> Self::Output {
        let ident = self.domain.rustify(span, ctx.clone());
        let ctx = ctx.next(ident.clone());

        let attrs = deprecated_docs_experimental(
            ctx.clone(),
            span,
            self.deprecated,
            self.description,
            self.experimental,
        )
        .collect();

        // Dsiabled dependencies
        let _dependencies = self
            .dependencies
            .into_iter()
            .flatten()
            .map(DomainDependency::rustified(span, ctx.clone()))
            .map(syn::Item::Use);

        let types = self
            .types
            .into_iter()
            .flatten()
            .flat_map(Type::rustified(span, ctx.clone()));

        let commands = self
            .commands
            .into_iter()
            .flatten()
            .flat_map(Command::rustified(span, ctx.clone()));

        let events = self
            .events
            .into_iter()
            .flatten()
            .flat_map(Event::rustified(span, ctx));

        let contents = iter::empty()
            .chain(types)
            .chain(commands)
            .chain(events)
            .map(Self::add_derive_attr(span))
            .collect();

        syn::ItemMod {
            semi: Default::default(),
            mod_token: Default::default(),
            unsafety: None,
            attrs,
            vis: syn::Visibility::Public(Default::default()),
            ident,
            content: Some((Default::default(), contents)),
        }
    }
}

impl Rustify for Protocol {
    type Output = syn::File;

    fn rustify(self, span: Span, _: Option<util::Context>) -> Self::Output {
        let ctx = Some(util::Context::Protocol);
        let version = self.version.to_string();
        let attrs = protocol_docs(version, span).into_iter()
            .chain(util::rust::allow(span, ["deprecated"].map(util::to_ident(span)).to_path()))
            .chain(util::rust::allow(span, ["clippy", "enum_variant_names"].map(util::to_ident(span)).to_path()))
            .collect();

        let mut items = self
            .domains
            .into_iter()
            .map(Domain::rustified(span, ctx))
            .map(syn::Item::Mod)
            .collect::<Vec<_>>();

        // Solve the problem of self-referential types by Box-ing them.
        // 0. Get all the struct definitions.
        let structs = items
            .iter_mut()
            .filter_map(|i| match i {
                syn::Item::Mod(m) => Some(m),
                _ => None,
            })
            .flat_map(|i| i.content.as_mut().map(|(_, i)| i.iter_mut()))
            .flatten()
            .filter_map(|i| match i {
                syn::Item::Struct(s) => Some(s),
                _ => None,
            });

        // 1. Run them through the algorithm to boxify when necessary..
        post_ast::boxify_self_referential_types(span, structs);

        syn::File {
            shebang: Default::default(),
            attrs,
            items,
        }
    }
}
