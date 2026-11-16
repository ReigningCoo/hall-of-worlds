use super::*;

#[test]
fn the_wedge_opens_towards_a_tilted_camera() {
    // South of what it looks at, and above.
    let camera = Transform::from_xyz(0.0, 50.0, 50.0).looking_at(Vec3::ZERO, Vec3::Y);
    let towards = towards_camera(&camera);
    assert!((towards - Vec2::Y).length() < 1e-4, "{towards}");
}

#[test]
fn a_camera_looking_straight_down_still_has_a_way() {
    // Straight down, with north at the top of the screen.
    let camera = Transform::from_xyz(0.0, 50.0, 0.0).looking_at(Vec3::ZERO, Vec3::NEG_Z);
    let towards = towards_camera(&camera);
    assert!((towards - Vec2::Y).length() < 1e-4, "{towards}");
}
