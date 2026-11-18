use shared::ability::Ability;

use super::icons::icon_path;
use super::page::ability_id;

/// Every ability's icon has been drawn into the assets (`cargo xtask icons`).
#[test]
fn every_ability_has_an_icon() {
    for ability in Ability::ALL {
        let file = format!(
            "{}/assets/{}",
            env!("CARGO_MANIFEST_DIR"),
            icon_path(ability)
        );
        assert!(std::path::Path::new(&file).exists(), "{file}");
    }
}

#[test]
fn ids_are_plain_names() {
    assert_eq!(ability_id(Ability::NightVision), "night_vision");
    assert_eq!(ability_id(Ability::Attack), "attack");
}

mod dropping {
    use shared::ability::{Ability, BarChange, BarThing};
    use shared::item::ItemKind;

    use super::super::drag::{From, drop_change};
    use super::super::spots::Spot;

    const ATTACK: BarThing = BarThing::Ability(Ability::Attack);
    const BREAD: BarThing = BarThing::Item(ItemKind::Bread);

    #[test]
    fn from_the_page_onto_a_slot_puts_it_there() {
        assert_eq!(
            drop_change(ATTACK, From::Page, Some(Spot::Slot(3))),
            Some(BarChange::Place {
                slot: 3,
                ability: Ability::Attack
            })
        );
    }

    #[test]
    fn from_slot_to_slot_swaps_and_onto_itself_does_nothing() {
        assert_eq!(
            drop_change(ATTACK, From::Slot(1), Some(Spot::Slot(4))),
            Some(BarChange::Swap { a: 1, b: 4 })
        );
        assert_eq!(
            drop_change(ATTACK, From::Slot(4), Some(Spot::Slot(4))),
            None
        );
    }

    #[test]
    fn from_a_slot_onto_the_page_takes_it_off_and_elsewhere_nothing() {
        for spot in [Spot::Page, Spot::Listed(Ability::Kick)] {
            assert_eq!(
                drop_change(ATTACK, From::Slot(2), Some(spot)),
                Some(BarChange::Clear { slot: 2 })
            );
        }
        assert_eq!(drop_change(ATTACK, From::Slot(2), None), None);
        assert_eq!(drop_change(ATTACK, From::Page, Some(Spot::Page)), None);
        assert_eq!(drop_change(ATTACK, From::Page, None), None);
    }

    #[test]
    fn a_kind_of_item_comes_off_anywhere_on_the_character_panel() {
        for spot in [Spot::Panel, Spot::Page] {
            assert_eq!(
                drop_change(BREAD, From::Slot(5), Some(spot)),
                Some(BarChange::Clear { slot: 5 })
            );
        }
        // An ability only on the Abilities page, as before.
        assert_eq!(drop_change(ATTACK, From::Slot(5), Some(Spot::Panel)), None);
        assert_eq!(
            drop_change(BREAD, From::Slot(5), Some(Spot::Slot(1))),
            Some(BarChange::Swap { a: 5, b: 1 })
        );
    }
}

#[test]
fn press_oils_requester_is_good_markup() {
    super::press_oil::PRESS_OIL.assert_good();
}

#[test]
fn grinds_mixing_panel_is_good_markup() {
    super::grind::GRIND.assert_good();
}

#[test]
fn brews_panel_is_good_markup() {
    super::brew::BREW.assert_good();
}
