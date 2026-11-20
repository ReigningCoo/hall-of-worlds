//! The Alchemy rules: chance and steps, the Mortar matching brewing, the
//! Retort and dilution at the end, quality, luck, learning and the preview.

use std::collections::{BTreeMap, BTreeSet};

use super::*;
use crate::brewing::{self, BrewSkills, GroundHerb};
use crate::fungus::FungusKnowledge;
use crate::herb::{Effect, HerbDef, HerbKnowledge, Herbs, Stat};
use crate::item::Quality;
use crate::potion::{Contents, Dose, Liquid, Oil, Water};
use crate::skill::{SkillId, Skills};

/// Skills at whole percents: Alchemy, Herbalism, and every sub-skill.
fn skills(alchemy: u32, herbalism: u32, sub: u32) -> AlchemySkills {
    AlchemySkills {
        brew: BrewSkills {
            alchemy: alchemy * 100,
            herbalism: herbalism * 100,
        },
        sub: [sub * 100; 6],
    }
}

fn herb(key: &str, effects: &[(Stat, i8)]) -> HerbDef {
    let mut def = Herbs::built_in().iter().next().unwrap().clone();
    def.key = key.into();
    def.effects = effects
        .iter()
        .map(|&(stat, change)| Effect { stat, change })
        .collect();
    def
}

fn common(def: &HerbDef) -> GroundHerb<'_> {
    GroundHerb::herb(def, Quality::Common)
}

fn mortar<'a>(herbs: &[&'a HerbDef], solvent: Solvent) -> Step<'a> {
    Step::Mortar {
        herbs: herbs.iter().map(|def| common(def)).collect(),
        solvent,
    }
}

fn retort<'a>(liquids: &[Liquid]) -> Step<'a> {
    Step::Retort {
        liquids: liquids.to_vec(),
        solid: None,
    }
}

