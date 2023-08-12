//!
//! Structs for parsing the `protoco.json` file,
//! which can then be proc-macro'd into Rust code (hopefully).
//!

use self::types::{
    convention::{self, Identifier},
    our,
};

pub mod rustify;
pub mod types;

///
/// Specifies the version of the DevTools Protocol.
///
#[derive(Debug, serde::Deserialize)]
pub struct ProtocolVersion {
    major: String,
    minor: String,
}

///
/// Protocol definition.
///
#[derive(Debug, serde::Deserialize)]
pub struct Protocol {
    version: ProtocolVersion,
    domains: Vec<Domain>,
}

///
/// Command sent over JSONrpc, with paramters sent,
/// and retrurn values (both objects).
///
#[derive(Debug, serde::Deserialize)]
pub struct Command {
    pub name: Identifier<convention::CommandIdentifier>,
    pub description: Option<String>,
    pub parameters: Option<Vec<our::Field>>,
    pub returns: Option<Vec<our::Field>>,
}

///
/// Event fired from the server to the client.
///
#[derive(Debug, serde::Deserialize)]
pub struct Event {
    pub name: Identifier<convention::EventIdentifier>,
    pub description: Option<String>,
    pub parameters: Option<Vec<our::Field>>,
}

///
/// Each domain defines a number of commands it supports and events it generates.
///
#[derive(Debug, serde::Deserialize)]
pub struct Domain {
    domain: Identifier<convention::DomainIdentitifer>,

    #[serde(default)]
    types: Vec<our::TypeDeclaration>,

    #[serde(default)]
    commands: Vec<Command>,

    #[serde(default)]
    events: Vec<Event>,

    #[serde(default)]
    dependencies: Vec<Identifier<convention::DomainIdentitifer>>,
}

#[cfg(test)]
mod tests {
    #[test]
    fn parse_command() {
        let raw = r#"{
            "name": "getPartialAXTree",
            "description": "Fetches the accessibility node and partial accessibility tree for this DOM node, if it exists.",
            "experimental": true,
            "parameters": [
                {
                    "name": "nodeId",
                    "description": "Identifier of the node to get the partial accessibility tree for.",
                    "optional": true,
                    "$ref": "DOM.NodeId"
                },
                {
                    "name": "backendNodeId",
                    "description": "Identifier of the backend node to get the partial accessibility tree for.",
                    "optional": true,
                    "$ref": "DOM.BackendNodeId"
                },
                {
                    "name": "objectId",
                    "description": "JavaScript object id of the node wrapper to get the partial accessibility tree for.",
                    "optional": true,
                    "$ref": "Runtime.RemoteObjectId"
                },
                {
                    "name": "fetchRelatives",
                    "description": "Whether to fetch this node's ancestors, siblings and children. Defaults to true.",
                    "optional": true,
                    "type": "boolean"
                }
            ],
            "returns": [
                {
                    "name": "nodes",
                    "description": "The `Accessibility.AXNode` for this DOM node, if it exists, plus its ancestors, siblings and\nchildren, if requested.",
                    "type": "array",
                    "items": {
                        "$ref": "AXNode"
                    }
                }
            ]
        }"#;

        let cmd: super::Command = serde_json::from_str(raw).expect("Valid parse!");

        println!("{cmd:#?}")
    }

    #[test]
    fn parse_protocol() {
        let raw = include_str!("../../test/protocol.json");
        let _: super::Protocol = serde_json::from_str(raw).expect("Valid parse!");
    }
}
