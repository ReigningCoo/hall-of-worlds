//! Whether a spell can be cast now, the one rule for the action bar, the
//! Spells tab and the spell design panel's Cast: its design allowed by the
//! skills, the Mana for it as the client refills it, and (in The Way Inn) only
//! on the caster. The server decides; this only keeps from asking for what it
//! would refuse.

use shared::protocol::LevelKind;
use shared::sheet::Sheet;
use shared::spell::{Spell, check, spell_cost};
use shared::vitals::{PER_POINT, Points};

use super::{CurrentDungeon, GameView, OwnVitals};

/// Whether there is this much Mana at `now` (`Time`'s seconds). Nothing told
/// yet: only what costs none.
pub fn affords(cost: Points, vitals: &OwnVitals, now: f64) -> bool {
    let cost = cost.milli() as f32 / PER_POINT as f32;
    cost == 0.0 || vitals.0.is_some_and(|told| told.now(now).1 >= cost)
}

/// Whether the player is in The Way Inn, where nothing may harm anyone.
pub fn in_inn(view: GameView, dungeon: &CurrentDungeon) -> bool {
    view == GameView::Dungeon
        && (dungeon.info.as_ref()).is_some_and(|info| info.kind == LevelKind::Inn)
}

/// Whether `spell` can be cast now by this character.
pub fn can_cast_spell(
    spell: &Spell,
    sheet: &Sheet,
    (vitals, now): (&OwnVitals, f64),
    in_inn: bool,
) -> bool {
    let skills = Some(&sheet.skills);
    check(spell, skills).is_ok()
        && affords(spell_cost(spell, skills), vitals, now)
        && !(in_inn && spell.first_where().is_some())
}

#[cfg(test)]
mod tests;
