//! Spells on the action bar (magic plan PR 5): how one looks, whether it can
//! be cast now, whom it goes at, and when an automatic one is cast
//! (`spell::auto_cast`). The server decides; this only keeps from asking for
//! what it would refuse.

use bevy::prelude::*;
use shared::ability::Ability;
use shared::combat::Body;
use shared::dungeon::Level;
use shared::facing::Heading;
use shared::id::Id;
use shared::lang::Lang;
use shared::protocol::{ClientMessage, SpellbookAsk};
use shared::sheet::Sheet;
use shared::spell::{
    AutoCast, Creature, Origin, Side, Spell, SpellKey, Step, Toward, Where, auto_cast, auto_name,
    hit_anyone, where_hits,
};

use super::page::look;
use super::use_ability::ASK_WAIT;
use super::vitals_now::wants_healing;
use crate::common::{OwnAbility, OwnVitals, can_cast_spell, spell_icon};
use crate::connection::ToServer;
use crate::widgets::AbilityLook;

/// The spell with this key in the character's book.
pub(super) fn spell_of(key: SpellKey, sheet: &Sheet) -> Option<&Spell> {
    sheet.spellbook.by_key(key).map(|kept| &kept.spell)
}

/// Whether it can be cast now: in the book, and `common::can_cast_spell`
/// (still allowed by the skills, with the Mana for it, and in The Way Inn
/// only on the caster).
pub(super) fn can_cast(
    key: SpellKey,
    sheet: &Sheet,
    (vitals, now): (&OwnVitals, f64),
    in_inn: bool,
) -> bool {
    spell_of(key, sheet).is_some_and(|spell| can_cast_spell(spell, sheet, (vitals, now), in_inn))
}

/// How a spell looks on the bar: Heal's icon for one that heals first, else
/// Magic's; its own name, or the automatic one.
pub(super) fn spell_look(
    key: SpellKey,
    sheet: &Sheet,
    lang: &Lang,
    usable: bool,
    assets: &AssetServer,
) -> AbilityLook {
    let mut shown = look(Ability::Magic, usable, assets);
    let Some(kept) = sheet.spellbook.by_key(key) else {
        shown.name = "A spell no longer in your spellbook".to_owned();
        shown.usable = false;
        return shown;
    };
    shown.icon = assets.load(spell_icon(&kept.spell));
    shown.name = kept
        .name
        .clone()
        .unwrap_or_else(|| auto_name(&kept.spell, lang));
    shown
}

/// Whom a spell cast by hand goes at: the target for one starting with a
/// where, otherwise the caster (`None`).
pub(super) fn hand_aim(spell: &Spell, target: Option<Id>) -> Option<Id> {
    spell.first_where().and(target)
}

/// Asks the server to cast it (`None`: on the player, or ahead).
pub(super) fn ask_cast(
    key: SpellKey,
    target: Option<Id>,
    now: f64,
    own: &mut OwnAbility,
    to_server: &mut MessageWriter<ToServer>,
) {
    let ask = SpellbookAsk::Cast { spell: key, target };
    own.asked_until = now + ASK_WAIT;
    to_server.write(ToServer(ClientMessage::Spellbook(ask)));
}

/// Asks the server to cast a design not in the book, as it is (the spell
/// design panel's Cast).
pub(super) fn ask_cast_design(
    spell: Spell,
    target: Option<Id>,
    now: f64,
    own: &mut OwnAbility,
    to_server: &mut MessageWriter<ToServer>,
) {
    let ask = SpellbookAsk::CastDesign { spell, target };
    own.asked_until = now + ASK_WAIT;
    to_server.write(ToServer(ClientMessage::Spellbook(ask)));
}

/// Where the player stands and an enemy targeted, for an automatic spell.
pub(super) struct Standing<'a> {
    pub level: &'a Level,
    pub me: Body,
    pub heading: Heading,
    pub enemy: Option<(Id, Body)>,
}

/// Whether an automatic spell is due now, and at whom: a helpful one landing
/// on the player when they are low (`WhenLow`); a harmful one when its first
/// step would hit the enemy targeted, as the server works it out (`AtEnemy`).
pub(super) fn auto_aim(
    spell: &Spell,
    (vitals, now): (&OwnVitals, f64),
    standing: &Standing,
) -> Option<Option<Id>> {
    match auto_cast(spell) {
        AutoCast::WhenLow => (lands_on_caster(spell) && wants_healing(vitals, now)).then_some(None),
        AutoCast::AtEnemy => {
            let (id, body) = standing.enemy?;
            first_hits(spell, standing, body).then_some(Some(id))
        }
        AutoCast::ByHand => None,
    }
}

/// Whether the first step lands on the caster: no where, or Self on Point.
fn lands_on_caster(spell: &Spell) -> bool {
    spell
        .first_where()
        .is_none_or(|step| step.place == Where::Point && step.filters.caster)
}

/// Whether the first where, turned to the enemy (as the server turns the
/// caster), would hit them.
fn first_hits(spell: &Spell, standing: &Standing, enemy: Body) -> bool {
    let Some(Step::Where(step)) = spell.steps.first() else {
        return false;
    };
    let creatures = [
        Creature {
            body: standing.me,
            side: Side::Caster,
        },
        Creature {
            body: enemy,
            side: Side::Enemy,
        },
    ];
    let heading = Heading::towards(standing.me.at, enemy.at).unwrap_or(standing.heading);
    let origin = Origin::from_creature(&creatures, 0, heading);
    let hits = where_hits(
        standing.level,
        step,
        &origin,
        Toward::Creature(1),
        &creatures,
    );
    hit_anyone(&hits)
}
