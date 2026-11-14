use shared::armour::Construction;
use shared::creature::CreatureKind;
use shared::id::Id;
use shared::item::{ItemKind, Quality};
use shared::item_form::{ItemForm, ItemOrder, make_item};
use shared::lang::{Lang, Language};
use shared::material::Material;
use shared::rng::Seed;

use super::*;

fn steel_plate() -> Item {
    let mut item = make_item(
        &ItemOrder::plain(ItemKind::TorsoArmour, Quality::Common),
        CreatureKind::Human,
        Id::for_test(1),
        Seed::new(3),
        None,
        &mut || Id::for_test(2),
    );
    if let Some(ItemForm::Torso(form)) = item.form.as_mut() {
        form.construction = Construction::Plate;
        form.material = Material::Steel;
    }
    item
}

fn human(attributes: &Attributes) -> Option<Viewer<'_>> {
    Some(Viewer {
        attributes,
        kind: CreatureKind::Human,
    })
}

#[test]
fn a_strong_enough_viewer_meets_it() {
    let lang = Lang::new(Language::English);
    let strong = Attributes::all(110);
    let (met, unmet) = need_lines(&steel_plate(), human(&strong), &lang);
    assert_eq!(met, vec!["Needs 110 Strength"]);
    assert!(unmet.is_empty());
}

#[test]
fn a_weaker_one_is_warned_how_well_it_works() {
    let lang = Lang::new(Language::English);
    let weak = Attributes {
        strength: 82,
        ..Attributes::all(100)
    };
    let (met, unmet) = need_lines(&steel_plate(), human(&weak), &lang);
    assert!(met.is_empty());
    assert_eq!(unmet, vec!["Needs 110 Strength", "Working at 49%"]);
}

#[test]
fn with_no_viewer_everything_is_met_and_other_items_ask_nothing() {
    let lang = Lang::new(Language::English);
    let (met, unmet) = need_lines(&steel_plate(), None, &lang);
    assert_eq!((met.len(), unmet.len()), (1, 0));
    let hat = Item::new(Id::for_test(1), ItemKind::Hat);
    let (met, unmet) = need_lines(&hat, human(&Attributes::all(1)), &lang);
    assert!(met.is_empty() && unmet.is_empty());
}

#[test]
fn food_for_some_kinds_says_who_red_for_others() {
    let lang = Lang::new(Language::English);
    let attributes = Attributes::all(100);
    let meat = shared::item::Item::new(Id::for_test(5), ItemKind::RawMeat);
    let (met, unmet) = need_lines(&meat, human(&attributes), &lang);
    assert!(met.is_empty());
    assert_eq!(unmet, vec!["Orcs and goblins only"]);
    let orc = Some(Viewer {
        attributes: &attributes,
        kind: CreatureKind::Orc,
    });
    let bread = shared::item::Item::new(Id::for_test(6), ItemKind::DwarvenBread);
    assert_eq!(
        need_lines(&meat, orc, &lang).0,
        vec!["Orcs and goblins only"]
    );
    assert_eq!(need_lines(&bread, orc, &lang).1, vec!["Dwarves only"]);
}
