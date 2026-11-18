use shared::poi::PoiKind;
use shared::protocol::{ClientMessage, PickNews, ServerMessage};
use shared::skill::SkillId;
use shared::vegetable::{CROPS, VegetablePlant, Vegetables};

use crate::connections::tests::TestClient;

/// Into the first farm (by seed) with a field of crops, on its ground floor.
fn at_a_crop_field() -> TestClient {
    for seed in 1..200 {
        let mut test = TestClient::new();
        test.send(&[ClientMessage::DevEnterSite {
            kind: PoiKind::Farm,
            seed,
            level: 0,
            ruin: 0,
        }]);
        if !plants(&test).is_empty() {
            return test;
        }
    }
    panic!("no farm has a field of crops");
}

fn now(test: &TestClient) -> u64 {
    test.world.minutes() as u64
}

/// Every vegetable plant on the player's level.
fn plants(test: &TestClient) -> Vec<((i64, i64), VegetablePlant)> {
    let visit = test.client.visit.as_ref().unwrap();
    (visit.level.level.vegetable_plants())
        .map(|(at, p)| (at, p.clone()))
        .collect()
}

/// A full grown plant (moving the clock on a while if none is), with the
/// player standing on it (crops never block).
fn on_a_grown_plant(test: &mut TestClient) -> (i64, i64) {
    for _ in 0..200 {
        let now = now(test);
        let grown = plants(test).into_iter().find(|(_, plant)| {
            let def = Vegetables::built_in().get(&plant.vegetable).unwrap();
            plant.can_pick(def, now)
        });
        if let Some(((x, y), _)) = grown {
            test.send(&[ClientMessage::DevTeleport { x, y }]);
            return (x, y);
        }
        test.world.set_time(Some(now as f64 + 300.0), None);
    }
    panic!("nothing grows");
}

/// Farms' fields of crops grow one cultivated crop each on every cell, never
/// in pasture or orchards; a full grown plant picked gives its vegetable and
/// Crops practice, is kept, and can't be picked again at once.
#[test]
fn picking_a_field_gives_vegetables_and_is_kept() {
    let mut test = at_a_crop_field();
    let visit = test.client.visit.as_ref().unwrap();
    let sown: Vec<_> = (visit.level.fields.iter())
        .filter(|field| !field.livestock && !field.orchard)
        .collect();
    for ((x, y), plant) in plants(&test) {
        let field = sown.iter().find(|field| field.contains(x, y));
        assert!(field.is_some(), "({x}, {y})");
        let def = Vegetables::built_in().get(&plant.vegetable).unwrap();
        assert!(!def.wild_only, "{}", def.key);
    }
    let at = on_a_grown_plant(&mut test);
    let when = now(&test);
    let answers = test.send(&[ClientMessage::Pick { x: at.0, y: at.1 }]);
    let picked = answers.iter().find_map(|answer| match answer {
        ServerMessage::Pick(PickNews::PickedVegetable {
            x,
            y,
            vegetable,
            count,
        }) => {
            assert_eq!((*x, *y), at);
            Some((vegetable.clone(), *count))
        }
        _ => None,
    });
    let (vegetable, count) = picked.unwrap_or_else(|| panic!("{answers:?}"));
    assert!(count >= 1);
    let carried = format!("{:?}", test.client.inventory);
    assert!(
        carried.contains(&format!("vegetable: {vegetable:?}")),
        "{carried}"
    );
    let crops = SkillId::find(CROPS).unwrap();
    assert!(test.client.sheet.skills.practice(crops) > 0);
    let visit = test.client.visit.as_ref().unwrap();
    let saved = &test.world.site(visit.id).levels[&0].plants_picked;
    assert_eq!(saved.get(&at), Some(&when));
    let again = test.send(&[ClientMessage::Pick { x: at.0, y: at.1 }]);
    assert!(again.contains(&ServerMessage::Pick(PickNews::Refused { x: at.0, y: at.1 })));
}
