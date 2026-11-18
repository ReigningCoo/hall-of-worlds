//! The wild trees' rows: where each grows, how common it is, its wood and how
//! many logs a grown one gives. From `docs/context/research-trees.md`'s tables
//! (logs by its formula from height and trunk).

use super::{Commonness, Group, TreeKind};
use crate::climate::Biome;
use crate::material::Material;

/// One wild tree's row.
pub struct TreeDef {
    pub kind: TreeKind,
    /// What players call it.
    pub name: &'static str,
    pub group: Group,
    /// Where it grows.
    pub biomes: &'static [Biome],
    /// The yearly average temperatures it grows in, °C, both ends included.
    pub celsius: (i32, i32),
    pub commonness: Commonness,
    /// Grows by water (willow, alder): common within reach of it, rare elsewhere.
    pub wet: bool,
    pub wood: Material,
    /// Logs a grown one gives when felled, 1 to 12.
    pub logs: u8,
}

use Biome::*;
use Commonness::*;
use Group::*;

#[allow(clippy::too_many_arguments)]
const fn row(
    kind: TreeKind,
    name: &'static str,
    group: Group,
    biomes: &'static [Biome],
    celsius: (i32, i32),
    commonness: Commonness,
    wood: Material,
    logs: u8,
) -> TreeDef {
    TreeDef {
        kind,
        name,
        group,
        biomes,
        celsius,
        commonness,
        wet: false,
        wood,
        logs,
    }
}

/// The same, growing by water.
const fn wet(def: TreeDef) -> TreeDef {
    TreeDef { wet: true, ..def }
}

/// Temperate biomes, for the trees that grow by water in any of them.
const TEMPERATE: &[Biome] = &[
    BorealForest,
    Grassland,
    Woodland,
    SeasonalForest,
    TemperateRainforest,
];

/// Every wild tree, in `TreeKind`'s order.
#[rustfmt::skip]
pub const TREES: [TreeDef; 27] = [
    row(TreeKind::ScotsPine, "Scots Pine", Conifer, &[BorealForest, Woodland, Grassland], (-3, 14), Common, Material::Pine, 7),
    row(TreeKind::NorwaySpruce, "Norway Spruce", Conifer, &[BorealForest, TemperateRainforest, Tundra], (-6, 10), Common, Material::Spruce, 9),
    row(TreeKind::SilverFir, "Silver Fir", Conifer, &[BorealForest, TemperateRainforest], (2, 12), Uncommon, Material::Fir, 10),
    row(TreeKind::Larch, "Larch", Conifer, &[BorealForest, Tundra], (-8, 10), Uncommon, Material::Larch, 9),
    row(TreeKind::DouglasFir, "Douglas Fir", Conifer, &[TemperateRainforest], (4, 14), Common, Material::DouglasFir, 12),
    row(TreeKind::RedCedar, "Western Red Cedar", Conifer, &[TemperateRainforest], (4, 13), Uncommon, Material::RedCedar, 12),
    row(TreeKind::CedarOfLebanon, "Cedar of Lebanon", Conifer, &[Woodland], (6, 16), Rare, Material::Cedar, 12),
    row(TreeKind::Cypress, "Cypress", Conifer, &[Woodland, Desert], (12, 20), Uncommon, Material::Cypress, 7),
    row(TreeKind::SilverBirch, "Silver Birch", Broadleaf, &[BorealForest, Woodland, Tundra], (-6, 12), Common, Material::Birch, 5),
    row(TreeKind::Aspen, "Aspen", Broadleaf, &[BorealForest, Grassland], (-6, 12), Uncommon, Material::Aspen, 6),
    row(TreeKind::Oak, "Oak", Broadleaf, &[SeasonalForest, Woodland], (6, 16), Common, Material::Oak, 11),
    row(TreeKind::Beech, "Beech", Broadleaf, &[SeasonalForest, TemperateRainforest], (6, 14), Common, Material::Beech, 10),
    row(TreeKind::Ash, "Ash", Broadleaf, &[SeasonalForest], (6, 14), Uncommon, Material::Ash, 9),
    row(TreeKind::Sycamore, "Sycamore", Broadleaf, &[SeasonalForest, TemperateRainforest], (5, 14), Uncommon, Material::Maple, 9),
    row(TreeKind::Linden, "Linden", Broadleaf, &[SeasonalForest], (6, 14), Uncommon, Material::Linden, 10),
    row(TreeKind::WychElm, "Wych Elm", Broadleaf, &[SeasonalForest, Woodland], (5, 14), Uncommon, Material::Elm, 9),
    wet(row(TreeKind::WhiteWillow, "White Willow", Broadleaf, TEMPERATE, (0, 18), Rare, Material::Willow, 8)),
    wet(row(TreeKind::BlackAlder, "Black Alder", Broadleaf, &[BorealForest, SeasonalForest], (0, 14), Rare, Material::Alder, 5)),
    row(TreeKind::HolmOak, "Holm Oak", Broadleaf, &[Woodland], (12, 20), Common, Material::HolmOak, 8),
    row(TreeKind::UmbrellaThorn, "Umbrella Thorn", Broadleaf, &[Savanna, Desert], (18, 30), Common, Material::Thorn, 3),
    row(TreeKind::CamelThorn, "Camel Thorn", Broadleaf, &[Savanna, Desert], (16, 26), Uncommon, Material::Thorn, 5),
    row(TreeKind::Mesquite, "Mesquite", Broadleaf, &[Desert, Grassland], (14, 24), Common, Material::Mesquite, 2),
    row(TreeKind::Mopane, "Mopane", Broadleaf, &[Savanna], (20, 28), Common, Material::Mopane, 6),
    row(TreeKind::Teak, "Teak", Broadleaf, &[Savanna, TropicalRainforest], (22, 28), Uncommon, Material::Teak, 10),
    row(TreeKind::Mahogany, "Mahogany", Broadleaf, &[TropicalRainforest], (22, 28), Uncommon, Material::Mahogany, 12),
    row(TreeKind::Kapok, "Kapok", Broadleaf, &[TropicalRainforest, Savanna], (22, 28), Rare, Material::Kapok, 12),
    row(TreeKind::Ebony, "Ebony", Broadleaf, &[TropicalRainforest], (22, 28), Rare, Material::Ebony, 6),
];

impl TreeKind {
    /// Every wild tree, in the enum's order.
    pub fn all() -> impl Iterator<Item = Self> {
        TREES.iter().map(|def| def.kind)
    }

    /// Its row.
    pub fn def(self) -> &'static TreeDef {
        &TREES[self as usize]
    }

    /// What players call it.
    pub fn name(self) -> &'static str {
        self.def().name
    }
}
