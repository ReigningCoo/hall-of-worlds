//! Who else is on a map: the mobs on site levels in use, shared by everyone on
//! them, what those mobs are doing, and telling each player who they can see.
//!
//! A level is kept while someone is there, for a while after, so that someone
//! reconnecting finds the same mobs, and while any mob on it isn't settled, so that
//! nobody can leave a chasing mob frozen at the stairs for when they come back.

use shared::combat::Wielded;
use shared::creature::CreatureKind;
use shared::dungeon::Level;
use shared::energy::{Energy, SITE_SCALE};
use shared::id::Id;
use shared::mob::Memory;
use shared::movement::STRAIGHT_COST;
use shared::sheet::{Appearance, Attributes};
use shared::vitals::Vitals;
use shared::wear::WornLook;
use std::collections::{BTreeMap, BTreeSet};
use std::time::{Duration, Instant};

use crate::combat::Fight;
use crate::storage::SiteId;

// Split into files (`cargo xtask lint` keeps files small).
mod alert;
mod chill;
mod choice;
mod close_in;
mod crossing;
mod curious;
#[cfg(test)]
mod curious_tests;
mod defend;
#[cfg(test)]
mod defend_tests;
mod desire;
mod dev_gift;
mod dev_mob;
mod doing;
mod doors;
mod doubling_back;
mod escortee;
mod facing;
#[cfg(test)]
mod facing_tests;
mod feeling;
mod feeling_shown;
mod fight;
mod foes;
mod gear;
mod harmed;
mod harmed_by;
mod heal;
mod hunt;
mod inspect;
#[cfg(test)]
mod light_tests;
mod made;
mod mob_blows;
mod moves;
mod noise;
#[cfg(test)]
mod noise_tests;
mod notice;
mod on_ground;
mod population;
mod quest_mobs;
#[cfg(test)]
mod remake_for_tests;
mod run;
mod scent;
mod shove;
mod spells;
mod suited;
mod sync;
mod target;
mod tell;
mod territory;
mod timid;
mod tracking;
mod watching;
#[cfg(test)]
mod watching_tests;
mod wild;

pub(crate) use alert::tell_alerts;
pub(crate) use chill::chill_mob;
use choice::{Scores, Ways};
use crossing::cross;
pub(crate) use dev_gift::{give_to_mob, mob_in_sight};
pub(crate) use dev_mob::dev_mobs;
pub use doing::Doing;
pub(crate) use doors::{MobDoors, Opened};
use doubling_back::Pacing;
pub(crate) use escortee::{Escortee, escortee_mob, escortee_worn};
pub(crate) use fight::{MobBlow, Striker, hit_mob};
use gear::dressed;
pub(super) use gear::{give, give_within, kill};
pub(crate) use harmed::{Harmer, harm_mob};
use harmed_by::HarmedBy;
pub(crate) use harmed_by::Kill;
pub(crate) use heal::{heal_mob, mobs_for_spell};
#[cfg(test)]
pub(crate) use made::BOSS_NUMBER;
use moves::{Taken, act, nearest_free, take_step};
pub(crate) use noise::make_noise;
use notice::{Maps, Someone, Spot, cells_apart, everyone_playing};
pub(super) use run::run_mobs;
pub(crate) use scent::Scents;
pub(crate) use shove::{knock_mob, shove, shoved_to};
use spells::MobSpells;
pub(crate) use spells::{end_mob_spells, lay_on_mob, poison_mobs};
pub(super) use sync::{in_hall, sync};
pub(crate) use watching::watched_warily;
pub(crate) use wild::{WildAreas, keep_wild_areas};

/// How long a level nobody is on stays in use.
const KEPT_FOR: Duration = Duration::from_secs(60);

/// How near a mob going up to someone friendly stops.
const FRIENDLY_GAP: i64 = 2;
/// How near one running at someone stops, until there is combat.
const ATTACK_GAP: i64 = 1;
/// How many steps a mob wanders after losing someone, before going home.
const SETTLING_STEPS: u8 = 6;
/// How long going home may take, in ticks: past this it just gets there, once
/// nobody can see it.
const HOME_WITHIN: u64 = 1200;
/// What thinking with nothing to do costs: a straight step's worth.
const THINKING_COST: u32 = STRAIGHT_COST * SITE_SCALE;

