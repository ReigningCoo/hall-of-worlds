use super::*;

#[test]
fn digit_boxes_only_keep_digits() {
    assert_eq!(allowed("a1 b2", true, false), "12");
    assert_eq!(allowed("a1 b2", false, false), "a1 b2");
    assert_eq!(allowed("a\nb", false, false), "ab");
}

#[test]
fn boxes_of_several_lines_keep_line_breaks() {
    assert_eq!(allowed("a\r\nb\rc\nd\t", false, true), "a\nb\nc\nd");
    assert_eq!(allowed("1\n2", true, true), "12");
}

#[test]
fn a_box_keeps_no_more_than_its_most() {
    let text_box = TextBox {
        max_length: Some(3),
        ..default()
    };
    assert_eq!(text_box.keeps("héllo"), "hél");
    assert_eq!(text_box.keeps("hi"), "hi");
}

#[test]
fn an_empty_box_keeps_a_lines_height() {
    // Bevy's own spacing, 1.2 of the font size, so a box stands as tall empty
    // as with writing in it.
    assert!((line_height(TextSize::Medium) - TextSize::Medium.pixels() * 1.2).abs() < 1e-4);
    assert!(line_height(TextSize::Small) < line_height(TextSize::Title));
}
