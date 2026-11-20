use super::super::super::Layout;
use super::super::castrates::has_castrate;
use crate::creature::CreatureKind;
use crate::gender::Gender;
use crate::livestock::{Livestock, Stage};

fn body(kind: CreatureKind, gender: Gender, stage: Stage) -> Layout {
    Layout::of_stage(kind, gender, stage).expect("a body")
}

/// Its head's length against its back's.
fn head_share(layout: &Layout) -> i32 {
    layout.head.length * 1000 / layout.spine.neck_base
}

#[test]
fn grown_males_and_females_are_as_their_sex() {
    for kind in [
        CreatureKind::Cattle,
        CreatureKind::Chicken,
        CreatureKind::Human,
    ] {
        for gender in [Gender::Male, Gender::Female] {
            assert_eq!(
                Layout::of_stage(kind, gender, Stage::Adult),
                Layout::of_gender(kind, gender)
            );
        }
    }
}

#[test]
fn people_look_the_same_at_any_stage() {
    for stage in [Stage::Young, Stage::Juvenile] {
        assert_eq!(
            Layout::of_stage(CreatureKind::Human, Gender::Female, stage),
            Layout::of_gender(CreatureKind::Human, Gender::Female)
        );
    }
}

#[test]
fn every_kept_animal_grows_up() {
    for def in Livestock::built_in().iter() {
        let Some(kind) = def.kind else { continue };
        let young = body(kind, Gender::Female, Stage::Young);
        let juvenile = body(kind, Gender::Female, Stage::Juvenile);
        let grown = body(kind, Gender::Female, Stage::Adult);
        let height = |layout: &Layout| layout.spine.base[1];
        assert!(height(&young) < height(&juvenile), "{kind:?}");
        assert!(height(&juvenile) < height(&grown), "{kind:?}");
        assert!(head_share(&young) > head_share(&grown), "{kind:?}");
        assert_eq!(young.parts.udder, 0, "{kind:?}");
        assert_eq!(young.parts.tusks, 0, "{kind:?}");
        assert!(
            young.parts.horn_length <= grown.parts.horn_length / 5,
            "{kind:?}"
        );
    }
}

#[test]
fn castrates_have_their_own_look_without_an_udder() {
    let cattle = CreatureKind::Cattle;
    let ox = body(cattle, Gender::Neutral, Stage::Adult);
    assert_ne!(Some(ox), Layout::of(cattle));
    assert_eq!(ox.parts.udder, 0);
    // An ox stands taller than a cow.
    let cow = body(cattle, Gender::Female, Stage::Adult);
    assert!(ox.spine.base[1] > cow.spine.base[1]);
    // A capon's comb is smaller than a cock's.
    let capon = body(CreatureKind::Chicken, Gender::Neutral, Stage::Adult);
    let cock = body(CreatureKind::Chicken, Gender::Male, Stage::Adult);
    assert!(capon.parts.comb < cock.parts.comb);
}

#[test]
fn kinds_without_a_castrate_look_keep_the_neutral_body() {
    for kind in [
        CreatureKind::Donkey,
        CreatureKind::Llama,
        CreatureKind::Duck,
    ] {
        assert!(!has_castrate(kind));
        assert_eq!(
            Layout::of_stage(kind, Gender::Neutral, Stage::Adult),
            Layout::of(kind)
        );
    }
}
