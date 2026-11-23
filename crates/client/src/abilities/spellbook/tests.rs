use super::markup::SPELLBOOK;

#[test]
fn the_spells_tab_is_good_markup() {
    SPELLBOOK.assert_good();
}
