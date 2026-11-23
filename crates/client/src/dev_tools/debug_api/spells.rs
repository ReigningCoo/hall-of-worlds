//! Casting designed spells without the design panel (magic plan PR 7, to
//! look at how wheres are drawn), and setting every skill so they unlock.

use bevy::prelude::*;
use serde::Deserialize;
use server::debug_api::{BrpResult, Value, params};
use shared::id::Id;
use shared::protocol::{ClientMessage, DevSkills, DevVitals, SpellbookAsk};
use shared::spell::{Spell, SpellKey};

use super::done;
use crate::connection::ToServer;

#[derive(Deserialize)]
pub(super) struct CastParams {
    /// A design as `shared::spell::Spell` is saved:
    /// `{"steps": [{"Where": {"place": "Beam", "count": 2, "filters":
    /// {"caster": false, "friend": false, "enemy": false}}},
    /// {"Effects": {"effects": [["Fire", 1]]}}]}`.
    spell: Spell,
    /// A creature's id (`server.mobs`); none at the caster or ahead.
    target: Option<Id>,
}

/// `client.cast {spell, target?}`: stores the spell in the book (refused
/// quietly if it is there already, or not allowed by the skills), fills
/// Health and Mana, and casts it, as the action bar would.
pub(super) fn cast(
    In(request): In<Option<Value>>,
    mut to_server: MessageWriter<ToServer>,
) -> BrpResult {
    let CastParams { spell, target } = params(request)?;
    let key = SpellKey::of(&spell);
    let store = SpellbookAsk::Store { spell, name: None };
    to_server.write(ToServer(ClientMessage::Spellbook(store)));
    to_server.write(ToServer(ClientMessage::DevVitals(DevVitals::Fill)));
    let cast = SpellbookAsk::Cast { spell: key, target };
    to_server.write(ToServer(ClientMessage::Spellbook(cast)));
    done()
}

#[derive(Deserialize)]
pub(super) struct SkillsParams {
    percent: u32,
}

/// `client.set_skills {percent}`: every skill of the tree at this whole
/// percentage, as Learn Skills' Set All does (100 unlocks every where).
pub(super) fn set_skills(
    In(request): In<Option<Value>>,
    mut to_server: MessageWriter<ToServer>,
) -> BrpResult {
    let SkillsParams { percent } = params(request)?;
    let change = DevSkills::SetAll(percent);
    to_server.write(ToServer(ClientMessage::DevSkills { who: None, change }));
    done()
}
