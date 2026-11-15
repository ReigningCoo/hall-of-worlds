//! What a new necklace must be, if anything: its metal, how many settings, and its
//! gems; and the weights its roll picks from, with what is asked for kept to.

use crate::item::Quality;
use crate::item_form::gem::{GemAsk, fine_weight};
use crate::item_form::request::{Field, Refused, RollRequest, keep_only};
use crate::material::{Category, Material};

/// The settings a necklace may have: none, a pendant's or amulet's one, an amulet's
/// 3, 4 or 6 round its rim, and 3, 5 or 7 stations.
pub(super) const SETTINGS: [u8; 7] = [0, 1, 3, 4, 5, 6, 7];

/// What goes round the neck, before its details are rolled.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum StrandKind {
    Cord,
    Chain,
    Torc,
}

/// What hangs from it, before its details are rolled.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum OrnamentKind {
    None,
    Pendant,
    Amulet,
    Stations,
    Charm,
}

/// A request's necklace options.
#[derive(Clone, Copy, Default)]
pub(super) struct NecklaceAsk {
    pub(super) metal: Option<Material>,
    pub(super) settings: Option<u8>,
    pub(super) gem: GemAsk,
}

/// Every metal (owner's decision), in `Material`'s order.
pub(super) fn necklace_metals() -> impl Iterator<Item = Material> {
    Material::ALL
        .into_iter()
        .filter(|material| material.category() == Category::Metal)
}

/// How fine a necklace's work is: its metal and quality.
pub(super) fn fine(metal: Material, quality: Quality) -> u64 {
    metal.tier() + quality as u64
}

impl NecklaceAsk {
    pub(super) fn of(request: &RollRequest) -> Self {
        Self {
            metal: request.material,
            settings: request.settings,
            gem: GemAsk::of(request),
        }
    }

    /// Asking for a gem means at least one stone to show it.
    fn wants_stones(self) -> bool {
        self.gem.material.is_some() || self.gem.cut.is_some()
    }

    /// The metals to roll from, by how often: finer ones rarer, less so at better
    /// quality; only the metal asked for.
    pub(super) fn metals(self, quality: Quality) -> Vec<(Material, u64)> {
        let mut metals: Vec<_> = necklace_metals()
            .map(|metal| (metal, fine_weight(metal, quality)))
            .collect();
        if !keep_only(&mut metals, self.metal) {
            metals.iter_mut().for_each(|(_, weight)| *weight = 0);
        }
        metals
    }

    /// Strands by how often: only those that can carry what is asked for.
    pub(super) fn strands(self, fine: u64) -> [(StrandKind, u64); 3] {
        [
            (StrandKind::Cord, 30),
            (StrandKind::Chain, 55),
            (StrandKind::Torc, 15),
        ]
        .map(|(strand, weight)| {
            let possible = self.ornaments(strand, fine).iter().any(|&(_, w)| w > 0);
            (strand, if possible { weight } else { 0 })
        })
    }

    /// What hangs from this strand, by how often: finer work has more gems. Only
    /// chains have stations, and torcs nothing.
    pub(super) fn ornaments(self, strand: StrandKind, fine: u64) -> [(OrnamentKind, u64); 5] {
        let (none, pendant, amulet, stations, charm) = match strand {
            StrandKind::Cord => (35, 20, fine * 2, 0, 35),
            StrandKind::Chain => (30, 25, fine * 3, fine * 2, 20),
            StrandKind::Torc => (1, 0, 0, 0, 0),
        };
        [
            (OrnamentKind::None, none),
            (OrnamentKind::Pendant, pendant),
            (OrnamentKind::Amulet, amulet),
            (OrnamentKind::Stations, stations),
            (OrnamentKind::Charm, charm),
        ]
        .map(|(ornament, weight)| {
            let possible = self.counts(ornament, fine).iter().any(|&(_, w)| w > 0);
            (ornament, if possible { weight } else { 0 })
        })
    }

    /// How many settings this ornament has, by how often: only the number asked
    /// for, and never none when a stone is wanted.
    pub(super) fn counts(self, ornament: OrnamentKind, fine: u64) -> Vec<(u8, u64)> {
        let mut counts = match ornament {
            OrnamentKind::None | OrnamentKind::Charm => vec![(0, 1)],
            OrnamentKind::Pendant => vec![(1, 1)],
            OrnamentKind::Amulet => vec![(1, 50), (3, fine * 3), (4, fine * 2), (6, fine)],
            OrnamentKind::Stations => vec![(3, 50), (5, fine * 4), (7, fine * 2)],
        };
        if self.wants_stones() {
            counts.retain(|&(count, _)| count > 0);
        }
        if !keep_only(&mut counts, self.settings) {
            counts.clear();
        }
        counts
    }

    /// Whether a necklace can be made so at this quality.
    pub(super) fn check(self, quality: Quality) -> Result<(), Refused> {
        if self
            .metal
            .is_some_and(|metal| metal.category() != Category::Metal)
        {
            return Err(Refused(Field::Material));
        }
        let any = self.metals(quality).into_iter().find(|&(_, w)| w > 0);
        let Some((metal, _)) = any else {
            return Err(Refused(Field::Material));
        };
        let fine = fine(metal, quality);
        if !self.strands(fine).iter().any(|&(_, weight)| weight > 0) {
            return Err(Refused(Field::Settings));
        }
        self.gem.check(quality, false)
    }
}
