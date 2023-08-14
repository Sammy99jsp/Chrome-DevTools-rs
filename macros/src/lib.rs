#![feature(return_position_impl_trait_in_trait, iter_intersperse)]
mod protocol;
mod util;
// mod protocol_test;

#[cfg(test)]
mod tests {
    use std::fs;

    use proc_macro2::Span;
    use quote::ToTokens;

    use crate::{
        protocol as p,
        // protocol_test as pt,
        util::{Context::Item, Rustify},
    };

    // #[test]
    // fn deprecated_test() {
    //     let a = pt::dom::GetFlattenedDocument;
    // }

    #[test]
    fn parse_type_decl() {
        let _: p::TypeDeclaration = serde_json::from_str(
            r#"{
            "id": "AXValueType",
            "description": "Enum of possible property types.",
            "type": "string",
            "enum": [
                "boolean",
                "tristate",
                "booleanOrUndefined",
                "idref",
                "idrefList",
                "integer",
                "node",
                "nodeList",
                "number",
                "string",
                "computedString",
                "token",
                "tokenList",
                "domRelation",
                "role",
                "internalRole",
                "valueUndefined"
            ]
        }"#,
        )
        .expect("Valid parse");

        let f: p::Field = serde_json::from_str(
            r#"{
            "name": "data",
            "description": "Object containing break-specific auxiliary properties.",
            "optional": true,
            "type": "object"
        }"#,
        )
        .expect("Valid parse");

        let (f, _) = f.rustify(
            Span::call_site(),
            Some(Item("debugger".to_string(), "PausedEvent".to_string())),
        );

        println!("{}", f.to_token_stream());
    }

    #[test]
    fn parse_protocol() {
        let mut p = serde_json::from_str::<p::Protocol>(include_str!("../test/protocol.json"))
            .expect("Valid parse")
            .rustify(Span::call_site(), None);
        let j = serde_json::from_str::<p::Protocol>(include_str!("../test/js_protocol.json"))
            .expect("Valid parse")
            .rustify(Span::call_site(), None);
        p.items.extend(j.items);

        fs::write("protocol_test.rs", prettyplease::unparse(&p)).expect("test file writable");
    }
}
