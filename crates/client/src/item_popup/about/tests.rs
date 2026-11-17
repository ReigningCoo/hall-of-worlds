use shared::id::Id;
use shared::lang::Language;

use super::*;

#[test]
fn ids_are_the_kind_in_kebab_case() {
    assert_eq!(about_id(ItemKind::DwarvenBread), "about-dwarven-bread");
    assert_eq!(about_id(ItemKind::Bread), "about-bread");
}

#[test]
fn dwarven_bread_is_described_and_an_apple_is_not() {
    let lang = Lang::new(Language::English);
    let bread = Item::new(Id::for_test(1), ItemKind::DwarvenBread);
    let about_bread = about(&bread, &lang).expect("described");
    assert!(about_bread.starts_with("Looks and feels like a lump of stone."));
    assert!(about(&Item::new(Id::for_test(2), ItemKind::Bread), &lang).is_none());
}
