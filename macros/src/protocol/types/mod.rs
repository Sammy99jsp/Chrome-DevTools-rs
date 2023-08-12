//!
//! Type definitions in `protocol.json`.
//!
//! Briefly, types can either be:
//! * declared inline {...}
//! * referenced {$ref: "<type_id>"}
//!

pub mod convention;

pub mod our {
    use serde;
    use serde::de::Error;
    use serde_json::Map;

    use super::convention::{self, Identifier, NamingConvention};

    #[derive(Debug)]
    pub struct TypePath {
        pub(crate) domain: Option<Identifier<convention::DomainIdentitifer>>,
        pub(crate) r#type: Identifier<convention::TypeIdentifier>,
    }

    impl<'de> serde::Deserialize<'de> for TypePath {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            let raw_string = String::deserialize(deserializer)?;
            let (domain, type_name) = raw_string
                .split_once('.')
                .map(|(d, t)| (Some(d), t))
                .unwrap_or((None, &raw_string));

            Ok(Self {
                domain: domain
                    .map(ToString::to_string)
                    .as_ref()
                    .map(convention::DomainIdentitifer::of),
                r#type: convention::TypeIdentifier::of(type_name),
            })
        }
    }

    pub mod types {
        #[derive(Debug, Clone, Copy, serde::Deserialize)]
        pub enum PrimitiveType {
            #[serde(rename = "boolean")]
            Boolean,

            #[serde(rename = "string")]
            String,

            #[serde(rename = "number")]
            Number,

            #[serde(rename = "any")]
            Any,

            #[serde(rename = "integer")]
            Integer,
        }
    }

    #[derive(Debug, thiserror::Error, Clone, Copy)]
    pub enum SemanticError {
        #[error("Field missing its `name` field.")]
        FieldMissingName,

        #[error("Value for optional is not a boolean.")]
        OptionalNotBoolean,

        #[error("Invalid path to type.")]
        InvalidTypePath,

        #[error("Invalid enum declaration.")]
        InvalidEnum,

        #[error("Missing `type` property.")]
        MissingTypeProperty,

        #[error("Invalid array declaration: missing/invalid `items` property.")]
        InvalidArray,

        #[error("Invalid object declaration: invalid `properties` property.")]
        InvalidProperties,

        #[error("Expected primitive type.")]
        ExpectedPrimitiveType,

        #[error("Missing/Invalid `id` field for type declaration.")]
        TypeDeclInvalidId,
    }

    #[derive(Debug)]
    pub struct TypeDeclaration {
        pub id: Identifier<convention::TypeIdentifier>,
        pub description: Option<String>,
        pub typedef: Type,
    }

    impl<'de> serde::Deserialize<'de> for TypeDeclaration {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            // Parse fields individually to allow for the typedef
            // to be mish-mashed into the same object in JSON form.

            let mut map: serde_json::Map<String, serde_json::Value> =
                serde_json::Map::deserialize(deserializer)?;

            let id = get_and_parse::<_, _, D>(&mut map, "id", SemanticError::TypeDeclInvalidId)?;

            let description = get_and_parse::<String, _, D>(
                &mut map,
                "description",
                SemanticError::TypeDeclInvalidId,
            )
            .map(Some)
            .unwrap_or(None);

            let typedef: Type =
                serde_json::from_value(serde_json::to_value(map).expect("Should serialize!"))
                    .map_err(D::Error::custom)?;

            Ok(Self {
                id,
                description,
                typedef,
            })
        }
    }

    #[derive(Debug)]
    pub struct Field {
        pub name: Identifier<convention::FieldIdentifier>,
        pub description: Option<String>,
        pub typedef: Type,
    }

    impl<'de> serde::Deserialize<'de> for Field {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            // Parse fields individually to allow for the typedef
            // to be mish-mashed into the same object in JSON form.

            let mut map: serde_json::Map<String, serde_json::Value> =
                serde_json::Map::deserialize(deserializer)?;

            if !map.contains_key("name") {
                // return an error
                return Err(SemanticError::FieldMissingName).map_err(D::Error::custom);
            }

            let name: Identifier<convention::FieldIdentifier> =
                serde_json::from_value(map.remove("name").unwrap()).map_err(D::Error::custom)?;

            let description = map
                .remove("description")
                .map(|d| {
                    serde_json::from_value::<String>(d)
                        .map_err(D::Error::custom)
                        .map(Option::from)
                })
                .unwrap_or(Ok(None))?;

            let typedef: Type = serde_json::from_value(serde_json::to_value(map).unwrap())
                .map_err(D::Error::custom)?;

            Ok(Self {
                name,
                description,
                typedef,
            })
        }
    }

    #[derive(Debug)]
    pub enum Type {
        ///
        /// Reference to an existing type.
        ///
        Reference {
            ///
            /// Path to type.
            /// Originally `$ref`
            ///
            r#ref: TypePath,
            optional: bool,
        },
        Simple {
            r#type: types::PrimitiveType,
            optional: bool,
        },
        ///
        /// Used here as a union of string literals.
        ///
        /// Will be de'd as an actual Rust `enum`.
        ///
        Enum {
            r#type: types::PrimitiveType,
            optional: bool,
            ///
            /// Contains the valid values for this type.
            ///
            r#enum: Vec<Identifier<convention::TypeIdentifier>>,
        },
        ///
        /// Array type.
        /// TS: `Fruit[]`
        /// Rust: `Vec<Fruit>`
        ///
        Array {
            // type == "array"
            optional: bool,
            items: Box<Type>,
        },
        ///
        /// Structured object type: `{ apples: number, pears: number }`
        ///
        /// Will be transformed into a Rust `struct`.
        /// In TS, known as an `interface`.
        ///
        Object {
            // type == "object"
            optional: bool,

            ///
            /// An object without properties is equivalent to a
            /// Map<String, serde_json::Value>
            ///
            properties: Option<Vec<Field>>,
        },
    }

    fn get_and_parse<'de, V, E, D>(
        map: &mut serde_json::Map<String, serde_json::Value>,
        key: &'static str,
        err: E,
    ) -> Result<V, D::Error>
    where
        D: serde::Deserializer<'de>,
        V: serde::de::DeserializeOwned,
        E: std::fmt::Display + Copy,
    {
        map.remove(key)
            .map(|v| serde_json::from_value::<V>(v).map_err(|_| err))
            .unwrap_or(Err(err))
            .map_err(D::Error::custom)
    }

    impl<'de> serde::Deserialize<'de> for Type {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            let mut raw: Map<String, serde_json::Value> = Map::deserialize(deserializer)?;

            let optional = get_and_parse::<bool, SemanticError, D>(
                &mut raw,
                "optional",
                SemanticError::OptionalNotBoolean,
            )
            .unwrap_or(false);

            // Parse type $reference.
            if let Ok(path) = get_and_parse::<TypePath, SemanticError, D>(
                &mut raw,
                "$ref",
                SemanticError::InvalidTypePath,
            ) {
                return Ok(Self::Reference {
                    r#ref: path,
                    optional,
                });
            }

            let r#type: serde_json::Value =
                get_and_parse::<_, _, D>(&mut raw, "type", SemanticError::MissingTypeProperty)?;

            // Parse array.
            if r#type.as_str().map(|s| s == "array").unwrap_or(false) {
                let items =
                    get_and_parse::<_, _, D>(&mut raw, "items", SemanticError::InvalidArray)?;

                return Ok(Self::Array { optional, items });
            }

            // Parse object.
            if r#type.as_str().map(|s| s == "object").unwrap_or(false) {
                let properties: Option<Vec<Field>> = raw
                    .remove("properties")
                    .map(|v| {
                        serde_json::from_value(v)
                            .map_err(|_| SemanticError::InvalidProperties)
                            .map_err(D::Error::custom)
                            .map(Some)
                    })
                    .unwrap_or(Ok(None))?;

                return Ok(Self::Object {
                    optional,
                    properties,
                });
            }

            let r#type = serde_json::from_value(r#type)
                .map_err(|_| SemanticError::ExpectedPrimitiveType)
                .map_err(D::Error::custom)?;

            // Parse enum.
            if let Ok(r#enum) =
                get_and_parse::<_, _, D>(&mut raw, "enum", SemanticError::InvalidEnum)
            {
                return Ok(Self::Enum {
                    r#type,
                    optional,
                    r#enum,
                });
            }

            // Parse Simple
            Ok(Self::Simple { r#type, optional })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::our;

    #[test]
    fn reference_type() {
        let raw = r#"{
            "name": "sourceCodeLocation",
            "description": "Source code position that referenced the failing stylesheet.",
            "$ref": "SourceCodeLocation"
        }"#;

        let field: our::Field = serde_json::from_str(raw).expect("Valid parse");

        assert!(matches!(field.typedef, our::Type::Reference { .. }));

        let raw = r#"{
            "name": "frameId",
            "description": "The frame for whose document the AX tree should be retrieved.\nIf omited, the root frame is used.",
            "optional": true,
            "$ref": "Page.FrameId"
        }"#;

        let _: our::Field = serde_json::from_str(raw).expect("Valid parse");
    }

    #[test]
    fn simple_type() {
        let raw = r#"{
            "name": "superseded",
            "description": "Whether this source is superseded by a higher priority source.",
            "type": "boolean"
        }"#;

        let field: our::Field = serde_json::from_str(raw).expect("Valid parse");

        assert!(matches!(field.typedef, our::Type::Simple { .. }));

        let raw = r#"{
            "name": "superseded",
            "description": "Whether this source is superseded by a higher priority source.",
            "optional": true,
            "type": "boolean"
        }"#;

        let field: our::Field = serde_json::from_str(raw).expect("Valid parse");

        assert!(matches!(
            field.typedef,
            our::Type::Simple { optional: true, .. }
        ));
    }

    #[test]
    fn enum_type() {
        let raw = r#"{
            "name": "state",
            "description": "Download status.",
            "type": "string",
            "enum": [
                "inProgress",
                "completed",
                "canceled"
            ]
        }"#;

        let field: our::Field = serde_json::from_str(raw).expect("Valid parse");

        assert!(matches!(field.typedef, our::Type::Enum { .. }));
    }

    #[test]
    fn array_type() {
        let raw = r#"{
            "name": "matches",
            "description": "Matches of CSS rules applicable to the pseudo style.",
            "type": "array",
            "items": {
                "$ref": "RuleMatch"
            }
        }"#;

        let field: our::Field = serde_json::from_str(raw).expect("Valid parse");

        assert!(
            matches!(field.typedef, our::Type::Array { items: t, .. } if matches!(*t, our::Type::Reference { .. }))
        );
    }

    #[test]
    fn object_type() {
        let raw = r#"{
            "name": "filterEntry",
            "description": "A filter used by target query/discovery/auto-attach operations.",
            "experimental": true,
            "type": "object",
            "properties": [
                {
                    "name": "exclude",
                    "description": "If set, causes exclusion of mathcing targets from the list.",
                    "optional": true,
                    "type": "boolean"
                },
                {
                    "name": "type",
                    "description": "If not present, matches any type.",
                    "optional": true,
                    "type": "string"
                }
            ]
        }"#;

        let field: our::Field = serde_json::from_str(raw).expect("Valid parse");

        assert!(matches!(field.typedef, our::Type::Object { .. }));
    }

    #[test]
    fn type_decls() {
        let raw = r#"{
            "id": "KeyframesRule",
            "description": "Keyframes Rule",
            "type": "object",
            "properties": [
                {
                    "name": "name",
                    "description": "CSS keyframed animation's name.",
                    "optional": true,
                    "type": "string"
                },
                {
                    "name": "keyframes",
                    "description": "List of animation keyframes.",
                    "type": "array",
                    "items": {
                        "$ref": "KeyframeStyle"
                    }
                }
            ]
        }"#;

        let _: our::TypeDeclaration = serde_json::from_str(raw).expect("Valid parse");

        let raw = r#"{
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
        }"#;

        let _: our::TypeDeclaration = serde_json::from_str(raw).expect("Valid parse");
    }

    #[test]
    fn difficult_def() {
        let raw = r#"{
            "id": "AXPropertyName",
            "description": "Values of AXProperty name:\n- from 'busy' to 'roledescription': states which apply to every AX node\n- from 'live' to 'root': attributes which apply to nodes in live regions\n- from 'autocomplete' to 'valuetext': attributes which apply to widgets\n- from 'checked' to 'selected': states which apply to widgets\n- from 'activedescendant' to 'owns' - relationships between elements other than parent/child/sibling.",
            "type": "string",
            "enum": [
                "busy",
                "disabled",
                "editable",
                "focusable",
                "focused",
                "hidden",
                "hiddenRoot",
                "invalid",
                "keyshortcuts",
                "settable",
                "roledescription",
                "live",
                "atomic",
                "relevant",
                "root",
                "autocomplete",
                "hasPopup",
                "level",
                "multiselectable",
                "orientation",
                "multiline",
                "readonly",
                "required",
                "valuemin",
                "valuemax",
                "valuetext",
                "checked",
                "expanded",
                "modal",
                "pressed",
                "selected",
                "activedescendant",
                "controls",
                "describedby",
                "details",
                "errormessage",
                "flowto",
                "labelledby",
                "owns"
            ]
        }"#;

        let _: our::TypeDeclaration = serde_json::from_str(raw).expect("Valid parse");
    }
}
