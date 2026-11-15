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
        // No cloth right behind where the tail leaves, below it (cloth between the
        // legs, well in front of the tail's root, is out of its way).
        let behind = |c: &Vec3| {
            c.y < root.y - 40.0 && c.y > root.y - 200.0 && c.x.abs() < 8.0 && c.z < root.z + 40.0
        };
        let blocking = corners(&model).any(|c| behind(&c));
        let at = corners(&model).find(|c| behind(c));
        assert!(!blocking, "{at:?} {root} {look:?}");
    }
}

/// Forms of one kind, rolled from many seeds.
fn rolled(piece: ClothingPiece) -> impl Iterator<Item = ClothingForm> {
    (0..60u64)
        .map(move |seed| shared::clothing::roll(&mut Seed::new(seed).rng(), piece, Quality::Rare))
}

/// How far off the skin the front of the cloth is at the waist, at most.
fn front_at_waist(model: &Model, frame: &Frame) -> f32 {
    near_waist(model, frame).fold(0.0, f32::max)
}

/// How far off the skin each corner at the front middle near the waist is.
fn near_waist<'a>(model: &'a Model, frame: &'a Frame) -> impl Iterator<Item = f32> + 'a {
    let waist = frame.waist as f32;
    corners(model)
        .filter(move |c| (c.y - waist).abs() < 22.0 && c.x.abs() < 15.0 && c.z > 0.0)
        .map(move |c| c.z - trunk_half(frame, c.y).deep(true))
}

#[test]
fn a_belt_cinches_a_tunic_at_the_waist() {
    let frame = Frame::of(CreatureKind::Human).unwrap();
    let belted = Wearer {
        belt: Some(45.0),
        ..Wearer::default()
    };
    let mut fuller = 0;
    for form in rolled(ClothingPiece::Tunic) {
        let loose = build(&form, &frame, Fit::Worn(Wearer::default()));
        let cinched = build(&form, &frame, Fit::Worn(belted));
        let (loose, cinched) = (
            front_at_waist(&loose, &frame),
            front_at_waist(&cinched, &frame),
        );
        assert!(cinched <= loose + 0.5, "{cinched} > {loose} {form:?}");
        if loose > cinched + 1.5 {
            fuller += 1;
        }
    }
    assert!(fuller > 15, "only {fuller} drawn in");
}

#[test]
fn a_belt_goes_round_outside_the_tunic_it_cinches() {
    let frame = Frame::of(CreatureKind::Human).unwrap();
    let belted = Wearer {
        belt: Some(45.0),
        ..Wearer::default()
    };
    let belts: Vec<_> = rolled(ClothingPiece::Belt)
        .filter(|form| matches!(form.garment, Garment::Belt(b) if b.form == shared::clothing::BeltForm::Strap))
        .collect();
    for tunic in
        rolled(ClothingPiece::Tunic).filter(|form| form.pattern == shared::clothing::Pattern::Plain)
    {
        // The cloth itself: lacing and buttons on it may be under the belt.
        let mut cloth = build(&tunic, &frame, Fit::Worn(belted));
        cloth.parts.truncate(1);
        let tunic = front_at_waist(&cloth, &frame);
        for belt in &belts {
            let mut model = build(belt, &frame, Fit::Worn(belted));
            let mut band = Model::default();
            band.parts.push(model.parts.swap_remove(0));
            let inside = near_waist(&band, &frame).fold(f32::MAX, f32::min);
            assert!(inside > tunic, "{inside} inside {tunic} {:?}", belt.garment);
        }
    }
}

#[test]
fn trousers_are_hidden_under_a_long_tunic() {
    let frame = Frame::of(CreatureKind::Human).unwrap();
    let knee = frame.knee as f32;
    let under = Wearer {
        torso_hem: Some(knee),
        ..Wearer::default()
    };
    for form in rolled(ClothingPiece::Trousers) {
        let model = build(&form, &frame, Fit::Worn(under));
        for corner in corners(&model) {
            assert!(corner.y < knee + 60.0, "{corner} {form:?}");
        }
    }
}
