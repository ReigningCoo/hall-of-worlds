use shared::creature::CreatureKind;
use shared::id::Id;
use shared::item::{ItemKind, Made, Quality};
use shared::item_form::{ItemOrder, make_item};
use shared::lang::{Lang, Language};
use shared::rng::Seed;

use super::*;

fn made(kind: ItemKind, size: Size) -> Item {
    let mut item = make_item(
        &ItemOrder::plain(kind, Quality::Common),
        CreatureKind::Human,
        Id::for_test(1),
        Seed::new(3),
        None,
        &mut || Id::for_test(2),
    );
    item.made = Made::for_body(kind, size, item.made.fingers, false);
    item
}

#[test]
fn armour_of_another_size_says_both() {
    let lang = Lang::new(Language::English);
    let plate = made(ItemKind::TorsoArmour, Size::Large);
    assert_eq!(
        size_line(&plate, Size::Medium, &lang).as_deref(),
        Some("Size: Large (you: Medium)")
    );
    let line = size_line(&plate, Size::Tiny, &lang);
    assert_eq!(line.as_deref(), Some("Size: Large (you: Tiny)"));
}

#[test]
fn nothing_when_it_is_yours() {
    let lang = Lang::new(Language::English);
    let plate = made(ItemKind::TorsoArmour, Size::Small);
    assert_eq!(size_line(&plate, Size::Small, &lang), None);
}

#[test]
fn every_size_has_a_name() {
    let lang = Lang::new(Language::English);
    for size in Size::ALL {
        assert_eq!(size_words(size, &lang), size.name());
    }
}

#[test]
fn gems_say_nothing() {
    let lang = Lang::new(Language::English);
    let gem = made(ItemKind::Gem, Size::Large);
    assert_eq!(size_line(&gem, Size::Medium, &lang), None);
}
