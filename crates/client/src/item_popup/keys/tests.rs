use shared::id::Id;
use shared::item::ItemKind;
use shared::item_form::{KeyForm, KeyUse};
use shared::lang::Language;
use uuid::Uuid;

use super::*;

const HERE: WorldId = WorldId(Uuid::from_u128(1));
const THERE: WorldId = WorldId(Uuid::from_u128(2));

fn ashford(world: WorldId) -> KeyPlace {
    KeyPlace::Planet {
        world,
        runes: Some(GateCode([1, 2, 3, 4, 5, 6, 7])),
        name: Some("Ashford".into()),
        x: 1204,
        y: -388,
    }
}

fn key(found: KeyFound, used: Option<KeyUse>) -> Item {
    let mut item = Item::new(Id::for_test(1), ItemKind::Key);
    item.form = Some(ItemForm::Key(Box::new(KeyForm {
        lock: Id::for_test(2),
        found,
        used,
    })));
    item
}

#[test]
fn a_key_says_where_it_was_found_and_what_it_did() {
    let lang = Lang::new(Language::English);
    let used = KeyUse {
        at: ashford(HERE),
        locked: false,
        unlocked: true,
    };
    let item = key(KeyFound::At(ashford(HERE)), Some(used));
    let (lines, runes) = key_lines(&item, &lang, Some(HERE));
    assert_eq!(
        lines,
        [
            "Found in Ashford (1204, −388)",
            "Has unlocked a door in Ashford (1204, −388)"
        ]
    );
    assert_eq!(runes, None);
}

#[test]
fn another_planet_is_said_and_its_runes_shown() {
    let lang = Lang::new(Language::English);
    let used = KeyUse {
        at: KeyPlace::Hall,
        locked: true,
        unlocked: true,
    };
    let item = key(KeyFound::At(ashford(THERE)), Some(used));
    let (lines, runes) = key_lines(&item, &lang, Some(HERE));
    assert_eq!(
        lines,
        [
            "Found in Ashford (1204, −388), on another planet",
            "Has locked and unlocked a door in the Hall of Worlds"
        ]
    );
    assert_eq!(runes, Some(GateCode([1, 2, 3, 4, 5, 6, 7])));
}

#[test]
fn a_key_that_remembers_nothing_says_nothing() {
    let lang = Lang::new(Language::English);
    for found in [KeyFound::NotYet, KeyFound::Unknown] {
        let (lines, runes) = key_lines(&key(found, None), &lang, Some(HERE));
        assert!(lines.is_empty() && runes.is_none());
    }
    let place = KeyPlace::Planet {
        world: HERE,
        runes: None,
        name: None,
        x: -3,
        y: 4,
    };
    let (lines, _) = key_lines(&key(KeyFound::At(place), None), &lang, Some(HERE));
    assert_eq!(lines, ["Found at (−3, 4)"]);
}
