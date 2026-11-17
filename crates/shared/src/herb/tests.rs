use std::collections::BTreeSet;

use super::*;
use crate::climate::Biome;
use crate::id::Id;
use crate::item::{Quality, StackLimits};

fn herbs() -> Herbs {
    Herbs::built_in()
}

fn first() -> HerbDef {
    herbs().iter().next().unwrap().clone()
}

#[test]
fn the_built_in_herbs_are_all_there_and_sound() {
    let herbs = herbs();
    assert_eq!(herbs.len(), 82);
    let keys: BTreeSet<&str> = herbs.iter().map(|herb| herb.key.as_str()).collect();
    assert_eq!(keys.len(), 82, "keys are unique");
    for herb in herbs.iter() {
        herb.check().unwrap();
        assert_eq!(herbs.get(&herb.key), Some(herb));
    }
    assert_eq!(herbs.get("roseroot").unwrap().name, "Roseroot");
    // Every biome has something growing in it.
    for biome in Biome::ALL {
        assert!(
            herbs.iter().any(|herb| herb.biomes.contains(&biome)),
            "{biome:?}"
        );
    }
}

#[test]
fn mods_add_herbs_and_replace_them_by_key_in_place() {
    let mut herbs = herbs();
    let mut changed = first();
    changed.name = "Rose Root".into();
    let mut new = first();
    new.key = "not_a_real_herb".into();
    assert_eq!(herbs.merge(vec![changed, new]), 1);
    assert_eq!(herbs.len(), 83);
    assert_eq!(herbs.iter().next().unwrap().name, "Rose Root");
    assert_eq!(herbs.iter().last().unwrap().key, "not_a_real_herb");
    assert!(herbs.get("not_a_real_herb").is_some());
}

#[test]
fn a_mod_with_a_wrong_herb_is_refused() {
    let mut bad = first();
    bad.effects[2].change = 4;
    let text = ron::to_string(&vec![bad]).unwrap();
    let error = Herbs::read(&text).unwrap_err();
    assert!(error.contains("roseroot"), "{error}");
    let mut bad = first();
    bad.key = "Rose Root".into();
    assert!(bad.check().is_err());
    assert!(Herbs::read("[(key: \"x\")]").is_err());
    // Written by hand, as a mod would be.
    let good = ron::ser::to_string_pretty(&vec![first()], Default::default()).unwrap();
    assert_eq!(Herbs::read(&good).unwrap(), vec![first()]);
}

#[test]
fn the_table_goes_through_messages_whole() {
    let herbs = herbs();
    let bytes = postcard::to_allocvec(&herbs).unwrap();
    let back: Herbs = postcard::from_bytes(&bytes).unwrap();
    assert_eq!(back, herbs);
    assert!(back.get("roseroot").is_some());
}

/// `count` of the first herb at `quality` (as an old save might have it).
fn item(n: u128, quality: Quality, count: u32) -> crate::item::Item {
    let mut item = new_item(&first(), count, Id::for_test(n), None);
    item.quality = Some(quality);
    item
}

#[test]
fn herbs_stack_by_herb_and_quality_whatever_they_look_like() {
    let limits = StackLimits::default();
    let stack = item(1, Quality::Common, 3);
    let more = item(2, Quality::Common, 4);
    assert_ne!(stack.herb().unwrap().look, more.herb().unwrap().look);
    assert_eq!(limits.room(&stack, &more), 4);
    let mut joined = stack.clone();
    joined.put_on(more);
    assert_eq!((joined.count(), joined.id), (7, Id::for_test(1)));
    assert_eq!(joined.herb(), stack.herb(), "the bottom's look stays");
    // Better ones, or another herb, don't join.
    assert_eq!(limits.room(&stack, &item(3, Quality::Uncommon, 1)), 0);
    let other = herbs().iter().nth(1).unwrap().clone();
    let other = new_item(&other, 1, Id::for_test(4), None);
    assert_eq!(limits.room(&stack, &other), 0);
}

#[test]
fn a_herb_weighs_what_its_herb_says() {
    let herb = item(1, Quality::Poor, 5);
    assert!(herb.is_counted());
    assert_eq!(herb.own_weight(), first().grams);
    assert_eq!(herb.total_weight(), first().grams * 5);
    assert_eq!(herb.shape().cells().count(), 1);
    assert_eq!(herb.name(), "Herb");
}

#[test]
fn herbs_are_never_magical_blessed_or_cursed() {
    for n in 0..200 {
        let mut herb = item(n, Quality::Artifact, 1);
        herb.made.magical = true;
        herb.roll_secrets();
        assert!(!herb.can_be_magical() && !herb.is_magical(), "{n}");
        assert!(herb.favour.is_none(), "{n}");
    }
}

