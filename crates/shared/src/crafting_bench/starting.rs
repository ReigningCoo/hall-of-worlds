//! The tools a generated bench starts with: the fewest that make it useful,
//! more in grander sites, rolled from hashes for the server to make once.

use super::Craft;
use crate::furniture::{Furniture, Piece};
use crate::item::{ItemKind, Quality};
use crate::poi::PoiKind;
use crate::rng::Seed;

/// Each tool a site of `kind` may start with on a bench of `craft`, and how
/// likely, in percent (owner's, chances Claude's first guesses): a herbalist
/// a Mortar and Pestle; a tower a Retort too; a castle besides half the time
/// an Alembic, and a quarter of the time each a Crucible, Filter and Pelican.
fn chances(craft: Craft, kind: PoiKind) -> &'static [(ItemKind, u64)] {
    use ItemKind::{Alembic, Crucible, Filter, MortarAndPestle, Pelican, Retort};
    match (craft, kind) {
        (Craft::Alchemy, PoiKind::StoneTower) => &[(MortarAndPestle, 100), (Retort, 100)],
        (Craft::Alchemy, PoiKind::Castle) => &[
            (MortarAndPestle, 100),
            (Retort, 100),
            (Alembic, 50),
            (Crucible, 25),
            (Filter, 25),
            (Pelican, 25),
        ],
        (Craft::Alchemy, _) => &[(MortarAndPestle, 100)],
    }
}

/// What `bench` (on level `index` of a site of `kind` from its map `seed`)
/// starts with: each tool's kind and quality, rolled as drops' are (mostly
/// Common; never an artifact). Nothing for a piece that is not a bench.
pub fn starting_tools(
    kind: PoiKind,
    seed: Seed,
    index: u32,
    bench: &Piece,
) -> Vec<(ItemKind, Quality)> {
    let Furniture::CraftingBench { craft } = bench.furniture else {
        return Vec::new();
    };
    let at = [i64::from(index), bench.at.0, bench.at.1];
    let mut rng = Seed::new(seed.derive("crafting bench tools").at(&at)).rng();
    let mut tools = Vec::new();
    for &(tool, percent) in chances(craft, kind) {
        let roll = rng.range_u64(0, 100);
        let quality = match Quality::roll(&mut rng) {
            Quality::Artifact => Quality::Legendary,
            quality => quality,
        };
        if roll < percent {
            tools.push((tool, quality));
        }
    }
    tools
}
