//! Who besides shops can be traded with (trading plan PR 6): other players
//! and NPCs within a trade's reach (`shared::trade::in_reach`), nearest first.
//! NPCs are the people kinds (those with an alignment) not unfriendly to the
//! player, as the server judges them; a player set never to trade is listed
//! greyed out.

use shared::creature::CreatureKind;
use shared::id::Id;
use shared::sheet::Feeling;
use shared::spot::Spot;
use shared::trade::in_reach;

use super::state::Partner;

/// Someone on the map as the window sees them.
#[derive(Clone, Debug)]
pub(super) struct Seen {
    pub(super) id: Id,
    pub(super) kind: CreatureKind,
    pub(super) feeling: Feeling,
    pub(super) player: bool,
    pub(super) never_trades: bool,
    pub(super) name: String,
    pub(super) at: Spot,
}

/// Whether the server would trade with this NPC (besides its being in sight).
fn trades(seen: &Seen) -> bool {
    seen.kind.alignment_leaning().is_some() && seen.feeling != Feeling::Unfriendly
}

/// The players and the NPCs within reach of `at`, each nearest first; NPCs
/// alike numbered as shops are.
pub(super) fn people_near(at: Spot, seen: &[Seen]) -> (Vec<Partner>, Vec<Partner>) {
    let mut near: Vec<&Seen> = (seen.iter())
        .filter(|seen| in_reach(at, seen.at) && (seen.player || trades(seen)))
        .collect();
    near.sort_by_key(|seen| (at.distance(seen.at), seen.id));
    let players = (near.iter().filter(|seen| seen.player))
        .map(|seen| Partner {
            with: seen.id,
            name: seen.name.clone(),
            never: seen.never_trades,
        })
        .collect();
    let mut npcs: Vec<Partner> = Vec::new();
    for seen in near.iter().filter(|seen| !seen.player) {
        npcs.push(Partner {
            with: seen.id,
            name: numbered(&npcs, &seen.name),
            never: false,
        });
    }
    (players, npcs)
}

/// `name`, with a number after it when others before are alike ("Human 2").
pub(super) fn numbered(before: &[Partner], name: &str) -> String {
    let alike = (before.iter())
        .filter(|other| other.name == name || other.name.starts_with(&format!("{name} ")))
        .count();
    match alike {
        0 => name.to_owned(),
        n => format!("{name} {}", n + 1),
    }
}
