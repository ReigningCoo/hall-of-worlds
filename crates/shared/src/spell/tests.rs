use super::*;
use crate::lang::{Lang, Language};
use crate::skill::{SkillId, Skills};
use crate::vitals::Points;

fn heal() -> Step {
    Step::effects(&[(Effect::Heal, 1)])
}

fn fire() -> Step {
    Step::effects(&[(Effect::Fire, 1)])
}

fn spell(steps: Vec<Step>) -> Spell {
    Spell::new(steps)
}

fn skills_at(path: &str, percent: u32) -> Skills {
    let mut skills = Skills::default();
    skills.set_percent(SkillId::find(path).unwrap(), percent);
    skills
}

/// Magic high enough for every where.
fn master() -> Skills {
    skills_at("magic", 50)
}

#[test]
fn every_effect_and_where_has_a_skill() {
    for effect in Effect::ALL {
        effect.skill();
    }
    for place in Where::ALL {
        place.skill();
        assert_eq!(Where::ALL[place.tier() as usize - 1], place);
    }
}

#[test]
fn effect_costs_follow_the_cube_root() {
    for (percent, cost) in [(0, 5000), (8, 3000), (27, 2000), (64, 1000), (100, 1000)] {
        let skills = skills_at("magic/divine/healing", percent);
        assert_eq!(effect_cost(Effect::Heal, Some(&skills)), cost, "{percent}%");
    }
    assert_eq!(effect_cost(Effect::Fire, None), 5000);
}

#[test]
fn the_plans_costs_at_no_skill() {
    let point3 = Step::at(Where::Point, 3);
    let cases = [
        (spell(vec![heal()]), 5),
        (spell(vec![Step::effects(&[(Effect::Heal, 2)])]), 11),
        (spell(vec![Step::at(Where::Missile, 1), heal()]), 10),
        (spell(vec![point3, heal()]), 85),
        (
            spell(vec![
                Step::at(Where::Missile, 1),
                heal(),
                Step::at(Where::Surround, 1),
                fire(),
            ]),
            150,
        ),
    ];
    for (spell, points) in cases {
        assert_eq!(spell_cost(&spell, None), Points::whole(points), "{spell:?}");
    }
}

#[test]
fn form_takes_off_up_to_half_never_below_one() {
    let skills = skills_at("magic/form/point", 100);
    let step = WhereStep {
        place: Where::Point,
        count: 3,
        filters: Filters::NONE,
        more: Vec::new(),
    };
    assert_eq!(where_step_cost(&step, Some(&skills)), 8500);
    let skills = skills_at("magic/form/touch", 100);
    let touch = Spell::new(vec![Step::at(Where::Touch, 1), heal()]);
    assert_eq!(spell_cost(&touch, Some(&skills)), Points::whole(5));
}

#[test]
fn filters_are_free() {
    let filters = Filters {
        enemy: true,
        ..Filters::NONE
    };
    let plain = spell(vec![Step::at(Where::Point, 2), fire()]);
    let filtered = spell(vec![Step::filtered(Where::Point, 2, filters), fire()]);
    assert_eq!(spell_cost(&plain, None), spell_cost(&filtered, None));
}

#[test]
fn wheres_unlock_in_order() {
    assert_eq!(unlocked_wheres(None), 0);
    assert_eq!(unlocked_wheres(Some(&Skills::default())), 0);
    // (Magic %, wheres): ⌊∛(% × 6)⌋.
    for (percent, count) in [(1, 1), (2, 2), (4, 2), (5, 3), (11, 4), (21, 5), (100, 5)] {
        let skills = skills_at("magic", percent);
        assert_eq!(unlocked_wheres(Some(&skills)), count, "{percent}%");
    }
    let skills = skills_at("magic", 2);
    assert!(is_unlocked(Where::Missile, Some(&skills)));
    assert!(!is_unlocked(Where::Surround, Some(&skills)));
}

