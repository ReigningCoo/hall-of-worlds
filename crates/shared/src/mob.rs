//! Who is on a site level to begin with: worked out from the site's secret seed,
//! apart from the level's shape, so adding or changing mobs moves no walls and
//! changes no generator version. See `docs/context/shared/mob.md`.

use serde::{Deserialize, Serialize};

use crate::alignment::Alignment;
use crate::behaviour::{Band, Family};
use crate::creature::CreatureKind;
use crate::dungeon::{Cell, Level};
use crate::gender::Gender;
use crate::id::Id;
use crate::item::{ItemKind, Quality};
use crate::poi::PoiKind;
use crate::rng::Seed;
use crate::sheet::{Appearance, Attributes, Build, Feeling};
use crate::wear::WornLook;

mod chosen;
mod gear;
mod herd;
mod kept;
mod looks;
mod memory;
mod purse;
mod scent;
mod trail;
mod wild;
mod wild_looks;
mod wild_shares;
pub use chosen::{chosen_spawns, spawn_at};
#[cfg(test)]
use gear::gear_of;
use gear::roll_gear;
pub use kept::site_spawns;
use looks::looks;
pub use memory::{
    Event, FIRST_HARM_OPINION, GIFT_OPINION, Memory, Regard, TRESPASS_FORGOTTEN_AFTER,
};
use purse::roll_coins;
pub use scent::{BASE_SCENT, MOST_SCENT, SCENT_KEPT_FOR, can_smell, scent_strength, smells_for};
pub use trail::{TRAIL_LONGEST, TRAIL_SHORTEST, wild_trail};
pub use wild::{Habitat, wild_mob_id, wild_spawns};

/// Marks these hashes as being for mobs, apart from anything else from the seed.
const PURPOSE: i64 = 0x006d_6f62;

/// One walkable cell in this many has a mob.
const CELLS_PER_MOB: usize = 60;
const MOST_MOBS: usize = 12;
const MOST_BEGGARS: usize = 3;
/// How far from any stairs a mob starts, in cells: nobody arrives beside one.
const FROM_STAIRS: i64 = 6;
/// How many places are tried for each.
const TRIES: i64 = 16;
/// The roll a mob's attributes come from, apart from its place, look and gear.
const ATTRIBUTE_ROLL: i64 = 2000;
/// Which roll of a mob's own is its gender's (`Gender::rolled`).
const GENDER_ROLL: i64 = 2001;
/// Which roll of a mob's own is its alignment's (`Alignment::rolled`).
const ALIGNMENT_ROLL: i64 = 2002;

/// One of a level's mobs, as generated.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Spawn {
    /// Which of the level's mobs it is: what its id is saved against.
    pub number: u32,
    pub kind: CreatureKind,
    pub position: (i64, i64),
    pub appearance: Appearance,
    /// Rolled from its own hash: 45% Male, 45% Female, 10% Neutral.
    pub gender: Gender,
    /// Rolled from its own hash about its kind's leaning; `None` for an animal.
    pub alignment: Option<Alignment>,
    /// Rolled near its kind's medians, with its kind's and gender's bonuses.
    pub attributes: Attributes,
    /// What it starts with, innermost clothes first: it wears what it can.
    pub gear: Vec<(ItemKind, Option<Quality>)>,
    /// The money it starts with, in copper, which it carries as the fewest coins.
    pub coins: u64,
    /// A kept animal in a pen or pasture (`site_spawns`): it never goes
    /// through a gate or over a fence.
    pub penned: bool,
    /// The band it belongs to (`shared::behaviour`): its wild pack or herd, or
    /// on a site every one of its race there.
    pub band: Band,
    /// Its family within the band: reserved, always `None` for now.
    pub family: Option<Family>,
}

/// How near someone must be to give something, in cells.
pub const GIVE_REACH: i64 = 2;

/// What can be found in a kind of place, with how many of each in 100.
pub fn who_lives_in(site: PoiKind) -> &'static [(CreatureKind, u64)] {
    match site {
        _ if site.is_town() => &[(CreatureKind::Beggar, 100)],
        PoiKind::Farm => &[
            (CreatureKind::Chicken, 70),
            (CreatureKind::Horse, 15),
            (CreatureKind::Cattle, 15),
        ],
        _ => &[
            (CreatureKind::Kobold, 40),
            (CreatureKind::Goblin, 20),
            (CreatureKind::Orc, 10),
            (CreatureKind::Chicken, 10),
            (CreatureKind::Wolf, 6),
            (CreatureKind::Giant, 8),
            (CreatureKind::Lizardfolk, 6),
        ],
    }
}

