//! Harm from handling (fungi plan PR 9): touch kept off by anything on the
//! hands, breath by something over the mouth.

use super::tests::EATER;
use super::*;

fn def(key: &str) -> &'static FungusDef {
    Fungi::built_in().get(key).expect("a built-in fungus")
}

const BARE: Guarded = Guarded {
    hands: false,
    mouth: false,
};

#[test]
fn touch_is_kept_off_by_the_hands_covered() {
    let oakmoss = def("oakmoss");
    assert_eq!(oakmoss.poison.as_ref().unwrap().guard(), Some(Guard::Hands));
    assert!(!handling(oakmoss, BARE, 5, EATER).is_empty());
    let gloved = Guarded {
        hands: true,
        mouth: false,
    };
    assert!(handling(oakmoss, gloved, 5, EATER).is_empty());
}

#[test]
fn breath_is_kept_off_by_the_mouth_covered() {
    for key in [
        "common_puffball",
        "cellar_cup",
        "split_gill",
        "sleepdust_puffball",
    ] {
        let fungus = def(key);
        assert_eq!(fungus.poison.as_ref().unwrap().guard(), Some(Guard::Mouth));
        assert!(!handling(fungus, BARE, 5, EATER).is_empty(), "{key}");
        let gloved = Guarded {
            hands: true,
            mouth: false,
        };
        assert!(!handling(fungus, gloved, 5, EATER).is_empty(), "{key}");
        let masked = Guarded {
            hands: false,
            mouth: true,
        };
        assert!(handling(fungus, masked, 5, EATER).is_empty(), "{key}");
    }
}

#[test]
fn handling_harms_as_eating_would_from_its_onset() {
    let puffball = def("common_puffball");
    let poison = puffball.poison.as_ref().unwrap();
    assert_eq!(
        handling(puffball, BARE, 5, EATER),
        poisoning(puffball, poison, 5, EATER)
    );
}

#[test]
fn poisons_only_eating_brings_on_are_safe_to_handle() {
    for key in ["fly_agaric", "death_cap", "common_ink_cap", "chanterelle"] {
        let Some(fungus) = Fungi::built_in().get(key) else {
            continue;
        };
        assert!(handling(fungus, BARE, 5, EATER).is_empty(), "{key}");
    }
}
