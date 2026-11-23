use shared::item::ItemKind;
use shared::protocol::ClientMessage;
use shared::site::{DevArea, DevLayout, DevMobs};

use crate::connections::tests::TestClient;
use crate::storage::SiteId;

/// A client in the Tavern Dev Area, and its id.
fn in_tavern() -> (TestClient, SiteId) {
    let mut test = TestClient::new();
    test.send(&[ClientMessage::DevGenerateArea(DevArea {
        size: 32,
        layout: DevLayout::Tavern,
        mobs: DevMobs::None,
        seed: 1,
    })]);
    let id = test.client.visit.as_ref().expect("in the tavern").id;
    (test, id)
}

#[test]
fn the_taverns_racks_are_stocked_and_its_fire_lit() {
    let (mut test, id) = in_tavern();
    let saved = &test.world.site(id).levels[&0];
    assert!(saved.furnishings_stocked);
    // A Firkin rack and a Tun stand; two wine and two spirit racks.
    assert_eq!(saved.racks.len(), 2);
    assert!(saved.racks.values().all(|casks| casks.items().count() > 0));
    let casks = saved.racks.values().flat_map(|casks| casks.items());
    assert!(casks.into_iter().all(|item| item.kind == ItemKind::Cask));
    assert_eq!(saved.bottle_racks.len(), 4);
    let bottles: usize = saved.bottle_racks.values().map(|b| b.items().count()).sum();
    assert!(bottles > 30, "{bottles}");
    assert_eq!(saved.fireplaces.len(), 1);
    assert!(saved.fireplaces.values().all(|fire| fire.always_lit));
}
