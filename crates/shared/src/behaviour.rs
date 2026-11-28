//! How each race of creature behaves (mob behaviour plan PR 1): its drives,
//! who it defends, its alarm call, what it hunts, how it flees, whether fences
//! stop it, whether it fears open fire and what it wants, as data
//! (`behaviour/behaviours.ron`, built in, plus a server's mods by race); its
//! temper towards strangers; and bands, the groups mobs belong to, each rolling
//! a variant of its race with its own allies, rivals and enemies.
//! Only the server uses the data beyond `Temper`, and it is never sent to
//! clients. See `docs/context/shared/behaviour.md` and `docs/context/mob-behaviour.md`.

mod band;
mod def;
mod desire;
mod relation;
mod table;
mod temper;

pub use band::{Band, BandVariant, Family, Member};
pub use def::{Alarm, BehaviourDef, Defends, Desire, Drives, Fences, Flee};
pub use relation::Relation;
pub use table::Behaviours;
pub use temper::Temper;

#[cfg(test)]
mod tests;
