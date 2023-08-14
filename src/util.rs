use serde::{
    de::{DeserializeOwned, Error},
    Deserialize, Serialize,
};

///
/// Any event sent by the server to the client.
/// 
pub trait Event : Serialize {
    fn id(&self) -> &'static str
        where Self: Sized;
}


///
/// Links a Command to its Parameter and Return types.
///  
pub trait Command {
    type Parameters: DeserializeOwned;
    type Returns: Serialize;
    type Error: Serialize;

    fn id() -> &'static str
        where Self: Sized;
}

#[derive(Debug, Clone)]
pub struct Nothing;

impl<'de> Deserialize<'de> for Nothing {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let map: serde_json::Map<String, serde_json::Value> =
            Deserialize::deserialize(deserializer)?;
        if map.is_empty() {
            Ok(Self)
        } else {
            Err(D::Error::custom("Nothing struct has content!"))
        }
    }
}

impl Serialize for Nothing {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serde_json::Map::<String, serde_json::Value>::new().serialize(serializer)
    }
}

#[derive(Debug, Clone)]
pub enum Infallible {}

impl Serialize for Infallible {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serde_json::Value::Null.serialize(serializer)
    }
}
