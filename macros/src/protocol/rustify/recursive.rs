use std::iter;

use syn::punctuated::Punctuated;

fn type_check<'a, 'b>(
    ty: &'a mut syn::Type,
    against: &'b syn::Ident,
) -> Vec<&'a mut syn::PathSegment>
where
    'b: 'a,
{
    match ty {
        syn::Type::Array(a) => type_check(a.elem.as_mut(), against),
        syn::Type::BareFn(_) => vec![],
        syn::Type::Group(g) => type_check(g.elem.as_mut(), against),
        syn::Type::ImplTrait(_) => vec![],
        syn::Type::Infer(_) => vec![],
        syn::Type::Macro(_) => vec![],
        syn::Type::Never(_) => vec![],
        syn::Type::Paren(p) => type_check(p.elem.as_mut(), against),
        syn::Type::Path(p) => flatten_path(&mut p.path, against),
        syn::Type::Ptr(_) => vec![],
        syn::Type::Reference(_) => vec![],
        syn::Type::Slice(p) => type_check(p.elem.as_mut(), against),
        syn::Type::TraitObject(_) => vec![],
        syn::Type::Tuple(t) => t
            .elems
            .iter_mut()
            .flat_map(|t| type_check(t, against).into_iter())
            .collect(),
        syn::Type::Verbatim(_) => vec![],
        _ => vec![],
    }
}

fn flatten_path<'a, 'b>(
    p: &'a mut syn::Path,
    against: &'b syn::Ident,
) -> Vec<&'a mut syn::PathSegment>
where
    'b: 'a,
{
    let segs = &p.segments;

    let r = segs.last().map(|s| s.ident.eq(against)).unwrap_or_default();

    if r {
        return p
            .segments
            .last_mut()
            .into_iter()
            .collect();
    }

    let out = p.segments.iter_mut().collect::<Vec<_>>();
    let mut internals = out.into_iter().flat_map(|s| match &mut s.arguments {
        syn::PathArguments::None => vec![],
        syn::PathArguments::AngleBracketed(c) => c
            .args
            .iter_mut()
            .flat_map(|s| match s {
                syn::GenericArgument::Type(syn::Type::Path(t)) => {
                    flatten_path(&mut t.path, against)
                }
                syn::GenericArgument::Type(t) => type_check(t, against),
                _ => vec![],
            })
            // Add exception for Vec<OurType> since Vec<T> is already heap-allocated, thanks clippy! 
            .filter(|_| s.ident != "Vec")
            .collect::<Vec<_>>(),
        syn::PathArguments::Parenthesized(_) => vec![],
    });

    let a = internals.find(|s| s.ident.eq(against));

    a.into_iter().collect()
}

pub fn boxify_struct_recursion(s: &mut syn::ItemStruct) {
    let against = &s.ident;
    let to_boxify = s
        .fields
        .iter_mut()
        .map(|f| &mut f.ty)
        .flat_map(|ty| type_check(ty, against).into_iter())
        .collect::<Vec<_>>();

    to_boxify.into_iter().for_each(|s| {
        let inner = s.clone();
        s.ident = syn::Ident::new("Box", s.ident.span());
        s.arguments = syn::PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments {
            colon2_token: None,
            lt_token: Default::default(),
            gt_token: Default::default(),
            args: Punctuated::from_iter(iter::once(syn::GenericArgument::Type(
                syn::Type::Path(syn::TypePath {
                    qself: None,
                    path: syn::Path {
                        leading_colon: None,
                        segments: Punctuated::from_iter(iter::once(inner)),
                    },
                })
            ))),
        })
    });
}

#[cfg(test)]
mod tests {
    fn small_file(it: syn::Item) -> syn::File {
        syn::File {
            shebang: None,
            attrs: vec![],
            items: vec![it],
        }
    }

    fn pretty_print(it: &syn::Item) -> String {
        prettyplease::unparse(&small_file(it.clone()))
    }

    use crate::protocol::{
        rustify::{recursive::boxify_struct_recursion as boxify, Context, Rustify},
        types::our,
    };

    #[test]
    fn find_me_a_node() {
        let node : our::TypeDeclaration = serde_json::from_str(r#"{
            "id": "StackTrace",
            "description": "Call frames for assertions or error messages.",
            "type": "object",
            "properties": [
                {
                    "name": "description",
                    "description": "String label of this stack trace. For async traces this may be a name of the function that\ninitiated the async call.",
                    "optional": true,
                    "$ref": "StackTrace"
                },
                {
                    "name": "callFrames",
                    "description": "JavaScript function name.",
                    "type": "array",
                    "items": {
                        "$ref": "StackTrace"
                    }
                },
                {
                    "name": "parent",
                    "description": "Asynchronous JavaScript stack trace that preceded this stack, if available.",
                    "optional": true,
                    "$ref": "StackTrace"
                },
                {
                    "name": "parentId",
                    "description": "Asynchronous JavaScript stack trace that preceded this stack, if available.",
                    "experimental": true,
                    "optional": true,
                    "$ref": "StackTrace"
                }
            ]
        }"#).expect("Parse");

        let (struct_repr, _) = node.rustify(
            proc_macro2::Span::call_site(),
            Some(Context::Domain("runtime".to_string())),
        );
        println!("{}", pretty_print(&struct_repr));

        let mut struct_repr = match struct_repr {
            syn::Item::Struct(s) => s,
            _ => unimplemented!(),
        };

        boxify(&mut struct_repr);
        println!("----------------------");
        println!("{}", pretty_print(&syn::Item::Struct(struct_repr)))
    }
}
