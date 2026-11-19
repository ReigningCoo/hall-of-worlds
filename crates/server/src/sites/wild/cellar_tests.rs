use shared::cellar::cellar_level;
use shared::item::ItemKind;

use super::chunks::ensure_chunk;
use super::tests::chunk_near_start;
use crate::connections::tests::TestClient;
use crate::storage::SiteId;

/// How many casks lie on the wilderness's ground.
fn casks_lying(test: &mut TestClient) -> usize {
    let ground = &test.world.site(SiteId::Wilderness).levels;
    (ground
        .get(&0)
        .map(|level| level.ground.values())
        .into_iter())
    .flatten()
    .flatten()
    .filter(|item| item.kind == ItemKind::Cask)
    .count()
}

#[test]
fn a_set_in_cellar_is_stocked_on_the_wilderness_ground_once() {
    let mut test = TestClient::new();
    let plan = chunk_near_start(&test, |cutout| {
        let poi = cutout.poi();
        cutout.offset().is_some() && cellar_level(poi.kind, poi.seed, 1, false) == Some(0)
    });
    let chunk = plan.cutout().unwrap().poi().chunk;
    assert_eq!(casks_lying(&mut test), 0);
    ensure_chunk(&mut test.world, chunk);
    let stocked = casks_lying(&mut test);
    assert!(stocked > 0, "the cellar's casks lie there");
    let saved = &test.world.site(SiteId::Poi(chunk)).levels[&0];
    assert!(saved.cellar_stocked);
    // Worked out again: nothing more.
    test.world.wilderness.forget(Some(chunk));
    ensure_chunk(&mut test.world, chunk);
    assert_eq!(casks_lying(&mut test), stocked);
}
