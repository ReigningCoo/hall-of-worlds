use std::collections::BTreeSet;

use super::*;
use crate::creature::{Location, Size};
use crate::item::Quality;
use crate::rng::Seed;

fn rolled(piece: ClothingPiece, quality: Quality) -> impl Iterator<Item = ClothingForm> {
    (0..4000u64).map(move |seed| {
        let mut rng = Seed::new(seed).derive("clothing").rng();
        roll(&mut rng, piece, quality)
    })
}

fn every(quality: Quality) -> Vec<ClothingForm> {
    ClothingPiece::ALL
        .into_iter()
        .flat_map(|piece| rolled(piece, quality))
        .collect()
}

#[test]
fn every_roll_keeps_to_its_grid() {
    for quality in [Quality::Common, Quality::Rare] {
        for form in every(quality) {
            assert!(form.allowed(), "{form:?}");
        }
    }
}

#[test]
fn each_piece_rolls_its_own_garment() {
    for piece in ClothingPiece::ALL {
        assert!(rolled(piece, Quality::Common).all(|form| form.piece() == piece));
        assert_eq!(ClothingPiece::of(piece.kind()), Some(piece));
    }
}

#[test]
fn every_name_from_the_research_turns_up() {
    let names: BTreeSet<_> = every(Quality::Rare)
        .iter()
        .map(ClothingForm::specific_name)
        .collect();
    for wanted in [
        "Tunic",
        "Cotte",
        "Kirtle",
        "Cotehardie",
        "Bliaut",
        "Chiton",
        "Peplos",
        "Exomis",
        "Tunica",
        "Stola",
        "Dalmatic",
        "Kurta",
        "Kosovorotka",
        "Huipil",
        "Smock-Frock",
        "Thawb",
        "Sarafan",
        "Robe",
        "Kosode",
        "Kimono",
        "Kaftan",
        "Deel",
        "Qaba",
        "Shenyi",
        "Khalat",
        "Angarkha",
        "Doublet",
        "Pourpoint",
        "Gipon",
        "Arming Doublet",
        "Jeogori",
        "Dolman",
        "Bodice",
        "Choli",
        "Pair of Bodies",
        "Vest",
        "Jerkin",
        "Buff Jerkin",
        "Waistcoat",
        "Yelek",
    ] {
        assert!(names.contains(wanted), "{wanted} never rolled");
    }
}

#[test]
fn floor_length_tunics_come_at_rare_and_better() {
    let floor = |quality| {
        rolled(ClothingPiece::Tunic, quality)
            .any(|form| matches!(form.garment, Garment::Tunic(t) if t.length == Length::Floor))
    };
    assert!(!floor(Quality::Common));
    assert!(floor(Quality::Rare));
}

#[test]
fn weaves_patterns_and_names_say_what_they_are() {
    let forms = every(Quality::Common);
    for weave in [Weave::Velvet, Weave::Brocade, Weave::Damask] {
        let woven: Vec<_> = forms.iter().filter(|form| form.weave == weave).collect();
        assert!(!woven.is_empty(), "no {weave:?}");
        for form in woven {
            assert!(
                form.name().contains(weave.name().unwrap()),
                "{}",
                form.name()
            );
        }
    }
    let tartan: Vec<_> = forms
        .iter()
        .filter(|form| form.pattern == Pattern::Tartan)
        .collect();
    assert!(!tartan.is_empty());
    for form in tartan {
        assert_eq!(form.material, crate::material::Material::Wool);
        assert!(form.contrast.is_some());
        assert!(form.name().starts_with("Tartan "), "{}", form.name());
    }
    assert!(forms.iter().any(|form| form.name() == "Velvet Doublet"));
}

#[test]
fn arming_doublets_have_points_for_armour() {
    let arming: Vec<_> = rolled(ClothingPiece::Doublet, Quality::Common)
        .filter_map(|form| match form.garment {
            Garment::Doublet(doublet) if doublet.arming() => Some(doublet),
            _ => None,
        })
        .collect();
    assert!(!arming.is_empty());
    assert!(arming.iter().all(|doublet| doublet.points));
}

#[test]
fn sleeveless_tunics_and_bodices_leave_the_arms_bare() {
    for form in every(Quality::Common) {
        let bare = form.garment.bare();
        match form.garment {
            Garment::Tunic(tunic) => {
                assert_eq!(bare.contains(&Location::Arms), tunic.sleeves.bare())
            }
            Garment::Bodice(bodice) => assert_eq!(
                bare.contains(&Location::Arms),
                bodice.sleeves == BodiceSleeves::None
            ),
            _ => assert!(bare.is_empty()),
        }
    }
}

#[test]
fn weights_and_cells_are_sensible() {
    for form in every(Quality::Rare) {
        let grams = form.grams();
        assert!(
            (50..=4000).contains(&grams),
            "{} weighs {grams}",
            form.name()
        );
        assert!(form.grams_at(Size::Large) > grams);
        let shape = form.shape();
        assert!(shape.width == 2 && (2..=3).contains(&shape.height));
    }
}

#[test]
fn a_form_is_saved_and_loaded_unchanged() {
    for form in every(Quality::Rare).into_iter().step_by(97) {
        let text = ron::to_string(&form).unwrap();
        assert_eq!(ron::from_str::<ClothingForm>(&text).unwrap(), form);
    }
}
