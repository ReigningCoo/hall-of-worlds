//! Rolling a new necklace: its metal, strand, length, ornament and settings with
//! their gems. First guesses at every weight.

use crate::creature::Size;
use crate::item::{Item, Quality};
use crate::item_form::gem::roll_gem;
use crate::item_form::necklace::{
    Charm, CordStyle, Link, Medallion, NecklaceForm, NecklaceLength, Ornament, Strand, Thickness,
    TorcEnd,
};
use crate::item_form::necklace_ask::{NecklaceAsk, OrnamentKind, StrandKind, fine};
use crate::item_form::ring_roll::{roll_gem_quality, roll_gem_size};
use crate::item_form::{GemForm, Mount, Setting};
use crate::material::Material;
use crate::rng::Rng;

/// The share out of 100 of settings left empty when a necklace is made.
const EMPTY_PERCENT: u64 = 5;
/// The share out of 100 of gems after the first that match it.
const SAME_GEM_PERCENT: u64 = 85;
/// How far apart stations are round the neck, in 256ths of a turn.
const STATION_SPACING: u8 = 12;

/// Quality, then metal (better favours finer), strand, length, ornament (finer has
/// more gems), how many settings, then the gems. `new_gem` makes each gem an item.
/// What is asked for is kept to, if `ask.check` passed.
pub(super) fn roll_necklace(
    rng: &mut Rng,
    quality: Quality,
    size: Size,
    ask: NecklaceAsk,
    new_gem: &mut impl FnMut(GemForm, Size, Quality) -> Item,
) -> NecklaceForm {
    let metal = rng.pick_weighted(&ask.metals(quality));
    let fine = fine(metal, quality);
    let strand_kind = rng.pick_weighted(&ask.strands(fine));
    let strand = roll_strand(rng, strand_kind);
    let length = match strand_kind {
        StrandKind::Torc => rng.pick_weighted(&[
            (NecklaceLength::Choker, 30),
            (NecklaceLength::Collarbone, 70),
        ]),
        _ => rng.pick_weighted(&[
            (NecklaceLength::Choker, 20),
            (NecklaceLength::Collarbone, 35),
            (NecklaceLength::Chest, 30),
            (NecklaceLength::Long, 15),
        ]),
    };
    let ornament_kind = rng.pick_weighted(&ask.ornaments(strand_kind, fine));
    let ornament = roll_ornament(rng, ornament_kind);
    let count = rng.pick_weighted(&ask.counts(ornament_kind, fine));
    let (setting_size, mount) = match ornament_kind {
        OrnamentKind::Pendant => (size.stepped(rng.pick_weighted(&[(0, 60), (1, 40)])), {
            rng.pick_weighted(&[(Mount::Claws, 60), (Mount::Bezel, 40)])
        }),
        OrnamentKind::Amulet if count == 1 => (size, Mount::Bezel),
        _ => (size.stepped(rng.pick_weighted(&[(-1, 60), (-2, 40)])), {
            rng.pick_weighted(&[(Mount::Bezel, 60), (Mount::Claws, 40)])
        }),
    };
    let mut first: Option<GemForm> = None;
    let mut settings = Vec::new();
    for index in 0..count {
        let mut gem = None;
        if rng.range_u64(0, 100) >= EMPTY_PERCENT {
            let form = match first {
                Some(same) if rng.range_u64(0, 100) < SAME_GEM_PERCENT => same,
                _ => roll_gem(rng, quality, false, ask.gem).0,
            };
            first.get_or_insert(form);
            let gem_size = roll_gem_size(rng, setting_size);
            gem = Some(new_gem(form, gem_size, roll_gem_quality(rng, quality)));
        }
        let angle = match ornament_kind {
            OrnamentKind::Stations => station_angle(index, count),
            _ if count == 1 => 0,
            _ => (index as u32 * 256 / count as u32) as u8,
        };
        settings.push(Setting {
            angle,
            size: setting_size,
            mount,
            gem,
        });
    }
    NecklaceForm {
        metal,
        strand,
        length,
        ornament,
        settings,
    }
}

fn roll_strand(rng: &mut Rng, kind: StrandKind) -> Strand {
    match kind {
        StrandKind::Cord => Strand::Cord {
            leather: rng.pick_weighted(&[
                (Material::Leather, 60),
                (Material::Suede, 25),
                (Material::Rawhide, 15),
            ]),
            style: rng.pick_weighted(&[
                (CordStyle::Plain, 60),
                (CordStyle::Braided, 25),
                (CordStyle::Knotted, 15),
            ]),
        },
        StrandKind::Chain => Strand::Chain {
            thickness: rng.pick_weighted(&[
                (Thickness::Fine, 40),
                (Thickness::Medium, 40),
                (Thickness::Heavy, 20),
            ]),
            link: rng.pick_weighted(&[
                (Link::Cable, 50),
                (Link::Rope, 20),
                (Link::Box, 15),
                (Link::Snake, 15),
            ]),
        },
        StrandKind::Torc => Strand::Torc {
            ends: rng.pick_weighted(&[
                (TorcEnd::Buffer, 30),
                (TorcEnd::Ball, 25),
                (TorcEnd::Scroll, 20),
                (TorcEnd::Beast, 15),
                (TorcEnd::Plain, 10),
            ]),
        },
    }
}

fn roll_ornament(rng: &mut Rng, kind: OrnamentKind) -> Ornament {
    match kind {
        OrnamentKind::None => Ornament::None,
        OrnamentKind::Pendant => Ornament::Pendant,
        OrnamentKind::Stations => Ornament::Stations,
        OrnamentKind::Amulet => Ornament::Amulet(rng.pick_weighted(&[
            (Medallion::Round, 60),
            (Medallion::Oval, 25),
            (Medallion::Octagon, 15),
        ])),
        OrnamentKind::Charm => Ornament::Charm(rng.pick_weighted(&Charm::ALL.map(|c| (c, 1)))),
    }
}

/// Evenly either side of the front, the first on the wearer's right.
fn station_angle(index: u8, count: u8) -> u8 {
    let from_first = index as i32 * STATION_SPACING as i32;
    let row = (count as i32 - 1) * STATION_SPACING as i32;
    (from_first - row / 2).rem_euclid(256) as u8
}
