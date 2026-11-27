//! Gathering steps' herbs: the name learned on taking the quest, the count
//! following what is carried (bags too, going down as well as up), and
//! handing in taking exactly the count, the poorest first.

use shared::herb::new_item;
use shared::id::Id;
use shared::item::{Item, ItemKind, Placed, Quality, Rotation};
use shared::protocol::{QuestNews, QuestRefusal, ServerMessage};
use shared::quest::{HerbAsk, Quest};

use super::dungeon_tests::a_dungeon_where;
use super::herbs::taken;
use super::tests::NOW;
use super::{follow::follow, turn_in::turn_in};
use crate::connections::tests::TestClient;
use crate::storage::Progress;

/// A quest open to anyone near the start whose only step gathers a herb.
fn a_herb_quest(test: &mut TestClient) -> (Quest, HerbAsk, u32) {
    let (_, quest) = a_dungeon_where(test, |quest| {
        quest.steps.len() == 1 && quest.steps[0].herb.is_some()
    });
    let step = &quest.steps[0];
    let ask = step.herb.clone().unwrap();
    let count = step.count.unwrap();
    (quest, ask, count)
}

/// `count` of the asked herb at `quality`, with id `n`.
fn herbs(test: &TestClient, ask: &HerbAsk, quality: Quality, count: u32, n: u128) -> Item {
    let def = test.world.herbs.get(&ask.herb).expect("the herb");
    let mut item = new_item(def, count, Id::for_test(n), None);
    item.quality = Some(quality);
    item
}

/// The client's inventory changed: told, and the tick's quest work done.
fn changed(test: &mut TestClient) {
    assert!(test.client.send_inventory());
    follow(std::slice::from_mut(&mut test.client), &mut test.world);
}

fn counted(test: &TestClient, quest: &Quest) -> u32 {
    test.client.character.quests.taken[&quest.id].counts[0]
}

fn refused(test: &mut TestClient, quest: &Quest) -> bool {
    (test.send(&[]).into_iter()).any(|message| {
        message
            == ServerMessage::Quest(QuestNews::Refused {
                id: quest.id,
                why: QuestRefusal::NotReady,
            })
    })
}

#[test]
fn herbs_carried_anywhere_count_and_the_poorest_are_handed_in() {
    let mut test = TestClient::new();
    let (quest, ask, count) = a_herb_quest(&mut test);
    let least = ask.least.max(Quality::Common);
    let better = Quality::ALL[least as usize + 1];
    test.client.inventory.items.clear();
    test.client.sheet.herbs.forget_all();

    // Taking it (as `accept` does once it is on offer) teaches the herb's name.
    let progress = Progress {
        at: NOW,
        counts: vec![0; quest.steps.len()],
        ready: false,
        escort: None,
    };
    test.client
        .character
        .quests
        .taken
        .insert(quest.id, progress);
    assert!(taken(&mut test.client, &test.world, &quest));
    assert!(test.client.sheet.herbs.knows_name(&ask.herb));
    assert_eq!(counted(&test, &quest), 0);

    // All but one in a pouch inside a bag, the last in the own grid.
    let mut pouch = Item::new(Id::for_test(901), ItemKind::HerbPouch);
    pouch.contents.push(Placed {
        item: herbs(&test, &ask, least, count - 1, 902),
        x: 0,
        y: 0,
        rotation: Rotation::None,
    });
    let mut bag = Item::new(Id::for_test(900), ItemKind::Bag);
    bag.contents.push(Placed {
        item: pouch,
        x: 0,
        y: 0,
        rotation: Rotation::None,
    });
    test.client.inventory.items.push(Placed {
        item: bag,
        x: 0,
        y: 0,
        rotation: Rotation::None,
    });
    changed(&mut test);
    assert_eq!(counted(&test, &quest), count - 1);
    let one = herbs(&test, &ask, least, 1, 903);
    let limits = test.world.stacks;
    let new_id = &mut shared::id::stand_ins();
    test.client.inventory.give(one, limits, new_id).unwrap();
    changed(&mut test);
    assert_eq!(counted(&test, &quest), count);
    assert!(test.client.character.quests.taken[&quest.id].ready);

    // Eaten, dropped or moved off: the count goes down, and it can't be
    // handed in.
    let grid_one = (test.client.inventory.items.iter())
        .find(|placed| placed.item.kind == ItemKind::Herb)
        .map(|placed| placed.item.id)
        .unwrap();
    test.client.inventory.take_one(grid_one, new_id).unwrap();
    changed(&mut test);
    assert_eq!(counted(&test, &quest), count - 1);
    assert!(!test.client.character.quests.taken[&quest.id].ready);
    test.send(&[]);
    turn_in(&mut test.client, &mut test.world, quest.id, NOW, false);
    assert!(refused(&mut test, &quest));
    assert!(test.client.character.quests.taken.contains_key(&quest.id));

    // Better ones too: the poorer go, the best are kept.
    let best = herbs(&test, &ask, better, 3, 904);
    test.client.inventory.give(best, limits, new_id).unwrap();
    changed(&mut test);
    turn_in(&mut test.client, &mut test.world, quest.id, NOW, false);
    assert!(!test.client.character.quests.taken.contains_key(&quest.id));
    let all = HerbAsk {
        least: Quality::Poor,
        ..ask.clone()
    };
    let kept = HerbAsk {
        least: better,
        ..ask
    };
    assert_eq!(all.carried(&test.client.inventory), 2);
    assert_eq!(kept.carried(&test.client.inventory), 2);
}
