use shared::creature::CreatureKind;
use shared::id::Id;
use shared::item::{Item, ItemKind, Quality};
use shared::item_form::{ItemOrder, make_item};
use shared::lang::{Lang, Language};
use shared::rng::Seed;

use super::*;

const MINUS: char = '\u{2212}';

#[test]
fn bread_restores_health_until_moving_or_hurt() {
    let lang = Lang::new(Language::English);
    let bread = Item::new(Id::for_test(1), ItemKind::Bread);
    assert_eq!(
        bonus_lines(&bread, &lang),
        vec!["Restores 1 Health a second for 45 s; stops on moving or being hurt"]
    );
}

#[test]
fn fruit_says_its_healing_its_effects_and_its_servings() {
    let lang = Lang::new(Language::English);
    let fruits = shared::fruit::Fruits::built_in();
    let apple = shared::fruit::new_item(fruits.get("apple").unwrap(), 1, Id::for_test(1), None);
    assert_eq!(
        bonus_lines(&apple, &lang),
        vec!["Restores 1 Health a second for 23 s; stops on moving or being hurt"]
    );
    let kola = shared::fruit::new_item(fruits.get("kola_nut").unwrap(), 1, Id::for_test(2), None);
    assert_eq!(
        bonus_lines(&kola, &lang).len(),
        3,
        "healing and two effects"
    );
    let melon = fruits.get("watermelon").unwrap();
    let melon = shared::fruit::new_item(melon, 1, Id::for_test(3), None);
    let lines = bonus_lines(&melon, &lang);
    assert_eq!(lines.last().unwrap(), "Cut into 11 servings when eaten");
}

#[test]
fn armour_gives_toughness_and_costs_agility_and_mana() {
    let lang = Lang::new(Language::English);
    let order = ItemOrder::plain(ItemKind::TorsoArmour, Quality::Common);
    let human = CreatureKind::Human;
    let next = &mut || Id::for_test(2);
    let piece = make_item(&order, human, Id::for_test(1), Seed::new(3), None, next);
    let lines = bonus_lines(&piece, &lang);
    assert_eq!(lines.len(), 3, "{lines:?}");
    assert!(lines[0].starts_with('+') && lines[0].ends_with(" Toughness"));
    assert!(lines[1].starts_with(MINUS) && lines[1].ends_with(" Agility"));
    assert!(lines[2].starts_with(MINUS) && lines[2].ends_with(" most Mana"));
}

#[test]
fn a_ring_gives_nothing() {
    let lang = Lang::new(Language::English);
    let ring = Item::new(Id::for_test(1), ItemKind::Ring);
    assert!(bonus_lines(&ring, &lang).is_empty());
}

#[test]
fn a_hair_shirt_costs_five_percent_will() {
    use shared::item_form::ItemForm;
    use shared::material::Material;
    use shared::underclothes::{UnderPiece, roll};
    let lang = Lang::new(Language::English);
    let form = (0..5000u64)
        .map(|seed| {
            roll(
                &mut Seed::new(seed).rng(),
                UnderPiece::Shirt,
                Quality::Common,
            )
        })
        .find(|form| form.material == Material::Haircloth)
        .expect("a hair shirt");
    let mut shirt = Item::new(Id::for_test(1), UnderPiece::Shirt.kind());
    shirt.form = Some(ItemForm::Under(form));
    assert_eq!(bonus_lines(&shirt, &lang), vec![format!("{MINUS}5% Will")]);
}

#[test]
fn raw_meat_says_it_poisons_other_kinds() {
    let lang = Lang::new(Language::English);
    let meat = Item::new(Id::for_test(1), ItemKind::RawMeat);
    assert_eq!(
        bonus_lines(&meat, &lang),
        vec![
            "Restores 2 Health a second for 60 s; stops on moving or being hurt",
            "Poisons other kinds: \u{2212}1 Health a second for 60 s",
        ]
    );
}
