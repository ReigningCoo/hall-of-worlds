use std::collections::{BTreeMap, BTreeSet};

use super::super::test_world::TestWorld;
use super::super::tests::every_town;
use super::super::world::size;
use super::OPEN;
use crate::quest::{Need, Quest, Slot, Standing};
use crate::story::Part;

fn least_standing(quest: &Quest) -> Standing {
    (quest.requirements.iter())
        .find_map(|r| match r.need {
            Need::Standing { least, .. } => Some(least),
            _ => None,
        })
        .unwrap_or(Standing::Stranger)
}

#[test]
fn a_stranger_is_offered_only_a_few_quests_one_a_giver() {
    let world = TestWorld::new();
    let mut spread = BTreeSet::new();
    for (town, quests) in every_town(&world) {
        let open: Vec<&Quest> = (quests.iter())
            .filter(|q| matches!(q.id.slot, Slot::Giver { .. }))
            .filter(|q| least_standing(q) == Standing::Stranger && q.id.earlier().is_none())
            .filter(|q| (q.requirements.iter()).all(|r| matches!(r.need, Need::Skill { .. })))
            .collect();
        let most = OPEN[size(town.kind).unwrap()];
        assert!(
            !open.is_empty() && open.len() <= most,
            "{town:?}: {}",
            open.len()
        );
        let buildings: BTreeSet<u16> = open.iter().map(|q| q.building).collect();
        assert_eq!(buildings.len(), open.len(), "{town:?}");
        for quest in &quests {
            if matches!(quest.id.slot, Slot::Giver { .. }) {
                spread.insert(least_standing(quest));
            }
        }
    }
    for tier in [
        Standing::Stranger,
        Standing::Known,
        Standing::Trusted,
        Standing::Honoured,
    ] {
        assert!(spread.contains(&tier), "{tier:?}");
    }
}

#[test]
fn each_part_of_a_chain_needs_more_standing_and_is_numbered() {
    let world = TestWorld::new();
    for (_, quests) in every_town(&world) {
        let mut chains: BTreeMap<(u16, u8), Vec<&Quest>> = BTreeMap::new();
        for quest in &quests {
            if let Slot::Giver { building, slot, .. } = quest.id.slot {
                chains.entry((building, slot)).or_default().push(quest);
            }
        }
        for chain in chains.values() {
            let parts = chain.len() as u8;
            for (i, quest) in chain.iter().enumerate() {
                let expected = (parts > 1).then_some(Part {
                    number: i as u8 + 1,
                    of: Some(parts),
                });
                assert_eq!(quest.story.part, expected);
                if i > 0 {
                    let before = least_standing(chain[i - 1]);
                    let now = least_standing(quest);
                    assert!(now > before || now == Standing::Honoured, "{:?}", quest.id);
                }
            }
        }
    }
}