/// The mobs of level `index` of a site. The same every time for the same seed.
/// A town has a few beggars for now. Never on furniture.
pub fn spawns(
    seed: Seed,
    index: u32,
    site: PoiKind,
    level: &Level,
    stairs: &[(i64, i64)],
) -> Vec<Spawn> {
    let (min_x, min_y, width, height) = level.bounds();
    let is_home = |x: i64, y: i64| {
        let far_from_stairs = stairs
            .iter()
            .all(|s| (s.0 - x).abs().max((s.1 - y).abs()) >= FROM_STAIRS);
        level.is_walkable(x, y)
            && level.cell(x, y) != Cell::Ground
            && level.furniture_at(x, y).is_none()
            && far_from_stairs
    };
    let cells = (min_y..min_y + height)
        .flat_map(|y| (min_x..min_x + width).map(move |x| (x, y)))
        .filter(|&(x, y)| is_home(x, y))
        .count();
    // Towns are huge, and only have a few people in the street so far.
    let most = if site.is_town() {
        MOST_BEGGARS
    } else {
        MOST_MOBS
    };
    let wanted = (cells / CELLS_PER_MOB).min(most);
    let lives_here = who_lives_in(site);
    let mut mobs: Vec<Spawn> = Vec::new();
    for number in 0..wanted as u32 {
        let roll = |what: i64| seed.at(&[PURPOSE, index as i64, number as i64, what]);
        let spot = (0..TRIES).find_map(|attempt| {
            let x = min_x + (roll(attempt * 2) % width as u64) as i64;
            let y = min_y + (roll(attempt * 2 + 1) % height as u64) as i64;
            let free = mobs.iter().all(|mob| mob.position != (x, y));
            (is_home(x, y) && free).then_some((x, y))
        });
        // A number stays with its mob, gaps or not, so none is shuffled along.
        let Some(position) = spot else {
            continue;
        };
        let mut left = roll(TRIES * 2) % 100;
        let kind = lives_here
            .iter()
            .find(|&&(_, share)| {
                let here = left < share;
                left = left.saturating_sub(share);
                here
            })
            .map_or(lives_here[0].0, |&(kind, _)| kind);
        let gender = Gender::rolled(roll(GENDER_ROLL));
        mobs.push(Spawn {
            number,
            kind,
            position,
            appearance: looks(&roll, kind, gender).fitted(kind, gender),
            gender,
            alignment: Alignment::rolled(kind, roll(ALIGNMENT_ROLL)),
            attributes: Attributes::for_kind(
                kind,
                gender,
                &Build::rolled(&mut Seed::new(roll(ATTRIBUTE_ROLL)).rng()),
            ),
            gear: roll_gear(&roll, kind),
            coins: roll_coins(&roll, kind),
            penned: false,
            band: Band::of_site(seed, kind),
            family: None,
        });
    }
    mobs
}

/// What a player is told about something on the map they can see: never stats.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct EntityLook {
    pub id: Id,
    pub kind: CreatureKind,
    /// Its gender, for words about it (pronouns, later).
    pub gender: Gender,
    /// Another player's character's name; `None` for a mob, which goes by
    /// its kind.
    pub name: Option<String>,
    pub appearance: Appearance,
    /// How it feels about the player being told.
    pub feeling: Feeling,
    /// Where it is, in the map's measure (`protocol::moves`).
    pub x: i64,
    pub y: i64,
    /// What can be seen of what it wears, innermost first, for its picture.
    pub worn: Vec<WornLook>,
    /// Another player's character shown as a marked role
    /// (`role::is_marked`): its badge, for the mark before its name.
    pub mark: Option<crate::role::RoleBadge>,
    /// The way it faces (`facing`).
    pub heading: crate::facing::Heading,
    /// A quest boss: who it is (its name, worded as "Grukk the Foul") and its
    /// tier, a quality, which it glows in (owner). `None` for anyone else.
    pub boss: Option<BossLook>,
    /// Its stage of life (`livestock::stage_of`), for its body: grown for
    /// players and anyone not in the livestock table.
    pub stage: crate::livestock::Stage,
    /// How far its light reaches, in cells: a player's `Inventory::light_reach`
    /// (`shared::lantern`); 0 for mobs, which carry none (no darkness rule yet).
    pub light: u32,
    /// A player set never to trade (`trade::TradingSetting::Never`), shown
    /// greyed out in the trade window's list; `false` for everyone else.
    pub never_trades: bool,
}

/// What everyone sees of a quest boss (`quest::boss_of`, `quest::boss_tier`).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct BossLook {
    pub boss: crate::story::Boss,
    pub tier: crate::item::Quality,
}

#[cfg(test)]
mod tests;
