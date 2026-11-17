use super::*;
use shared::potion::BottleForm;
use std::collections::HashSet;

fn form(look: u64) -> BottleForm {
    BottleForm::new(None, look)
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

mod liquid_tests {
    use super::super::liquid::{Base, LiquidLook};
    use super::super::liquid_colour::{alpha, main_colour};
    use crate::item_art::{Look, look_of};
    use shared::herb::Stat;
    use shared::id::Id;
    use shared::item::{Item, ItemKind};
    use shared::item_form::ItemForm;
    use shared::potion::{BottleForm, Contents, Dose};

    fn contents(doses: &[(Stat, i16)]) -> Contents {
        Contents::new(doses.iter().map(|&(stat, step)| Dose { stat, step }))
    }

    fn liquid(doses: &[(Stat, i16)]) -> LiquidLook {
        LiquidLook::of(&contents(doses)).expect("a potion")
    }

    /// A bottle item with this id and look seed, holding these.
    fn bottle(id: u128, look: u64, doses: &[(Stat, i16)]) -> Item {
        let mut item = Item::new(Id::for_test(id), ItemKind::Bottle);
        let potion = shared::potion::Liquid::Potion(contents(doses));
        item.form = Some(ItemForm::Bottle(BottleForm::new(Some(potion), look)));
        item
    }

    fn liquid_of(item: &Item) -> Option<LiquidLook> {
        match look_of(item) {
            Some(Look::Bottle { liquid, .. }) => liquid,
            _ => panic!("a bottle's look"),
        }
    }

    #[test]
    fn equal_contents_look_the_same_whatever_the_seed() {
        let doses = [(Stat::Mana, 5), (Stat::Health, -3), (Stat::Agility, 2)];
        let first = liquid_of(&bottle(1, 7, &doses));
        // The same doses in another order, in another bottle.
        let reordered = [doses[2], doses[0], doses[1]];
        for seed in 0..50 {
            assert_eq!(
                liquid_of(&bottle(100 + seed as u128, seed, &reordered)),
                first
            );
        }
        assert!(first.is_some());
    }

    #[test]
    fn waters_and_oils_have_their_own_looks() {
        use shared::potion::{Liquid, Oil, Water};
        let of = |liquid| LiquidLook::of_bottle(&BottleForm::new(Some(liquid), 0)).unwrap();
        let moon = of(Liquid::Water(Water::Moon));
        assert_eq!(moon.main, Base::Water(Water::Moon));
        assert!(moon.sparkles);
        let nut = of(Liquid::Oil(Oil::Nut));
        assert_eq!(nut.main, Base::Oil(Oil::Nut));
        assert!(alpha(&of(Liquid::Water(Water::Plain))) < alpha(&nut));
        assert_ne!(main_colour(&nut), main_colour(&of(Liquid::Oil(Oil::Seed))));
    }

    #[test]
    fn an_empty_bottle_has_no_liquid() {
        assert_eq!(liquid_of(&bottle(1, 3, &[])), None);
    }

    #[test]
    fn the_strongest_effect_sets_the_colour_ties_by_stat_order() {
        let look = liquid(&[(Stat::Mana, 3), (Stat::Will, -6), (Stat::Agility, 2)]);
        assert_eq!(look.main, Base::Stat(Stat::Will));
        assert_eq!(look.second, Some(Stat::Mana));
        let tie = liquid(&[(Stat::Health, 4), (Stat::Strength, -4)]);
        assert_eq!(tie.main, Base::Stat(Stat::Strength));
        assert_eq!(tie.second, Some(Stat::Health));
    }

    #[test]
    fn helping_is_paler_and_harming_darker() {
        let light = |doses: &[(Stat, i16)]| {
            let colour = main_colour(&liquid(doses)).to_srgba();
            colour.red + colour.green + colour.blue
        };
        assert!(light(&[(Stat::Health, 8)]) > light(&[(Stat::Health, 2)]));
        assert!(light(&[(Stat::Health, -8)]) < light(&[(Stat::Health, -2)]));
    }

    #[test]
    fn stronger_is_less_see_through() {
        let weak = alpha(&liquid(&[(Stat::Aura, 2)]));
        let strong = alpha(&liquid(&[(Stat::Aura, -6), (Stat::Will, 8)]));
        assert!(strong > weak);
    }

    #[test]
    fn features_follow_the_stats() {
        let look = liquid(&[(Stat::Agility, 2), (Stat::Toughness, 3)]);
        assert!(look.fizz && look.layered && !look.bubbles && !look.sparkles);
        assert_eq!(look.blobs, None);
        let mixed = liquid(&[(Stat::Health, 6), (Stat::Will, -2), (Stat::Mana, -4)]);
        assert!(mixed.sparkles);
        assert_eq!(mixed.blobs, Some(Stat::Mana));
    }

    #[test]
    fn every_shape_holds_its_liquid_inside_its_glass() {
        use super::super::liquid_body::{band, levels};
        for shape in super::Shape::ALL {
            let (bottom, top) = levels(shape);
            assert!(top > bottom + 10.0, "{shape:?}");
            let surface = band(shape, 0.0, 1.0);
            assert!(
                surface
                    .rows
                    .iter()
                    .flatten()
                    .all(|p| p.y >= bottom - 1e-3 && p.y <= top + 1e-3)
            );
        }
    }
}
