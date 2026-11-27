use super::super::*;
use crate::lang::{Lang, Language};
use crate::skill::{SkillId, Skills};
use crate::vitals::Points;

fn skills_at(path: &str, percent: u32) -> Skills {
    let mut skills = Skills::default();
    skills.set_percent(SkillId::find(path).unwrap(), percent);
    skills
}

fn texts(spell: &Spell, skills: Option<&Skills>) -> Vec<String> {
    let lang = Lang::new(Language::English);
    let lines = tip_lines(spell, skills, &lang);
    lines.into_iter().map(|line| line.text).collect()
}

#[test]
fn heal_says_its_cost_and_what_it_restores() {
    let spell = Spell::new(vec![Step::effects(&[(Effect::Heal, 2)])]);
    let tip = spell_tip(&spell, None);
    assert_eq!(tip.cost, spell_cost(&spell, None));
    let StepTip::Effects(outcomes) = &tip.steps[0] else {
        panic!("an effect step");
    };
    assert_eq!(outcomes[0].count, 2);
    assert_eq!(outcomes[0].facts, vec![Fact::Heals(heal_restores(2, None))]);
    let lines = texts(&spell, None);
    let cost = spell_cost(&spell, None).shown();
    assert_eq!(lines[0], format!("Costs {cost} Mana"));
    assert_eq!(lines[1], "1. On you:");
    let healed = heal_restores(2, None).shown();
    assert_eq!(lines[2], format!("Heal ×2: heals {healed} Health"));
    assert!(lines.last().unwrap().contains("before the target resists"));
}

#[test]
fn harm_is_the_casters_now_before_resisting() {
    let fire = Outcome::Part(Part::plain(Effect::Fire));
    let novice = facts(fire, 1, Points::ZERO, None);
    let skilled = skills_at("magic/arcane/fire", 100);
    let master = facts(fire, 1, Points::ZERO, Some(&skilled));
    assert_eq!(novice[0], Fact::Harms(harm_before_resisting(fire, 1, None)));
    let (Fact::Harms(less), Fact::Harms(more)) = (novice[0], master[0]) else {
        panic!("harm first");
    };
    assert!(more > less);
    assert!(novice.contains(&Fact::Dries) && novice.contains(&Fact::LightsFires));
}

#[test]
fn wheres_say_how_far_they_reach() {
    assert_eq!(
        reach(Where::Missile, 1),
        Reach::Missile {
            cells: 8,
            speed: 16
        }
    );
    assert_eq!(reach(Where::Touch, 1), Reach::Touch { degrees: 90 });
    assert_eq!(reach(Where::Surround, 2), Reach::Surround { cells: 2 });
    let parts = [
        WherePart {
            place: Where::Missile,
            count: 1,
        },
        WherePart {
            place: Where::Surround,
            count: 1,
        },
    ];
    let filters = Filters {
        enemy: true,
        ..Filters::NONE
    };
    let first = Step::merged(&parts, filters).unwrap();
    let spell = Spell::new(vec![first, Step::effects(&[(Effect::Fire, 1)])]);
    let lines = texts(&spell, None);
    assert_eq!(
        lines[1],
        "1. Missile: up to 8 cells, 16 cells a second, the first in its way, \
         then Surround: everyone within a cell round"
    );
    assert_eq!(lines[2], "Filters: Enemy");
}

#[test]
fn every_outcome_is_worded() {
    let mut steps: Vec<Step> = Effect::ALL
        .iter()
        .filter(|&&effect| effect != Effect::Void)
        .flat_map(|&effect| {
            [
                Step::effects(&[(effect, 1)]),
                Step::effects(&[(Effect::Void, 1), (effect, 3)]),
            ]
        })
        .collect();
    steps.push(Step::effects(&[(Effect::Fire, 1), (Effect::Water, 1)]));
    for place in Where::ALL {
        steps.push(Step::at(place, 1));
        steps.push(Step::at(place, 3));
    }
    let skills = skills_at("magic", 40);
    for step in steps {
        let spell = Spell::new(vec![step]);
        for line in texts(&spell, Some(&skills)) {
            let broken = line.contains('{') || line.contains("spell-") || line.ends_with(": ");
            assert!(!broken, "{line}");
        }
    }
}

#[test]
fn knocks_and_poison_in_tenths() {
    let air = Outcome::Part(Part::plain(Effect::Air));
    let spell = Spell::new(vec![Step::effects(&[(Effect::Air, 1)])]);
    assert_eq!(
        facts(air, 1, Points::ZERO, None),
        vec![Fact::Knocks {
            milli_cells: 1000,
            away: true
        }]
    );
    assert_eq!(texts(&spell, None)[2], "Wind: knocks back 1 cell");
    let poison = Spell::new(vec![Step::effects(&[(Effect::Poison, 1)])]);
    let per = poison_per_second(None).milli();
    let shown = texts(&poison, None)[2].clone();
    assert!(shown.starts_with("Poison: "), "{shown}");
    assert!(shown.contains("harm a second for 10 s"), "{shown}");
    assert!(per > 0);
}
