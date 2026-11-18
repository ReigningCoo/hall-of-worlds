use shared::dungeon::Cell;
use shared::herb::{FORAGING, HerbPatch};
use shared::protocol::{ClientMessage, PickNews, ServerMessage};
use shared::skill::SkillId;

use crate::connections::tests::TestClient;
use crate::storage::SiteId;

/// Into a herbalist's hut, its first level with its beds.
fn at_a_herbalist() -> TestClient {
    let mut test = TestClient::new();
    test.send(&[ClientMessage::DevEnterSite {
        kind: shared::poi::PoiKind::HerbalistHut,
        seed: 1,
        level: 0,
        ruin: 0,
    }]);
    test
}

fn now(test: &TestClient) -> u64 {
    test.world.minutes() as u64
}

/// Every patch on the player's level.
fn patches(test: &TestClient) -> Vec<((i64, i64), HerbPatch)> {
    let visit = test.client.visit.as_ref().unwrap();
    let level = &visit.level.level;
    level.herbs().map(|(at, p)| (at, p.clone())).collect()
}

/// A patch that can be harvested now (moving the clock on a while if none
/// can), with the player standing on it.
fn on_a_ripe_patch(test: &mut TestClient) -> (i64, i64) {
    for _ in 0..40 {
        let now = now(test);
        let ripe = patches(test).into_iter().find(|(_, patch)| {
            let def = test.world.herbs.get(&patch.herb).unwrap();
            patch.can_pick(def, now) && def.early.taken > 0
        });
        if let Some(((x, y), _)) = ripe {
            test.send(&[ClientMessage::DevTeleport { x, y }]);
            return (x, y);
        }
        test.world.set_time(Some(now as f64 + 300.0), None);
    }
    panic!("nothing ripens");
}

fn pick(test: &mut TestClient, (x, y): (i64, i64)) -> Vec<ServerMessage> {
    test.send(&[ClientMessage::Pick { x, y }])
}

/// The beds grow herbs, each picked gives its herb, Foraging practice and its
/// name, and is kept so it grows back from then, also after coming in again.
#[test]
fn picking_a_bed_gives_herbs_and_is_kept() {
    let mut test = at_a_herbalist();
    let all = patches(&test);
    assert!(!all.is_empty());
    let visit = test.client.visit.as_ref().unwrap();
    for ((x, y), _) in &all {
        assert_eq!(visit.level.level.cell(*x, *y), Cell::HerbBed);
    }
    let at = on_a_ripe_patch(&mut test);
    let herb = patches(&test)
        .into_iter()
        .find(|(p, _)| *p == at)
        .unwrap()
        .1
        .herb;
    assert!(!test.client.sheet.herbs.knows_name(&herb));
    let when = now(&test);
    let answers = pick(&mut test, at);
    let count = answers.iter().find_map(|answer| match answer {
        ServerMessage::Pick(PickNews::Picked {
            x,
            y,
            herb: got,
            count,
            ..
        }) => {
            assert_eq!(((*x, *y), got), (at, &herb));
            Some(*count)
        }
        _ => None,
    });
    assert!(count.is_some_and(|count| count > 0), "{answers:?}");
    assert!(format!("{:?}", test.client.inventory).contains(&format!("herb: {herb:?}")));
    assert!(test.client.sheet.herbs.knows_name(&herb));
    let foraging = SkillId::find(FORAGING).unwrap();
    assert!(test.client.sheet.skills.practice(foraging) > 0);
    // Told with its cells, kept in the site's save.
    let told = answers.iter().any(|answer| match answer {
        ServerMessage::LevelCells(chunk) => (chunk.herbs.iter())
            .any(|seen| (seen.x, seen.y) == at && seen.patch.picked == Some(when)),
        _ => false,
    });
    assert!(told, "{answers:?}");
    let visit = test.client.visit.as_ref().unwrap();
    let saved = &test.world.site(visit.id).levels[&0].plants_picked;
    assert_eq!(saved.get(&at), Some(&when));
    // Not again straight away.
    let answers = pick(&mut test, at);
    assert!(answers.contains(&ServerMessage::Pick(PickNews::Refused { x: at.0, y: at.1 })));
    // Coming in again, it is as it was left.
    let mut test = TestClient::of(test.leave());
    test.send(&[ClientMessage::DevEnterSite {
        kind: shared::poi::PoiKind::HerbalistHut,
        seed: 1,
        level: 0,
        ruin: 0,
    }]);
    let again = patches(&test)
        .into_iter()
        .find(|(p, _)| *p == at)
        .unwrap()
        .1;
    assert_eq!(again.picked, Some(when));
}

