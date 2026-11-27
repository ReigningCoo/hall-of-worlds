//! Walking into reach before an ability chosen by hand (owner, 2026-11-27):
//! Attack, Kick and spells aimed at a target out of reach walk there first
//! (`player::Approach`), then are asked for as usual. Automatic ones only go
//! when in reach (`use_ability.rs`).

use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use shared::ability::{Ability, BarThing};
use shared::combat::{Body, Weapon, Wielded, reach_between, within};
use shared::id::Id;
use shared::item::Inventory;
use shared::sheet::Sheet;
use shared::spell::{Spell, where_reach};

use super::manual::Manual;
use super::spells::spell_of;
use crate::common::{CurrentDungeon, DrawnGround, Other, PlayerSize, SpotPosition};
use crate::player::{Approach, Approached, Player};

/// How much nearer than the reach the player walks, in units: the server
/// sees the target a moment later than this client did.
pub(super) const MARGIN: u64 = 24;

/// How far a blow reaches with what is held: the shorter hand's weapon when
/// dual wielding, as hands take turns.
pub(super) fn weapon_reach(wielded: &Wielded) -> u64 {
    let main = wielded.main.reach();
    wielded.off.map_or(main, |off| main.min(off.reach()))
}

/// How near (between middles, in 3D) `me` must be to `them` for `thing` to
/// reach them, if it is aimed at them: Attack by the weapons held, Kick
/// barefoot, a spell by its first where (none for one aimed at the caster).
pub(super) fn reach_needed(
    thing: BarThing,
    (sheet, wielded): (&Sheet, &Wielded),
    me: &Body,
    them: &Body,
) -> Option<u64> {
    match thing {
        BarThing::Ability(Ability::Attack) => {
            Some(reach_between(me.size, weapon_reach(wielded), them.size))
        }
        BarThing::Ability(Ability::Kick) => {
            Some(reach_between(me.size, Weapon::UNARMED.reach(), them.size))
        }
        BarThing::Spell(key) => spell_reach(spell_of(key, sheet)?, me, them),
        _ => None,
    }
}

/// How near a spell's first where must be to reach `them`; none for one aimed
/// at the caster.
fn spell_reach(spell: &Spell, me: &Body, them: &Body) -> Option<u64> {
    (spell.first_where())
        .filter(|step| !step.filters.caster)
        .map(|step| where_reach(step, me, them))
}

/// Where the player and everyone else stand, for reaching.
#[derive(SystemParam)]
pub(super) struct Reaching<'w, 's> {
    dungeon: Res<'w, CurrentDungeon>,
    ground: Res<'w, DrawnGround>,
    size: Res<'w, PlayerSize>,
    player: Query<'w, 's, &'static SpotPosition, With<Player>>,
    others: Query<'w, 's, (&'static Other, &'static SpotPosition), Without<Player>>,
}

impl Reaching<'_, '_> {
    /// The approach `thing` needs first: `target` is seen out of its reach.
    /// `spell`: the spell a `BarThing::Spell` casts (one in the book, or the
    /// design panel's), else `None`.
    pub(super) fn approach_for(
        &self,
        (thing, spell): (BarThing, Option<&Spell>),
        target: Option<Id>,
        (sheet, carried): (&Sheet, &Inventory),
    ) -> Option<Approach> {
        let target = target?;
        let relief = self.ground.relief.as_ref();
        let me = Body::on(relief, self.player.single().ok()?.0, self.size.0);
        let (other, at) = (self.others.iter()).find(|(other, _)| other.id == target)?;
        let them = Body::on(relief, at.0, other.kind.def().size);
        let wielded = Wielded::of(carried);
        let reach = match spell {
            Some(spell) => spell_reach(spell, &me, &them),
            None => reach_needed(thing, (sheet, &wielded), &me, &them),
        }?;
        let reached = within(&self.dungeon.level, &me, &them, reach);
        (!reached).then_some(Approach {
            target,
            reach: reach.saturating_sub(MARGIN),
        })
    }
}

/// An approach has ended: in reach, what waited is asked for (a frame later,
/// once the last stretch walked has gone to the server); given up, it is
/// dropped.
pub(super) fn take_approached(
    mut approached: MessageReader<Approached>,
    mut manual: ResMut<Manual>,
) {
    for &Approached { target, reached } in approached.read() {
        if manual.approaching != Some(target) {
            continue;
        }
        manual.approaching = None;
        if reached {
            manual.settle = true;
        } else {
            manual.queued.pop_front();
        }
    }
}

#[cfg(test)]
mod tests;
