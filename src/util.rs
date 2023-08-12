#[derive(Debug, serde::Serialize)]
pub struct Nothing;

use serde::de::Error;

impl<'de> serde::Deserialize<'de> for Nothing {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let map: serde_json::Map<String, serde_json::Value> =
            serde_json::Map::deserialize(deserializer)?;
        if !map.is_empty() {
            return Err(D::Error::custom("Expected empty params."));
        }

        Ok(Self)
    }
}

pub trait Command {
    type Parameters: serde::de::DeserializeOwned;
    type Returns: serde::ser::Serialize;
    type Error: serde::ser::Serialize = Infallible;

    fn id() -> &'static str;
}

pub type CommandExecutor<C> = Box<
    dyn Fn(<C as Command>::Parameters) -> Result<<C as Command>::Returns, <C as Command>::Error>,
>;

pub trait IntoCommandExecutor<C: Command> {
    fn into_executor(self) -> CommandExecutor<C>;
}

#[derive(Debug, serde::Serialize)]
pub enum Infallible {}

impl<C, F> IntoCommandExecutor<C> for F
where
    C: Command,
    F: Fn(<C as Command>::Parameters) -> Result<<C as Command>::Returns, <C as Command>::Error>
        + 'static,
{
    fn into_executor(self) -> CommandExecutor<C> {
        Box::new(self)
    }
}