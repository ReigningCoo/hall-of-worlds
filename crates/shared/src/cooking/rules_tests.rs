use super::*;
use crate::alchemy::{CERTAIN, LUCKY_TWICE};
use crate::item::{ItemKind, Quality};
use crate::skill::{SkillId, Skills};

fn food(source: Source, key: &str, grams: u32) -> Portion {
    Portion::Food {
        source,
        key: key.into(),
        grams,
        quality: None,
    }
}

fn of(source: Source, key: &str, grams: u32, quality: Quality) -> Portion {
    Portion::Food {
        source,
        key: key.into(),
        grams,
        quality: Some(quality),
    }
}

fn water() -> Portion {
    Portion::Water { ml: 1000 }
}

fn skilled(percent: u32) -> CookingSkills {
    CookingSkills {
        cooking: percent * 100,
        sub: [percent * 100; Tool::ALL.len()],
    }
}

/// Cut beef, carrot and onion, then a Cauldron of water: a stew.
fn stew(heat: Heat) -> Process {
    Process {
        rows: 2,
        sources: Vec::new(),
        steps: vec![
            Step::at(
                Tool::Knife,
                0,
                0,
                vec![
                    food(Source::Meat, "cattle", 500),
                    food(Source::Vegetable, "carrot", 200),
                    food(Source::Vegetable, "onion", 100),
                ],
            ),
            Step::at(Tool::Cauldron, 1, 0, vec![water()]),
        ],
        heat,
    }
}

fn ingredients() -> &'static Ingredients {
    Ingredients::built_in()
}

#[test]
fn every_tool_has_its_sub_skill_under_cooking() {
    assert!(SkillId::find(COOKING).is_some());
    for tool in Tool::ALL {
        let id = SkillId::find(tool.sub_skill()).unwrap();
        assert_eq!(id.parent().map(|p| p.path()), Some(COOKING));
    }
    let mut skills = Skills::default();
    skills.set_percent(SkillId::find(FRYING).unwrap(), 27);
    assert_eq!(CookingSkills::of(&skills).sub_root(Tool::Pan), 3);
}

#[test]
fn tools_from_items() {
    assert_eq!(Tool::of_item(ItemKind::Dagger), Some(Tool::Knife));
    assert_eq!(Tool::of_item(ItemKind::Knife), Some(Tool::Knife));
    assert_eq!(Tool::of_item(ItemKind::Cauldron), Some(Tool::Cauldron));
    assert_eq!(Tool::of_item(ItemKind::MortarAndPestle), Some(Tool::Mortar));
    assert_eq!(Tool::of_item(ItemKind::Retort), None);
    let heated: Vec<Tool> = Tool::ALL.into_iter().filter(|t| t.needs_heat()).collect();
    assert_eq!(heated, vec![Tool::Pan, Tool::Cauldron, Tool::Oven]);
}

#[test]
fn chance_as_alchemy_with_a_join_a_step_and_a_half() {
    assert_eq!(chance(0, 2), CERTAIN);
    assert_eq!(chance(0, 4), CERTAIN / 2);
    // One step allowed untrained: a join alone is half a step over.
    assert_eq!(chance(0, 3), CERTAIN / 2);
    // 4% allows 2: two steps and a join is 3.5, so two halvings.
    assert_eq!(chance(4, 7), (CERTAIN + 400) / 4);
    assert_eq!(chance(100, 20), 2 * CERTAIN);
}

#[test]
fn a_stew_worked_out() {
    let planned = plan(&stew(Heat::Hearth), ingredients(), dishes(), skilled(0)).unwrap();
    assert_eq!(planned.chance, CERTAIN / 2);
    assert_eq!(planned.practised, vec![Tool::Knife, Tool::Cauldron]);
    let [output] = planned.outputs.as_slice() else {
        panic!("one output");
    };
    assert_eq!(output.step, 1);
    assert_eq!(output.portions.len(), 4);
    assert_eq!(output.history.halves, 4);
    // Protein, Fat, Vegetable, Aromatic (and Sweet) counted, nothing past
    // its share.
    assert_eq!(output.balance, ROLES_COUNTED);
    assert_eq!(output.quality, Quality::Rare);
}

#[test]
fn heat_and_liquid() {
    let cold = plan(&stew(Heat::Cold), ingredients(), dishes(), skilled(0));
    assert_eq!(cold, Err(Refused::NoHeat { step: 1 }));
    let mut dry = stew(Heat::Fire);
    dry.steps[1].portions.clear();
    assert_eq!(
        plan(&dry, ingredients(), dishes(), skilled(0)),
        Err(Refused::NoLiquid { step: 1 })
    );
    dry.steps[0]
        .portions
        .push(food(Source::Milk, "cattle", 300));
    assert!(plan(&dry, ingredients(), dishes(), skilled(0)).is_ok());
    let mut unknown = stew(Heat::Fire);
    unknown.steps[0]
        .portions
        .push(food(Source::Meat, "dragon", 10));
    let refused = plan(&unknown, ingredients(), dishes(), skilled(0));
    assert_eq!(refused, Err(Refused::UnknownIngredient { step: 0 }));
}

