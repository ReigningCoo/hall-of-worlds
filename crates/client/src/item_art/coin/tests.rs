use super::outline::relief;
use super::parts::{coin_mesh, placed};
use super::*;
use crate::item_art::bottle::picture_axes;
use bevy::prelude::Vec2;
use shared::material::Material;

const METALS: [Material; 3] = [Material::Copper, Material::Silver, Material::Gold];
/// Half the picture, less the black edge's room: 4 pixels of 64 a cell.
const ROOM: f32 = 1.25 - 2.5 * 4.0 / 64.0;

/// How far the coin reaches across and up its picture, each way from the middle.
fn reach(metal: Material) -> Vec2 {
    let mesh = coin_mesh(Coin::of(metal));
    let placed = placed(&mesh.positions);
    let (right, up) = picture_axes();
    mesh.positions.iter().fold(Vec2::ZERO, |most, &p| {
        let at = placed.transform_point(p);
        most.max(Vec2::new(at.dot(right).abs(), at.dot(up).abs()))
    })
}

#[test]
fn each_coin_fits_inside_its_picture() {
    for metal in METALS {
        let reach = reach(metal);
        assert!(reach.max_element() < ROOM, "{metal:?} reaches {reach}");
    }
}

#[test]
fn coins_keep_their_size_order() {
    let [copper, silver, gold] = METALS.map(reach);
    assert!(copper.x > silver.x && silver.x > gold.x);
    assert!(copper.y > silver.y && silver.y > gold.y);
    let [copper, silver, gold] = METALS.map(Coin::of);
    assert!(copper.thickness > silver.thickness && silver.thickness > gold.thickness);
    assert!((gold.radius * 2.0 - copper.radius).abs() < 1e-4);
    assert!((gold.thickness * 2.0 - copper.thickness).abs() < 1e-4);
    // Copper about 90% of the cell across, silver about 70%.
    assert!((silver.radius / copper.radius - 7.0 / 9.0).abs() < 1e-4);
}

#[test]
fn the_coo_stands_inside_the_rim() {
    let outlines = coo::coo();
    for i in 0..360 {
        let edge = Vec2::from_angle((i as f32).to_radians());
        assert_eq!(relief(&outlines, edge), 0.0);
    }
    // The crown's band is the highest, the muzzle nearly as high.
    assert!(relief(&outlines, Vec2::new(0.1, 0.47)) > 0.95);
    assert!(relief(&outlines, Vec2::new(0.0, -0.36)) > 0.8);
    // The nostrils are pressed in, and the field beside the head is bare.
    assert!(relief(&outlines, Vec2::new(0.12, -0.49)) < 0.6);
    assert_eq!(relief(&outlines, Vec2::new(0.6, -0.6)), 0.0);
}

#[test]
fn every_coin_faces_out() {
    let mesh = coin_mesh(Coin::of(Material::Copper));
    let up = mesh.normals.iter().filter(|n| n.y > 0.5).count();
    let down = mesh.normals.iter().filter(|n| n.y < -0.5).count();
    assert!(up > down * 10, "{up} up, {down} down");
    assert!(mesh.normals.iter().all(|n| n.is_normalized()));
}
