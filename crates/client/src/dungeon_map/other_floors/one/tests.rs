use super::*;

#[test]
fn floors_below_stand_a_storey_apart_and_stretch_to_meet_the_one_above() {
    let (height, stretch) = Placed::Below(2).height_and_stretch(1.5);
    assert_eq!(height, -3.0 * WALL_HEIGHT);
    // Its walls' tops stop just under the floor a storey up.
    let top = height + WALL_HEIGHT * stretch;
    assert!((top - (-1.5 * WALL_HEIGHT - TOP_GAP)).abs() < 1e-4);
}

#[test]
fn the_floor_above_stands_on_the_players_walls() {
    let (first, stretch) = Placed::Above(1).height_and_stretch(1.5);
    assert!((first - WALL_HEIGHT - TOP_GAP).abs() < 1e-4);
    // The next stands on its walls' tops, just over them.
    let (second, _) = Placed::Above(2).height_and_stretch(1.5);
    let top = first + WALL_HEIGHT * stretch;
    assert!((second - top - TOP_GAP).abs() < 1e-4);
}
