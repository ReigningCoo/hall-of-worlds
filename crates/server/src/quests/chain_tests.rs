//! Chains between a town's givers ("I heard you helped X"), and quests that
//! wait for a moon: the hints a giver gives, and what they say once met.

use shared::poi::{CHUNK_SIZE, ChunkPos};
use shared::protocol::{Heard, Hint, OfferState};
use shared::quest::{Need, Quest, QuestId};

use super::needs::{Verdict, judge};
use super::tests::NOW;
use super::{cache, offers};
use crate::connections::tests::TestClient;

/// Every town's quests within `chunks` of the start, nearest chunk first.
fn quests_near(test: &mut TestClient, chunks: i64) -> Vec<Quest> {
    let (x, y) = test.client.world_position();
    let reach = chunks * CHUNK_SIZE;
    let mut towns: Vec<ChunkPos> = (test.world.pois)
        .in_area((x - reach, y - reach), (x + reach, y + reach))
        .into_iter()
        .filter(|poi| poi.kind.is_town())
        .map(|poi| poi.chunk)
        .collect();
    towns.sort_unstable();
    (towns.into_iter())
        .filter_map(|chunk| cache::town(&mut test.world, chunk))
        .flat_map(|town| town.quests.iter().cloned().collect::<Vec<_>>())
        .collect()
}

/// A quest's needs but standing, which these tests give the character.
fn needs_but_standing(quest: &Quest) -> Vec<Need> {
    (quest.requirements.iter())
        .map(|requirement| requirement.need)
        .filter(|need| !matches!(need, Need::Standing { .. }))
        .collect()
}

/// Standing enough for any quest in `town`.
fn champion(test: &mut TestClient, town: ChunkPos) {
    test.client.character.quests.standing.insert(town, 150);
}

/// The other giver's quest `quest` needs done first, in its own town.
fn neighbour_need(quest: &Quest) -> Option<QuestId> {
    (quest.requirements.iter()).find_map(|requirement| match requirement.need {
        Need::Done(id)
            if id.town == quest.id.town
                && quest.id.earlier() != Some(id)
                && needs_but_standing(quest).len() == 1 =>
        {
            Some(id)
        }
        _ => None,
    })
}

#[test]
fn a_giver_names_the_neighbour_to_help_then_has_heard_of_it() {
    let mut test = TestClient::new();
    let quests = quests_near(&mut test, 24);
    let (quest, needed) = (quests.iter())
        .find_map(|quest| Some((quest.clone(), neighbour_need(quest)?)))
        .expect("a giver needing a neighbour's quest near the start");
    champion(&mut test, quest.id.town);
    let Verdict::Missing(hints) = judge(&quest, &test.client, &test.world) else {
        panic!("not yet");
    };
    assert!(matches!(&hints[..], [Hint::Neighbour { .. }]), "{hints:?}");

    test.client.character.quests.done.insert(needed, NOW);
    assert_eq!(judge(&quest, &test.client, &test.world), Verdict::Met);
    let offered = offers::giver_offers(
        &test.client,
        &mut test.world,
        quest.id.town,
        quest.building,
        NOW,
    );
    let offer = offered.iter().find(|offer| offer.quest.id == quest.id);
    assert!(
        matches!(
            offer.map(|offer| &offer.state),
            Some(OfferState::Available {
                heard: Some(Heard::Neighbour { .. }),
                ..
            })
        ),
        "{:?}",
        offer.map(|offer| &offer.state)
    );
}

#[test]
fn a_moon_is_waited_for_until_it_is_up() {
    let mut test = TestClient::new();
    assert!(test.world.moon_count() > 0, "the test world has moons");
    let quests = quests_near(&mut test, 40);
    let quest = (quests.into_iter())
        .find(|quest| {
            let needs = needs_but_standing(quest);
            needs.iter().all(|need| matches!(need, Need::Moon { .. })) && !needs.is_empty()
        })
        .expect("a quest waiting only for a moon near the start");
    champion(&mut test, quest.id.town);
    let Need::Moon { moon } = needs_but_standing(&quest)[0] else {
        unreachable!()
    };
    let (mut up, mut down) = (false, false);
    for hour in 0..96 {
        test.world.set_time(Some(f64::from(hour) * 60.0), None);
        match judge(&quest, &test.client, &test.world) {
            Verdict::Met => {
                assert!(test.world.moon_up(moon));
                up = true;
            }
            Verdict::Missing(hints) => {
                assert!(matches!(&hints[..], [Hint::Moon { .. }]), "{hints:?}");
                down = true;
            }
            Verdict::Hidden => panic!("a moon hides nothing"),
        }
    }
    assert!(up && down, "up {up}, down {down}");
}

#[test]
fn a_giver_offers_one_quest_at_a_time() {
    let mut test = TestClient::new();
    let quests = quests_near(&mut test, 24);
    let busy = (quests.iter())
        .find(|quest| {
            (quests.iter())
                .filter(|other| {
                    other.id.town == quest.id.town
                        && other.building == quest.building
                        && other.id.earlier().is_none()
                })
                .count()
                > 1
        })
        .expect("a giver with two chains near the start");
    let (town, building) = (busy.id.town, busy.building);
    champion(&mut test, town);
    let normal = |test: &mut TestClient| {
        let offers = offers::giver_offers(&test.client, &mut test.world, town, building, NOW);
        (offers.into_iter())
            .filter(|offer| offer.quest.kind != shared::quest::QuestKind::Daily)
            .count()
    };
    assert_eq!(normal(&mut test), 1);
}