/// Something on a level that isn't a player.
#[derive(Clone, Debug)]
pub(crate) struct Mob {
    pub id: Id,
    pub kind: CreatureKind,
    /// Rolled with it (`Spawn::gender`), for others to be told.
    pub gender: shared::gender::Gender,
    /// Rolled with it (`Spawn::alignment`): secret, never told to players.
    pub alignment: Option<shared::alignment::Alignment>,
    pub appearance: Appearance,
    /// The cell it thinks in: always the one under `spot` (`place`).
    pub position: (i64, i64),
    /// Where its body is, in units (240 a cell): the middle of its circle.
    pub spot: shared::spot::Spot,
    /// The way it faces (`facing.rs`): not saved.
    pub heading: shared::facing::Heading,
    /// Where it started, and goes back to.
    pub home: (i64, i64),
    pub doing: Doing,
    /// Who it has in its sights: a player or another creature it means to get
    /// close to as an enemy (`Doing::target`). Never told to players.
    pub target: Option<Id>,
    /// Which of its level's mobs it is.
    number: u32,
    /// What it remembers.
    memory: Memory,
    /// The level it belongs on, and the stairs back there, last taken last.
    home_level: u32,
    way_back: Vec<Hop>,
    energy: Energy,
    speed: u32,
    /// Its speed without spells (`chill.rs`: `speed` is this, chilled and
    /// paced).
    own_speed: u32,
    /// Chilled by a spell: until when (`chill.rs`).
    chilled: Option<u64>,
    /// Spells' effects lying on it (`spells.rs`). Not saved.
    spells: MobSpells,
    /// What can be seen of what it wears, from when it was made: mobs don't change
    /// clothes.
    shown: Vec<WornLook>,
    /// How strong a trail it leaves, as a percentage.
    scent: u32,
    /// What it is like, for fighting (`Spawn::attributes`), with what it wears'.
    attributes: Attributes,
    /// Every bonus from what it wears, for its most Health and Mana.
    bonuses: shared::bonus::Totals,
    /// Its Health: not saved, so a mob unloaded is whole again.
    vitals: Vitals,
    /// What it holds to fight with, from what it was made with.
    wielded: Wielded,
    fight: Fight,
    /// Who has harmed it lately, to share in killing it. Not saved.
    harmed_by: HarmedBy,
    /// Its last steps, so it doesn't pace back and forth. Not saved.
    pacing: Pacing,
    /// Developer tools: it does nothing, even when hit (Create Mob's Still).
    still: bool,
    /// A site's own mob about in the wilderness: the site whose first level
    /// keeps it (`home_level` 0), its `home` in wilderness cells.
    kept_in: Option<SiteId>,
    /// Someone a quester escorts (`escortee`): never kept, friendly to all.
    escort: Option<Escortee>,
    /// A kept animal (`Spawn::penned`): never through a gate or over a fence.
    penned: bool,
    /// Its wild pack or herd (`Spawn::band`): worked out, never saved.
    band: shared::behaviour::Band,
    /// How its race gets about (`choice::Ways`), from the server's behaviours.
    ways: Ways,
    /// Its choices as last scored (`choice`), for Inspect. Not saved.
    scores: Scores,
    /// The mobs on its level it would fight, and its prey (`foes::Foes`).
    foes: foes::Foes,
    /// Unaware, Alert or Aware (`alert.rs`). Not saved.
    wariness: shared::alertness::Wariness,
    /// A bag it couldn't get to, and until when it passes it over (`desire.rs`).
    passed_over: Option<(shared::spot::Spot, u64)>,
}

/// Stairs a mob has come by: where they are here, and where they lead back to.
#[derive(Clone, Copy, Debug)]
struct Hop {
    stairs: (i64, i64),
    to_level: u32,
    arrives: (i64, i64),
}

/// Someone has taken stairs: mobs after them that saw will follow.
pub(crate) struct StairsTaken {
    pub who: Id,
    pub site: SiteId,
    pub from_level: u32,
    pub stairs: (i64, i64),
    pub to_level: u32,
    pub arrives: (i64, i64),
}

/// A mob going to another level.
struct Crossing {
    mob: Id,
    site: SiteId,
    from_level: u32,
    to_level: u32,
    from_stairs: (i64, i64),
    arrives: (i64, i64),
    /// It is on its way home, not after someone.
    going_back: bool,
}

/// The mobs of one level, and how long nobody has been there.
#[derive(Debug)]
pub(crate) struct Population {
    pub(crate) mobs: Vec<Mob>,
    /// Its shape, to find ways about.
    level: Level,
    /// Where its stairs are, which new mobs keep away from.
    stairs: Vec<(i64, i64)>,
    /// The shape of its ground (`None` where flat), for how far apart creatures
    /// are in 3D (`combat::Body::on`).
    relief: Option<shared::relief::Relief>,
    /// For opening doors on the way, a site level's (not the wilderness's).
    doors: Option<MobDoors>,
    /// Who is due to act: the tick, and which mob. A mob at rest isn't here, so
    /// costs nothing however long it rests.
    due: BTreeSet<(u64, Id)>,
    /// How many of its mobs are on other levels, so it must stay for their return.
    away: u32,
    empty_since: Option<Instant>,
    /// Everyone's scent marks on it.
    scents: Scents,
    /// Where each player on it was last tick, and how strongly they smell, to
    /// mark the cells they leave.
    last_at: BTreeMap<Id, ((i64, i64), u32)>,
    /// Noises made since its mobs last ran (doors, spells: `noise::make_noise`;
    /// alarm calls, by whom: `curious::call_out`).
    noises: Vec<noise::Made>,
}

impl Population {
    /// Whether every mob is at rest, so nothing is going on here.
    fn is_settled(&self) -> bool {
        self.away == 0 && self.mobs.iter().all(|mob| mob.doing == Doing::Idle)
    }
}

/// The mobs on the levels in use.
/// The second is how each race behaves on this server (`BehaviourTable`, its
/// mods'), which mobs' choices go by.
#[derive(Debug, Default)]
pub(crate) struct Populations(
    pub(crate) BTreeMap<(SiteId, u32), Population>,
    pub(crate) std::sync::Arc<shared::behaviour::Behaviours>,
);

impl Populations {
    /// Forgets a site's: it is being made anew.
    pub(crate) fn forget_site(&mut self, id: SiteId) {
        self.0.retain(|(site, _), _| *site != id);
    }

    pub(crate) fn forget_all(&mut self) {
        self.0.clear();
    }
}

#[cfg(test)]
mod alert_tests;
#[cfg(test)]
mod dev_mob_tests;
#[cfg(test)]
mod doors_tests;
#[cfg(test)]
mod feeling_shown_tests;
#[cfg(test)]
mod harmed_tests;
#[cfg(test)]
mod mob_fights_tests;
#[cfg(test)]
mod quest_mobs_tests;
#[cfg(test)]
mod tests;
#[cfg(test)]
mod timid_tests;
