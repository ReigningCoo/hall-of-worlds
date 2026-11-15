use bevy::prelude::*;
use shared::body::Frame;
use shared::creature::{CreatureKind, Size};
use shared::id::Id;
use shared::item::{Item, ItemKind, Quality};
use shared::item_form::{ItemForm, give_form};

use super::path::Skin;
use super::{NecklaceLook, WORN_TOP, Wear, build};

/// Necklaces as the server would roll them, through every quality.
fn rolled(count: u128) -> Vec<NecklaceLook> {
    (1..=count)
        .filter_map(|seed| {
            let mut item = Item::new(Id::for_test(seed), ItemKind::Necklace);
            item.quality = Some(Quality::ALL[seed as usize % Quality::ALL.len()]);
            let mut gems = seed << 32;
            give_form(&mut item, Some(Size::Medium), &mut || {
                gems += 1;
                Id::for_test(gems)
            });
            match &item.form {
                Some(ItemForm::Necklace(form)) => Some(NecklaceLook::new(form, item.made.size)),
                _ => None,
            }
        })
        .collect()
}

fn corners(look: &NecklaceLook, frame: &Frame, wear: Wear) -> Vec<Vec3> {
    let (model, gems) = build(look, frame, wear);
    let mut all: Vec<Vec3> = model
        .parts
        .iter()
        .flat_map(|(_, triangles)| triangles.iter().flatten().copied())
        .collect();
    for gem in gems {
        all.extend(gem.stone.iter().flatten().copied());
    }
    all
}

#[test]
fn every_rolled_necklace_is_built() {
    let frame = Frame::of(CreatureKind::Human).unwrap();
    let looks = rolled(300);
    assert!(looks.len() == 300);
    for look in &looks {
        for wear in [Wear::Alone, Wear::Worn] {
            let points = corners(look, &frame, wear);
            assert!(!points.is_empty(), "{look:?}");
            assert!(points.iter().all(|point| point.is_finite()), "{look:?}");
        }
    }
}

/// Worn, it lies close on the skin in front, so a shirt over it hides it.
#[test]
fn worn_necklaces_keep_close_to_the_chest() {
    for kind in [
        CreatureKind::Human,
        CreatureKind::Dwarf,
        CreatureKind::Halfling,
    ] {
        let frame = Frame::of(kind).unwrap();
        let skin = Skin::of(&frame);
        for look in rolled(200) {
            for point in corners(&look, &frame, Wear::Worn) {
                if point.y > skin.neck_base() - 20.0 || point.z < 0.0 || point.x.abs() > 25.0 {
                    continue;
                }
                // How far it is from the nearest skin, looking a little up and down.
                let off = (-24..=24)
                    .map(|step| {
                        let (on, _) = skin.front(point.x, point.y + step as f32 * 0.25);
                        point.distance(on)
                    })
                    .fold(f32::MAX, f32::min);
                assert!(
                    off <= WORN_TOP + 0.3,
                    "{kind:?} {look:?}: {off} off at {point}"
                );
            }
        }
    }
}
