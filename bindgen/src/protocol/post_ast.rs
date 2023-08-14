//!
//! Operations done after Rust AST assembly.
//! 
//! Right now:
//! * Box-ing of recursive types.
//! * Default for enum.
//!

use std::iter;

use proc_macro2::Span;
use syn::punctuated::Punctuated;

use crate::util::{self, ToTypedPath};

///
/// Walk along types recursively with a predicate function
/// until it returns true, then return a mutable reference
/// to the statisfying type.
/// 
fn walk_type_path(
    ty: &mut syn::Type,
    pred: impl Fn(&syn::Type, Option<&syn::PathSegment>) -> bool + Clone,
) -> Option<&mut syn::Type> {
    if pred(ty, None) {
        return Some(ty);
    }

    let ty = match ty {
        syn::Type::Array(arr) => return walk_type_path(arr.elem.as_mut(), pred),
        syn::Type::BareFn(_) => return None,
        syn::Type::Group(g) => return walk_type_path(g.elem.as_mut(), pred),
        syn::Type::ImplTrait(_) => return None,
        syn::Type::Infer(_) => return None,
        syn::Type::Macro(_) => return None,
        syn::Type::Never(_) => return None,
        syn::Type::Paren(g) => return walk_type_path(g.elem.as_mut(), pred),
        syn::Type::Path(p) => p,
        syn::Type::Ptr(_) => return None,
        syn::Type::Reference(_) => return None,
        syn::Type::Slice(s) => return walk_type_path(s.elem.as_mut(), pred),
        syn::Type::TraitObject(_) => return None,
        syn::Type::Tuple(t) => {
            return t
                .elems
                .iter_mut()
                .find_map(|ty| walk_type_path(ty, pred.clone()))
        }
        syn::Type::Verbatim(_) => return None,
        _ => return None,
    };

    for s in ty.path.segments.last_mut().into_iter() {
        let s_copy = s.clone();
        if let syn::PathArguments::AngleBracketed(ref mut args) = s.arguments {
            for arg in args.args.iter_mut() {
                if let syn::GenericArgument::Type(ref mut ty) = arg {
                    if pred(ty, Some(&s_copy)) {
                        return Some(ty);
                    }
                }
            }
        }
    }

    None
}

///
/// Function that returns a predicate to check if a
/// type matches a provided "self" type. 
/// 
fn is_self_referential(
    self_type: syn::TypePath,
) -> impl Fn(&syn::Type, Option<&syn::PathSegment>) -> bool + Clone {
    move |ty, seg| match ty {
        syn::Type::Path(p) => {
            iter::zip(self_type.path.segments.iter(), p.path.segments.iter())
                .all(|(l, r)| l.ident.eq(&r.ident))
                && seg.map(|s| s.ident.ne("Vec")).unwrap_or(true)
        }
        _ => false,
    }
}

///
/// Boxifies self-referential types (recursive types)
/// where necessary.
/// 
pub fn boxify_self_referential_types<'a>(
    span: Span,
    items: impl Iterator<Item = &'a mut syn::ItemStruct>,
) {
    items.for_each(|s| {
        let mut flag = false;
        let p = [s.ident.clone()].to_type_path();
        s.fields
            .iter_mut()
            .filter_map(|f| walk_type_path(&mut f.ty, is_self_referential(p.clone())))
            .filter_map(|f| match f {
                syn::Type::Path(ref mut p) => Some(p),
                _ => None,
            })
            .for_each(|t| {
                let inner = t.clone();
                t.path.segments = Punctuated::from_iter(iter::once(syn::PathSegment {
                    ident: syn::Ident::new("Box", span),
                    arguments: syn::PathArguments::AngleBracketed(
                        syn::AngleBracketedGenericArguments {
                            colon2_token: Default::default(),
                            lt_token: Default::default(),
                            gt_token: Default::default(),
                            args: Punctuated::from_iter(iter::once(syn::GenericArgument::Type(
                                syn::Type::Path(inner),
                            ))),
                        },
                    ),
                }));
                flag = true;
            });
    })
}

///
/// Adds the `#[default]` helper macro to the first of an enum's variants.
/// 
pub fn defaultify<'a>(span: Span, iter: impl Iterator<Item = &'a mut syn::ItemEnum>) {
    iter.for_each(|e| {
        if let Some(v) = e.variants.first_mut() {
            v.attrs.extend(util::rust::default_variant(span))
        }
    });
}

#[cfg(test)]
mod tests {
    use std::iter;

    use proc_macro2::Span;
    use quote::ToTokens;
    use syn::punctuated::Punctuated;

    use crate::{
        protocol,
        util::{Context, Rustify},
    };

    use super::{is_self_referential, walk_type_path};

    #[test]
    fn test_referentialty() {
        let test_case: protocol::TypeDeclaration = serde_json::from_str(
            r#"{
            "id": "Referential",
            "type": "object",
            "properties": [
                {
                    "name": "ref_1",
                    "$ref": "Referential"
                },
                {
                    "name": "ref_2",
                    "optional": true,
                    "$ref": "Referential"
                },
                {
                    "name": "callFrames",
                    "description": "JavaScript function name.",
                    "type": "array",
                    "items": {
                        "$ref": "Referential"
                    }
                },
                {
                    "name": "callFrames",
                    "description": "JavaScript function name.",
                    "optional": true,
                    "type": "array",
                    "items": {
                        "$ref": "Referential"
                    }
                }
            ]
        }"#,
        )
        .expect("valid parse");

        let test_case = &mut test_case.rustify(
            proc_macro2::Span::call_site(),
            Context::Domain("Debugger".to_string()).into(),
        )[0];

        println!("{}\n\n", test_case.into_token_stream());

        if let syn::Item::Struct(ref mut s) = test_case {
            let ident = s.ident.clone();
            let p = syn::TypePath {
                qself: None,
                path: ident.into(),
            };

            s.fields
                .iter_mut()
                .filter_map(|f| walk_type_path(&mut f.ty, is_self_referential(p.clone())))
                .filter_map(|f| match f {
                    syn::Type::Path(ref mut p) => Some(p),
                    _ => None,
                })
                .for_each(|t| {
                    let inner = t.clone();
                    t.path.segments = Punctuated::from_iter(iter::once(syn::PathSegment {
                        ident: syn::Ident::new("Box", Span::call_site()),
                        arguments: syn::PathArguments::AngleBracketed(
                            syn::AngleBracketedGenericArguments {
                                colon2_token: Default::default(),
                                lt_token: Default::default(),
                                gt_token: Default::default(),
                                args: Punctuated::from_iter(iter::once(
                                    syn::GenericArgument::Type(syn::Type::Path(inner)),
                                )),
                            },
                        ),
                    }));
                });
        }
        println!("{}\n\n", test_case.into_token_stream());
    }
}
