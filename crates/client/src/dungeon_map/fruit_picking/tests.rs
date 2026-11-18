use super::*;

/// A ray straight down onto the middle of the top goes in at the top.
#[test]
fn a_ray_from_above_goes_in_at_the_top() {
    let ray = Ray3d::new(Vec3::new(0.0, 100.0, 0.0), Dir3::NEG_Y);
    let distance = into_cylinder(ray, Vec3::ZERO, 30.0, 10.0);
    assert_eq!(distance, Some(70.0));
}

/// A level ray through the side goes in where it meets the side, and one
/// passing by or over misses.
#[test]
fn a_level_ray_goes_in_at_the_side() {
    let ray = Ray3d::new(Vec3::new(-50.0, 5.0, 0.0), Dir3::X);
    let distance = into_cylinder(ray, Vec3::ZERO, 30.0, 10.0).unwrap();
    assert!((distance - 40.0).abs() < 1e-3);
    let by = Ray3d::new(Vec3::new(-50.0, 5.0, 12.0), Dir3::X);
    assert_eq!(into_cylinder(by, Vec3::ZERO, 30.0, 10.0), None);
    let over = Ray3d::new(Vec3::new(-50.0, 35.0, 0.0), Dir3::X);
    assert_eq!(into_cylinder(over, Vec3::ZERO, 30.0, 10.0), None);
}

/// A ray pointing away misses what is behind it.
#[test]
fn a_ray_pointing_away_misses() {
    let ray = Ray3d::new(Vec3::new(-50.0, 5.0, 0.0), Dir3::NEG_X);
    assert_eq!(into_cylinder(ray, Vec3::ZERO, 30.0, 10.0), None);
}
