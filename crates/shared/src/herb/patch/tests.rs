use super::*;
use crate::herb::{Growth, Herbs, Regrow};

fn def(growth: Growth, regrow: Regrow) -> HerbDef {
    let mut def = Herbs::built_in().iter().next().unwrap().clone();
    def.growth = growth;
    def.regrow = regrow;
    def
}

fn patch(phase: u32, picked: Option<u64>) -> HerbPatch {
    HerbPatch {
        herb: "x".into(),
        phase,
        picked,
    }
}

/// Fast herbs take two game days for all four stages: half a day each.
#[test]
fn a_stage_takes_its_share_of_the_growth() {
    for growth in [Growth::Fast, Growth::Medium, Growth::Slow, Growth::VerySlow] {
        let def = def(growth, Regrow::Cut);
        let days = u64::from(growth.days());
        assert_eq!(stage_minutes(&def) * 4, days * MINUTES_PER_DAY);
    }
}

/// Unpicked, a patch grows through the stages, stays full grown as long, and
/// starts again, each from its own phase.
#[test]
fn unpicked_patches_go_round() {
    let def = def(Growth::Fast, Regrow::Cut);
    let step = stage_minutes(&def);
    let stages: Vec<u8> = (0..10)
        .map(|n| patch(0, None).stage(&def, n * step))
        .collect();
    assert_eq!(stages, [1, 2, 3, 4, 4, 4, 4, 4, 1, 2]);
    // A phase a stage and a half on is a stage ahead.
    assert_eq!(patch((step * 3 / 2) as u32, None).stage(&def, 0), 2);
    assert!(patch(0, None).can_pick(&def, 2 * step));
    assert!(!patch(0, None).can_pick(&def, step));
}

/// Harvesting starts it again from its herb's stage: cut back to 2, picked
/// back to 3, coppiced back to 1, dug up gone for a whole growth.
#[test]
fn harvesting_starts_it_again() {
    let at = 1_000_000;
    for (regrow, back) in [(Regrow::Cut, 2), (Regrow::Pick, 3), (Regrow::Coppice, 1)] {
        let def = def(Growth::Medium, regrow);
        let step = stage_minutes(&def);
        let picked = patch(77, Some(at));
        assert_eq!(picked.stage(&def, at), back);
        assert_eq!(picked.stage(&def, at + step), (back + 1).min(4));
        assert_eq!(picked.stage(&def, at + 4 * step), 4);
    }
    let def = def(Growth::Medium, Regrow::Dig);
    let step = stage_minutes(&def);
    let dug = patch(77, Some(at));
    assert_eq!(dug.stage(&def, at), 0);
    assert_eq!(dug.stage(&def, at + 4 * step - 1), 0);
    assert_eq!(dug.stage(&def, at + 4 * step), 1);
    assert!(!dug.can_pick(&def, at + 5 * step));
    assert!(dug.can_pick(&def, at + 6 * step));
}

/// Picked fruit comes back at the early stage, but can't be picked again until
/// it has grown a stage.
#[test]
fn picked_fruit_waits_a_stage() {
    let def = def(Growth::Fast, Regrow::Pick);
    let step = stage_minutes(&def);
    let picked = patch(0, Some(500));
    assert_eq!(picked.stage(&def, 500), 3);
    assert!(!picked.can_pick(&def, 500));
    assert!(!picked.can_pick(&def, 500 + step - 1));
    assert!(picked.can_pick(&def, 500 + step));
}
