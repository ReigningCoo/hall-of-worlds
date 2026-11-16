use super::button::BUTTON;
use super::markup::{MOST_PROBLEM, PANEL};

#[test]
fn the_markup_is_good() {
    BUTTON.assert_good();
    PANEL.assert_good();
}

#[test]
fn the_box_takes_the_most_a_description_may_have() {
    let wanted = format!("maxlength=\"{MOST_PROBLEM}\"");
    assert!(PANEL.source.contains(&wanted));
}

#[test]
fn the_button_sits_left_of_the_gear() {
    // The gear is 8 from the right and 34 across; 6 between them.
    assert!(BUTTON.source.contains(r#"right="48""#));
    assert!(BUTTON.source.contains(r#"top="8""#));
}
