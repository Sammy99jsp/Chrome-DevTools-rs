//!
//! Implementations of [Deserialize](serde::Deserialize)
//! for more complicated (mixed) structures.
//!

use serde::{
    de::{DeserializeOwned, Error},
    Deserialize, Deserializer,
};
use serde_json::Map;

use crate::protocol;

use super::{convention as conv, modular::{self as m, Identifier}};

impl<'de, NC: conv::NamingConvention> Deserialize<'de> for m::NamedIdentifier<NC> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let raw = String::deserialize(deserializer)?;

        Ok(<Self as Identifier>::new(raw))
    }
}

impl<'de> Deserialize<'de> for m::Deperecated {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let b = bool::deserialize(deserializer)?;

        assert!(b, "Deprecated, if defined, should always be `true`.");

        Ok(Self)
    }
}

impl<'de> Deserialize<'de> for m::Experimental {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let b = bool::deserialize(deserializer)?;

        assert!(b, "Experimental, if defined, should always be `true`.");

        Ok(Self)
    }
}

impl<'de> Deserialize<'de> for m::Documentation {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let raw_str = String::deserialize(deserializer)?;
        Ok(Self(raw_str.split('\n').map(ToString::to_string).collect()))
    }
}

trait FlipFlop<T, E> {
    fn flip(self) -> Result<Option<T>, E>;
}

impl<T, E> FlipFlop<T, E> for Option<Result<T, E>> {
    fn flip(self) -> Result<Option<T>, E> {
        match self {
            Some(Ok(t)) => Ok(Some(t)),
            Some(Err(e)) => Err(e),
            None => Ok(None),
        }
    }
}

///
/// Take a key from a Map, and attempt to deserialize it.
///
trait TakeAndDeserialize {
    ///
    /// If present, attempt to deserialize a value.
    /// If we encounter an error whilst deserializing, throw it.
    ///
    fn take_optional<'de, D, F>(&mut self, key: &'static str) -> Result<Option<D>, F::Error>
    where
        D: DeserializeOwned,
        F: Deserializer<'de>;

    ///
    /// Attempt to seserialize a value: if not present, return with an error message.
    ///
    fn take<'de, D, F>(&mut self, key: &'static str, err_msg: &'static str) -> Result<D, F::Error>
    where
        D: DeserializeOwned,
        F: Deserializer<'de>,
    {
        self.take_optional::<D, F>(key)
            .and_then(|r| r.map(Ok).unwrap_or(Err(F::Error::custom(err_msg))))
    }
}

impl TakeAndDeserialize for Map<String, serde_json::Value> {
    fn take_optional<'de, D, F>(&mut self, key: &'static str) -> Result<Option<D>, F::Error>
    where
        D: DeserializeOwned,
        F: Deserializer<'de>,
    {
        self.remove(key)
            .map(serde_json::from_value)
            .map(|r| r.map_err(F::Error::custom))
            .flip()
    }
}

impl<'de> Deserialize<'de> for m::TypePath {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = String::deserialize(deserializer)?;

        let (domain, ty) = raw
            .split_once('.')
            .map(|(domain, ty)| {
                (
                    Some(m::NamedIdentifier::<conv::Domain>::new(domain)),
                    m::NamedIdentifier::<conv::Type>::new(ty),
                )
            })
            .unwrap_or_else(|| (None, m::NamedIdentifier::<conv::Type>::new(raw)));

        Ok(Self(domain, ty))
    }
}

impl<'de> Deserialize<'de> for protocol::Primitive {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use protocol::Primitive::*;
        let raw = std::string::String::deserialize(deserializer)?;

        match raw.as_str() {
            "string" => Ok(String),
            "number" => Ok(Number),
            "boolean" => Ok(Boolean),
            "integer" => Ok(Integer),
            "any" => Ok(Any),
            _ => Err(D::Error::custom(&format!("Invalid primitive type `{raw}`"))),
        }
    }
}

impl<'de> Deserialize<'de> for super::Type {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use super::Type::*;
        let mut raw: Map<String, serde_json::Value> = Deserialize::deserialize(deserializer)?;

        let optional: bool = raw.take_optional::<_, D>("optional")?.unwrap_or(false);

        if let Some(path) = raw.take_optional::<m::TypePath, D>("$ref")? {
            return Ok(Reference { path, optional });
        }

        if let Some(values) = raw.take_optional::<Vec<m::NamedIdentifier<conv::Type>>, D>("enum")? {
            return Ok(Enum { optional, values });
        }

        let declared_type = raw
            .get("type")
            .and_then(serde_json::Value::as_str)
            .expect("Must have a `type` field");

        if declared_type == "array" {
            let inner: super::Type =
                raw.take::<_, D>("items", "Array type declaration missing `items` field")?;

            return Ok(Array {
                item_type: Box::new(inner),
                optional,
            });
        }

        if declared_type == "object" {
            return Ok(Object {
                fields: raw.take_optional::<_, D>("properties")?,
                optional,
            });
        }

        // Parse as primitive type.
        Ok(Primitive {
            ty: raw.take::<_, D>("type", "Type missing `type` field")?,
            optional,
        })
    }
}

impl<'de> Deserialize<'de> for super::Field {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let mut raw: Map<String, serde_json::Value> = Deserialize::deserialize(deserializer)?;

        Ok(Self {
            name: raw.take::<_, D>("name", "Missing field `name` from Field declaration")?,
            description: raw.take_optional::<_, D>("description")?,
            experimental: raw.take_optional::<_, D>("experimental")?,
            deprecated: raw.take_optional::<_, D>("deprecated")?,
            ty: serde_json::from_value(serde_json::Value::Object(raw)).map_err(D::Error::custom)?,
        })
    }
}

impl<'de> Deserialize<'de> for super::TypeDeclaration {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let mut raw: Map<String, serde_json::Value> = Deserialize::deserialize(deserializer)?;

        Ok(Self {
            id: raw.take::<_, D>("id", "Missing field `id` from TypeDeclaration declaration")?,
            description: raw.take_optional::<_, D>("description")?,
            experimental: raw.take_optional::<_, D>("experimental")?,
            deprecated: raw.take_optional::<_, D>("deprecated")?,
            ty: serde_json::from_value(serde_json::Value::Object(raw)).map_err(D::Error::custom)?,
        })
    }
}
