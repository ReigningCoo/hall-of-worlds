//! What can be asked for: `choices`, every value each option of a `RollRequest` can
//! take for a kind of item, so whoever offers them needn't list them itself.

use super::Cut;
use super::armour::Piece;
use super::armour_choices::armour_choices;
use super::gem::GemAsk;
use super::necklace_ask::{NecklaceAsk, SETTINGS as NECKLACE_SETTINGS, necklace_metals};
use super::request::{Field, RollRequest, fields, keep_only};
use super::ring_ask::RingAsk;
use super::ring_roll::RING_METALS;
use crate::armour::{Construction, Dye, Finish, Style};
use crate::blade;
use crate::item::{ItemKind, Quality};
use crate::material::Material;

/// The values each option can take. An option the kind doesn't take is empty (and
/// `limits` false). Each list is in its type's own order.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Choices {
    pub styles: Vec<Style>,
    pub constructions: Vec<Construction>,
    pub materials: Vec<Material>,
    pub finishes: Vec<Finish>,
    pub dyes: Vec<Dye>,
    pub gem_materials: Vec<Material>,
    pub gem_cuts: Vec<Cut>,
    pub settings: Vec<u8>,
    /// What a bottle may hold: the waters, the oils, then a potion.
    pub liquids: Vec<crate::potion::LiquidKind>,
    /// How many effects a bottle may hold, 0 (empty) first.
    pub effects: Vec<u8>,
    /// What size a cask may be, smallest first.
    pub cask_sizes: Vec<crate::cask::CaskSize>,
    /// Which vessels, smallest first.
    pub vessels: Vec<crate::vessel::VesselKind>,
    /// Whether it takes limits (sizes and weights, which aren't a list).
    pub limits: bool,
}

/// The settings a ring may have.
const SETTINGS: [u8; 5] = [0, 1, 2, 3, 5];

/// What each option can be for a new item of this kind and quality, given the other
/// options asked for in `request` (the option's own value there is ignored): a style
/// narrows the constructions, a construction the materials, quality the finishes. The
/// same rules and tables the rolls use. For armour and footwear, each value listed is
/// one a roll asked for it makes from one of a fixed seed or two, so it can be made,
/// though a different seed may still refuse a rare combination.
pub fn choices(kind: ItemKind, quality: Quality, request: &RollRequest) -> Choices {
    let mut choices = match kind {
        _ if let Some(piece) = Piece::of(kind) => armour_choices(kind, piece, quality, request),
        ItemKind::Gem => gem_choices(quality, request),
        ItemKind::Ring => ring_choices(quality, request),
        ItemKind::Necklace => necklace_choices(quality, request),
        ItemKind::Bottle => bottle_choices(request),
        // Any size and wood; empty, or full of a water or an oil.
        ItemKind::Cask => Choices {
            cask_sizes: crate::cask::CaskSize::ALL.to_vec(),
            materials: {
                let mut woods = crate::cask::WOODS.to_vec();
                woods.sort();
                woods
            },
            liquids: (crate::potion::LiquidKind::all().into_iter())
                .filter(|liquid| *liquid != crate::potion::LiquidKind::Potion)
                .collect(),
            ..Choices::default()
        },
        // Any vessel; empty, or full of a water or an oil.
        ItemKind::Vessel => Choices {
            vessels: crate::vessel::VesselKind::ALL.to_vec(),
            liquids: (crate::potion::LiquidKind::all().into_iter())
                .filter(|liquid| *liquid != crate::potion::LiquidKind::Potion)
                .collect(),
            ..Choices::default()
        },
        ItemKind::Ointment => Choices {
            effects: (1..=crate::potion::MOST_EFFECTS).collect(),
            ..Choices::default()
        },
        _ if kind.blade_family().is_some() => Choices {
            materials: passing(Material::ALL, |metal| {
                keep_only(&mut blade::blade_metals(quality), Some(metal))
            }),
            ..Choices::default()
        },
        _ => Choices::default(),
    };
    choices.limits = fields(kind).contains(&Field::Limits);
    choices
}

/// Any liquid; effects only for a potion, or with no liquid asked for (0 an
/// empty bottle then), as `potion::bottle_to` makes them.
fn bottle_choices(request: &RollRequest) -> Choices {
    use crate::potion::{LiquidKind, MOST_EFFECTS};
    let effects = match request.liquid {
        None => (0..=MOST_EFFECTS).collect(),
        Some(LiquidKind::Potion) => (1..=MOST_EFFECTS).collect(),
        Some(_) => Vec::new(),
    };
    Choices {
        liquids: LiquidKind::all(),
        effects,
        ..Choices::default()
    }
}

fn gem_choices(quality: Quality, request: &RollRequest) -> Choices {
    let ask = GemAsk::of(request);
    Choices {
        gem_materials: passing(Material::GEMS, |material| {
            let ask = GemAsk {
                material: Some(material),
                ..ask
            };
            ask.check(quality, true).is_ok()
        }),
        gem_cuts: passing(Cut::ALL, |cut| {
            let ask = GemAsk {
                cut: Some(cut),
                ..ask
            };
            ask.check(quality, true).is_ok()
        }),
        ..Choices::default()
    }
}

fn ring_choices(quality: Quality, request: &RollRequest) -> Choices {
    let ask = RingAsk::of(request);
    let ok = |ask: RingAsk| ask.check(quality).is_ok();
    let gem = ask.gem;
    Choices {
        materials: passing(RING_METALS, |metal| {
            ok(RingAsk {
                metal: Some(metal),
                ..ask
            })
        }),
        settings: passing(SETTINGS, |count| {
            ok(RingAsk {
                settings: Some(count),
                ..ask
            })
        }),
        gem_materials: passing(Material::GEMS, |material| {
            let gem = GemAsk {
                material: Some(material),
                ..gem
            };
            ok(RingAsk { gem, ..ask })
        }),
        gem_cuts: passing(Cut::ALL, |cut| {
            let gem = GemAsk {
                cut: Some(cut),
                ..gem
            };
            ok(RingAsk { gem, ..ask })
        }),
        ..Choices::default()
    }
}

fn necklace_choices(quality: Quality, request: &RollRequest) -> Choices {
    let ask = NecklaceAsk::of(request);
    let ok = |ask: NecklaceAsk| ask.check(quality).is_ok();
    let gem = ask.gem;
    Choices {
        materials: passing(necklace_metals(), |metal| {
            ok(NecklaceAsk {
                metal: Some(metal),
                ..ask
            })
        }),
        settings: passing(NECKLACE_SETTINGS, |count| {
            ok(NecklaceAsk {
                settings: Some(count),
                ..ask
            })
        }),
        gem_materials: passing(Material::GEMS, |material| {
            let gem = GemAsk {
                material: Some(material),
                ..gem
            };
            ok(NecklaceAsk { gem, ..ask })
        }),
        gem_cuts: passing(Cut::ALL, |cut| {
            let gem = GemAsk {
                cut: Some(cut),
                ..gem
            };
            ok(NecklaceAsk { gem, ..ask })
        }),
        ..Choices::default()
    }
}

/// The values that pass, in order.
pub(super) fn passing<T: Copy>(
    values: impl IntoIterator<Item = T>,
    ok: impl Fn(T) -> bool,
) -> Vec<T> {
    values.into_iter().filter(|&value| ok(value)).collect()
}
