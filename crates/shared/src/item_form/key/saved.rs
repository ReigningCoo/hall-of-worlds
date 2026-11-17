//! Reading a `KeyForm`. Messages always have the whole form. A save may also
//! have a key from before keys remembered anything, written as only its lock's
//! id (`Key("…")`): that reads as a key found somewhere no longer known.

use std::fmt;

use serde::de::{self, MapAccess, Visitor};
use serde::{Deserialize, Deserializer};

use super::{KeyForm, KeyFound, KeyUse};
use crate::id::Id;

/// The form's fields as written.
#[derive(Deserialize)]
#[serde(rename = "KeyForm")]
struct Fields {
    lock: Id,
    #[serde(default)]
    found: KeyFound,
    #[serde(default)]
    used: Option<KeyUse>,
}

impl From<Fields> for KeyForm {
    fn from(fields: Fields) -> Self {
        Self {
            lock: fields.lock,
            found: fields.found,
            used: fields.used,
        }
    }
}

impl<'de> Deserialize<'de> for KeyForm {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        if !deserializer.is_human_readable() {
            return Fields::deserialize(deserializer).map(Into::into);
        }
        deserializer.deserialize_any(OldOrNew)
    }
}

/// A save's key: an old one's id as text, or the fields.
struct OldOrNew;

impl<'de> Visitor<'de> for OldOrNew {
    type Value = KeyForm;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("a key's form, or the id of its lock")
    }

    fn visit_str<E: de::Error>(self, text: &str) -> Result<KeyForm, E> {
        let lock = Id::parse(text).ok_or_else(|| E::custom("not a lock's id"))?;
        Ok(KeyForm {
            lock,
            found: KeyFound::Unknown,
            used: None,
        })
    }

    fn visit_map<A: MapAccess<'de>>(self, map: A) -> Result<KeyForm, A::Error> {
        let fields = Fields::deserialize(de::value::MapAccessDeserializer::new(map))?;
        Ok(fields.into())
    }
}
