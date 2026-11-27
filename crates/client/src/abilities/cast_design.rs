//! The spell design panel's Cast (`spell_design::CastDesign`): the design is
//! queued as a spell slot pressed (`Manual`, `BarThing::Spell` of its key),
//! so it is aimed the same way (`spells::hand_aim`), walks into reach first
//! (`approach.rs`) and waits its turn. It is cast as the book's spell when the
//! book holds it, else as it is (`SpellbookAsk::CastDesign`).

use bevy::prelude::*;
use shared::ability::BarThing;
use shared::spell::SpellKey;

use super::manual::Manual;
use crate::spell_design::CastDesign;

/// Queues each design cast, kept on `Manual` until the next.
pub(super) fn take_cast_designs(mut cast: MessageReader<CastDesign>, mut manual: ResMut<Manual>) {
    for CastDesign(spell) in cast.read() {
        let thing = BarThing::Spell(SpellKey::of(spell));
        manual.design = Some(spell.clone());
        if !manual.queued.contains(&thing) {
            manual.press(thing);
        }
    }
}
