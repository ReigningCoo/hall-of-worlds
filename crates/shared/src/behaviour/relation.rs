//! `Relation`: how one member of a band takes to another, from the first's
//! race and band variant.

use serde::{Deserialize, Serialize};

use super::{BandVariant, Behaviours, Member};

/// How one creature takes to another, from the first one's side.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Relation {
    /// The same band.
    Kin,
    /// Its band's variant lists the other's race as allies.
    Ally,
    Neutral,
    /// Contests the same ground: other bands of its race, by its data, or
    /// races its variant lists as rivals.
    Rival,
    /// Its band's variant lists the other's race as enemies.
    Enemy,
}

impl Behaviours {
    /// How `one` takes to `other`: the same band is `Kin`; then what `one`'s
    /// band variant lists the other's race as; then, for another band of its
    /// own race, its race's `other_bands`; else `Neutral`. Not always the same
    /// both ways round.
    pub fn relation(&self, one: Member, other: Member) -> Relation {
        if one.kind == other.kind && one.band == other.band {
            return Relation::Kin;
        }
        let def = self.of(one.kind);
        if let Some(variant) = BandVariant::rolled(&def.bands, one.band) {
            let listed = |races: &[_]| races.contains(&other.kind);
            if listed(&variant.enemies) {
                return Relation::Enemy;
            }
            if listed(&variant.rivals) {
                return Relation::Rival;
            }
            if listed(&variant.allies) {
                return Relation::Ally;
            }
        }
        if one.kind == other.kind {
            def.other_bands
        } else {
            Relation::Neutral
        }
    }
}
