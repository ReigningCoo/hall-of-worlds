//! What Teleport, Light, Haste, Calm, Blessing and Levitate leave on a
//! creature for a while, and their reverses (magic plan PR 13), as timed
//! effects (`bonus::Source::Spell`) like `lasting`'s: Haste and Slow change
//! speed (`Stat::Speed`), Blessing and Weakness every attribute, Light gives
//! its own light (`Stat::Light`); Darkness, Calm, Fear, Levitate, a Teleport
//! waiting for its move and being held from teleporting change what the
//! creature may do, read by their part. Their strength grows with the
//! effect's skill; counts last longer and stack. Numbers are first guesses.

use crate::bonus::{Bonus, PER_CENT, Stat};
use crate::combat::skill_percent;
use crate::sheet::Attribute;
use crate::skill::Skills;

use super::effect::Effect;
use super::lasting::{LESS_PERCENT, MORE_PERCENT, MOST_LESS_PERCENT};
use super::part::Part;

/// Acting faster.
pub const HASTED: Part = Part::plain(Effect::Haste);
/// Haste reversed: acting slower.
pub const SLOWED: Part = Part::reversed(Effect::Haste);
/// Every attribute up.
pub const BLESSED: Part = Part::plain(Effect::Blessing);
/// Blessing reversed: every attribute down.
pub const WEAKENED: Part = Part::reversed(Effect::Blessing);
/// Glowing with its own light.
pub const LIT: Part = Part::plain(Effect::Light);
/// Light reversed: no light of its own, a lantern's swallowed too.
pub const DARKENED: Part = Part::reversed(Effect::Light);
/// Calmed: no longer unfriendly to anyone.
pub const CALMED: Part = Part::plain(Effect::Calm);
/// Calm reversed: fleeing whoever frightened it.
pub const FRIGHTENED: Part = Part::reversed(Effect::Calm);
/// Floating over holes and water.
pub const LEVITATING: Part = Part::plain(Effect::Levitate);
/// The next move goes at once to the cell chosen (`teleport_to`).
pub const TELEPORTING: Part = Part::plain(Effect::Teleport);
/// Teleport reversed: held from teleporting.
pub const ANCHORED: Part = Part::reversed(Effect::Teleport);

/// Seconds Calm, Fear, Levitate, a waiting Teleport and being held last,
/// each; Haste, Slow, Blessing, Weakness, Light and Darkness last
/// `LASTING_SECONDS`.
pub const SHORT_SECONDS: u64 = 15;
/// The most Haste adds to speed, in percent.
pub const MOST_HASTE_PERCENT: i32 = 100;
/// How far a Light reaches at no skill, in cells; twice as far at 100%.
pub const LIGHT_CELLS: i32 = 6;
/// The most a Light reaches, in cells.
pub const MOST_LIGHT_CELLS: i32 = 16;

/// Whether `part` is one of this file's.
pub fn is_utility(part: Part) -> bool {
    [
        HASTED,
        SLOWED,
        BLESSED,
        WEAKENED,
        LIT,
        DARKENED,
        CALMED,
        FRIGHTENED,
        LEVITATING,
        TELEPORTING,
        ANCHORED,
    ]
    .contains(&part)
}

/// Seconds one count of a utility `part` lasts.
pub fn utility_seconds(part: Part) -> u64 {
    match part {
        CALMED | FRIGHTENED | LEVITATING | TELEPORTING | ANCHORED => SHORT_SECONDS,
        _ => super::lasting::LASTING_SECONDS,
    }
}

/// What `count` of a utility `part` give, by a caster with `skills`: Haste
/// `MORE_PERCENT` speed a count, Slow `LESS_PERCENT` less, Blessing and
/// Weakness the same on each attribute, Light `LIGHT_CELLS` of light (more
/// counts reach no further, only last longer); twice each at 100% skill.
pub fn utility_bonuses(part: Part, count: u32, skills: Option<&Skills>) -> Vec<Bonus> {
    let percent = skills.map_or(0, |s| skill_percent(s.value(part.effect.skill()))) as i32;
    let by = |each: i32| each * (100 + percent) / 100 * count.min(100) as i32;
    let more = by(MORE_PERCENT);
    let less = by(LESS_PERCENT).min(MOST_LESS_PERCENT);
    let every = |amount: i32| {
        (Attribute::ALL.iter())
            .map(|&a| Bonus::percent(Stat::Attribute(a), amount * PER_CENT))
            .collect()
    };
    match part {
        HASTED => vec![Bonus::percent(
            Stat::Speed,
            more.min(MOST_HASTE_PERCENT) * PER_CENT,
        )],
        SLOWED => vec![Bonus::percent(Stat::Speed, -less * PER_CENT)],
        BLESSED => every(more),
        WEAKENED => every(-less),
        LIT => {
            let cells = (LIGHT_CELLS * (100 + percent) / 100).min(MOST_LIGHT_CELLS);
            vec![Bonus::flat(Stat::Light, cells * 1000)]
        }
        _ => Vec::new(),
    }
}
