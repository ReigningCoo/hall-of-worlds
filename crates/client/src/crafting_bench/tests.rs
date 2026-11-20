use shared::crafting_bench::Craft;
use shared::door::Side;
use shared::dungeon::{Cell, Level};
use shared::furniture::{Furniture, Material, Piece, Wood};
use shared::spot::Spot;

use super::state::{front_cell, in_reach, nearest_cell};
use super::*;

fn bench() -> Piece {
    Piece {
        at: (3, 2),
        size: (2, 1),
        facing: Side::South,
        material: Material::Wood(Wood::Oak),
        furniture: Furniture::CraftingBench {
            craft: Craft::Alchemy,
        },
    }
}

#[test]
fn the_panel_is_good_markup_with_the_grids_columns() {
    markup::BENCH.assert_good();
    let columns = format!("columns=\"{COLUMNS}\"");
    assert!(markup::BENCH.source.contains(&columns));
}

/// From in front of it, beside it or diagonally off a corner it is in reach;
/// two cells away it isn't.
#[test]
fn a_bench_is_reached_from_the_cells_round_it() {
    let piece = bench();
    for cell in [(3, 3), (4, 3), (5, 2), (2, 3), (5, 3)] {
        assert!(in_reach(Spot::middle_of(cell), &piece), "{cell:?}");
    }
    assert!(!in_reach(Spot::middle_of((3, 5)), &piece));
    assert_eq!(nearest_cell(Spot::middle_of((5, 3)), &piece), (4, 2));
}

/// The walk goes to the free cell in front nearest the player.
#[test]
fn the_walk_ends_in_front_of_it() {
    let mut level = Level::new(0, 0, 8, 8);
    for y in 0..8 {
        for x in 0..8 {
            level.set(x, y, Cell::Floor);
        }
    }
    let piece = bench();
    level.place_furniture(piece);
    assert_eq!(
        front_cell(&level, Spot::middle_of((7, 6)), &piece),
        Some((4, 3))
    );
    assert_eq!(
        front_cell(&level, Spot::middle_of((0, 6)), &piece),
        Some((3, 3))
    );
    level.set(3, 3, Cell::Wall);
    level.set(4, 3, Cell::Wall);
    assert_eq!(front_cell(&level, Spot::middle_of((0, 6)), &piece), None);
}

/// Bevy only finds two parameters clashing when a system starts.
#[test]
fn no_system_asks_for_what_another_of_its_parameters_has() {
    fn starts<M>(system: impl IntoSystem<(), (), M>) {
        IntoSystem::into_system(system).initialize(&mut World::new());
    }
    starts(open::change_of_mind);
    starts(open::follow_clicked_bench);
    starts(open::open_on_arrival);
    starts(here::keep_bench_here);
    starts(server::read_server);
    starts(grid::bind_grid);
    starts(grid::note_gifts);
    starts(grid::draw_grid);
    starts(grid::show_gift_aim);
    starts(close::close_on_escape);
    starts(close::close_when_left);
}
