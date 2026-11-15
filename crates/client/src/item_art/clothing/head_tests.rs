use super::*;
use shared::body::Layout;
use shared::clothing::ClothingPiece;
use shared::clothing::{Hood, HoodCape, HoodFace};
use shared::rng::Seed;

use crate::item_art::armour::Stuff;

fn corners(model: &Model) -> impl Iterator<Item = Vec3> + '_ {
    model
        .parts
        .iter()
        .flat_map(|(_, t)| t.iter().flatten().copied())
}

fn rolled(piece: shared::clothing::ClothingPiece) -> impl Iterator<Item = ClothingForm> {
    (0..60u64)
        .map(move |seed| shared::clothing::roll(&mut Seed::new(seed).rng(), piece, Quality::Rare))
}

/// The head's ellipsoid, `off` out: how far a point is, 1 on it.
fn on_head(frame: &Frame, point: Vec3, off: f32) -> f32 {
    let high = (frame.height - frame.chin) as f32 / 2.0;
    let centre = Vec3::Y * (frame.chin as f32 + high);
    let from = point - centre;
    Vec3::new(
        from.x / (frame.head.0 as f32 + off),
        from.y / (high + off),
        from.z / (frame.head.1 as f32 + off),
    )
    .length()
}

#[test]
fn hoods_caps_and_headcloths_stand_clear_of_a_linen_coif() {
    let frame = Frame::of(CreatureKind::Human).unwrap();
    let crown = frame.height as f32 - 60.0;
    for piece in [
        ClothingPiece::Hood,
        ClothingPiece::Cap,
        ClothingPiece::Headcloth,
    ] {
        // Patterns split triangles onto chords inside the surface: plain cloth only.
        let plain = rolled(piece).filter(|form| {
            form.pattern == shared::clothing::Pattern::Plain
                && form.weave == shared::clothing::Weave::Plain
        });
        for form in plain {
            let model = build(&form, &frame, Fit::Worn(Wearer::default()));
            for corner in corners(&model).filter(|c| c.y > crown) {
                // A coif is 11 mm off the head.
                assert!(on_head(&frame, corner, 12.0) > 1.0, "{corner} {form:?}");
            }
        }
    }
}

#[test]
fn a_snout_gets_a_longer_face_opening() {
    let layout = Layout::of(CreatureKind::Minotaur).unwrap();
    let (frame, _) = layout.worn_frame().unwrap();
    let hood = ClothingForm {
        garment: Garment::Hood(Hood {
            cape: HoodCape::None,
            tail: shared::clothing::Tail::None,
            face: HoodFace::Close,
            edge: shared::clothing::Edge::Plain,
            lappets: false,
            chaperon: false,
        }),
        ..rolled(ClothingPiece::Hood).next().unwrap()
    };
    // Cloth right in front of the face, low on it, where the muzzle comes out.
    let low = frame.chin as f32 + 10.0;
    let in_front = |model: &Model| {
        corners(model)
            .filter(|c| c.x.abs() < 15.0 && c.z > 0.0 && (c.y - low).abs() < 15.0)
            .count()
    };
    let plain = build(&hood, &frame, Fit::Worn(Wearer::default()));
    let snouted = build(&hood, &frame, Fit::Worn(Wearer::of(&layout, &[])));
    assert!(in_front(&plain) > 0);
    assert_eq!(in_front(&snouted), 0);
}

#[test]
fn horns_come_out_through_bound_slits() {
    let layout = Layout::of(CreatureKind::Minotaur).unwrap();
    let (frame, _) = layout.worn_frame().unwrap();
    let wearer = Wearer::of(&layout, &[]);
    assert!(wearer.head.horns.is_some());
    for form in rolled(ClothingPiece::Hood).take(10) {
        let bare = build(&form, &frame, Fit::Worn(Wearer::default()));
        let horned = build(&form, &frame, Fit::Worn(wearer));
        let shade = |model: &Model| {
            model
                .parts
                .iter()
                .filter(|(stuff, _)| *stuff == Stuff::Shade)
                .map(|(_, t)| t.len())
                .sum::<usize>()
        };
        assert!(shade(&horned) > shade(&bare), "{form:?}");
    }
}

#[test]
fn a_hat_goes_over_a_hood() {
    let frame = Frame::of(CreatureKind::Human).unwrap();
    let crown = frame.height as f32;
    for hood in rolled(ClothingPiece::Hood)
        .filter(|form| matches!(form.garment, Garment::Hood(hood) if !hood.chaperon))
    {
        let under = super::hat::reach_under(&hood.garment);
        let wearer = Wearer {
            head: super::worn_head::WornHead {
                under,
                ..Default::default()
            },
            ..Wearer::default()
        };
        let top = corners(&build(&hood, &frame, Fit::Worn(wearer)))
            .map(|c| c.y)
            .fold(f32::MIN, f32::max);
        for hat in rolled(ClothingPiece::Hat).take(10) {
            let model = build(&hat, &frame, Fit::Worn(wearer));
            let hat_top = corners(&model).map(|c| c.y).fold(f32::MIN, f32::max);
            assert!(hat_top > top.max(crown), "{hat_top} {top} {hat:?}");
        }
    }
}
