use std::marker::PhantomData;

use convert_case::{Case, Casing};

#[derive(Debug, Clone)]
pub struct Identifier<NC: NamingConvention>(pub String, pub PhantomData<NC>);


impl<NC: NamingConvention> Identifier<NC> {
    pub fn rust(self, _: impl Into<proc_macro2::Span>) -> syn::Ident
    where
        Self: Sized,
    {
        syn::parse_str(&NC::to_case(&self.0))
            .or_else(|_| {
                syn::parse_str(&format!("{}_", self.0))
            })
            .expect("Could not escape ident")
    }

    pub fn rust_and_original(self, span: impl Into<proc_macro2::Span>) -> (syn::Ident, String) {
        (self.clone().rust(span.into()), self.0)
    }
}

pub trait NamingConvention: Clone {
    const TO_CASE: Case;

    fn to_case(raw: &impl ToString) -> String {
        raw.to_string().to_case(Self::TO_CASE)
    }

    fn of(s: impl ToString) -> Identifier<Self>
    where
        Self: Sized,
    {
        Identifier(s.to_string(), PhantomData::<Self>)
    }
}

impl<'de, NC: NamingConvention> serde::Deserialize<'de> for Identifier<NC> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(Self(String::deserialize(deserializer)?, PhantomData::<NC>))
    }
}

#[derive(Debug, Copy, Clone)]
pub struct DomainIdentitifer;

impl NamingConvention for DomainIdentitifer {
    const TO_CASE: Case = Case::Snake;
}

#[derive(Debug, Copy, Clone)]
pub struct TypeIdentifier;

impl NamingConvention for TypeIdentifier {
    const TO_CASE: Case = Case::Pascal;
}

#[derive(Debug, Copy, Clone)]
pub struct CommandIdentifier;

impl NamingConvention for CommandIdentifier {
    const TO_CASE: Case = Case::Pascal;
}

#[derive(Debug, Copy, Clone)]
pub struct FieldIdentifier;

// Rustlang keywords from this Gist:
// https://gist.github.com/ritz078/1be714dea593838587c8a5df463a583a

impl NamingConvention for FieldIdentifier {
    const TO_CASE: Case = Case::Snake;
}

#[derive(Debug, Copy, Clone)]
pub struct EventIdentifier;
impl NamingConvention for EventIdentifier {
    const TO_CASE: Case = Case::Pascal;

    fn to_case(raw: &impl ToString) -> String {
        let ident = raw.to_string();
        format!("{}Event", ident.to_case(Self::TO_CASE))
    }
}

#[cfg(test)]
mod tests {
}
