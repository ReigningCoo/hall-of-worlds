//! How a ghost is drawn (`ghosts`): the kind's usual item, faint, with how
//! many are missing (liquids in 100 ml), framed red unless something held is
//! aimed at it; and a row's missing intermediate, its source space framed
//! red. Each tip says what is wanted.

use shared::cooking::RecipeInput;
use shared::item::Inventory;
use shared::item_text::KindToken;

use super::bench::Bench;
use super::boxes::space_shown;
use super::reading::{foods_named, input_token};
use super::spaces::{ML_STEP, Place, SpaceIcons};
use crate::dom::SlotShown;

/// The step's `n`th ghost's look and tip, if it has one.
pub(super) fn ghost_shown(
    (bench, carried): (&Bench, &Inventory),
    (step, n): (usize, usize),
    aimed: Option<bool>,
    icons: &SpaceIcons,
) -> Option<(SlotShown, String)> {
    let station = bench.steps.get(step)?;
    let ghost = station.ghosts(carried).get(n).copied()?;
    let wanted = station.wanted.get(ghost.input)?;
    let item = input_token(wanted).as_ref().and_then(KindToken::stand_in);
    let (count, words) = match wanted {
        RecipeInput::Food { foods, .. } => (
            ghost.missing,
            format!("{} × {}", ghost.missing, foods_named(foods)),
        ),
        RecipeInput::Liquid { foods, .. } => (
            ghost.missing.div_ceil(ML_STEP),
            format!("{} ml of {}", ghost.missing, foods_named(foods)),
        ),
        RecipeInput::Water { .. } => (
            ghost.missing.div_ceil(ML_STEP),
            format!("{} ml of water", ghost.missing),
        ),
    };
    let stack = item.as_ref().map(|item| (item, count.max(1)));
    let mut shown = space_shown(
        stack,
        icons.of(Place::Ghost(step, n)),
        aimed.or(Some(false)),
    );
    if let Some(thing) = &mut shown.thing {
        thing.faded = true;
    }
    let tip = format!("Missing {words}: drop one here, or click or tap to cook without it");
    Some((shown, tip))
}

/// A row's source space while the intermediate a recipe wants is missing.
pub(super) fn source_missing_tip(bench: &Bench, row: u8) -> Option<String> {
    let rest = bench.source_missing(usize::from(row))?;
    Some(format!(
        "Missing your {}, to start the row from",
        rest.name().to_lowercase()
    ))
}
