use super::button::BUTTON;
use super::markup::TRADE;

#[test]
fn the_markup_is_good() {
    BUTTON.assert_good();
    TRADE.assert_good();
}

#[test]
fn the_icon_has_ink_on_both_arrows() {
    let icon = super::icon::trade_icon();
    let data = icon.data.expect("drawn");
    let alpha = |x: usize, y: usize| data[(y * 32 + x) * 4 + 3];
    // Each arrow's shaft and head.
    assert_eq!(alpha(10, 10), 255);
    assert_eq!(alpha(22, 10), 255);
    assert_eq!(alpha(21, 22), 255);
    assert_eq!(alpha(9, 22), 255);
    assert_eq!(alpha(16, 16), 0, "a gap between them");
}