#[test]
fn capped_at_rare_away_from_a_hearth() {
    let mut process = stew(Heat::Hearth);
    for portion in &mut process.steps[0].portions {
        if let Portion::Food { quality, .. } = portion {
            *quality = Some(Quality::Epic);
        }
    }
    let at_hearth = plan(&process, ingredients(), dishes(), skilled(100)).unwrap();
    assert_eq!(at_hearth.outputs[0].quality, Quality::Legendary);
    process.heat = Heat::Fire;
    let by_fire = plan(&process, ingredients(), dishes(), skilled(100)).unwrap();
    assert_eq!(by_fire.outputs[0].quality, AWAY_CAP);
    let Outcome::Made { qualities } = outcome(&by_fire, 0, 0) else {
        panic!("made");
    };
    assert_eq!(qualities, vec![AWAY_CAP]);
}

#[test]
fn ingredient_quality_weighed_by_grams() {
    let portions = [
        of(Source::Vegetable, "carrot", 300, Quality::Poor),
        of(Source::Vegetable, "onion", 100, Quality::Epic),
        water(),
    ];
    // (0 × 300 + 4 × 100) ÷ 400 = 1.
    assert_eq!(ingredient_quality(&portions), Quality::Common);
    assert_eq!(ingredient_quality(&[water()]), Quality::Common);
}

#[test]
fn balance_loses_for_excess_and_bitterness() {
    // An egg alone is all Protein, and 40 points past its 60% share.
    let egg = [food(Source::Egg, "chicken", 100)];
    assert_eq!(balance(&egg, ingredients(), &Wants::default()), 1 - 2);
    // Basil is at most a few percent: half the dish is far past it.
    let basil = [
        food(Source::Egg, "chicken", 100),
        food(Source::Herb, "basil", 100),
    ];
    let def = ingredients().get(Source::Herb, "basil").unwrap();
    let past = 50 - u32::from(def.max_share);
    assert_eq!(
        balance(&basil, ingredients(), &Wants::default()),
        2 - past.div_ceil(EXCESS_STEP) as i32
    );
    let mut mixed = vec![
        food(Source::Egg, "chicken", 500),
        food(Source::Vegetable, "carrot", 400),
    ];
    assert_eq!(
        balance(&mixed, ingredients(), &Wants::default()),
        ROLES_COUNTED
    );
    // A medicinal herb in its share still tastes bitter.
    let mut bitter = mixed.clone();
    bitter.push(food(Source::Herb, "bearberry", 20));
    assert_eq!(
        balance(&bitter, ingredients(), &Wants::default()),
        ROLES_COUNTED - 1
    );
    // Anything inedible is past its share of none.
    mixed.push(food(Source::Herb, "foxglove", 1));
    assert_eq!(
        balance(&mixed, ingredients(), &Wants::default()),
        ROLES_COUNTED - 1
    );
}

#[test]
fn an_intermediate_carries_its_history() {
    let dough = Carried {
        portions: vec![food(Source::Vegetable, "wheat", 500), water()],
        history: History {
            halves: 6,
            roots: vec![(Tool::Bowl, 1), (Tool::Knife, 4)],
            ..History::default()
        },
    };
    let process = Process {
        rows: 1,
        sources: vec![Some(dough)],
        steps: vec![Step::at(Tool::Knife, 0, 0, Vec::new())],
        heat: Heat::Cold,
    };
    let planned = plan(&process, ingredients(), dishes(), skilled(8)).unwrap();
    // Only this session's step for the chance.
    assert_eq!(planned.chance, CERTAIN + 800);
    let history = &planned.outputs[0].history;
    assert_eq!(history.halves, 8);
    // The Knife's lower root kept: this cook's ∛8 = 2.
    assert_eq!(history.roots, vec![(Tool::Knife, 2), (Tool::Bowl, 1)]);
    // 8 halves × (2 + 1) ÷ (2 × 20) rounds down to 0.
    assert_eq!(skill_steps(history), 0);
    let long = History {
        halves: 20,
        roots: vec![(Tool::Bowl, 4), (Tool::Pan, 4)],
        ..History::default()
    };
    assert_eq!(skill_steps(&long), 4);
}

#[test]
fn outcome_rolls_chance_then_luck() {
    let planned = plan(&stew(Heat::Hearth), ingredients(), dishes(), skilled(0)).unwrap();
    assert_eq!(outcome(&planned, planned.chance, 0), Outcome::Failed);
    let Outcome::Made { qualities } = outcome(&planned, 0, LUCKY_TWICE - 1) else {
        panic!("made");
    };
    assert_eq!(qualities, vec![Quality::Legendary]);
}

fn dishes() -> &'static Dishes {
    Dishes::built_in()
}
