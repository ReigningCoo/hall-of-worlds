//! A key's form: the lock it opens, and what it remembers for good, whoever
//! carries it later: where it was first found (or made, by Key for This Door)
//! and the door it first locked or unlocked. Recorded by the server only.

use serde::Serialize;

use crate::hall::GateCode;
use crate::id::Id;
use crate::world::WorldId;

mod saved;

/// A key: `ItemForm::Key`. Read through `saved`, so a key saved before it
/// remembered anything (only its lock's id) still loads, as `KeyFound::Unknown`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct KeyForm {
    /// The lock it opens (`door::Lock::id`).
    pub lock: Id,
    /// Where it was first picked up by a character.
    pub found: KeyFound,
    /// The first door it locked or unlocked, and which it has done to it.
    pub used: Option<KeyUse>,
}

/// Whether, and where, a key was first found.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, serde::Deserialize)]
pub enum KeyFound {
    /// No character has had it yet (a mob's, one on the ground, a quest's).
    #[default]
    NotYet,
    /// Found before keys remembered where (an old save): never recorded now.
    Unknown,
    At(KeyPlace),
}

/// Where something happened to a key.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, serde::Deserialize)]
pub enum KeyPlace {
    /// On a planet: at the world map point `(x, y)`, in the place `name` if the
    /// poi there has one (`place_name`). `runes`: the planet's own name
    /// (`GateCode::of_planet`), `None` off the Hall's planets.
    Planet {
        world: WorldId,
        runes: Option<GateCode>,
        name: Option<String>,
        x: i64,
        y: i64,
    },
    /// In the Hall of Worlds.
    Hall,
}

/// The first door a key turned in, and whether it has locked and unlocked it.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, serde::Deserialize)]
pub struct KeyUse {
    pub at: KeyPlace,
    pub locked: bool,
    pub unlocked: bool,
}

/// Which way a key was turned.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Turned {
    Locked,
    Unlocked,
}

impl KeyForm {
    /// A new key to `lock`, which remembers nothing yet.
    pub fn new(lock: Id) -> Self {
        Self {
            lock,
            found: KeyFound::NotYet,
            used: None,
        }
    }

    /// Records where it was found, only the first time: `true` if it did.
    pub fn note_found(&mut self, place: &KeyPlace) -> bool {
        if self.found != KeyFound::NotYet {
            return false;
        }
        self.found = KeyFound::At(place.clone());
        true
    }

    /// Records it turning in its door at `place`: the first door's place is
    /// kept, and each way it has turned. `true` if anything changed.
    pub fn note_turned(&mut self, place: &KeyPlace, turned: Turned) -> bool {
        let used = self.used.get_or_insert_with(|| KeyUse {
            at: place.clone(),
            locked: false,
            unlocked: false,
        });
        let flag = match turned {
            Turned::Locked => &mut used.locked,
            Turned::Unlocked => &mut used.unlocked,
        };
        let changed = !*flag;
        *flag = true;
        changed
    }
}

#[cfg(test)]
mod tests;
