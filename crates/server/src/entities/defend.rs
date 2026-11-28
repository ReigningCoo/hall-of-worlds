//! Packs defending each other (mob behaviour plan PR 3): a mob that sees one of
//! its band, or an ally if it defends allies (`behaviour::Defends`), harmed by a
//! player remembers that player as an enemy who struck first
//! (`Memory::remember_band_harmed`, so killing it is self-defence for them) and
//! joins in through the scorer (`choice::after_harm`), the attacker's place known.

use shared::behaviour::{Behaviours, Defends, Relation};
use shared::dungeon::line_is_clear;
use shared::id::Id;
use shared::mob::Memory;
use shared::protocol::CharacterId;
use shared::sheet::Feeling;

use super::choice::{Other, after_harm, friends_seen};
use super::harmed::Harmer;
use super::{Doing, Mob, Population, cells_apart};

/// What a defender must keep once it has rallied: its memory, saved with its
/// record (`gear::record_of`), and what finds that record.
pub(super) struct Rallied {
    pub memory: Memory,
    pub kept_with: crate::storage::SiteId,
    pub keep: (Id, u32, (i64, i64)),
}

/// Whether `defender` joins a fight for `victim`, by who it defends.
fn defends(defender: &Mob, victim: &Mob, behaviours: &Behaviours) -> bool {
    let relation = behaviours.relation(defender.member(), victim.member());
    match behaviours.of(defender.kind).defends {
        Defends::Nobody => false,
        Defends::Band => relation == Relation::Kin,
        Defends::Allies => matches!(relation, Relation::Kin | Relation::Ally),
    }
}

/// Whether `defender` sees `victim`: within its sight, in its cone, the way
/// clear.
fn sees(defender: &Mob, victim: &Mob, population: &Population) -> bool {
    cells_apart(defender.position, victim.position) <= defender.kind.senses().sight_range()
        && defender.cone().sees(victim.spot)
        && line_is_clear(&population.level, defender.position, victim.position)
}

/// Every mob on `population` that sees `victim` harmed by `by` and defends it
/// rallies: it turns on them (or, timid, runs from them) as if harmed itself.
/// `others`: its mobs as they stood at the start of the tick. Returns what each
/// must keep, for saving.
pub(super) fn rally(
    population: &mut Population,
    victim: Id,
    by: &Harmer,
    (others, behaviours): (&[Other], &Behaviours),
    site: crate::storage::SiteId,
    tick: u64,
) -> Vec<Rallied> {
    let Some(victim) = population.mobs.iter().find(|one| one.id == victim).cloned() else {
        return Vec::new();
    };
    let character = CharacterId(by.id);
    let defenders: Vec<usize> = (population.mobs.iter().enumerate())
        .filter(|(_, one)| one.id != victim.id && !one.still && one.escort.is_none())
        .filter(|(_, one)| defends(one, &victim, behaviours) && sees(one, &victim, population))
        .map(|(index, _)| index)
        .collect();
    let mut rallied = Vec::new();
    for index in defenders {
        let friends = friends_seen(
            &population.mobs[index],
            &population.level,
            others,
            behaviours,
        );
        let one = &mut population.mobs[index];
        one.memory.remember_band_harmed(character);
        let feeling = one.feeling(by.kind, Some(&character));
        debug_assert_eq!(feeling, Feeling::Unfriendly);
        let def = behaviours.of(one.kind);
        let (doing, scores) = after_harm(one, by, feeling, (def, friends), tick);
        one.scores = scores;
        if let Some(doing) = doing {
            let at_rest = one.doing == Doing::Idle;
            one.doing = doing;
            one.target = doing.target();
            if at_rest && doing != Doing::Idle {
                let due = one.next_due(tick);
                population.due.insert((due, one.id));
            }
        }
        let one = &population.mobs[index];
        rallied.push(Rallied {
            memory: one.memory.clone(),
            kept_with: one.kept_with(site),
            keep: (one.id, one.home_level, one.home),
        });
    }
    rallied
}
