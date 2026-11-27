//! Reading an item's set. Messages always have `Option<SetPiece>`. A save
//! may also have one from before sets had pieces (2026-11-27): `set: true`,
//! which becomes a piece of a set chosen from the item's id
//! (`SetPiece::fitting`).

use std::fmt;

use serde::de::{self, MapAccess, Visitor};
use serde::{Deserialize, Deserializer};

use super::SetPiece;
use crate::id::Id;
use crate::item::{ItemKind, Quality};

/// An item's set as written.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum SavedSet {
    /// An old save's flag.
    Old(bool),
    Piece(Option<SetPiece>),
}

impl Default for SavedSet {
    fn default() -> Self {
        Self::Piece(None)
    }
}

impl SavedSet {
    /// The piece it is: an old save's set item gets one fitting its kind
    /// and quality, if any set has a piece of its kind.
    pub(crate) fn piece(
        self,
        kind: ItemKind,
        quality: Option<Quality>,
        id: Id,
    ) -> Option<SetPiece> {
        match self {
            Self::Old(false) => None,
            Self::Old(true) => SetPiece::fitting(kind, quality, id),
            Self::Piece(piece) => piece,
        }
    }
}

impl<'de> Deserialize<'de> for SavedSet {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        if !deserializer.is_human_readable() {
            return Option::<SetPiece>::deserialize(deserializer).map(Self::Piece);
        }
        deserializer.deserialize_any(OldOrNew)
    }
}

/// A save's set: an old flag, nothing, or a piece.
struct OldOrNew;

impl<'de> Visitor<'de> for OldOrNew {
    type Value = SavedSet;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("a set piece, nothing, or true or false")
    }

    fn visit_bool<E: de::Error>(self, old: bool) -> Result<SavedSet, E> {
        Ok(SavedSet::Old(old))
    }

    fn visit_none<E: de::Error>(self) -> Result<SavedSet, E> {
        Ok(SavedSet::Piece(None))
    }

    fn visit_unit<E: de::Error>(self) -> Result<SavedSet, E> {
        Ok(SavedSet::Piece(None))
    }

    fn visit_some<D: Deserializer<'de>>(self, inner: D) -> Result<SavedSet, D::Error> {
        SetPiece::deserialize(inner).map(|piece| SavedSet::Piece(Some(piece)))
    }

    fn visit_map<A: MapAccess<'de>>(self, map: A) -> Result<SavedSet, A::Error> {
        let piece = SetPiece::deserialize(de::value::MapAccessDeserializer::new(map))?;
        Ok(SavedSet::Piece(Some(piece)))
    }
}
