use super::*;
use crate::creature::CreatureKind;
use crate::id::Id;
use crate::item::{Item, ItemKind, Quality};
use crate::item_form::{ItemOrder, make_item};
use crate::rng::Seed;
use crate::skill::SkillId;

fn made(kind: ItemKind, quality: Quality, n: u128) -> Item {
    make_item(
        &ItemOrder::plain(kind, quality),
        CreatureKind::Human,
        Id::for_test(n),
        Seed::new(n as u64),
        None,
        &mut || Id::for_test(n + 1_000_000),
    )
}

fn ring(quality: Quality, n: u128) -> Item {
    let mut ring = Item::new(Id::for_test(n), ItemKind::Ring);
    ring.quality = Some(quality);
    ring
}

/// The usual amount of a bonus of this stat on this item, from the list.
fn usual(item: &Item, stat: Stat) -> Amount {
    match stat {
        Stat::Skill(_) => Amount::Flat(SKILL_LEVEL),
        _ => {
            let places = places_of(&item.worn_locations());
            let gear = GEAR_STATS.iter().find(|gear| gear.stat == stat);
            let gear = gear.expect("a gear stat");
            assert!(gear.places & places != 0, "{stat:?} suits {:?}", item.kind);
            gear.amount
        }
    }
}

fn milli(amount: Amount) -> i32 {
    match amount {
        Amount::Flat(milli) | Amount::Percent(milli) => milli,
    }
}

#[test]
fn one_bonus_a_step_above_common() {
    for (quality, count) in [
        (Quality::Poor, 0),
        (Quality::Common, 0),
        (Quality::Uncommon, 1),
        (Quality::Rare, 2),
        (Quality::Epic, 3),
        (Quality::Legendary, 4),
        (Quality::Artifact, 5),
    ] {
        assert_eq!(
            quality_bonuses(&ring(quality, 7)).len(),
            count,
            "{quality:?}"
        );
    }
    assert!(quality_bonuses(&Item::new(Id::for_test(1), ItemKind::Ring)).is_empty());
}

#[test]
fn the_same_item_always_rolls_the_same_and_others_differ() {
    let rolled = |n| quality_bonuses(&ring(Quality::Legendary, n));
    assert_eq!(rolled(3), rolled(3));
    assert!((4..20).any(|n| rolled(n) != rolled(3)));
}

#[test]
fn each_bonus_a_different_stat_suiting_the_item() {
    for n in 1..200 {
        for item in [
            ring(Quality::Legendary, n),
            made(ItemKind::Sword, Quality::Legendary, n),
            made(ItemKind::Helmet, Quality::Legendary, n),
            made(ItemKind::Boots, Quality::Legendary, n),
            made(ItemKind::Tunic, Quality::Legendary, n),
        ] {
            let bonuses = quality_bonuses(&item);
            assert_eq!(bonuses.len(), 4, "{:?}", item.kind);
            for (at, bonus) in bonuses.iter().enumerate() {
                assert!(bonuses[..at].iter().all(|other| other.stat != bonus.stat));
                let usual = milli(usual(&item, bonus.stat));
                let got = milli(bonus.amount);
                assert!([usual, 2 * usual, -usual].contains(&got), "{bonus:?}");
            }
        }
    }
}

#[test]
fn a_sword_raises_only_sword_skills_and_melee() {
    let axes = SkillId::find("combat/melee/axe").unwrap();
    for n in 1..300 {
        let sword = made(ItemKind::Sword, Quality::Artifact, n);
        for bonus in quality_bonuses(&sword) {
            if let Stat::Skill(skill) = bonus.stat {
                let path = skill.path();
                assert!(
                    path == "combat/melee"
                        || path.starts_with("combat/melee/sword")
                        || path == "combat/melee/defence/sword",
                    "{path}"
                );
                assert!(!skill.upwards().any(|node| node == axes));
            }
        }
    }
}

#[test]
fn bags_and_food_have_none() {
    for kind in [ItemKind::Backpack, ItemKind::Bread] {
        let mut item = Item::new(Id::for_test(1), kind);
        item.quality = Some(Quality::Legendary);
        assert!(quality_bonuses(&item).is_empty(), "{kind:?}");
    }
}

#[test]
fn uniques_give_twice_as_much_and_never_a_negative() {
    for n in 1..300 {
        let artifact = ring(Quality::Artifact, n);
        let mut set = ring(Quality::Rare, n);
        set.set = true;
        for item in [artifact, set] {
            assert!(is_unique(&item));
            for bonus in quality_bonuses(&item) {
                let usual = milli(usual(&item, bonus.stat));
                assert_eq!(milli(bonus.amount), 2 * usual, "{bonus:?}");
            }
        }
    }
}

#[test]
fn a_negative_is_rare_and_doubles_another() {
    let (mut bonuses, mut negatives) = (0, 0);
    for n in 1..2000 {
        let item = ring(Quality::Legendary, n);
        let rolled = quality_bonuses(&item);
        let sign = |bonus: &Bonus| milli(bonus.amount) * milli(usual(&item, bonus.stat)).signum();
        let negative = rolled.iter().filter(|bonus| sign(bonus) < 0).count();
        let usual_of = |bonus: &Bonus| milli(usual(&item, bonus.stat)).abs();
        let doubled = rolled
            .iter()
            .filter(|bonus| sign(bonus) == 2 * usual_of(bonus))
            .count();
        assert_eq!(negative, doubled, "{rolled:?}");
        assert!(negative < rolled.len());
        bonuses += rolled.len();
        negatives += negative;
    }
    // About one in twenty.
    assert!(
        negatives * 30 > bonuses && negatives * 12 < bonuses,
        "{negatives} of {bonuses}"
    );
}

#[test]
fn one_bonus_is_never_negative() {
    for n in 1..2000 {
        let item = ring(Quality::Uncommon, n);
        let rolled = quality_bonuses(&item);
        let usual = milli(usual(&item, rolled[0].stat));
        assert_eq!(milli(rolled[0].amount), usual);
    }
}

#[test]
fn raised_skills_add_their_levels() {
    let melee = SkillId::find("combat/melee").unwrap();
    let bonuses = [
        Bonus::flat(Stat::Skill(melee), SKILL_LEVEL),
        Bonus::flat(Stat::Skill(melee), SKILL_LEVEL),
    ];
    let raised = Raised::of(&Totals::of(&bonuses));
    // Two levels: two percentage points, 200 hundredths.
    assert_eq!(raised.value(melee, 1000), 1200);
    let axes = SkillId::find("combat/melee/axe").unwrap();
    assert_eq!(raised.value(axes, 1000), 1000);
}

#[test]
fn quality_bonuses_are_among_the_items_bonuses() {
    let item = ring(Quality::Epic, 5);
    assert_eq!(item_bonuses(&item), quality_bonuses(&item));
}

#[test]
fn every_kind_of_gear_has_room_for_an_artifacts_five() {
    for kind in ItemKind::ALL {
        let mut item = Item::new(Id::for_test(1), kind);
        item.quality = Some(Quality::Artifact);
        if takes_quality_bonuses(&item) {
            assert_eq!(quality_bonuses(&item).len(), 5, "{kind:?}");
        }
    }
}
