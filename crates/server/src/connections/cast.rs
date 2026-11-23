//! A player casting a spell from their spellbook (magic plan PR 5): asked for
//! with `SpellbookAsk::Cast`, wound up as the Magic ability, then its
//! steps resolve about a second apart once every client has had its turn
//! (`cast/run.rs`). A first step that hits nobody fizzles, costing only the
//! time; otherwise the Mana is paid all at once as it lands.

mod creatures;
mod effects;
mod run;
mod step;

use bevy::prelude::*;
use shared::ability::Ability;
use shared::facing::Heading;
use shared::id::Id;
use shared::protocol::ServerMessage;
use shared::skill::SkillId;
use shared::spell::{CAST_COST, CAST_WIND_UP, Spell, SpellKey, spell_cost};
use shared::spot::Spot;
use shared::vitals::Vital;

use super::Client;
use super::action::Action;
use crate::ServerWorld;
use crate::combat::{Act, Pending, ticks_for};
use crate::storage::SiteId;

pub(super) use run::run_casts;
#[cfg(test)]
pub(super) use run::run_due;

/// A player's spell: the one asked for, and the one whose steps are going on.
#[derive(Clone, Debug, Default)]
pub(crate) struct Casting {
    /// Asked for (`SpellbookAsk::Cast`), waiting to begin as the Magic ability.
    asked: Option<SpellKey>,
    /// Wound up, or landed with steps still to go.
    running: Option<Cast>,
}

/// A spell under way.
#[derive(Clone, Debug)]
struct Cast {
    spell: Spell,
    /// The step to resolve next, at action tick `due`.
    next: usize,
    due: u64,
    /// The level it is cast on: it stops if the caster leaves.
    place: (SiteId, u32),
    /// The caster's target, for the first where only.
    toward: Option<Id>,
    /// What the last where hit: the next steps' targets. Empty: the caster.
    aimed: Vec<Aimed>,
    /// The Form skills of the wheres so far, practised when an effect they
    /// carried does something.
    forms: Vec<SkillId>,
}

/// Something a where hit, kept between steps by id so it is followed if it
/// moves; a spot alone where it hit nobody.
#[derive(Clone, Copy, Debug)]
struct Aimed {
    creature: Option<Id>,
    at: Spot,
    height: i32,
    heading: Heading,
}

/// `SpellbookAsk::Cast`: the spell kept for when it begins, and Magic
/// asked for like any ability, one at a time. Always stays.
pub(super) fn ask_cast(client: &mut Client, spell: SpellKey, target: Option<Id>) -> bool {
    if !(client.waiting.iter()).any(|a| matches!(a, Action::UseAbility { .. })) {
        client.casting.asked = Some(spell);
        client.waiting.push_back(Action::UseAbility {
            ability: Ability::Magic,
            target,
        });
    }
    true
}

/// Begins the spell asked for, if it is in the book, its design still allowed,
/// there is the Mana for it, nothing else is under way, and (in The Way Inn)
/// it stays on the caster. Whether the client is still there, and the cost.
pub(super) fn start_cast(
    client: &mut Client,
    world: &mut ServerWorld,
    target: Option<Id>,
) -> (bool, u32) {
    let tick = world.action_clock.ticks();
    let Some(key) = client.casting.asked.take() else {
        debug!("Not casting: no spell asked for");
        return (true, 0);
    };
    let Some(spell) = client
        .sheet
        .spellbook
        .by_key(key)
        .map(|kept| kept.spell.clone())
    else {
        debug!("Not casting: the spell isn't in the book");
        return (true, 0);
    };
    let skills = Some(&client.sheet.skills);
    let limits = client.limits(tick);
    let mana = client.sheet.vitals.at(Vital::Mana, limits, tick);
    let refused = if client.casting.running.is_some() {
        Some("a spell is still under way")
    } else if shared::spell::check(&spell, skills).is_err() {
        Some("the design isn't allowed")
    } else if mana < spell_cost(&spell, skills) {
        Some("not enough Mana")
    } else if spell.first_where().is_some() && crate::sites::in_inn(client) {
        Some("only on themselves in The Way Inn")
    } else {
        None
    };
    let Some(place) = client.visit.as_ref().map(|visit| visit.population()) else {
        debug!("Not casting: off levels");
        return (true, 0);
    };
    if let Some(why) = refused {
        debug!("Not casting: {why}");
        return (true, 0);
    }
    let me = client.entity_id();
    let lands_in = ticks_for(CAST_WIND_UP, client.speed);
    client.fight.pending = Some(Pending {
        target: target.unwrap_or(me),
        act: Act::Cast,
        lands_at: tick + lands_in,
    });
    client.casting.running = Some(Cast {
        spell,
        next: 0,
        due: u64::MAX,
        place,
        toward: target,
        aimed: Vec::new(),
        forms: Vec::new(),
    });
    let stayed = client.link.send(&ServerMessage::AbilityStarted {
        who: None,
        ability: Ability::Magic,
        target: target.filter(|target| *target != me),
        lands_in: lands_in as u32,
        ready_in: ticks_for(CAST_COST, client.speed) as u32,
    });
    (stayed, CAST_COST)
}

/// The wind-up is over: the first step resolves once every client has had
/// its turn (`run_casts`). Always stays.
pub(super) fn land_cast(client: &mut Client, world: &ServerWorld) -> bool {
    if let Some(cast) = client.casting.running.as_mut() {
        cast.due = world.action_clock.ticks();
    }
    true
}

/// A wind-up called off: a spell's goes with it.
pub(super) fn call_off(client: &mut Client, act: Act) {
    if matches!(act, Act::Cast) {
        client.casting.running = None;
    }
}
