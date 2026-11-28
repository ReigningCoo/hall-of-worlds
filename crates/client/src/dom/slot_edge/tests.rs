use super::*;

#[test]
fn a_slot_is_edged_one_screen_pixel_wide() {
    let mut app = App::new();
    app.insert_resource(UiScale(1.25));
    app.add_systems(Update, fit_slot_edges);
    let slot = app.world_mut().spawn(SlotShown::default()).id();
    app.update();
    let edge = *app.world().get::<Outline>(slot).unwrap();
    assert_eq!(edge.width, px(0.8));
    assert_eq!(edge.color, GRID_LINE);

    app.insert_resource(UiScale(2.0));
    app.update();
    assert_eq!(app.world().get::<Outline>(slot).unwrap().width, px(0.5));
}

#[test]
fn one_pixel_follows_both_scales() {
    assert_eq!(one_pixel(1.0, 1.0), 1.0);
    assert_eq!(one_pixel(2.0, 1.5), 1.0 / 3.0);
}
