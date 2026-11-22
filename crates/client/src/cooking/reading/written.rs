//! A recipe written out as numbered steps from `recipe.ftl` (recipes plan
//! PR 4), in the order they are worked (column by column, the top row
//! first): what each starts from (nothing, its row so far, both rows joined,
//! an intermediate), the foods, liquids and water put in, and what is done
//! with the tool. Foods, liquids and tools are `${kind:…}` tokens; each line
//! says which of its tokens are missing (`missing`, counted from 0).

use shared::cooking::{Recipe, RecipeInput, RecipeStep, Tool};
use shared::item_text::{KindToken, Piece, pieces};
use shared::lang::{Args, Lang};

use super::have::{Have, input_token};

/// One line: its words, and the tokens in them missing for the reader.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct Line {
    pub(super) words: String,
    pub(super) missing: Vec<usize>,
}

/// Every step of `recipe` written out, in the order they are worked.
pub(super) fn written(recipe: &Recipe, have: &Have, lang: &Lang) -> Vec<Line> {
    let mut order: Vec<usize> = (0..recipe.steps.len()).collect();
    order.sort_by_key(|&s| (recipe.steps[s].column, recipe.steps[s].row));
    let mut lines = Vec::new();
    for (number, &s) in order.iter().enumerate() {
        let step = &recipe.steps[s];
        let mut missing_kinds: Vec<KindToken> = Vec::new();
        let foods: Vec<String> = (step.inputs.iter().enumerate())
            .filter_map(|(i, input)| {
                let token = input_token(input)?;
                if !have.input(s, i) {
                    missing_kinds.push(token.clone());
                }
                Some(food_words(input, &token, lang))
            })
            .collect();
        if !have.tool(s) {
            missing_kinds.push(KindToken::Tool(step.tool));
        }
        let start = start_of(recipe, step, s);
        let source = match start {
            "source" => source_name(recipe, step),
            _ => String::new(),
        };
        let foods: Vec<&str> = foods.iter().map(String::as_str).collect();
        let args = Args::new()
            .count("number", number as i64 + 1)
            .text("start", start)
            .text("source", source)
            // The first step of a row names its foods as it starts.
            .text(
                "added",
                match foods.is_empty() || start == "first" {
                    true => "no",
                    false => "yes",
                },
            )
            .text("foods", lang.rules().list(&foods))
            .text("tool-kind", tool_kind(step.tool))
            .text("tool", KindToken::Tool(step.tool).token());
        let words = render(lang, "recipe-step", &args);
        let missing = missing_in(&words, &missing_kinds);
        lines.push(Line { words, missing });
    }
    lines
}

/// "2 ${kind:vegetable:carrot}", "300 ml of ${kind:milk}".
fn food_words(input: &RecipeInput, token: &KindToken, lang: &Lang) -> String {
    let item = token.token();
    match input {
        RecipeInput::Food { count, .. } => {
            let args = Args::new()
                .count("count", i64::from(*count))
                .text("item", item);
            render(lang, "recipe-food", &args)
        }
        RecipeInput::Liquid { ml, .. } | RecipeInput::Water { ml } => {
            let args = Args::new().count("ml", i64::from(*ml)).text("item", item);
            render(lang, "recipe-liquid", &args)
        }
    }
}

/// What a step starts from: `join` (it spans both rows), `row` (an earlier
/// step feeds it: one before it in its row, or a join), `source` (its row
/// starts from an intermediate), else `first`.
fn start_of(recipe: &Recipe, step: &RecipeStep, at: usize) -> &'static str {
    if step.height > 1 {
        return "join";
    }
    let before = (recipe.steps.iter().enumerate()).any(|(other, earlier)| {
        other != at
            && earlier.column < step.column
            && (earlier.row == step.row || earlier.height > 1)
    });
    if before {
        return "row";
    }
    match recipe.sources.get(usize::from(step.row)) {
        Some(Some(_)) => "source",
        _ => "first",
    }
}

/// The intermediate a step's row starts from, in lower case ("dough").
fn source_name(recipe: &Recipe, step: &RecipeStep) -> String {
    (recipe.sources.get(usize::from(step.row)))
        .and_then(|source| *source)
        .map(|rest| rest.name().to_lowercase())
        .unwrap_or_default()
}

/// A tool's key in `recipe-use`.
fn tool_kind(tool: Tool) -> &'static str {
    match tool {
        Tool::Knife => "knife",
        Tool::Pan => "pan",
        Tool::Cauldron => "cauldron",
        Tool::Bowl => "bowl",
        Tool::Oven => "oven",
        Tool::Sieve => "sieve",
        Tool::Mortar => "mortar",
    }
}

/// Which of the items and kinds in `words` are missing, counted from 0.
pub(super) fn missing_in(words: &str, missing: &[KindToken]) -> Vec<usize> {
    let tokens = pieces(words)
        .into_iter()
        .filter(|piece| !matches!(piece, Piece::Text(_)));
    (tokens.enumerate())
        .filter(|(_, piece)| matches!(piece, Piece::Kind(kind) if missing.contains(kind)))
        .map(|(at, _)| at)
        .collect()
}

/// A message, or what of it could be written.
pub(super) fn render(lang: &Lang, id: &str, args: &Args) -> String {
    lang.render(id, args).unwrap_or_else(|error| error.text)
}
