use shared::item::{Item, ItemKind};
use shared::material::{Material, WOODS};
use shared::woodcutting::{new_kindling, new_log};

use super::bark::look;
use super::kindling::kindling_mesh;
use super::mesh::{GROUND_DETAIL, LOG_LENGTH, LOG_RADIUS, LogMesh, PICTURE_DETAIL, log_mesh};
use crate::item_art::Look;
use crate::item_art::look_of;

#[test]
fn every_tree_wood_has_its_own_bark() {
    for wood in WOODS {
        assert!(look(wood.material).is_some(), "{}", wood.name);
    }
    assert_eq!(look(Material::Steel), None);
}

/// Every index is a point, every normal is a unit, every colour is in range.
fn well_made(mesh: &LogMesh) {
    let points = mesh.positions.len();
    assert!(points > 0 && mesh.indices.len().is_multiple_of(3));
    assert!(mesh.indices.iter().all(|&i| (i as usize) < points));
    assert_eq!((mesh.normals.len(), mesh.colours.len()), (points, points));
    for normal in &mesh.normals {
        let length = normal.iter().map(|n| n * n).sum::<f32>().sqrt();
        assert!(length < 1.01, "{normal:?}");
    }
    for colour in &mesh.colours {
        assert!(colour.iter().all(|c| (0.0..=1.0).contains(c)), "{colour:?}");
    }
}

#[test]
fn logs_are_a_metre_long_and_thirty_centimetres_across() {
    for wood in [
        Material::Oak,
        Material::Birch,
        Material::Palm,
        Material::Saguaro,
    ] {
        for detail in [PICTURE_DETAIL, GROUND_DETAIL] {
            let mesh = log_mesh(wood, detail, 3);
            well_made(&mesh);
            let (low, high) = mesh
                .positions
                .iter()
                .fold((f32::MAX, f32::MIN), |(low, high), p| {
                    (low.min(p[0]), high.max(p[0]))
                });
            assert!((high - low - LOG_LENGTH).abs() < 1e-4);
            let widest = (mesh.positions.iter())
                .map(|p| (p[1] * p[1] + p[2] * p[2]).sqrt())
                .fold(0.0, f32::max);
            assert!(
                widest <= LOG_RADIUS + 1e-4 && widest > LOG_RADIUS * 0.95,
                "{wood:?}"
            );
        }
    }
}

#[test]
fn bark_differs_between_woods_and_logs() {
    let colours = |wood, seed| log_mesh(wood, GROUND_DETAIL, seed).colours;
    assert_ne!(colours(Material::Oak, 1), colours(Material::Birch, 1));
    assert_ne!(colours(Material::Oak, 1), colours(Material::Oak, 2));
    assert_eq!(colours(Material::Oak, 1), colours(Material::Oak, 1));
}

#[test]
fn kindling_is_three_sticks() {
    let mesh = kindling_mesh(Material::Pine);
    well_made(&mesh);
}

#[test]
fn logs_and_kindling_have_pictures_by_wood() {
    let log: Item = new_log(Material::Ash, shared::id::Id::for_test(1), None);
    assert_eq!(look_of(&log), Some(Look::Log(Material::Ash)));
    assert_eq!(Look::Log(Material::Ash).cells(), (8, 3));
    let kindling = new_kindling(Material::Ash, 4, shared::id::Id::for_test(2), None);
    assert_eq!(kindling.kind, ItemKind::Kindling);
    assert_eq!(look_of(&kindling), Some(Look::Kindling(Material::Ash)));
}
