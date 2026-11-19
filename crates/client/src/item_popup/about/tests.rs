use shared::id::Id;
use shared::lang::Language;

use super::*;

#[test]
fn ids_are_the_kind_in_kebab_case() {
    assert_eq!(about_id(ItemKind::DwarvenBread), "about-dwarven-bread");
    assert_eq!(about_id(ItemKind::Bread), "about-bread");
}

#[test]
fn amounts_read_as_people_say_them() {
    assert_eq!(amount(250), "250 ml");
    assert_eq!(amount(1000), "1 L");
    assert_eq!(amount(19_500), "19.5 L");
    assert_eq!(amount(960_000), "960 L");
}

#[test]
fn a_cask_says_what_it_holds_and_whether_it_is_sealed() {
    use shared::cask::{CaskForm, CaskSize};
    use shared::item_form::ItemForm;
    use shared::material::Material;
    use shared::potion::{Liquid, Water};
    let lang = Lang::new(Language::English);
    let full = CaskForm::new(CaskSize::Pin, Material::Oak).full_of(Liquid::Water(Water::Plain));
    let cask = Item {
        form: Some(ItemForm::Cask(full)),
        ..Item::new(Id::for_test(1), ItemKind::Cask)
    };
    let words = about(&cask, &lang).unwrap();
    assert!(words.ends_with("Holds 20 L of 20 L, sealed"), "{words}");
}

#[test]
fn dwarven_bread_is_described_and_an_apple_is_not() {
    let lang = Lang::new(Language::English);
    let bread = Item::new(Id::for_test(1), ItemKind::DwarvenBread);
    let about_bread = about(&bread, &lang).expect("described");
    assert!(about_bread.starts_with("Looks and feels like a lump of stone."));
    assert!(about(&Item::new(Id::for_test(2), ItemKind::Bread), &lang).is_none());
}

#[test]
fn a_fermenting_cask_says_what_it_will_be_and_when() {
    use shared::cask::{CaskForm, CaskSize};
    use shared::ferment::plan;
    use shared::item::Quality;
    use shared::item_form::ItemForm;
    use shared::material::Material;
    use shared::potion::Liquid;
    let lang = Lang::new(Language::English);
    let juice = CaskForm::new(CaskSize::Pin, Material::Oak).full_of(Liquid::Juice("apple".into()));
    let mut cask = Item {
        form: Some(ItemForm::Cask(juice.clone())),
        ..Item::new(Id::for_test(1), ItemKind::Cask)
    };
    let started = plan(&cask, None, &[Quality::Common])
        .unwrap()
        .started(&juice);
    cask.form = Some(ItemForm::Cask(started));
    let words = about(&cask, &lang).unwrap();
    assert!(
        words.ends_with("sealed\n5.8%, dry, Common\nFermenting: ready in 36 days"),
        "{words}"
    );
}