fn process(steps: Vec<Step<'_>>) -> Process<'_> {
    Process {
        source: None,
        steps,
    }
}

fn doses(contents: &Contents) -> Vec<(Stat, i16)> {
    contents.doses().iter().map(|d| (d.stat, d.step)).collect()
}

const WATER: Liquid = Liquid::Water(Water::Plain);

/// Four herbs raising Strength strongly, and one lowering Will a little.
fn strong() -> Vec<HerbDef> {
    let mut herbs: Vec<HerbDef> = (0..4)
        .map(|n| herb(&format!("s{n}"), &[(Stat::Strength, 3)]))
        .collect();
    herbs.push(herb("w", &[(Stat::Will, -2)]));
    herbs
}

#[test]
fn steps_and_chance() {
    let allowed: Vec<u32> = [0, 1, 2, 4, 5, 100].map(steps_allowed).to_vec();
    assert_eq!(allowed, [1, 1, 2, 2, 3, 10]);
    assert_eq!(chance(0, 1, 0), CERTAIN);
    assert_eq!(chance(0, 2, 0), CERTAIN / 2);
    assert_eq!(chance(0, 3, 1), CERTAIN / 8);
    assert_eq!(chance(100, 10, 0), 2 * CERTAIN);
    assert_eq!(chance(100, 11, 0), CERTAIN);
    assert_eq!(chance(50, 1, 0), 15_000);
    assert_eq!(chance(0, 40, 0), 0);
    assert!(succeeds(CERTAIN, CERTAIN - 1));
    assert!(!succeeds(CERTAIN / 2, CERTAIN / 2));
}

#[test]
fn sub_skills_are_under_alchemy() {
    let alchemy = SkillId::find(crate::ability::ALCHEMY).unwrap();
    for apparatus in Apparatus::ALL {
        let id = SkillId::find(apparatus.sub_skill()).unwrap();
        assert_eq!(id.parent(), Some(alchemy), "{apparatus:?}");
        assert_eq!(Apparatus::ALL[apparatus.index()], apparatus);
    }
    let mut known = Skills::default();
    known.practise(SkillId::find(INFUSING).unwrap(), 1000);
    assert!(known.practice(alchemy) > 0);
    assert!(AlchemySkills::of(&known).sub[Apparatus::Retort.index()] > 0);
}

#[test]
fn the_mortar_keeps_brewings_values() {
    let herbs = strong();
    let refs: Vec<&HerbDef> = herbs[..4].iter().collect();
    let ground: Vec<GroundHerb> = herbs[..4].iter().map(common).collect();
    for (alchemy, herbalism) in [(0, 0), (30, 10), (100, 100)] {
        let s = skills(alchemy, herbalism, 0);
        let old = brewing::grind(&ground, Oil::Grain, 1, s.brew).unwrap();
        let p = process(vec![mortar(&refs, Solvent::Oil(Oil::Grain))]);
        let planned = plan(&p, s).unwrap();
        let ointment = Product::Mixture {
            form: Form::Ointment,
            contents: old.contents.clone(),
        };
        assert_eq!(planned.product, ointment, "at {alchemy}%");
    }
}

#[test]
fn herbs_alone_make_a_paste_and_extra_kinds_halve() {
    let herbs = strong();
    let refs: Vec<&HerbDef> = herbs.iter().collect();
    let planned = plan(&process(vec![mortar(&refs, Solvent::Dry)]), skills(0, 0, 0)).unwrap();
    // Untrained: a step a herb, capped at 4; one kind over the limit of 4.
    let paste = Product::Mixture {
        form: Form::Paste,
        contents: Contents::new([
            Dose {
                stat: Stat::Strength,
                step: 4,
            },
            Dose {
                stat: Stat::Will,
                step: -1,
            },
        ]),
    };
    assert_eq!(planned.product, paste);
    assert_eq!(planned.chance, CERTAIN / 2);
    assert_eq!(planned.practised, [Apparatus::Mortar]);
    let same = [&herbs[0], &herbs[0]];
    let refused = plan(&process(vec![mortar(&same, Solvent::Dry)]), skills(0, 0, 0));
    assert_eq!(refused, Err(Refused::SameHerb));
}

/// The same as brewing, but for the quarter Moon and Cave Water add: exact in
/// hundredths, rounded once at the end (so half a step above brewing at most).
#[test]
fn a_retort_with_one_water_brews_as_before() {
    let herbs = strong();
    let refs: Vec<&HerbDef> = herbs[..4].iter().collect();
    let ground: Vec<GroundHerb> = herbs[..4].iter().map(common).collect();
    let s = skills(9, 0, 0);
    let waters = [Water::Plain, Water::Salt, Water::Rain];
    for oil in Oil::ALL {
        let ointment = brewing::grind(&ground, oil, 1, s.brew).unwrap().contents;
        for water in waters {
            let steps = vec![
                mortar(&refs, Solvent::Oil(oil)),
                retort(&[Liquid::Water(water)]),
            ];
            let planned = plan(&process(steps), s).unwrap();
            let expected = match brewing::brew(&ointment, water) {
                Some(contents) => Product::Potion {
                    contents,
                    bottles: 1,
                },
                None => Product::Nothing,
            };
            assert_eq!(planned.product, expected, "{oil:?} {water:?}");
        }
    }
}

#[test]
fn liquids_dilute_only_at_the_end() {
    // Alchemy 25%, Herbalism 100%: Strength 4 × 3 = 12, so 1200 hundredths.
    let herbs = strong();
    let refs: Vec<&HerbDef> = herbs[..4].iter().collect();
    let s = skills(25, 100, 0);
    let three = vec![
        mortar(&refs, Solvent::Dry),
        retort(&[Liquid::Oil(Oil::Grain)]),
        retort(&[WATER, WATER]),
    ];
    // 1200 → 1500 by the Grain Oil at full amount, then 3 units: 5 a bottle.
    let planned = plan(&process(three), s).unwrap();
    let potion = Product::Potion {
        contents: Contents::new([Dose {
            stat: Stat::Strength,
            step: 5,
        }]),
        bottles: 3,
    };
    assert_eq!(planned.product, potion);
    assert_eq!(planned.practised.len(), 3);
    // 1500 over 5 units is 3 a bottle: under the threshold once diluted.
    let five = vec![
        mortar(&refs, Solvent::Dry),
        retort(&[Liquid::Oil(Oil::Grain), WATER, WATER, WATER, WATER]),
    ];
    assert_eq!(plan(&process(five), s).unwrap().product, Product::Nothing);
    let six = vec![mortar(&refs, Solvent::Dry), retort(&[WATER; 6])];
    assert_eq!(plan(&process(six), s), Err(Refused::TooMuchLiquid));
}

#[test]
fn dilution_rounds_once_halves_away_from_zero() {
    let mut mix = Mix::default();
    mix.0[Stat::Strength as usize] = 1000;
    mix.0[Stat::Will as usize] = -500;
    let third = mix.diluted(3);
    assert_eq!(doses(&third), [(Stat::Strength, 3), (Stat::Will, -2)]);
    assert_eq!(
        doses(&mix.diluted(4)),
        [(Stat::Strength, 3), (Stat::Will, -1)]
    );
    assert_eq!(boosted(200), 300);
    assert_eq!(boosted(-1200), -1500);
}

#[test]
fn infusing_is_practised_only_for_a_mixture() {
    let liquids_alone = process(vec![retort(&[WATER]), retort(&[WATER])]);
    let planned = plan(&liquids_alone, skills(100, 0, 0)).unwrap();
    assert!(planned.practised.is_empty());
    assert_eq!(planned.product, Product::Nothing);
    let stored = Process {
        source: Some(Stored {
            form: Form::Ointment,
            contents: Contents::new([Dose {
                stat: Stat::Aura,
                step: 6,
            }]),
        }),
        steps: vec![retort(&[WATER])],
    };
    assert_eq!(
        plan(&stored, skills(0, 0, 0)).unwrap().practised,
        [Apparatus::Retort]
    );
}

#[test]
fn refusals() {
    let herbs = strong();
    let refs: Vec<&HerbDef> = herbs.iter().collect();
    let s = skills(0, 0, 0);
    assert_eq!(plan(&process(vec![]), s), Err(Refused::Empty));
    let late = vec![retort(&[WATER]), mortar(&refs, Solvent::Dry)];
    assert_eq!(plan(&process(late), s), Err(Refused::MortarNotFirst));
    assert_eq!(plan(&process(vec![retort(&[])]), s), Err(Refused::NoLiquid));
    let juice = Liquid::Juice("apple".into());
    assert_eq!(
        plan(&process(vec![retort(&[juice])]), s),
        Err(Refused::Liquid)
    );
    let pelican = vec![Step::Pelican];
    let refused = Err(Refused::NothingToCirculate);
    assert_eq!(plan(&process(pelican), s), refused);
}

#[test]
fn quality_from_purity_sub_skills_age_and_luck() {
    assert_eq!(
        skill_steps(
            10,
            &[Apparatus::Mortar, Apparatus::Retort],
            skills(0, 0, 100)
        ),
        4
    );
    assert_eq!(skill_steps(2, &[Apparatus::Mortar], skills(0, 0, 100)), 0);
    assert_eq!(skill_steps(5, &[Apparatus::Mortar; 5], skills(0, 0, 8)), 1);
    // Bonus only when certain, a step less when old, Poor to Legendary.
    assert_eq!(
        least_quality(Quality::Common, CERTAIN, 2, false),
        Quality::Rare
    );
    assert_eq!(
        least_quality(Quality::Common, CERTAIN - 1, 2, false),
        Quality::Common
    );
    assert_eq!(
        least_quality(Quality::Poor, CERTAIN, 0, true),
        Quality::Poor
    );
    assert_eq!(
        least_quality(Quality::Epic, CERTAIN, 4, false),
        Quality::Legendary
    );
    assert_eq!((luck_steps(0), luck_steps(9), luck_steps(10)), (2, 2, 1));
    assert_eq!((luck_steps(109), luck_steps(110)), (1, 0));
}

#[test]
fn an_old_source_loses_a_step_and_failing_makes_nothing() {
    let contents = Contents::new([Dose {
        stat: Stat::Aura,
        step: 12,
    }]);
    let source = |form| Stored {
        form,
        contents: contents.clone(),
    };
    let run = |form| {
        let p = Process {
            source: Some(source(form)),
            steps: vec![retort(&[WATER])],
        };
        plan(&p, skills(0, 0, 0)).unwrap()
    };
    // 12 ÷ 3 is 4: Common; old, Poor; a Powder is never old.
    assert_eq!(run(Form::Paste).quality, Some(Quality::Poor));
    assert_eq!(run(Form::Powder).quality, Some(Quality::Common));
    let planned = run(Form::Ointment);
    assert_eq!(outcome(&planned, CERTAIN, 0), Outcome::Failed);
    let lucky = outcome(&planned, 0, 0);
    let made = Outcome::Made {
        product: planned.product.clone(),
        quality: Some(Quality::Uncommon),
    };
    assert_eq!(lucky, made);
}

/// What a character knowing only these herbs knows (no fungi).
pub(super) fn knowing(herbs: &HerbKnowledge) -> Known<'_> {
    static NO_FUNGI: FungusKnowledge = FungusKnowledge {
        poisons: BTreeSet::new(),
        names: BTreeSet::new(),
        effects: BTreeMap::new(),
    };
    Known {
        herbs,
        fungi: &NO_FUNGI,
    }
}

