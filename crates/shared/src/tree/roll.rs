//! Which wild tree a `Cell::Tree` is: rolled from its place's climate and cover
//! and a hash of the cell, so it needs saving only once felled.

use super::{Group, TreeDef, TreeKind};
use crate::climate::{Biome, ClimateSample};
use crate::vegetation::Cover;

/// What a cell's tree is rolled from. Fractions are per mille, so the roll
/// itself is whole numbers.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Setting {
    pub biome: Biome,
    /// The yearly average temperature, whole °C.
    pub celsius: i32,
    /// How conifer-like the trees are (`Cover::narrow`), 0 to 1000.
    pub narrow: u32,
    /// How rainforest-like (`Cover::lush`), 0 to 1000.
    pub lush: u32,
    /// Within reach of a river or the shore: willow and alder grow there.
    pub near_water: bool,
}

/// Over this `lush`, only rainforest kinds grow.
const LUSH_OVER: u32 = 500;

/// A kind this many °C outside its range still grows, at half weight.
const BORDER: i32 = 3;

/// From this many °C, lush trees are tropical rainforest kinds.
const TROPICAL: i32 = 20;

impl Setting {
    /// The setting at a place, from the climate and cover there.
    pub fn of(sample: ClimateSample, cover: Cover, near_water: bool) -> Self {
        let per_mille = |share: f64| (share.clamp(0.0, 1.0) * 1000.0).round() as u32;
        Self {
            biome: sample.biome(),
            celsius: sample.celsius().round() as i32,
            narrow: per_mille(f64::from(cover.narrow)),
            lush: per_mille(f64::from(cover.lush)),
            near_water,
        }
    }
}

impl TreeKind {
    /// The wild tree growing in `setting`, picked by `hash` (a hash of the
    /// cell): conifer with the chance `narrow`, else broadleaf (the other group
    /// if none suits); kinds by biome and temperature, weighted by how common;
    /// only rainforest kinds where lush; the kind nearest by temperature where
    /// nothing suits.
    pub fn roll(setting: Setting, hash: u64) -> Self {
        let conifer = hash % 1000 < u64::from(setting.narrow);
        let (first, second) = match conifer {
            true => (Group::Conifer, Group::Broadleaf),
            false => (Group::Broadleaf, Group::Conifer),
        };
        let pick = hash / 1000;
        for group in [first, second] {
            if let Some(kind) = pick_weighted(&weights(setting, group), pick) {
                return kind;
            }
        }
        nearest(setting)
    }
}

/// Each kind of the group with its weight here; rainforest kinds only, where
/// lush and any suit.
fn weights(setting: Setting, group: Group) -> Vec<(TreeKind, u64)> {
    let all: Vec<(TreeKind, u64)> = (TreeKind::all())
        .map(|kind| (kind, weight(kind.def(), setting)))
        .filter(|&(kind, weight)| weight > 0 && kind.def().group == group)
        .collect();
    if setting.lush <= LUSH_OVER {
        return all;
    }
    let rainforest = match setting.celsius >= TROPICAL {
        true => Biome::TropicalRainforest,
        false => Biome::TemperateRainforest,
    };
    let lush: Vec<_> = (all.iter().copied())
        .filter(|(kind, _)| kind.def().biomes.contains(&rainforest))
        .collect();
    if lush.is_empty() { all } else { lush }
}

/// A kind's weight in `setting`: twice its commonness within its range, its
/// commonness within `BORDER` of it, nothing further off or in another biome.
/// Willow and alder count as common by water and rare elsewhere.
fn weight(def: &TreeDef, setting: Setting) -> u64 {
    if !def.biomes.contains(&setting.biome) {
        return 0;
    }
    let (low, high) = def.celsius;
    let at = setting.celsius;
    let base = match def.wet && setting.near_water {
        true => super::Commonness::Common.weight(),
        false => def.commonness.weight(),
    };
    if (low..=high).contains(&at) {
        2 * base
    } else if (low - BORDER..=high + BORDER).contains(&at) {
        base
    } else {
        0
    }
}

/// One of `weighted`, by `pick`; `None` if nothing has a weight.
fn pick_weighted(weighted: &[(TreeKind, u64)], pick: u64) -> Option<TreeKind> {
    let total: u64 = weighted.iter().map(|(_, weight)| weight).sum();
    if total == 0 {
        return None;
    }
    let mut left = pick % total;
    for &(kind, weight) in weighted {
        if left < weight {
            return Some(kind);
        }
        left -= weight;
    }
    None
}

/// The kind whose range is nearest the temperature, preferring the biome's:
/// where nothing suits.
fn nearest(setting: Setting) -> TreeKind {
    let off = |kind: &TreeKind| {
        let (low, high) = kind.def().celsius;
        let degrees = (low - setting.celsius).max(setting.celsius - high).max(0);
        (!kind.def().biomes.contains(&setting.biome), degrees)
    };
    TreeKind::all()
        .min_by_key(off)
        .unwrap_or(TreeKind::ScotsPine)
}