#[test]
fn designs_are_checked() {
    let skills = master();
    let ok = |spell: &Spell| check(spell, Some(&skills));
    assert_eq!(ok(&spell(vec![heal()])), Ok(()));
    assert_eq!(ok(&spell(vec![])), Err(DesignError::Empty));
    assert_eq!(ok(&spell(vec![heal(); 11])), Err(DesignError::TooManySteps));
    assert_eq!(ok(&spell(vec![heal(); 10])), Ok(()));
    let touch = Step::at(Where::Touch, 1);
    assert_eq!(ok(&spell(vec![touch.clone()])), Err(DesignError::NoEffect));
    assert_eq!(
        ok(&spell(vec![heal(), touch.clone()])),
        Err(DesignError::WhereAtEnd)
    );
    assert_eq!(
        ok(&spell(vec![Step::effects(&[])])),
        Err(DesignError::EmptyStep)
    );
    assert_eq!(
        ok(&spell(vec![Step::at(Where::Beam, 0), heal()])),
        Err(DesignError::ZeroCount)
    );
    let friend = Filters {
        friend: true,
        ..Filters::NONE
    };
    let first = Step::filtered(Where::Surround, 1, friend);
    assert_eq!(ok(&spell(vec![first.clone(), heal()])), Ok(()));
    assert_eq!(
        ok(&spell(vec![touch, heal(), first, heal()])),
        Err(DesignError::FilterNotFirst)
    );
    let novice = skills_at("magic", 2);
    let beam = spell(vec![Step::at(Where::Beam, 1), fire()]);
    assert_eq!(
        check(&beam, Some(&novice)),
        Err(DesignError::Locked(Where::Beam))
    );
}

#[test]
fn void_reverses_what_follows_and_a_second_puts_it_back() {
    let step = EffectStep {
        effects: vec![
            (Effect::Heal, 1),
            (Effect::Void, 1),
            (Effect::Fire, 2),
            (Effect::Void, 1),
            (Effect::Heal, 1),
        ],
    };
    assert_eq!(
        parts(&step),
        vec![
            (Part::plain(Effect::Heal), 2),
            (Part::reversed(Effect::Fire), 2)
        ]
    );
    let twice = EffectStep {
        effects: vec![(Effect::Void, 2), (Effect::Fire, 1)],
    };
    assert_eq!(parts(&twice), vec![(Part::plain(Effect::Fire), 1)]);
    let alone = EffectStep {
        effects: vec![(Effect::Void, 1)],
    };
    assert!(parts(&alone).is_empty());
}

#[test]
fn fire_and_water_make_steam() {
    let step = EffectStep {
        effects: vec![(Effect::Fire, 3), (Effect::Water, 1), (Effect::Light, 1)],
    };
    assert_eq!(
        outcomes(&step),
        vec![
            (Outcome::Mixture(Mixture::Steam), 1),
            (Outcome::Part(Part::plain(Effect::Fire)), 2),
            (Outcome::Part(Part::plain(Effect::Light)), 1),
        ]
    );
    // Cold and water aren't in the table: each does its own.
    let cold = EffectStep {
        effects: vec![(Effect::Water, 1), (Effect::Void, 1), (Effect::Fire, 1)],
    };
    assert_eq!(outcomes(&cold).len(), 2);
}

#[test]
fn automatic_casting_by_aims() {
    let missile = Step::at(Where::Missile, 1);
    assert_eq!(auto_cast(&spell(vec![heal()])), AutoCast::WhenLow);
    assert_eq!(
        auto_cast(&spell(vec![missile.clone(), fire()])),
        AutoCast::AtEnemy
    );
    let harm = Step::effects(&[(Effect::Void, 1), (Effect::Heal, 1)]);
    assert_eq!(auto_cast(&spell(vec![missile, harm])), AutoCast::AtEnemy);
    assert_eq!(auto_cast(&spell(vec![heal(), fire()])), AutoCast::ByHand);
    let light = Step::effects(&[(Effect::Light, 1)]);
    assert_eq!(auto_cast(&spell(vec![light])), AutoCast::ByHand);
}

#[test]
fn every_outcome_has_words() {
    let lang = Lang::new(Language::English);
    for effect in Effect::ALL {
        for part in [Part::plain(effect), Part::reversed(effect)] {
            let id = format!("spell-part-{}", part.key());
            assert!(lang.has(&id), "{id}");
            assert!(lang.has(&format!("{id}.adjective")), "{id}");
        }
    }
    for row in COMBINATIONS {
        assert!(lang.has(&format!("spell-part-{}", row.makes.key())));
    }
}

#[test]
fn automatic_names() {
    let lang = Lang::new(Language::English);
    let name = |steps| auto_name(&spell(steps), &lang);
    assert_eq!(name(vec![heal()]), "Heal");
    assert_eq!(
        name(vec![Step::at(Where::Missile, 1), heal()]),
        "Healing Missile"
    );
    assert_eq!(
        name(vec![Step::at(Where::Surround, 1), fire()]),
        "Ball of Fire"
    );
    let cold = Step::effects(&[(Effect::Void, 1), (Effect::Fire, 1)]);
    assert_eq!(
        name(vec![Step::at(Where::Touch, 1), cold]),
        "Chilling Touch"
    );
    let steam = Step::effects(&[(Effect::Fire, 1), (Effect::Water, 1)]);
    assert_eq!(name(vec![Step::at(Where::Beam, 2), steam]), "Scalding Beam");
    assert_eq!(name(vec![]), "Spell");
}