#[test]
fn a_herb_is_as_good_as_its_plant_is_rare() {
    for def in herbs().iter() {
        let item = new_item(def, 1, Id::for_test(1), None);
        assert_eq!(item.quality, Some(def.abundance.quality()), "{}", def.key);
    }
    assert_eq!(Abundance::Common.quality(), Quality::Common);
    assert_eq!(Abundance::Uncommon.quality(), Quality::Uncommon);
    assert_eq!(Abundance::Rare.quality(), Quality::Rare);
}

#[test]
fn foraging_is_read_from_the_skill_tree() {
    let mut skills = crate::skill::Skills::default();
    assert_eq!(skill_value(&skills, FORAGING), 0);
    let id = crate::skill::SkillId::find(FORAGING).unwrap();
    skills.practise(id, 1_000_000);
    assert!(skill_value(&skills, FORAGING) > 0);
    assert!(skill_value(&skills, FORAGING) <= 10_000);
    assert!(crate::skill::SkillId::find(HERBS).is_some());
    assert_eq!(skill_percent(10_000), 100);
    assert_eq!(skill_percent(199), 1);
    assert!(crate::skill::SkillId::find(HERBALISM).is_some());
    assert_eq!(skill_value(&skills, "no/such/skill"), 0);
}

#[test]
fn quality_counts_only_as_far_as_herbalism_reaches() {
    // With none, every herb is as weak as a poor one.
    for quality in Quality::ALL {
        assert_eq!(potency(quality, 0), quality_percent(Quality::Poor));
    }
    // With all of it, each is as strong as it can be.
    for quality in Quality::ALL {
        assert_eq!(potency(quality, 10_000), quality_percent(quality));
    }
    assert_eq!(skill_cap(5_000), 140);
    assert_eq!(potency(Quality::Rare, 5_000), 130);
    assert_eq!(potency(Quality::Artifact, 5_000), 140);
    // More skill never makes a herb weaker.
    for quality in Quality::ALL {
        let steps: Vec<u32> = (0..=10).map(|n| potency(quality, n * 1_000)).collect();
        assert!(steps.windows(2).all(|w| w[0] <= w[1]), "{quality:?}");
    }
}

#[test]
fn knowledge_is_learned_a_name_and_an_effect_at_a_time() {
    let def = first();
    let mut known = HerbKnowledge::default();
    assert!(!known.knows_name("roseroot") && known.is_empty());
    assert!(known.learn_name("roseroot"));
    assert!(!known.learn_name("roseroot"), "only once new");
    assert_eq!(known.known_effects(&def).count(), 0);
    assert!(known.learn_effect("roseroot", 2));
    assert!(!known.learn_effect("roseroot", 2));
    assert!(!known.learn_effect("roseroot", 4), "there are four");
    assert!(known.knows_effect("roseroot", 2) && !known.knows_effect("roseroot", 0));
    let effects: Vec<&Effect> = known.known_effects(&def).collect();
    assert_eq!(effects, vec![&def.effects[2]]);
    let text = ron::to_string(&known).unwrap();
    assert_eq!(ron::from_str::<HerbKnowledge>(&text).unwrap(), known);
}

#[test]
fn a_sheet_saved_before_herbs_knows_none() {
    let mut sheet = crate::sheet::Sheet::default();
    let text = ron::to_string(&sheet).unwrap();
    assert!(text.contains(",herbs:{}"), "{text}");
    let old = text.replace(",herbs:{}", "");
    assert_eq!(ron::from_str::<crate::sheet::Sheet>(&old).unwrap(), sheet);
    sheet.herbs.learn_name("roseroot");
    let text = ron::to_string(&sheet).unwrap();
    assert_eq!(ron::from_str::<crate::sheet::Sheet>(&text).unwrap(), sheet);
}

#[test]
fn growth_and_regrowing_follow_the_research() {
    let bands = [Growth::Fast, Growth::Medium, Growth::Slow, Growth::VerySlow];
    assert_eq!(bands.map(Growth::days), [2, 4, 8, 16]);
    assert_eq!(Regrow::Cut.back_to(), Some(2));
    assert_eq!(Regrow::Pick.back_to(), Some(3));
    assert_eq!(Regrow::Dig.back_to(), None);
    assert_eq!(Regrow::Coppice.back_to(), Some(1));
    assert_eq!(
        Stat::ALL.iter().filter(|s| s.attribute().is_some()).count(),
        9
    );
}