#[test]
fn preview_and_lesson() {
    let a = herb("a", &[(Stat::Strength, 3), (Stat::Will, -1)]);
    let b = herb("b", &[(Stat::Strength, 1), (Stat::Aura, 3)]);
    let c = herb("c", &[(Stat::Strength, 3)]);
    let p = process(vec![mortar(&[&a, &b, &c], Solvent::Dry)]);
    // Alchemy 9%: 4 steps a herb at most, 16 a stat; Herbalism 100% cuts Will.
    let planned = plan(&p, skills(9, 100, 0)).unwrap();
    let mut known = HerbKnowledge::default();
    let seen = preview(&planned, knowing(&known));
    assert!(seen.iter().all(|s| *s == Seen::Unknown));
    // The strongest: Strength 7, from a (3), b (1) and c (3): a first on the tie.
    let first = lesson(&planned, knowing(&known)).unwrap();
    assert_eq!((first.herb.as_str(), first.effect), ("a", 0));
    known.learn_effect("a", 0);
    let next = lesson(&planned, knowing(&known)).unwrap();
    assert_eq!((next.herb.as_str(), next.effect), ("c", 0));
    known.learn_effect("c", 0);
    known.learn_effect("b", 0);
    let strength = Dose {
        stat: Stat::Strength,
        step: 7,
    };
    assert_eq!(preview(&planned, knowing(&known))[0], Seen::Known(strength));
    // Then Aura (3).
    let aura = lesson(&planned, knowing(&known)).unwrap();
    assert_eq!((aura.herb.as_str(), aura.effect), ("b", 1));
}

#[test]
fn moon_water_adds_an_exact_quarter() {
    // Herbalism 0: each herb 3 × 80% is 2, so Strength 8, 10 with Grain Oil.
    let herbs = strong();
    let refs: Vec<&HerbDef> = herbs[..4].iter().collect();
    let steps = vec![
        mortar(&refs, Solvent::Oil(Oil::Grain)),
        retort(&[Liquid::Water(Water::Moon)]),
    ];
    // 1000 → 1250 → 13 (brewing's whole steps gave 12).
    let planned = plan(&process(steps), skills(9, 0, 0)).unwrap();
    assert_eq!(doses(planned.product.contents()), [(Stat::Strength, 13)]);
}
