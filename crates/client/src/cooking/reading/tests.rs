use shared::cooking::Source;
use shared::cooking::{Dishes, Recipe, RecipeInput, RecipeStep, Tool, Wanted, dish_recipe};
use shared::item_text::{KindToken, Piece, pieces};
use shared::lang::{Lang, Language};

use super::have::Have;
use super::written::{missing_in, written};

/// Everything to hand, for `recipe`.
fn all_there(recipe: &Recipe) -> Have {
    Have {
        tools: vec![true; recipe.steps.len()],
        qualities: Vec::new(),
        inputs: (recipe.steps.iter())
            .map(|step| vec![true; step.inputs.len()])
            .collect(),
        sources: vec![true; recipe.sources.len()],
    }
}

/// The kinds written in `words`, in order.
fn kinds(words: &str) -> Vec<KindToken> {
    (pieces(words).into_iter())
        .filter_map(|piece| match piece {
            Piece::Kind(kind) => Some(kind),
            _ => None,
        })
        .collect()
}

#[test]
fn every_real_dish_is_written_out_whole() {
    let lang = Lang::new(Language::English);
    for dish in Dishes::built_in().iter() {
        let recipe = dish_recipe(&dish.key).expect("every dish has a recipe");
        let lines = written(recipe, &all_there(recipe), &lang);
        assert_eq!(lines.len(), recipe.steps.len(), "{}", dish.key);
        for line in &lines {
            assert!(!line.words.contains("{$"), "{}: {}", dish.key, line.words);
            assert!(line.missing.is_empty(), "{}: {}", dish.key, line.words);
            // Every input and the tool are drawn.
            assert!(
                !kinds(&line.words).is_empty(),
                "{}: {}",
                dish.key,
                line.words
            );
        }
    }
}

/// A two-row recipe joined in a Bowl: carrots cut, eggs fried, both mixed.
fn joined() -> Recipe {
    let food = |source, key: &str, count| RecipeInput::Food {
        foods: vec![Wanted::One(source, key.to_owned())],
        count,
    };
    let step = |tool, column, row, height, inputs| RecipeStep {
        tool,
        column,
        row,
        height,
        inputs,
    };
    Recipe {
        rows: 2,
        sources: Vec::new(),
        steps: vec![
            step(
                Tool::Knife,
                0,
                0,
                1,
                vec![food(Source::Vegetable, "carrot", 2)],
            ),
            step(Tool::Pan, 0, 1, 1, vec![food(Source::Egg, "hen", 1)]),
            step(Tool::Bowl, 1, 0, 2, vec![RecipeInput::Water { ml: 200 }]),
        ],
    }
}

#[test]
fn steps_say_what_they_start_from() {
    let lang = Lang::new(Language::English);
    let recipe = joined();
    let lines = written(&recipe, &all_there(&recipe), &lang);
    let first = "1. Take 2 ${kind:vegetable:carrot}, and cut it with a ${kind:knife}.";
    assert_eq!(lines[0].words, first);
    assert!(
        lines[1].words.starts_with("2. Take 1 "),
        "{}",
        lines[1].words
    );
    assert!(
        lines[2]
            .words
            .starts_with("3. Bring both rows together, add 200 ml of ")
    );
    assert!(lines[2].words.ends_with("and mix it in a ${kind:bowl}."));
}

#[test]
fn what_is_missing_is_counted_among_the_kinds() {
    let lang = Lang::new(Language::English);
    let recipe = joined();
    let mut have = all_there(&recipe);
    have.tools[0] = false;
    have.inputs[2][0] = false;
    let lines = written(&recipe, &have, &lang);
    // The carrots, then the knife.
    assert_eq!(lines[0].missing, vec![1], "{}", lines[0].words);
    assert!(lines[1].missing.is_empty());
    // The water, then the bowl.
    assert_eq!(lines[2].missing, vec![0]);
}

#[test]
fn missing_kinds_are_found_wherever_they_are_written() {
    let words = "Cut ${kind:vegetable:carrot} with a ${kind:knife}, then ${kind:egg}.";
    let missing = [
        KindToken::Tool(Tool::Knife),
        KindToken::Food(Wanted::Any(Source::Egg)),
    ];
    assert_eq!(missing_in(words, &missing), vec![1, 2]);
}
