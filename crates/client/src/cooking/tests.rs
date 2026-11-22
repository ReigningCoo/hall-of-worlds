use bevy::prelude::*;
use shared::alchemy::CERTAIN;
use shared::cooking::Tool;
use shared::id::Id;

use super::bench::Bench;
use super::kitchen::KitchenNow;
use super::markup::COOKING;
use super::run::stage_colour;
use super::spaces::{MOST_FACED, MOST_FOODS, Place, places};
use super::words::chance_words;

#[test]
fn markup_is_good() {
    COOKING.assert_good();
}

#[test]
fn steps_go_in_move_join_and_come_out() {
    let mut bench = Bench::default();
    assert!(bench.add(Tool::Bowl));
    assert!(bench.add(Tool::Knife));
    let place = |bench: &Bench| {
        (bench.steps.iter())
            .map(|s| (s.tool, s.column, s.row, s.height))
            .collect::<Vec<_>>()
    };
    assert_eq!(
        place(&bench),
        [(Tool::Bowl, 0, 0, 1), (Tool::Knife, 1, 0, 1)]
    );
    // A Pan in the second row, under the Bowl.
    assert!(bench.place(Tool::Pan, 0, 1));
    // Not on top of another.
    assert!(!bench.place(Tool::Oven, 1, 0));
    // The Knife joins both rows, as the place below it is free.
    assert!(bench.toggle_join(1));
    assert_eq!(bench.steps[1].height, 2);
    // Nothing can go under a join now; a click splits it again.
    assert!(!bench.place(Tool::Sieve, 1, 1));
    assert!(bench.toggle_join(1));
    assert_eq!(bench.steps[1].height, 1);
    // The Pan can't join up: the Bowl is above it.
    assert!(!bench.toggle_join(2));
    // Moved along, then taken out.
    assert!(bench.move_to(2, 3, 1));
    assert!(bench.toggle_join(2));
    assert_eq!(place(&bench)[2], (Tool::Pan, 3, 0, 2));
    // Run by column, then row.
    assert_eq!(bench.in_order(), [0, 1, 2]);
    bench.remove(0);
    assert_eq!(bench.steps.len(), 2);
    assert_eq!(bench.columns_shown(), 5);
}

#[test]
fn spaces_grow_as_they_fill() {
    let mut bench = Bench::default();
    bench.add(Tool::Cauldron);
    let kitchen = KitchenNow::default();
    assert_eq!(
        places(0, &bench.steps[0], kitchen.water_faced, 0),
        [Place::Food(0, 0), Place::Liquid(0, 0)]
    );
    for n in 0..MOST_FOODS + 2 {
        bench.steps[0].foods.push((Id::stand_in(n as u64 + 1), 1));
    }
    let shown = places(0, &bench.steps[0], true, 0);
    assert_eq!(shown.len(), MOST_FOODS + 2);
    assert_eq!(shown.last(), Some(&Place::Faced(0)));
    // Water faced goes up by 100 ml and back to none past the most.
    for _ in 0..MOST_FACED / 100 {
        super::spaces::more_faced(&mut bench, 0);
    }
    assert_eq!(bench.steps[0].faced, MOST_FACED);
    super::spaces::more_faced(&mut bench, 0);
    assert_eq!(bench.steps[0].faced, 0);
}

#[test]
fn the_order_lists_inputs_by_step() {
    let mut bench = Bench::default();
    bench.add(Tool::Knife);
    bench.place(Tool::Cauldron, 0, 1);
    bench.steps[0].foods.push((Id::stand_in(1), 2));
    bench.steps[1].liquids.push((Id::stand_in(2), 300));
    bench.steps[1].faced = 200;
    let order = bench.order(3, &KitchenNow::default());
    assert_eq!(order.count, 3);
    assert_eq!(order.rows, 2);
    assert_eq!(order.steps.len(), 2);
    assert_eq!(order.steps[1].inputs.len(), 2);
    let uses = bench.uses();
    assert_eq!(uses, [(Id::stand_in(1), 2, 0), (Id::stand_in(2), 0, 300)]);
}

#[test]
fn chance_in_words() {
    assert_eq!(chance_words(CERTAIN), "Chance 100%");
    assert_eq!(chance_words(CERTAIN / 8), "Chance 12.5%");
}

#[test]
fn colour_runs_black_white_gold_product() {
    let product = Color::srgb(0.0, 0.0, 1.0);
    assert_eq!(
        stage_colour(0.0, product).to_srgba(),
        Color::BLACK.to_srgba()
    );
    assert_eq!(stage_colour(1.0, product).to_srgba(), product.to_srgba());
}

#[test]
fn a_rows_source_goes_with_its_first_step() {
    let mut bench = Bench::default();
    bench.place(Tool::Pan, 2, 0);
    bench.place(Tool::Bowl, 0, 1);
    bench.place(Tool::Knife, 0, 0);
    // The Knife leads the top row, the Bowl the bottom.
    assert_eq!(bench.first_in_row(0), Some(2));
    assert_eq!(bench.first_in_row(1), Some(1));
    bench.sources = [Some(Id::stand_in(1)), Some(Id::stand_in(2))];
    // A join at the front leads both rows.
    bench.remove(2);
    bench.move_to(0, 1, 0);
    assert!(bench.toggle_join(1));
    assert_eq!(bench.first_in_row(0), Some(1));
    assert_eq!(bench.first_in_row(1), Some(1));
    // A row left without steps forgets its source.
    assert!(bench.toggle_join(1));
    bench.remove(1);
    assert_eq!(bench.sources, [Some(Id::stand_in(1)), None]);
}

#[test]
fn the_side_drops_below_a_wide_grid() {
    use super::fit::grid_width;
    assert!(grid_width(7) <= 520.0);
    assert!(grid_width(8) > 520.0);
}

#[test]
fn tools_are_outlined_by_the_one_used() {
    use shared::item::{Inventory, Quality};
    let kitchen = KitchenNow::default();
    let carried = Inventory::default();
    assert_eq!(kitchen.used_quality(Tool::Pan, &carried), None);
    assert_eq!(kitchen.used_quality(Tool::Oven, &carried), None);
    let town = KitchenNow {
        oven: true,
        tools: vec![shared::item::ItemKind::Pan],
        ..KitchenNow::default()
    };
    assert_eq!(
        town.used_quality(Tool::Pan, &carried),
        Some(Quality::Common)
    );
    assert_eq!(
        town.used_quality(Tool::Oven, &carried),
        Some(Quality::Common)
    );
}
