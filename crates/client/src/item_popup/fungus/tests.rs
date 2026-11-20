//! A fungus's popup tells only what the character knows (never wrong
//! information).

use shared::fungus::{Fungi, FungusKnowledge, new_item};
use shared::id::Id;
use shared::lang::{Lang, Language};

use super::*;

fn words(key: &str, known: &FungusKnowledge) -> HerbWords {
    let lang = Lang::new(Language::English);
    let def = Fungi::built_in().get(key).unwrap();
    let item = new_item(def, 1, Id::for_test(1), None);
    fungus_words(&item, known, &lang).expect("a fungus")
}

#[test]
fn an_unknown_fungus_says_nothing_of_what_it_does() {
    let mut known = FungusKnowledge::default();
    // Even having suffered its poison, it can't be told apart unnamed.
    known.learn_poison("death_cap");
    let death_cap = words("death_cap", &known);
    assert_eq!(death_cap.name, "Unknown mushroom");
    assert!(death_cap.effects.is_empty(), "{:?}", death_cap.effects);
}

#[test]
fn a_known_fungus_tells_its_poison() {
    let mut known = FungusKnowledge::default();
    known.learn_name("death_cap");
    let death_cap = words("death_cap", &known);
    assert_eq!(death_cap.name, "Death cap");
    let lines = death_cap.effects;
    assert!(
        lines.iter().any(|l| l.starts_with("Poisonous:")),
        "{lines:?}"
    );
    assert!(
        lines.iter().any(|l| l == "Deadly unless healed"),
        "{lines:?}"
    );
    assert!(lines.iter().any(|l| l.starts_with("Poison: ")), "{lines:?}");
}

#[test]
fn the_ink_caps_window_shows_once_known() {
    let mut known = FungusKnowledge::default();
    assert!(words("common_ink_cap", &known).effects.is_empty());
    known.learn_name("common_ink_cap");
    let lines = words("common_ink_cap", &known).effects;
    assert!(
        lines
            .iter()
            .any(|l| l.starts_with("For 3 days after eating: ")),
        "{lines:?}"
    );
    let alcohol = "Alcohol within 3 days brings: ";
    assert!(lines.iter().any(|l| l.starts_with(alcohol)), "{lines:?}");
    // Its herb effects stay unknown until learned.
    assert_eq!(lines.iter().filter(|l| *l == "?").count(), 4, "{lines:?}");
    known.learn_effect("common_ink_cap", 0);
    let lines = words("common_ink_cap", &known).effects;
    assert_eq!(lines.iter().filter(|l| *l == "?").count(), 3, "{lines:?}");
}

#[test]
fn known_fungi_say_their_raw_healing_and_servings() {
    let mut known = FungusKnowledge::default();
    for key in ["button_mushroom", "giant_puffball"] {
        known.learn_name(key);
    }
    assert_eq!(
        words("button_mushroom", &known).effects,
        vec!["Restores 1 Health a second for 1 s; stops on moving or being hurt"]
    );
    let puffball = words("giant_puffball", &known).effects;
    assert_eq!(
        puffball[..2],
        ["Heals nothing raw", "Cut into 10 servings when eaten"]
    );
    // Unknown, a piece still says how many servings it is.
    let none = FungusKnowledge::default();
    assert_eq!(
        words("giant_puffball", &none).effects,
        vec!["Cut into 10 servings when eaten"]
    );
}

#[test]
fn handled_poisons_say_what_protects_once_known() {
    let mut known = FungusKnowledge::default();
    assert!(words("oakmoss", &known).effects.is_empty());
    known.learn_name("oakmoss");
    known.learn_name("sleepdust_puffball");
    let oakmoss = words("oakmoss", &known).effects;
    let touch = "Harms bare hands that touch it, about 10 minutes later";
    assert!(oakmoss.iter().any(|l| l == touch), "{oakmoss:?}");
    assert!(oakmoss.iter().any(|l| l == "Gloves protect"), "{oakmoss:?}");
    assert!(
        oakmoss.iter().any(|l| l.starts_with("Poison: ")),
        "{oakmoss:?}"
    );
    let sleepdust = words("sleepdust_puffball", &known).effects;
    let breath = "Its spores harm when breathed";
    assert!(sleepdust.iter().any(|l| l == breath), "{sleepdust:?}");
    assert!(
        sleepdust.iter().any(|l| l == "Cover the mouth"),
        "{sleepdust:?}"
    );
    assert!(!sleepdust.iter().any(|l| l.starts_with("Poisonous:")));
}