/// Out of reach, or where nothing grows, is refused, and nothing is given.
#[test]
fn picking_out_of_reach_is_refused() {
    let mut test = at_a_herbalist();
    let at = on_a_ripe_patch(&mut test);
    let before = test.client.inventory.clone();
    let far = (at.0 + 3, at.1);
    assert!(
        pick(&mut test, far).contains(&ServerMessage::Pick(PickNews::Refused {
            x: far.0,
            y: far.1
        }))
    );
    let visit = test.client.visit.as_ref().unwrap();
    let (min_x, min_y, width, height) = visit.level.level.bounds();
    let bare = (min_y..min_y + height)
        .flat_map(|y| (min_x..min_x + width).map(move |x| (x, y)))
        .find(|&(x, y)| {
            (x - at.0).abs().max((y - at.1).abs()) == 1 && visit.herb_at(x, y).is_none()
        })
        .unwrap();
    assert!(
        pick(&mut test, bare).contains(&ServerMessage::Pick(PickNews::Refused {
            x: bare.0,
            y: bare.1
        }))
    );
    assert_eq!(test.client.inventory, before);
}

/// The wilderness has herbs growing wild, on open ground and grass, sent with
/// its cells, and a harvest there is kept in the wilderness's save.
#[test]
fn herbs_grow_wild() {
    let mut test = TestClient::new();
    // Somewhere herbs grow (the start is hot desert, where none do), on land.
    let (sx, sy) = test.client.world_position();
    let wild = &test.world.wilderness.wild;
    let herbs = &test.world.herbs;
    let start = (0..400)
        .flat_map(|d| [(sx + d * 7, sy - d * 5), (sx - d * 7, sy + d * 5)])
        .find(|&(x, y)| {
            let x = x.rem_euclid(wild.points());
            let habitat = wild.habitat_at(x, y);
            habitat.metres > 20 && shared::herb::wild_herbs(herbs, &habitat).len() > 5
        })
        .expect("herbs grow somewhere near");
    test.client.character.world_position = start;
    let answers = test.send(&[ClientMessage::Down]);
    let all = patches(&test);
    assert!(all.len() > 20, "{}", all.len());
    let visit = test.client.visit.as_ref().unwrap();
    for ((x, y), _) in &all {
        let cell = visit.level.level.cell(*x, *y);
        assert!(
            matches!(cell, Cell::Ground | Cell::Grass | Cell::HerbBed),
            "{cell:?}"
        );
    }
    let sent = answers.iter().any(|answer| match answer {
        ServerMessage::LevelCells(chunk) => !chunk.herbs.is_empty(),
        _ => false,
    });
    assert!(sent);
    let at = on_a_ripe_patch(&mut test);
    let when = now(&test);
    let answers = pick(&mut test, at);
    assert!(
        answers
            .iter()
            .any(|a| matches!(a, ServerMessage::Pick(PickNews::Picked { .. }))),
        "{answers:?}"
    );
    let visit = test.client.visit.as_ref().unwrap();
    let (site, index, kept) = visit.plant_key(at.0, at.1);
    if site == SiteId::Wilderness {
        assert_eq!(kept, (visit.kept_at(at.0), at.1));
    }
    let saved = &test.world.site(site).levels[&index].plants_picked;
    assert_eq!(saved.get(&kept), Some(&when));
}
