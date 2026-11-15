use super::wearer::Covered;
use super::*;
use crate::item_art::body::trunk_half;
use shared::body::Layout;
use shared::rng::Seed;

/// Every piece drawn, rolled from many seeds.
fn looks() -> impl Iterator<Item = ClothingLook> {
    ClothingPiece::ALL
        .into_iter()
        .filter(|&piece| drawn(piece))
        .flat_map(|piece| {
            (0..120u64).map(move |seed| {
                let form = shared::clothing::roll(&mut Seed::new(seed).rng(), piece, Quality::Rare);
                let shape = form.shape();
                ClothingLook::new(form, (shape.width, shape.height))
            })
        })
}

fn corners(model: &Model) -> impl Iterator<Item = Vec3> + '_ {
    model
        .parts
        .iter()
        .flat_map(|(_, t)| t.iter().flatten().copied())
}

#[test]
fn every_piece_stays_inside_its_picture_with_room_for_its_edge() {
    for look in looks() {
        let (wide, long) = (look.cells.0 as f32, look.cells.1 as f32);
        let per_cell = VIEW_WIDTH / wide.max(long);
        let edge = OUTLINE_PIXELS / PIXELS_PER_CELL * per_cell;
        let model = fitted(&look);
        assert!(!model.parts.is_empty(), "{look:?}");
        for corner in corners(&model) {
            assert!(corner.is_finite(), "{look:?}");
            assert!(
                corner.x.abs() + edge <= wide * per_cell / 2.0 + 0.001,
                "{look:?}"
            );
            assert!(
                corner.y.abs() + edge <= long * per_cell / 2.0 + 0.001,
                "{look:?}"
            );
        }
    }
}

#[test]
fn every_piece_builds_worn_on_every_people() {
    for kind in CreatureKind::ALL {
        let Some(layout) = Layout::of(kind) else {
            continue;
        };
        let Some((frame, _)) = layout.worn_frame() else {
            continue;
        };
        let wearer = Wearer::of(&layout, &[]);
        for look in looks().step_by(5) {
            let model = build(&look.form, &frame, Fit::Worn(wearer));
            assert!(corners(&model).count() > 0, "{kind:?} {look:?}");
            assert!(corners(&model).all(|c| c.is_finite()), "{kind:?} {look:?}");
        }
    }
}

/// Worn, the trunk's cloth is outside a tucked shirt (2.5 mm off the skin) and, under
/// torso armour, inside its 15 mm.
#[test]
fn worn_it_lies_between_the_underclothes_and_armour() {
    let frame = Frame::of(CreatureKind::Human).unwrap();
    let armoured = Wearer {
        covered: Covered {
            torso: true,
            arms: true,
            skirt: true,
        },
        ..Wearer::default()
    };
    // Between the chest and the waist, round the middle of the front and back.
    let (low, high) = (frame.waist as f32 + 20.0, frame.chest as f32);
    // Patterns split triangles, with new corners on chords between the rows, not on
    // the surface: only plain cloth is measured.
    let plain = looks().filter(|look| {
        let form = &look.form;
        let all_over = matches!(form.garment, Garment::Robe(robe) if robe.decoration == RobeDecoration::AllOver);
        form.pattern == shared::clothing::Pattern::Plain
            && form.weave == shared::clothing::Weave::Plain
            && !all_over
    });
    for look in plain {
        let model = build(&look.form, &frame, Fit::Worn(armoured));
        let (stuff, triangles) = &model.parts[0];
        assert_eq!(*stuff, crate::item_art::armour::Stuff::Main);
        for corner in triangles.iter().flatten() {
            if corner.y < low || corner.y > high || corner.x.abs() > 40.0 {
                continue;
            }
            let girth = trunk_half(&frame, corner.y);
            // The skin's oval, where this corner is across it.
            let across = (corner.x / girth.across).min(1.0);
            let skin = girth.deep(corner.z >= 0.0) * (1.0 - across * across).sqrt();
            let off = corner.z.abs() - skin;
            assert!(off > 2.5, "{off} at {corner} {look:?}");
            assert!(off < 15.0, "{off} at {corner} {look:?}");
        }
    }
}

#[test]
fn a_tail_comes_out_through_a_slit() {
    let layout = Layout::of(CreatureKind::Lizardfolk).unwrap();
    let (frame, _) = layout.worn_frame().unwrap();
    let wearer = Wearer::of(&layout, &[]);
    let (root, _) = wearer.tail.expect("lizardfolk have tails");
    for look in looks() {
        let model = build(&look.form, &frame, Fit::Worn(wearer));
        // No cloth right behind where the tail leaves, below it.
        let blocking = corners(&model)
            .any(|c| c.y < root.y - 40.0 && c.y > root.y - 200.0 && c.x.abs() < 8.0 && c.z < 0.0);
        let at = corners(&model)
            .find(|c| c.y < root.y - 40.0 && c.y > root.y - 200.0 && c.x.abs() < 8.0 && c.z < 0.0);
        assert!(!blocking, "{at:?} {root} {look:?}");
    }
}
