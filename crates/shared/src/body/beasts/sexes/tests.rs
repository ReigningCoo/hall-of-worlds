use super::*;

fn coo(gender: Gender) -> Layout {
    Layout::of_gender(CreatureKind::Coo, gender).unwrap()
}

#[test]
fn coo_bulls_are_bigger_with_forward_horns_and_cows_upturned() {
    let (bull, cow, neutral) = (coo(Gender::Male), coo(Gender::Female), coo(Gender::Neutral));
    assert_eq!(neutral, Layout::of(CreatureKind::Coo).unwrap());
    assert_eq!(bull.parts.horns, Horns::Forward);
    assert_eq!(cow.parts.horns, Horns::Upturned);
    assert_eq!(neutral.parts.horns, Horns::Upturned);
    // About 125 cm and 105 cm high, by the hips' height.
    let hips = |layout: &Layout| layout.spine.base[1];
    assert!(hips(&bull) > hips(&neutral) && hips(&neutral) > hips(&cow));
    let ratio = hips(&bull) * 100 / hips(&cow);
    assert!((115..=122).contains(&ratio), "{ratio}");
    // Cows' horns a little longer, even though they are smaller.
    assert!(cow.parts.horn_length > bull.parts.horn_length);
    assert_eq!(bull.parts.udder, 0);
    assert!(neutral.parts.fringe > 0 && neutral.parts.tail_hair > 0);
}

#[test]
fn only_kinds_with_sexes_change_by_gender() {
    for kind in CreatureKind::ALL {
        if Beast::of(kind).is_some_and(|beast| beast.sexes.is_some()) {
            continue;
        }
        for gender in [Gender::Male, Gender::Female] {
            assert_eq!(
                Layout::of_gender(kind, gender),
                Layout::of(kind),
                "{kind:?}"
            );
        }
    }
}
