//! Gathering steps' herbs over the test world's towns: named, growing wild
//! where the step sends the player, rarer ones paying more, and no gathering
//! where no herb fits.

use super::herb_ask::{choose_ask, herb_choices};
use super::plan::Plan;
use super::reward::reward;
use super::test_world::{TestWorld, buildings};
use super::tests::every_town;
use super::*;
use crate::herb::{Abundance, Herbs};
use crate::item::Quality;
use crate::lang::{Lang, Language};
use crate::rng::Seed;
use crate::story::{Goal, RewardPart};

/// Every herb-gathering step of the test world's towns' quests and a month
/// of their dailies, with its quest.
fn gathering(world: &TestWorld) -> Vec<(Quest, Step)> {
    let view = world.view();
    let mut found = Vec::new();
    for (town, mut quests) in every_town(world) {
        let buildings = buildings(&town);
        for building in daily_givers(&town, &buildings) {
            quests.extend(
                (0..30).filter_map(|day| daily_quest(&view, &town, &buildings, building, day)),
            );
        }
        for quest in quests {
            for step in &quest.steps {
                let herbs = matches!(
                    step.target,
                    Target::Wilds { .. }
                        | Target::Poi {
                            hook: Hook::Gather,
                            ..
                        }
                );
                if herbs {
                    found.push((quest.clone(), step.clone()));
                }
            }
        }
    }
    found
}

#[test]
fn gathering_names_a_herb_growing_wild_where_it_sends_you() {
    let world = TestWorld::new();
    let view = world.view();
    let lang = Lang::new(Language::English);
    let found = gathering(&world);
    assert!(found.len() > 5, "{}", found.len());
    for (quest, step) in &found {
        assert_eq!(step.goal, Goal::Gather);
        let ask = step.herb.as_ref().expect("a herb named");
        let def = view.herbs.get(&ask.herb).expect("a known herb");
        let poi = view.pois.in_chunk(step.target.chunk()).unwrap();
        assert!(
            def.suits(&(view.habitat_of)(&poi)),
            "{} at {poi:?}",
            def.key
        );
        // Written as an item in the story and the step's words.
        let token = format!("${{kind:herb:{}}}", ask.herb);
        assert_eq!(quest.story.facts.herb.as_deref(), Some(ask.herb.as_str()));
        let index = quest.steps.iter().position(|s| s == step).unwrap();
        let words = quest.story.step_text(&lang, index, 2).unwrap();
        assert!(words.contains(&token), "{words}");
    }
    // Other steps name none; a creature part's "Gather" included.
    for (_, quests) in every_town(&world) {
        for step in quests.iter().flat_map(|quest| &quest.steps) {
            if !matches!(step.target, Target::Wilds { .. }) {
                let gather_hook = matches!(
                    step.target,
                    Target::Poi {
                        hook: Hook::Gather,
                        ..
                    }
                );
                assert_eq!(step.herb.is_some(), gather_hook, "{step:?}");
            }
        }
    }
}

#[test]
fn some_ask_a_rarer_herb_at_its_own_quality_and_pay_more() {
    let world = TestWorld::new();
    let view = world.view();
    let found = gathering(&world);
    let rarer: Vec<&HerbAsk> = (found.iter())
        .filter_map(|(_, step)| step.herb.as_ref())
        .filter(|ask| ask.least != Quality::Poor)
        .collect();
    assert!(!rarer.is_empty());
    for ask in rarer {
        let def = view.herbs.get(&ask.herb).unwrap();
        assert_ne!(def.abundance, Abundance::Common);
        assert_eq!(ask.least, def.abundance.quality());
    }
    // The same step pays half as much again per quality step asked.
    let town = world.towns(24)[0];
    let coins = |least: Quality| {
        let step = Step {
            goal: Goal::Gather,
            target: Target::Wilds { chunk: town.chunk },
            count: Some(8),
            after: Vec::new(),
            alternatives: Vec::new(),
            herb: Some(HerbAsk {
                herb: "yarrow".to_owned(),
                least,
            }),
        };
        let plan = Plan {
            steps: vec![step],
            ..Plan::default()
        };
        let (parts, _) = reward(view.pois, &town, &plan, 0, true, &mut Seed::new(1).rng());
        match parts[0] {
            RewardPart::Coins(copper) => copper,
            _ => panic!("{parts:?}"),
        }
    };
    let any = coins(Quality::Poor);
    assert_eq!(coins(Quality::Common), any);
    assert!(coins(Quality::Uncommon) > any * 14 / 10, "{any}");
    assert!(coins(Quality::Rare) >= any * 2 - 5, "{any}");
}

#[test]
fn choosing_keeps_off_what_shops_could_stock() {
    let world = TestWorld::new();
    let view = world.view();
    let town = world.towns(24)[0];
    let habitat = (view.habitat_of)(&town);
    let wild = herb_choices(view.herbs, &habitat, &|_| false);
    assert!(wild.len() > 1);
    // No shop sells herbs yet, so the town's shops rule nothing out.
    let shops = super::town::shops_of(&buildings(&town));
    let stocked = |herb: &str| super::herb_ask::shops_stock(&shops, herb);
    assert_eq!(
        herb_choices(view.herbs, &habitat, &stocked).len(),
        wild.len()
    );
    // One a shop could stock is never asked for.
    let kept_off = wild[0].key.clone();
    let choices = herb_choices(view.herbs, &habitat, &|herb| herb == kept_off);
    assert_eq!(choices.len(), wild.len() - 1);
    let mut rng = Seed::new(7).rng();
    for _ in 0..200 {
        let ask = choose_ask(&choices, &mut rng).unwrap();
        assert_ne!(ask.herb, kept_off);
    }
    assert!(choose_ask(&[], &mut rng).is_none());
}

#[test]
fn with_no_herb_to_ask_for_other_quests_are_given() {
    let world = TestWorld::new();
    let none = Herbs::default();
    let view = QuestWorld {
        herbs: &none,
        ..world.view()
    };
    let mut quests = 0;
    for town in world.towns(12) {
        for quest in town_quests(&view, &town, &buildings(&town)) {
            quests += 1;
            assert!(
                (quest.steps.iter()).all(|step| step.herb.is_none()
                    && !matches!(step.target, Target::Wilds { .. })
                    && !matches!(
                        step.target,
                        Target::Poi {
                            hook: Hook::Gather,
                            ..
                        }
                    )),
                "{quest:?}"
            );
        }
    }
    assert!(quests > 10, "{quests}");
}
