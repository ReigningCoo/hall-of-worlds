//! Stone tables: a slab on blocks, or a slab on one solid block with a plinth.
//! Stone's layers lie level (`Grain::Up`).

use bevy::prelude::*;

use super::super::CELL_SIZE;
use super::tables::Table;
use super::{Grain, INSET, Part, TABLE_TOP};

/// How thick a stone top is.
const SLAB: f32 = CELL_SIZE * 0.08;

fn stone(low: Vec3, high: Vec3) -> Part {
    Part::block(low, high, Grain::Up)
}

/// The top slab.
fn top(table: &Table) -> Part {
    table.vary(
        stone(
            Vec3::new(INSET, TABLE_TOP - SLAB, INSET),
            Vec3::new(table.length - INSET, TABLE_TOP, table.width - INSET),
        ),
        0,
    )
}

/// A slab on a block near each end, and one in the middle of a long table.
pub(super) fn slab(table: &Table) -> Vec<Part> {
    let mut parts = vec![top(table)];
    let thick = CELL_SIZE * 0.22;
    let (near, far) = (
        INSET + 0.1 * CELL_SIZE,
        table.width - INSET - 0.1 * CELL_SIZE,
    );
    let mut places = vec![
        INSET + 0.08 * CELL_SIZE,
        table.length - INSET - 0.08 * CELL_SIZE - thick,
    ];
    if table.length >= CELL_SIZE * 2.5 {
        places.push((table.length - thick) / 2.0);
    }
    for (i, x) in places.into_iter().enumerate() {
        let block = stone(
            Vec3::new(x, 0.0, near),
            Vec3::new(x + thick, TABLE_TOP - SLAB, far),
        );
        parts.push(table.vary(block, 50 + i as i64));
    }
    parts
}

/// A slab on one solid block, standing on a low plinth.
pub(super) fn block(table: &Table) -> Vec<Part> {
    let inset = INSET + 0.12 * CELL_SIZE;
    let plinth = INSET + 0.08 * CELL_SIZE;
    vec![
        top(table),
        table.vary(
            stone(
                Vec3::new(inset, 0.0, inset),
                Vec3::new(table.length - inset, TABLE_TOP - SLAB, table.width - inset),
            ),
            60,
        ),
        table.vary(
            stone(
                Vec3::new(plinth, 0.0, plinth),
                Vec3::new(
                    table.length - plinth,
                    0.05 * CELL_SIZE,
                    table.width - plinth,
                ),
            ),
            61,
        ),
    ]
}
