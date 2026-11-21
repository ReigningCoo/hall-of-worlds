//! Generated benches' starting tools, made once with the server's ids, and
//! benches made with the developer tool put back on their level.

use shared::crafting_bench::{BenchTools, always_lit, starting_tools};
use shared::furniture::Furniture;
use shared::item::Item;
use shared::poi::Poi;
use shared::site::SiteLevel;

use crate::ServerWorld;
use crate::ids::new_id;
use crate::storage::SiteId;

/// Gives level `index` of a poi's or developer's site (`level` as generated,
/// before benches made by hand go on) its benches' starting tools
/// (`crafting_bench::starting_tools`), each bench once: a bench with nothing
/// kept for it yet (so hearths new to an older save are stocked too). A
/// hearth in a household's building is always lit (`always_lit`). A first
/// level is stocked in its own site's save wherever it is played.
pub(in crate::sites) fn stock_benches(
    world: &mut ServerWorld,
    (id, poi): (SiteId, &Poi),
    index: u32,
    level: &SiteLevel,
) {
    if !matches!(id, SiteId::Poi(_) | SiteId::Dev { .. }) {
        return;
    }
    let saved = world.site(id).levels.get(&index);
    let stocked = |at| saved.is_some_and(|level| level.benches.contains_key(&at));
    let new = (level.level.furniture())
        .filter(|piece| matches!(piece.furniture, Furniture::CraftingBench { .. }))
        .any(|piece| !stocked(piece.at));
    if !new {
        return;
    }
    let saved = world.site_mut(id).levels.entry(index).or_default();
    saved.benches_stocked = true;
    for piece in level.level.furniture() {
        let Furniture::CraftingBench { craft } = piece.furniture else {
            continue;
        };
        if saved.benches.contains_key(&piece.at) {
            continue;
        }
        let mut tools = BenchTools::new(craft);
        tools.always_lit = craft.is_hearth() && always_lit(poi.kind);
        for (kind, quality) in starting_tools(poi.kind, poi.seed, index, piece) {
            let mut item = Item::new(new_id(), kind);
            item.quality = Some(quality);
            // Each kind is rolled once, so nothing is swapped off.
            let _ = tools.put(item);
        }
        saved.benches.entry(piece.at).or_insert(tools);
    }
}

/// Puts the benches made by hand on level `index` of a site onto `level`,
/// over whatever they cover.
pub(in crate::sites) fn put_made_benches(
    world: &mut ServerWorld,
    id: SiteId,
    index: u32,
    level: &mut SiteLevel,
) {
    let Some(saved) = world.site(id).levels.get(&index) else {
        return;
    };
    for &piece in &saved.made_benches {
        level.level.place_furniture(piece);
    }
}
