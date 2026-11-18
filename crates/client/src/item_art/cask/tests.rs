use super::*;
use shared::cask::CaskSize;
use shared::material::Material;

#[test]
fn casks_hold_about_their_litres() {
    for size in CaskSize::ALL {
        let height = height_of(size);
        let radius = mesh::radius_share(size) * height;
        // A barrel's volume by the old gaugers' rule: π h (2R² + r²) / 3.
        let ends = radius * 0.82;
        let litres = std::f32::consts::PI * height * (2.0 * radius * radius + ends * ends) / 3.0;
        let litres = litres * 1000.0;
        let wanted = size.litres() as f32;
        assert!((litres / wanted - 1.0).abs() < 0.01, "{size:?}: {litres} L");
    }
    assert!(height_of(CaskSize::Pin) < 0.5 && height_of(CaskSize::Tun) > 1.0);
}

#[test]
fn every_point_has_a_normal_and_colour() {
    let look = CaskLook {
        size: CaskSize::Firkin,
        wood: Material::Cherry,
    };
    let cask = cask_mesh(look);
    assert_eq!(cask.positions.len(), cask.normals.len());
    assert_eq!(cask.positions.len(), cask.colours.len());
    let points = cask.positions.len() as u32;
    assert!(cask.indices.iter().all(|&index| index < points));
    assert!(cask.indices.len().is_multiple_of(3));
}
