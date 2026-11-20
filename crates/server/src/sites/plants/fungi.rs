//! The fungi a site's level grows (`shared::fungus`): on a cave's levels and
//! a basement by their cells (`grow_underground`), and on its wild trees for
//! when they are felled (`felled_patch`), by the land at its poi and its seed.

use shared::dungeon::Cell;
use shared::fungus::{Fungi, FungusPatch, Underground, felled_patch, grow_underground};
use shared::mob::Habitat;
use shared::poi::{Poi, PoiKind};
use shared::site::{Site, SiteLevel};

/// The fungus patches on level `index` of `site` (at `poi`, whose land is
/// `habitat`), in its own cells.
pub(crate) fn site_fungi(
    poi: &Poi,
    site: &Site,
    index: u32,
    level: &SiteLevel,
    habitat: &Habitat,
) -> Vec<((i64, i64), FungusPatch)> {
    let fungi = Fungi::built_in();
    let kind = if site.is_cave() {
        let crypt = matches!(
            poi.kind,
            PoiKind::Crypt | PoiKind::MossyCrypt | PoiKind::BloodyCrypt
        );
        Some(Underground::Cave {
            depth: index,
            levels: site.levels(),
            crypt,
        })
    } else {
        site.is_underground(index).then_some(Underground::Basement)
    };
    let mut patches = match kind {
        Some(kind) => grow_underground(level, fungi, kind, habitat, poi.seed),
        None => Vec::new(),
    };
    let felled = poi.seed.derive("felled fungi");
    let (min_x, min_y, width, height) = level.level.bounds();
    for y in min_y..min_y + height {
        for x in min_x..min_x + width {
            if level.level.cell(x, y) == Cell::Tree
                && let Some(patch) = felled_patch(fungi, habitat, felled, (x, y))
            {
                patches.push(((x, y), patch));
            }
        }
    }
    patches
}
