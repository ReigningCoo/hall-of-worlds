//! One yeast as an item: `ItemKind::Yeast` with a `YeastForm` keeping its
//! clock (how long it has gone unfed), its progress towards a better quality,
//! and whether it has been used since it was last fed.

use serde::{Deserialize, Serialize};

use crate::id::Id;
use crate::item::{Item, ItemKind, MadeAt, Quality};
use crate::item_form::ItemForm;

/// Fed before this long unfed (12 hours, in seconds), a feed is early: it
/// splits the yeast (`feed.rs`). From it on, a feed is on time.
pub const FEED_AFTER: u32 = 12 * 3600;

/// Each time this long passes unfed (24 hours, in seconds), it loses a quality
/// step, down to Poor.
pub const WEAKENS_EVERY: u32 = 24 * 3600;

/// What one yeast is: its clock and progress. Every field is saved on the item,
/// so it passes on with it, and none of them keeps two yeasts from stacking.
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct YeastForm {
    /// Seconds since it was last fed, by its clock: playtime while carried,
    /// real time while on the ground.
    #[serde(default)]
    pub unfed: u32,
    /// Points from on-time feeds by the feeder's Fermenting skill (`feed.rs`),
    /// towards a chance of a quality step up.
    #[serde(default)]
    pub progress: u32,
    /// Used in something (PR 6), or split by an early feed, since it was last
    /// fed: it can't be used again until it is fed.
    #[serde(default)]
    pub used: bool,
    /// When it was put on the ground (seconds since 1970), while it lies there:
    /// the server counts real time from it once it is picked up.
    #[serde(default)]
    pub lying_since: Option<u64>,
    /// Seconds of its clock left before a starter begun from flour and water
    /// (`starter.rs`) is ready; 0 for a yeast that is. Until then it can't be
    /// fed or used, and its clock counts this down before `unfed`.
    #[serde(default)]
    pub developing: u32,
    /// Which culture it is: bread yeast, as every yeast was before cultures
    /// (cooking plan PR 6).
    #[serde(default)]
    pub strain: super::Strain,
}

impl YeastForm {
    /// Seconds until a feed is on time; 0 once it is.
    pub fn feed_in(&self) -> u32 {
        FEED_AFTER.saturating_sub(self.unfed)
    }

    /// Seconds until it next loses a quality step if it isn't fed.
    pub fn weakens_in(&self) -> u32 {
        WEAKENS_EVERY - self.unfed % WEAKENS_EVERY
    }

    /// Whether it can go into something now: ready, and not used since it was
    /// fed.
    pub fn usable(&self) -> bool {
        !self.used && self.developing == 0
    }
}

/// One new yeast of this quality, fed just now, with id `id`.
pub fn new_item(quality: Quality, id: Id, made_at: Option<MadeAt>) -> Item {
    let mut item = Item::new(id, ItemKind::Yeast);
    item.quality = Some(quality);
    item.made_at = made_at;
    item.form = Some(ItemForm::Yeast(YeastForm::default()));
    item
}

impl Item {
    /// What it is, if it is yeast.
    pub fn yeast(&self) -> Option<&YeastForm> {
        match &self.form {
            Some(ItemForm::Yeast(yeast)) => Some(yeast),
            _ => None,
        }
    }

    /// The same, to change.
    pub fn yeast_mut(&mut self) -> Option<&mut YeastForm> {
        match &mut self.form {
            Some(ItemForm::Yeast(yeast)) => Some(yeast),
            _ => None,
        }
    }
}
