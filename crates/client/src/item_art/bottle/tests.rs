use super::*;
use shared::potion::{BottleForm, Contents};
use std::collections::HashSet;

fn form(look: u64) -> BottleForm {
    BottleForm {
        contents: Contents::EMPTY,
        look,
    }
}

#[test]
fn seeds_give_every_shape_glass_and_stopper() {
    let looks: Vec<BottleLook> = (0..2000).map(|seed| BottleLook::of(&form(seed))).collect();
    let shapes: HashSet<Shape> = looks.iter().map(|look| look.shape).collect();
    let glasses: HashSet<Glass> = looks.iter().map(|look| look.glass).collect();
    let stoppers: HashSet<Stopper> = looks.iter().map(|look| look.stopper).collect();
    assert_eq!(shapes.len(), Shape::ALL.len());
    assert_eq!(glasses.len(), Glass::ALL.len());
    assert_eq!(stoppers.len(), Stopper::ALL.len());
}

#[test]
fn the_same_seed_looks_the_same() {
    assert_eq!(BottleLook::of(&form(42)), BottleLook::of(&form(42)));
}

#[test]
fn every_outline_rises_and_ends_at_its_mouth() {
    for shape in Shape::ALL {
        let outline = profile(shape);
        assert!(
            outline.mouth() > 0.0 && outline.mouth() < outline.neck(),
            "{shape:?}"
        );
        assert!(outline.lip().0 > outline.neck(), "{shape:?}");
        assert!(outline.points.iter().all(|p| p.x >= 0.0), "{shape:?}");
        assert!(outline.top() > outline.shoulder, "{shape:?}");
    }
}

#[test]
fn surfaces_face_outwards() {
    for shape in Shape::ALL {
        let (positions, normals, _) = Surface::lathe(&profile(shape)).triangles();
        let inward = positions
            .iter()
            .zip(&normals)
            .filter(|(p, n)| p.xz().dot(n.xz()) < -1e-3)
            .count();
        assert_eq!(inward, 0, "{shape:?}");
    }
    let (positions, normals, _) = heart::heart_body().triangles();
    let inward = positions
        .iter()
        .zip(&normals)
        .filter(|(p, n)| p.z * n.z < -1e-3);
    assert_eq!(inward.count(), 0);
}

#[test]
fn the_heart_neck_rises_from_its_dip() {
    let neck = profile(Shape::Heart);
    assert!(neck.points[0].y < heart::dip());
    assert!(neck.lip().1 > heart::dip() + 10.0);
}

#[test]
fn fitting_keeps_a_bottle_inside_its_picture() {
    let turn = Quat::from_rotation_x(-FRAC_PI_2 + TILT);
    let (right, up) = picture_axes();
    for shape in Shape::ALL {
        let points: Vec<Vec3> = Surface::lathe(&profile(shape))
            .rows
            .into_iter()
            .flatten()
            .collect();
        let fitted = fit(&points, turn);
        for point in points {
            let at = fitted.transform_point(point);
            assert!(at.dot(right).abs() <= FIT + 1e-3, "{shape:?}");
            assert!(at.dot(up).abs() <= FIT + 1e-3, "{shape:?}");
        }
    }
}
