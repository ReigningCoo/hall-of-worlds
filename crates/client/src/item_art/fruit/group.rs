//! Which shape group a fruit is drawn in, from its look in the table, with a
//! few fruits named where the table's coarse shapes don't say enough (a
//! pineapple's scales and crown, a pomegranate's crown, grapes in a bunch).

use shared::fruit::{FruitDef, FruitShape, Skin, Unit};

/// How a fruit is drawn.
pub(super) enum Group {
    /// One fruit or a handful of them, each in this style.
    Fruit(Style),
    /// Berries hanging from a branching stalk (grapes, currants).
    Bunch,
    Banana,
    Pod(PodKind),
    Nut(NutKind),
}

/// One fruit's shape, skin and top, and how many make an item.
pub(super) struct Style {
    pub(super) body: Body,
    pub(super) surface: Surface,
    pub(super) top: Top,
    /// How many in one item: 1, or a handful.
    pub(super) count: usize,
    /// Its length against its width, lying along x.
    pub(super) length: f32,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum Body {
    Round,
    /// Lying along x.
    Oval,
    /// Lying along x, its neck at +x.
    Pear,
    /// An oval with a nub at each end.
    Lemon,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum Surface {
    Plain,
    /// Pale specks (lenticels, russet, netting).
    Speckled,
    Spikes,
    Hairs,
    Bumps,
    Scales,
    /// Tufts of tiny spines (cactus fruit).
    Areoles,
    Drupelets,
    /// Dark bands along it (watermelon).
    Stripes,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum Top {
    /// A stalk, and maybe a leaf, at the stalk end.
    Stalk,
    /// A green cap of sepals at the stalk end (mangosteen, persimmon).
    Calyx,
    /// A dry star of sepals at the far end (rose hip, blueberry).
    Sepals,
    /// A crown of points (pomegranate).
    Crown,
    /// A pineapple's tuft of leaves.
    Leaves,
    Bare,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(super) enum PodKind {
    /// Long and flat, a little curved (carob).
    Flat,
    /// Lumpy with the seeds inside, pinched between (tamarind).
    Beaded,
    /// Furrowed lengthwise, pointed (cacao).
    Ridged,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(super) enum NutKind {
    Round,
    Walnut,
    Long,
    Acorn,
    Chestnut,
    Triangular,
    Pistachio,
    Cashew,
    PineNut,
    Coconut,
}

impl Group {
    pub(super) fn of(def: &FruitDef) -> Self {
        let look = &def.look;
        match def.key.as_str() {
            "grape" | "redcurrant" | "blackcurrant" | "elder" | "oregon_grape" | "chokecherry" => {
                return Self::Bunch;
            }
            "banana" => return Self::Banana,
            "carob" => return Self::Pod(PodKind::Flat),
            "tamarind" => return Self::Pod(PodKind::Beaded),
            "cacao_pod" => return Self::Pod(PodKind::Ridged),
            _ => {}
        }
        if look.shape == FruitShape::Nut {
            return Self::Nut(nut_kind(&def.key));
        }
        let mut style = Style {
            body: match look.shape {
                FruitShape::Pear => Body::Pear,
                FruitShape::Oval | FruitShape::Long => Body::Oval,
                _ => Body::Round,
            },
            surface: match look.skin {
                Skin::Spiny => Surface::Spikes,
                Skin::Rough => Surface::Speckled,
                _ => Surface::Plain,
            },
            top: Top::Stalk,
            count: count(def),
            length: match look.shape {
                FruitShape::Long => 1.9,
                FruitShape::Oval | FruitShape::Pear => 1.3,
                _ => 1.0,
            },
        };
        if look.shape == FruitShape::Aggregate {
            style.surface = Surface::Drupelets;
            style.top = Top::Bare;
        }
        named(&def.key, &mut style);
        Self::Fruit(style)
    }
}

/// How many fruits make one item: one, or a handful by size.
fn count(def: &FruitDef) -> usize {
    if def.unit != Unit::Handful {
        return 1;
    }
    // The first size it is no bigger than.
    let sizes = [(8, 12), (12, 10), (16, 8), (22, 6), (30, 5), (45, 3)];
    let size = def.look.size;
    sizes
        .iter()
        .find(|(most, _)| size <= *most)
        .map_or(2, |&(_, count)| count)
}

/// What the table's shapes don't say of some fruits.
fn named(key: &str, style: &mut Style) {
    use Surface::*;
    let (surface, top, body, length) = match key {
        "pineapple" => (Scales, Top::Leaves, Body::Oval, 1.5),
        "durian" => (Spikes, Top::Stalk, Body::Oval, 1.3),
        "jackfruit" => (Bumps, Top::Stalk, Body::Oval, 1.6),
        "breadfruit" | "lychee" | "wild_custard_apple" => (Bumps, Top::Bare, Body::Round, 1.1),
        "rambutan" => (Hairs, Top::Bare, Body::Oval, 1.15),
        "prickly_pear" | "saguaro_fruit" => (Areoles, Top::Bare, Body::Oval, 1.4),
        "watermelon" => (Stripes, Top::Stalk, Body::Oval, 1.25),
        "melon" => (Speckled, Top::Stalk, Body::Oval, 1.3),
        "orange" | "mangosteen" | "persimmon" | "pomegranate" | "passion_fruit" => {
            let top = match key {
                "mangosteen" | "persimmon" => Top::Calyx,
                "pomegranate" => Top::Crown,
                _ => Top::Stalk,
            };
            (style.surface, top, Body::Round, 1.0)
        }
        "lemon" => (Plain, Top::Bare, Body::Lemon, 1.35),
        "rose_hip" | "blueberry" | "bog_blueberry" | "saskatoon" | "medlar" | "bilberry"
        | "huckleberry" | "salal" | "cranberry" => {
            (style.surface, Top::Sepals, style.body, style.length)
        }
        "date" | "baobab" | "pawpaw" => (Plain, Top::Bare, Body::Oval, 2.0),
        "mango" | "marula" => (Plain, Top::Stalk, Body::Oval, 1.35),
        _ => return,
    };
    style.surface = surface;
    style.top = top;
    style.body = body;
    style.length = length;
}

/// Which kind of nut a nut is drawn as.
fn nut_kind(key: &str) -> NutKind {
    match key {
        "walnut" | "hickory_nut" => NutKind::Walnut,
        "pecan" | "almond" | "pili_nut" | "mongongo_nut" => NutKind::Long,
        "acorn" => NutKind::Acorn,
        "sweet_chestnut" | "chinquapin" => NutKind::Chestnut,
        "beechnut" | "brazil_nut" => NutKind::Triangular,
        "pistachio" => NutKind::Pistachio,
        "cashew" => NutKind::Cashew,
        "siberian_pine_nut" | "stone_pine_nut" => NutKind::PineNut,
        "coconut" => NutKind::Coconut,
        _ => NutKind::Round,
    }
}
