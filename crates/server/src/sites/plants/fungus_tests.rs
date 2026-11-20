use shared::dungeon::Cell;
use shared::fungus::{Fungi, FungusPatch, Where};
use shared::herb::FORAGING;
use shared::poi::PoiKind;
use shared::protocol::{ClientMessage, PickNews, ServerMessage};
use shared::skill::SkillId;

use crate::connections::tests::TestClient;

/// Into the first cave (by seed) with fungi on its second level.
fn in_a_cave_with_fungi() -> TestClient {
    for seed in 1..50 {
        let mut test = TestClient::new();
        test.send(&[ClientMessage::DevEnterSite {
            kind: PoiKind::RockCave,
            seed,
            level: 1,
            ruin: 0,
        }]);
        if !patches(&test).is_empty() {
            return test;
        }
    }
    panic!("no cave grows fungi");
}

fn now(test: &TestClient) -> u64 {
    test.world.minutes() as u64
}

/// Every fungus patch growing on the player's level.
fn patches(test: &TestClient) -> Vec<((i64, i64), FungusPatch)> {
    let visit = test.client.visit.as_ref().unwrap();
    (visit.level.level.fungus_patches())
        .map(|(at, p)| (at, p.clone()))
        .collect()
}

/// A full grown patch (moving the clock on a while if none is), with the
/// player standing on it (fungi never block).
fn on_a_grown_patch(test: &mut TestClient) -> (i64, i64) {
    for _ in 0..200 {
        let now = now(test);
        let grown = patches(test).into_iter().find(|(_, patch)| {
            let def = Fungi::built_in().get(&patch.fungus).unwrap();
            patch.can_pick(def, now)
        });
        if let Some(((x, y), _)) = grown {
            test.send(&[ClientMessage::DevTeleport { x, y }]);
            return (x, y);
        }
        test.world.set_time(Some(now as f64 + 300.0), None);
    }
    panic!("nothing grows");
}

/// A cave's fungi grow on its floor, never above-ground kinds; a full grown
/// patch picked gives its fungus and Foraging practice, is kept, and can't be
/// picked again at once.
#[test]
fn picking_cave_fungi_gives_them_and_is_kept() {
    let mut test = in_a_cave_with_fungi();
    let visit = test.client.visit.as_ref().unwrap();
    for ((x, y), patch) in patches(&test) {
        assert_eq!(visit.level.level.cell(x, y), Cell::Floor);
        let def = Fungi::built_in().get(&patch.fungus).unwrap();
        assert_ne!(def.grows, Where::Surface, "{}", def.key);
    }
    let at = on_a_grown_patch(&mut test);
    let when = now(&test);
    let answers = test.send(&[ClientMessage::Pick { x: at.0, y: at.1 }]);
    let picked = answers.iter().find_map(|answer| match answer {
        ServerMessage::Pick(PickNews::PickedFungus {
            x,
            y,
            fungus,
            count,
            ..
        }) => {
            assert_eq!((*x, *y), at);
            Some((fungus.clone(), *count))
        }
        _ => None,
    });
    let (fungus, count) = picked.unwrap_or_else(|| panic!("{answers:?}"));
    assert!(count >= 1);
    let carried = format!("{:?}", test.client.inventory);
    assert!(
        carried.contains(&format!("fungus: {fungus:?}")),
        "{carried}"
    );
    let foraging = SkillId::find(FORAGING).unwrap();
    assert!(test.client.sheet.skills.practice(foraging) > 0);
    let visit = test.client.visit.as_ref().unwrap();
    let saved = &test.world.site(visit.id).levels[&visit.index].plants_picked;
    assert_eq!(saved.get(&at), Some(&when));
    let again = test.send(&[ClientMessage::Pick { x: at.0, y: at.1 }]);
    assert!(again.contains(&ServerMessage::Pick(PickNews::Refused { x: at.0, y: at.1 })));
}
