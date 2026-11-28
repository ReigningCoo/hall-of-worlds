//! Players as mobs see them, and where they are (what mobs do about them is
//! `choice`).

use shared::creature::{CreatureKind, Size};
use shared::dungeon::Level;
use shared::id::Id;
use shared::mob::scent_strength;
use shared::pathfinding::DistanceMap;
use shared::protocol::CharacterId;
use shared::sheet::Appearance;
use shared::wear::WornLook;
use std::collections::BTreeMap;

use crate::connections::{Client, Phase};
use crate::storage::SiteId;

/// A player as others see them.
pub(crate) struct Someone {
    pub(super) id: Id,
    pub(super) character: Option<CharacterId>,
    /// Their character's name.
    pub(super) name: String,
    pub(super) look: (CreatureKind, Appearance),
    pub(super) gender: shared::gender::Gender,
    /// What can be seen of what they wear.
    pub(super) worn: Vec<WornLook>,
    /// The mark before their name (`Client::mark`).
    pub(super) mark: Option<shared::role::RoleBadge>,
    /// The site level they are on, if any.
    pub(super) level: Option<(SiteId, u32)>,
    /// They are in the Hall of Worlds (and `level` is `None`).
    pub(super) in_hall: bool,
    /// The cell (or world map point) they are in: what sight and mobs go by.
    pub(super) position: (i64, i64),
    /// Exactly where, in units, on a level or in the Hall (the Hall's own).
    pub(super) spot: shared::spot::Spot,
    /// How fast they act (`Client::speed`), for how quickly they are shown
    /// moving.
    pub(super) speed: u32,
    /// Mobs take no notice of them.
    pub(super) god_mode: bool,
    /// How strong a trail they leave, as a percentage.
    pub(super) scent: u32,
    /// The way they face.
    pub(super) heading: shared::facing::Heading,
    /// Winding up a blow: a noise mobs near enough hear.
    pub(super) fighting: bool,
    /// How far their light reaches (`Inventory::light_reach`: a lit lantern's
    /// further), which everyone who sees them is told.
    pub(super) light: u32,
    /// Set never to trade, which everyone who sees them is told.
    pub(super) never_trades: bool,
}

pub(super) fn everyone_playing(clients: &mut [Client]) -> Vec<Someone> {
    clients
        .iter_mut()
        .filter(|client| client.phase == Phase::Playing && !client.is_gone())
        .map(|client| {
            let (level, spot) = match (&client.visit, client.character.hall) {
                (Some(visit), _) => (Some(visit.population()), visit.spot()),
                (None, Some(at)) => (None, at),
                (None, None) => (
                    None,
                    shared::spot::Spot::middle_of(client.character.world_position),
                ),
            };
            let position = match (&client.visit, client.character.hall) {
                (None, None) => client.character.world_position,
                _ => spot.cell(),
            };
            Someone {
                id: client.entity_id(),
                character: client.character_id,
                name: client.name.clone(),
                look: (client.inventory.creature, client.sheet.appearance.clone()),
                gender: client.sheet.gender,
                worn: client.inventory.shown(),
                mark: client.mark(),
                level,
                in_hall: client.visit.is_none() && client.character.hall.is_some(),
                position,
                spot,
                speed: client.speed,
                god_mode: client.god_mode,
                scent: scent_strength(&client.inventory),
                heading: client.heading,
                fighting: client.winding_up(),
                light: client.sheet.light_reach(&client.inventory),
                never_trades: client.never_trades(),
            }
        })
        .collect()
}

/// How far apart, counting a diagonal as one.
pub(super) fn cells_apart(a: (i64, i64), b: (i64, i64)) -> i64 {
    (a.0 - b.0).abs().max((a.1 - b.1).abs())
}

/// The distance maps worked out this tick on one level: mobs after the same place
/// share one, one for each size after it.
#[derive(Default)]
pub(super) struct Maps(BTreeMap<((i64, i64), Size), DistanceMap>);

impl Maps {
    pub(super) fn to(&mut self, level: &Level, size: Size, goal: (i64, i64)) -> &DistanceMap {
        self.0
            .entry((goal, size))
            .or_insert_with(|| DistanceMap::sized(level, size, goal))
    }
}

/// A player on a level, as far as its mobs go, or another mob (`member`).
#[derive(Clone, Debug)]
pub(super) struct Spot {
    pub(super) id: Id,
    pub(super) character: Option<CharacterId>,
    pub(super) kind: CreatureKind,
    pub(super) position: (i64, i64),
    /// Exactly where, in units (`Someone::spot`).
    pub(super) at: shared::spot::Spot,
    /// How strong a trail they leave, as a percentage.
    pub(super) scent: u32,
    /// Another mob's race and band (mob behaviour plan PR 4); `None` for a
    /// player.
    pub(super) member: Option<shared::behaviour::Member>,
}
