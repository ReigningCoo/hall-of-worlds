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

#[test]
fn a_strong_enough_viewer_meets_it() {
    let lang = Lang::new(Language::English);
    let strong = Attributes::all(110);
    let (met, unmet) = need_lines(&steel_plate(), Some(&strong), &lang);
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
    let (met, unmet) = need_lines(&steel_plate(), Some(&weak), &lang);
    assert!(met.is_empty());
    assert_eq!(unmet, vec!["Needs 110 Strength", "Working at 49%"]);
}

#[test]
fn with_no_viewer_everything_is_met_and_other_items_ask_nothing() {
    let lang = Lang::new(Language::English);
    let (met, unmet) = need_lines(&steel_plate(), None, &lang);
    assert_eq!((met.len(), unmet.len()), (1, 0));
    let hat = Item::new(Id::for_test(1), ItemKind::Hat);
    let (met, unmet) = need_lines(&hat, Some(&Attributes::all(1)), &lang);
    assert!(met.is_empty() && unmet.is_empty());
}
