//! How a vegetable is drawn: its family from the part picked (`Part`), and a
//! shape within it from its key where the family alone doesn't say enough.

use shared::vegetable::{Part, Unit, VegetableDef, VegetableSet};

/// How a vegetable's picture is drawn.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(super) enum Drawing {
    Root(Root),
    Tuber(Tuber),
    Bulb,
    Leaves(Leaves),
    Head(Head),
    Stalk(Stalk),
    Shoot(Shoot),
    Fruit(Fruit),
    Pod(Pod),
    Seed(Seed),
    /// A sheaf of grain with its heads, or a cob of maize.
    Sheaf(Ear),
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(super) enum Root {
    /// A globe with a thin tail (turnip, beetroot, swede, round radishes).
    Globe,
    /// A long taper (carrot, parsnip, burdock).
    Taper,
    /// Plump in the middle, pointed at both ends (sweet potato, cassava).
    Spindle,
    /// Slender fingers from one crown (skirret).
    Cluster,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(super) enum Tuber {
    /// Round to oval with eyes (potato, wapato, prairie turnip).
    Lumpy,
    /// Knobbly like ginger (Jerusalem artichoke, alpine bistort).
    Knobbly,
    /// A corm ringed with hairy bands (taro).
    Ringed,
    /// A jointed creeping rhizome (cattail, arrowroot).
    Rhizome,
    /// A length cut from a huge tuber, its cut end showing (yam).
    Length,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(super) enum Leaves {
    Curly,
    Oval,
    Kidney,
    Toothed,
    Diamond,
    Long,
    /// Fleshy sprigs of small round leaves (purslane).
    Sprig,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(super) enum Head {
    Cabbage,
    Lettuce,
    Artichoke,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(super) enum Stalk {
    Leek,
    Celery,
    Rhubarb,
    /// A heart of cut leaf bases (agave).
    Heart,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(super) enum Shoot {
    Fiddlehead,
    /// Spears tied in a bundle (asparagus).
    Spear,
    /// Slender pale stalks with crinkled tips (sea kale).
    Blanched,
    Bamboo,
    /// Hollow stems with arrow leaves (water spinach).
    Vine,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(super) enum Fruit {
    Tomato,
    Pumpkin,
    Cucumber,
    Aubergine,
    Gourd,
    Chayote,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(super) enum Pod {
    /// Five-ridged and pointed (okra).
    Ridged,
    /// Flat, narrow and bumpy over the seeds (mesquite).
    Flat,
    /// Four frilly wings (winged bean).
    Winged,
    /// Netted shells pinched between the nuts (peanut).
    Shell,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(super) enum Seed {
    /// Peas, soybeans, mung beans, Bambara groundnuts.
    Round,
    /// Kidney beans and their kin, curved.
    Kidney,
    /// Lentils: flat discs.
    Lens,
    /// Broad, lima and lupin beans: flat ovals.
    Flat,
    /// Chickpeas, with a beak.
    Beaked,
    /// Grass peas: wedges.
    Wedge,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(super) enum Ear {
    /// Grains in two rows up a spike, bearded if `awns` (wheat, barley, rye).
    Spike { awns: bool },
    /// A loose drooping head of grains on branches (oats, rice, millet).
    Panicle,
    /// A dense head of tiny grains (sorghum, quinoa).
    Dense,
    /// A branched tapering plume (amaranth).
    Plume,
    /// A thick cylinder like a cattail (pearl millet).
    Cattail,
    /// Finger-like spikes from the stalk's top (finger millet, fonio).
    Fingers,
    /// A cob in its husk, not a sheaf (maize).
    Cob,
    /// Red stems with clusters of three-cornered seeds (buckwheat).
    Buckwheat,
}

impl Drawing {
    pub(super) fn of(def: &VegetableDef) -> Self {
        let key = def.key.as_str();
        if def.set == VegetableSet::Grain {
            return Self::Sheaf(ear(key));
        }
        match def.part {
            Part::Root => Self::Root(match key {
                "turnip" | "swede" | "beetroot" | "radish" => Root::Globe,
                "sweet_potato" | "cassava" => Root::Spindle,
                "skirret" => Root::Cluster,
                _ if def.unit == Unit::Cluster => Root::Cluster,
                _ => Root::Taper,
            }),
            Part::Tuber => Self::Tuber(match key {
                "jerusalem_artichoke" | "alpine_bistort" => Tuber::Knobbly,
                "taro" => Tuber::Ringed,
                "cattail" | "arrowroot" => Tuber::Rhizome,
                _ if def.unit == Unit::Piece => Tuber::Length,
                _ => Tuber::Lumpy,
            }),
            Part::Bulb => Self::Bulb,
            Part::LeafBunch => match key {
                "cabbage" => Self::Head(Head::Cabbage),
                "lettuce" => Self::Head(Head::Lettuce),
                _ => Self::Leaves(leaves(key)),
            },
            Part::FlowerHead => Self::Head(Head::Artichoke),
            Part::StalkBunch => Self::Stalk(match key {
                "leek" => Stalk::Leek,
                "rhubarb" => Stalk::Rhubarb,
                "agave_heart" => Stalk::Heart,
                _ => Stalk::Celery,
            }),
            Part::Shoots => Self::Shoot(match key {
                "fiddlehead" => Shoot::Fiddlehead,
                "sea_kale" => Shoot::Blanched,
                "bamboo_shoot" => Shoot::Bamboo,
                "water_spinach" => Shoot::Vine,
                _ => Shoot::Spear,
            }),
            Part::Fruit => Self::Fruit(match key {
                "pumpkin" => Fruit::Pumpkin,
                "cucumber" => Fruit::Cucumber,
                "aubergine" => Fruit::Aubergine,
                "bottle_gourd" => Fruit::Gourd,
                "chayote" => Fruit::Chayote,
                _ => Fruit::Tomato,
            }),
            Part::Pods => Self::Pod(match key {
                "mesquite_pod" => Pod::Flat,
                "winged_bean" => Pod::Winged,
                _ => Pod::Ridged,
            }),
            Part::Seeds | Part::EarOfGrain => match key {
                "peanut" => Self::Pod(Pod::Shell),
                _ => Self::Seed(seed(key)),
            },
        }
    }
}

fn leaves(key: &str) -> Leaves {
    match key {
        "kale" => Leaves::Curly,
        "mountain_sorrel" => Leaves::Kidney,
        "dandelion" => Leaves::Toothed,
        "lambs_quarters" => Leaves::Diamond,
        "chicory" => Leaves::Long,
        "purslane" => Leaves::Sprig,
        _ => Leaves::Oval,
    }
}

fn seed(key: &str) -> Seed {
    match key {
        "kidney_bean" | "runner_bean" | "cowpea" | "tepary_bean" => Seed::Kidney,
        "lentil" => Seed::Lens,
        "broad_bean" | "lima_bean" | "lupin_bean" => Seed::Flat,
        "chickpea" => Seed::Beaked,
        "grass_pea" => Seed::Wedge,
        _ => Seed::Round,
    }
}

fn ear(key: &str) -> Ear {
    match key {
        "wheat" | "spelt" => Ear::Spike { awns: false },
        "barley" | "rye" => Ear::Spike { awns: true },
        "sorghum" | "quinoa" => Ear::Dense,
        "amaranth" => Ear::Plume,
        "pearl_millet" => Ear::Cattail,
        "finger_millet" | "fonio" => Ear::Fingers,
        "maize" => Ear::Cob,
        "buckwheat" => Ear::Buckwheat,
        _ => Ear::Panicle,
    }
}
