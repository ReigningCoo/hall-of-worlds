use shared::fruit::{FruitPlant, Fruits, ORCHARDS};
use shared::poi::PoiKind;
use shared::protocol::{ClientMessage, PickNews, ServerMessage};
use shared::skill::SkillId;

use crate::connections::tests::TestClient;

/// Into the first farm (by seed) with an orchard, on its ground floor.
fn at_an_orchard() -> TestClient {
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
    panic!("no farm has an orchard");
}

fn now(test: &TestClient) -> u64 {
    test.world.minutes() as u64
}

/// Every fruit plant on the player's level.
fn plants(test: &TestClient) -> Vec<((i64, i64), FruitPlant)> {
    let visit = test.client.visit.as_ref().unwrap();
    let level = &visit.level.level;
    level
        .fruit_plants()
        .map(|(at, p)| (at, p.clone()))
        .collect()
}

/// A ripe plant (moving the clock on a while if none is), with the player
/// standing just north of it, outside its row.
fn by_a_ripe_plant(test: &mut TestClient) -> (i64, i64) {
    for _ in 0..80 {
        let now = now(test);
        let ripe = plants(test).into_iter().find(|(_, plant)| {
            let def = Fruits::built_in().get(&plant.fruit).unwrap();
            plant.can_pick(def, now)
        });
        if let Some(((x, y), _)) = ripe {
            test.send(&[ClientMessage::DevTeleport { x, y: y - 1 }]);
            return (x, y);
        }
        test.world.set_time(Some(now as f64 + 300.0), None);
    }
    panic!("nothing ripens");
}

/// Orchards grow in farms' orchard fields; a ripe plant picked gives its
/// fruit and Orchards practice, is kept, and can't be picked again at once;
/// one out of reach is refused.
#[test]
fn picking_an_orchard_gives_fruit_and_is_kept() {
    let mut test = at_an_orchard();
    let visit = test.client.visit.as_ref().unwrap();
    let orchards: Vec<_> = (visit.level.fields.iter())
        .filter(|field| field.orchard)
        .collect();
    for ((x, y), _) in plants(&test) {
        assert!(orchards.iter().any(|field| field.contains(x, y)));
    }
    let at = by_a_ripe_plant(&mut test);
    let far = (at.0 + 5, at.1 + 5);
    let refused = test.send(&[ClientMessage::Pick { x: far.0, y: far.1 }]);
    assert!(refused.contains(&ServerMessage::Pick(PickNews::Refused {
        x: far.0,
        y: far.1
    })));
    let when = now(&test);
    let answers = test.send(&[ClientMessage::Pick { x: at.0, y: at.1 }]);
    let picked = answers.iter().find_map(|answer| match answer {
        ServerMessage::Pick(PickNews::PickedFruit { x, y, fruit, count }) => {
            assert_eq!((*x, *y), at);
            Some((fruit.clone(), *count))
        }
        _ => None,
    });
    let (fruit, count) = picked.unwrap_or_else(|| panic!("{answers:?}"));
    assert!(count >= 1);
    let carried = format!("{:?}", test.client.inventory);
    assert!(carried.contains(&format!("fruit: {fruit:?}")), "{carried}");
    let orchards = SkillId::find(ORCHARDS).unwrap();
    assert!(test.client.sheet.skills.practice(orchards) > 0);
    let visit = test.client.visit.as_ref().unwrap();
    let saved = &test.world.site(visit.id).levels[&0].plants_picked;
    assert_eq!(saved.get(&at), Some(&when));
    let again = test.send(&[ClientMessage::Pick { x: at.0, y: at.1 }]);
    assert!(again.contains(&ServerMessage::Pick(PickNews::Refused { x: at.0, y: at.1 })));
}
