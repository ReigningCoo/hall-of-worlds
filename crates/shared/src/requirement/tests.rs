use super::*;
use crate::armour::Construction;
use crate::bonus::{Amount, Stat, item_bonuses};
use crate::creature::CreatureKind;
use crate::id::Id;
use crate::item::{Item, ItemKind, Quality};
use crate::item_form::{ItemForm, ItemOrder, make_item};
use crate::material::Material;
use crate::rng::Seed;
use crate::sheet::{Attribute, Attributes};

/// A torso piece, made this way of this.
fn torso(construction: Construction, material: Material) -> Item {
    let mut item = make_item(
        &ItemOrder::plain(ItemKind::TorsoArmour, Quality::Common),
        CreatureKind::Human,
        Id::for_test(1),
        Seed::new(3),
        None,
        &mut || Id::for_test(2),
    );
    let Some(ItemForm::Torso(form)) = item.form.as_mut() else {
        panic!("torso armour has a torso form");
    };
    form.construction = construction;
    form.material = material;
    item
}

fn needs(item: &Item) -> Vec<(Attribute, u16)> {
    item_requirements(item)
        .into_iter()
        .map(|requirement| match requirement {
            Requirement::Attribute { attribute, min } => (attribute, min),
            other => panic!("armour asks only for attributes, not {other:?}"),
        })
        .collect()
}

#[test]
fn steel_plate_asks_a_little_over_the_median_strength() {
    let plate = torso(Construction::Plate, Material::Steel);
    assert_eq!(needs(&plate), vec![(Attribute::Strength, 110)]);
}

#[test]
fn riveted_mail_asks_both_and_ring_mail_only_agility() {
    let mail = needs(&torso(Construction::Mail, Material::Steel));
    assert_eq!(
        mail,
        vec![(Attribute::Agility, 95), (Attribute::Strength, 95)]
    );
    let rings = needs(&torso(Construction::RingMail, Material::Steel));
    assert_eq!(rings, vec![(Attribute::Agility, 51)]);
}

#[test]
fn padding_and_leather_ask_little_agility() {
    let padded = needs(&torso(Construction::Padded, Material::Linen));
    assert_eq!(padded, vec![(Attribute::Agility, 28)]);
    let leather = needs(&torso(Construction::Leather, Material::HardenedLeather));
    assert_eq!(leather, vec![(Attribute::Agility, 51)]);
}

#[test]
fn lighter_metals_ask_less_and_non_metal_plates_go_by_their_thickness() {
    let mithril = needs(&torso(Construction::Plate, Material::Mithril));
    assert_eq!(mithril, vec![(Attribute::Strength, 55)]);
    // Horn scales 6 mm thick: 13 × 60 = 780 mg/cm².
    assert_eq!(milligrams_per_cm2(Construction::Scale, Material::Horn), 780);
}

#[test]
fn every_piece_of_a_set_asks_the_same_whatever_its_size() {
    // Only how it is made and of what count, never the piece or its size.
    let mut big = torso(Construction::Plate, Material::Steel);
    big.made.size = crate::creature::Size::Large;
    assert_eq!(needs(&big), vec![(Attribute::Strength, 110)]);
}

#[test]
fn the_factor_rises_from_half_to_all_of_the_requirement() {
    assert_eq!(factor_for(100, 50), 0);
    assert_eq!(factor_for(100, 30), 0);
    assert_eq!(factor_for(100, 75), 500);
    assert_eq!(factor_for(100, 100), FULL);
    assert_eq!(factor_for(100, 180), FULL);
    assert_eq!(factor_for(0, 1), FULL);
}

#[test]
fn with_two_requirements_the_lower_factor_counts() {
    let mail = item_requirements(&torso(Construction::Mail, Material::Steel));
    let strong_but_clumsy = Attributes {
        agility: 71,
        ..Attributes::all(200)
    };
    // Agility 71 of 95: (142 − 95) ÷ 95.
    assert_eq!(effectiveness(&mail, &strong_but_clumsy), 494);
}

#[test]
fn good_bonuses_shrink_and_bad_ones_stay_in_full() {
    let plate = torso(Construction::Plate, Material::Steel);
    let full = item_bonuses(&plate);
    // Strength 82 of 110: (164 − 110) ÷ 110, about half.
    let weak = Attributes {
        strength: 82,
        ..Attributes::all(100)
    };
    let factor = effectiveness(&item_requirements(&plate), &weak);
    assert_eq!(factor, 490);
    for (before, after) in full.iter().zip(effective_bonuses(&plate, &weak)) {
        let (Amount::Flat(was), Amount::Flat(now)) = (before.amount, after.amount) else {
            panic!("armour's bonuses are flat");
        };
        if was > 0 {
            assert_eq!(i64::from(now), i64::from(was) * 490 / 1000);
        } else {
            assert_eq!(now, was, "{:?} costs in full", before.stat);
        }
    }
    let toughness = Stat::Attribute(Attribute::Toughness);
    assert!(full.iter().any(|bonus| bonus.stat == toughness));
}

#[test]
fn strong_enough_wearers_get_everything() {
    let plate = torso(Construction::Plate, Material::Steel);
    let strong = Attributes::all(110);
    assert_eq!(effective_bonuses(&plate, &strong), item_bonuses(&plate));
}

#[test]
fn other_items_ask_nothing() {
    let hat = Item::new(Id::for_test(1), ItemKind::Hat);
    assert!(item_requirements(&hat).is_empty());
    assert_eq!(effectiveness(&[], &Attributes::all(1)), FULL);
}
