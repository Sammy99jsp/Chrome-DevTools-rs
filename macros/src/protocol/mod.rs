pub mod convention;
pub mod modular;
pub mod parsing;
pub mod rustify;
mod post_ast;

use convention as conv;
use modular as m;

///
/// The simplest of types.
///
#[derive(Debug, Clone, Copy)]
pub enum Primitive {
    ///
    /// Boolean type ([bool]).
    ///
    Boolean,

    ///
    /// JavaScript `number` type:
    /// floating-point number type ([f64]).
    ///
    Number,

    ///
    /// Integer type ([i64]).
    ///
    Integer,

    ///
    /// String type ([String]).
    ///
    String,

    ///
    /// Any type ([serde_json::Value])
    /// 
    Any,
}

///
/// All type information for a field.
///
/// Can ocassionally return an extra enum
/// (if an enum was declared inline in a field).
///
#[derive(Debug, Clone)]
pub enum Type {
    ///
    /// Simple type.
    ///
    Primitive { ty: Primitive, optional: bool },
    ///
    /// Reference to another type.
    ///
    Reference {
        path: modular::TypePath,
        optional: bool,
    },
    ///
    /// Array of an inner type.
    ///
    Array {
        item_type: Box<Type>,
        optional: bool,
    },
    ///
    /// Complex type: Map<String => Type>
    /// made of fields.
    ///
    /// Equivalent to a struct.
    ///
    Object { fields: Option<Vec<Field>>, optional: bool },
    ///
    /// Enum of different values of the same type.
    ///
    /// For now, only string enums are allowed.
    ///
    Enum {
        values: Vec<modular::Identifier<conv::Type>>,
        optional: bool,
    },
}

///
/// Part of a larger type definition,
/// a id => Type association, with optional
/// attributes (`experimental`, `deprecated`)
/// and documentation.
///
#[derive(Debug, Clone)]
pub struct Field {
    ///
    /// Identifier for this field.
    ///
    name: modular::Identifier<conv::Field>,

    //
    /// Optional documentation.
    ///
    description: Option<m::Documentation>,

    ///
    /// `Experimental` flag for non-stable items
    /// (translated into the `#[experimental]` macro).
    ///
    experimental: Option<m::Experimental>,

    ///
    /// `Deprecated` flag (translated into the `#[deprecated]` rustdoc macro).
    ///
    deprecated: Option<m::Deperecated>,

    ///
    /// Type for this field.
    ///
    ty: Type,
}

///
/// A type alias, struct, or enum.
///
#[derive(Debug, Clone)]
pub struct TypeDeclaration {
    ///
    /// Identifier for this type declaration.
    ///
    id: m::Identifier<conv::Type>,

    //
    /// Optional documentation.
    ///
    description: Option<m::Documentation>,

    ///
    /// `Experimental` flag for non-stable items
    /// (translated into the `#[experimental]` macro).
    ///
    experimental: Option<m::Experimental>,

    ///
    /// `Deprecated` flag (translated into the `#[deprecated]` rustdoc macro).
    ///
    deprecated: Option<m::Deperecated>,

    ///
    /// The type defined.
    ///
    ty: Type,
}

///
/// Also known as a `Method`.
///
/// Commands are called by a client, and a responded to by the server.
///
/// * Has its own Optional Parameter and Return types.
///
/// * Represented by multiple structs (Parameters, Returns, and a unit struct for itself.)
/// Uses a `Command` trait to link the parameter and return types.
///
#[derive(Debug, Clone, serde::Deserialize)]
pub struct Command {
    ///
    /// This command's identifier.
    ///
    name: m::Identifier<conv::Command>,

    //
    /// Optional documentation.
    ///
    description: Option<m::Documentation>,

    ///
    /// `Experimental` flag for non-stable items
    /// (translated into the `#[experimental]` macro).
    ///
    experimental: Option<m::Experimental>,

    ///
    /// `Deprecated` flag (translated into the `#[deprecated]` rustdoc macro).
    ///
    deprecated: Option<m::Deperecated>,

    ///
    /// Optional parameters this command needs.
    ///
    parameters: Option<Vec<Field>>,

    ///
    /// Optional return type.
    ///
    returns: Option<Vec<Field>>,
}

///
/// Sent from the server to the client.
/// Has its own parameeter type.
///
/// Represented as a struct of its parameters.
///
#[derive(Debug, Clone, serde::Deserialize)]
pub struct Event {
    ///
    /// This event's identifier, with the additional naming convention:
    /// `${EVENT}Event`.
    ///
    name: m::Identifier<conv::Event>,

    //
    /// Optional documentation.
    ///
    description: Option<m::Documentation>,

    ///
    /// `Experimental` flag for non-stable items
    /// (translated into the `#[experimental]` macro).
    ///
    experimental: Option<m::Experimental>,

    ///
    /// `Deprecated` flag (translated into the `#[deprecated]` rustdoc macro).
    ///
    deprecated: Option<m::Deperecated>,

    ///
    /// Optional structured data for this event.
    ///
    parameters: Option<Vec<Field>>,
}

///
/// Listed dependency in a domain.
///
/// Represented as a `use` item.
///
#[derive(Debug, Clone, serde::Deserialize)]
pub struct DomainDependency(m::Identifier<conv::Domain>);

///
/// A collection of [TypeDeclaration], [Command], [Event] types,
/// with optional [DomainDependencies](DomainDependency).
///
#[derive(Debug, Clone, serde::Deserialize)]
pub struct Domain {
    ///
    /// Identifier for this domain.
    ///
    domain: m::Identifier<conv::Domain>,

    ///
    /// Optional documentation.
    ///
    description: Option<m::Documentation>,

    ///
    /// `Experimental` flag for non-stable items
    /// (translated into the `#[experimental]` macro).
    ///
    experimental: Option<m::Experimental>,

    ///
    /// `Deprecated` flag (translated into the `#[deprecated]` rustdoc macro).
    ///
    deprecated: Option<m::Deperecated>,

    ///
    /// Other [Domains](Domain) this domain references.
    ///
    dependencies: Option<Vec<DomainDependency>>,

    ///
    /// Type declarations present in this domain.
    ///
    types: Option<Vec<TypeDeclaration>>,

    ///
    /// Commands present in this domain.
    ///
    commands: Option<Vec<Command>>,

    ///
    /// Events present in this domain.
    ///
    events: Option<Vec<Event>>,
}

///
/// (Stable) version of this protocol.
///
#[derive(Debug, Clone, serde::Deserialize)]
pub struct ProtocolVersion {
    major: String,
    minor: String,
}

impl ToString for ProtocolVersion {
    fn to_string(&self) -> String {
        format!("{}.{}", self.major, self.minor)
    }
}

///
/// Definition of the Chrome DevTools protocol.
///
#[derive(Debug, Clone, serde::Deserialize)]
pub struct Protocol {
    ///
    /// Current version.
    ///
    version: ProtocolVersion,

    ///
    /// Domain definitions.
    ///
    domains: Vec<Domain>,
}
