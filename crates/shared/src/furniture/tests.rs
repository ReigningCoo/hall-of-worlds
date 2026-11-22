use std::collections::BTreeSet;

use super::{Furniture, Piece, Setting};
use crate::door::Side;
use crate::dungeon::{Cell, Level};
use crate::poi::PoiKind;
use crate::rng::Seed;
use crate::site::{Generator, Site, SiteLevel};

/// Every furnished kind's levels, intact and ruined, for `count` seeds each.
fn furnished_levels(count: u64) -> impl Iterator<Item = (PoiKind, u8, SiteLevel)> {
    (PoiKind::ALL.into_iter())
        .filter(|&kind| Setting::for_kind(kind).is_some())
        .flat_map(move |kind| (0..count).map(move |n| (kind, n)))
        .flat_map(|(kind, n)| {
            let ruins: &[u8] = if Generator::can_be_ruined(kind) {
                &[0, 3]
            } else {
                &[0]
            };
            ruins.iter().filter_map(move |&ruin| {
                let generator = Generator::for_poi(kind).with_ruin(ruin);
                let seed = Seed::new(n * 7919 + 101);
                Site::with_generator(kind, seed, generator.version(), generator)
                    .map(|site| (kind, ruin, site))
            })
        })
        .flat_map(|(kind, ruin, site)| {
            (0..site.levels()).map(move |index| (kind, ruin, site.level(index)))
        })
}

/// Every cell a 4-way fill reaches from `start` over walkable cells (every
/// door open), not counting furnished ones if `furnished`.
fn fill(level: &Level, start: (i64, i64), furnished: bool) -> BTreeSet<(i64, i64)> {
    let open = level.with_all_doors_open();
    let mut reached = BTreeSet::new();
    let mut queue = vec![start];
    while let Some((x, y)) = queue.pop() {
        let blocked = furnished && level.furniture_at(x, y).is_some();
        if !open.is_walkable(x, y) || blocked || !reached.insert((x, y)) {
            continue;
        }
        queue.extend([(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)]);
    }
    reached
}

fn arrivals(site: &SiteLevel) -> Vec<(i64, i64)> {
    let stairs = site.stairs.iter().map(|s| (s.x, s.y));
    site.entrance.into_iter().chain(stairs).collect()
}

#[test]
fn furniture_never_cuts_anything_off() {
    let mut pieces = 0;
    for (kind, ruin, site) in furnished_levels(6) {
        let level = &site.level;
        pieces += level.furniture().count();
        for start in arrivals(&site) {
            let without = fill(level, start, false);
            let with = fill(level, start, true);
            for cell in without {
                assert!(
                    with.contains(&cell) || level.furniture_at(cell.0, cell.1).is_some(),
                    "{kind:?} ruin {ruin}: {cell:?} cut off from {start:?}"
                );
            }
        }
    }
    assert!(pieces > 500, "only {pieces} pieces");
}

#[test]
fn furniture_stands_on_room_floor_away_from_doors_and_stairs() {
    for (kind, _, site) in furnished_levels(4) {
        let level = &site.level;
        for piece in level.furniture() {
            for (x, y) in piece.cells() {
                assert_eq!(level.cell(x, y), Cell::Floor, "{kind:?} {piece:?}");
                assert!(level.room(x, y).is_some());
                for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                    assert!(!matches!(
                        level.cell(x + dx, y + dy),
                        Cell::Door | Cell::SecretDoor
                    ));
                }
                for s in &site.stairs {
                    assert!((s.x - x).abs() > 1 || (s.y - y).abs() > 1);
                }
                assert_ne!(site.entrance, Some((x, y)));
            }
        }
    }
}

/// The cell a seat faces.
fn faced(piece: &Piece, cell: (i64, i64)) -> (i64, i64) {
    let (dx, dy) = match piece.facing {
        Side::North => (0, -1),
        Side::East => (1, 0),
        Side::South => (0, 1),
        Side::West => (-1, 0),
    };
    (cell.0 + dx, cell.1 + dy)
}

#[test]
fn seats_face_a_table_or_have_a_wall_behind() {
    let mut at_tables = 0;
    // (A ruin may have lost the table.)
    for (kind, _, site) in furnished_levels(4).filter(|&(_, ruin, _)| ruin == 0) {
        let level = &site.level;
        for piece in level.furniture().filter(|p| !p.furniture.is_table()) {
            let cells: Vec<_> = piece.cells().collect();
            let at_table = cells.iter().all(|&cell| {
                let (x, y) = faced(piece, cell);
                level
                    .furniture_at(x, y)
                    .is_some_and(|p| p.furniture.is_table())
            });
            let backed = cells.iter().all(|&(x, y)| {
                let (fx, fy) = faced(piece, (x, y));
                level.cell(2 * x - fx, 2 * y - fy) == Cell::Wall
            });
            assert!(at_table || backed, "{kind:?} {piece:?}");
            at_tables += usize::from(at_table);
        }
    }
    assert!(at_tables > 100);
}

#[test]
fn seats_match_their_rooms_tables() {
    for (kind, _, site) in furnished_levels(4) {
        let level = &site.level;
        for table in level.furniture() {
            let Furniture::Table { style } = table.furniture else {
                continue;
            };
            for seat in level.furniture() {
                let room = |p: &Piece| level.room(p.at.0, p.at.1);
                let bench = matches!(seat.furniture, Furniture::CraftingBench { .. });
                if seat.furniture.is_table() || bench || room(seat) != room(table) {
                    continue;
                }
                assert_eq!(seat.material, table.material, "{kind:?}");
                match seat.furniture {
                    Furniture::Chair { style: chair, .. } => assert!(
                        style.chairs().contains(&chair) || table.size.0.max(table.size.1) > 2,
                        "{kind:?} {style:?} {chair:?}"
                    ),
                    Furniture::Bench { style: bench } => assert!(
                        style.benches().contains(&bench) || table.size.0.max(table.size.1) > 2,
                        "{kind:?} {style:?} {bench:?}"
                    ),
                    Furniture::Table { .. }
                    | Furniture::CraftingBench { .. }
                    | Furniture::CaskRack { .. }
                    | Furniture::BottleRack { .. }
                    | Furniture::BarCounter { .. } => {}
                }
            }
        }
    }
}

#[test]
fn unfurnished_kinds_have_none() {
    for kind in [
        PoiKind::RockCave,
        PoiKind::Village,
        PoiKind::RedTent,
        PoiKind::IceShelter,
        PoiKind::Oasis,
    ] {
        for n in 0..3 {
            let site = Site::new(kind, Seed::new(n));
            for index in 0..site.levels() {
                assert_eq!(site.level(index).level.furniture().count(), 0, "{kind:?}");
            }
        }
    }
}

#[test]
fn furniture_is_the_same_every_time() {
    for kind in [PoiKind::Castle, PoiKind::Crypt, PoiKind::Farm] {
        let site = Site::new(kind, Seed::new(77));
        for index in 0..site.levels() {
            let once: Vec<Piece> = site.level(index).level.furniture().copied().collect();
            let again = Site::new(kind, Seed::new(77)).level(index);
            let again: Vec<Piece> = again.level.furniture().copied().collect();
            assert_eq!(once, again);
        }
    }
}

/// Golden test: one site's furniture. If this changes, furniture in saved
/// sites moves.
#[test]
fn golden_furniture() {
    let site = Site::new(PoiKind::StoneCottage, Seed::new(12345));
    let level = site.level(0);
    let pieces: Vec<String> = (level.level.furniture())
        .map(|p| format!("{:?} {:?} {:?}", p.at, p.size, p.furniture))
        .collect();
    assert_eq!(pieces, GOLDEN_FURNITURE);
}

const GOLDEN_FURNITURE: [&str; 19] = [
    "(4, 10) (1, 1) Chair { style: LadderBack, arms: false }",
    "(4, 11) (1, 1) Table { style: Pedestal }",
    "(4, 12) (1, 1) Chair { style: LadderBack, arms: false }",
    "(5, 6) (1, 1) Chair { style: LadderBack, arms: false }",
    "(5, 7) (1, 1) Chair { style: LadderBack, arms: false }",
    "(5, 8) (1, 1) Chair { style: LadderBack, arms: false }",
    "(5, 11) (1, 1) Chair { style: LadderBack, arms: true }",
    "(6, 6) (1, 3) Table { style: Joined }",
    "(7, 6) (1, 1) Chair { style: LadderBack, arms: false }",
    "(7, 7) (1, 1) Chair { style: LadderBack, arms: false }",
    "(7, 8) (1, 1) Chair { style: LadderBack, arms: false }",
    "(9, 4) (1, 1) Chair { style: LadderBack, arms: false }",
    "(11, 12) (1, 1) Bench { style: LongStool }",
    "(12, 8) (1, 1) Chair { style: PanelBack, arms: true }",
    "(13, 7) (3, 1) Bench { style: LongStool }",
    "(13, 8) (3, 1) Table { style: Joined }",
    "(13, 9) (3, 1) Bench { style: LongStool }",
    "(16, 4) (1, 1) Chair { style: PanelBack, arms: false }",
    "(16, 8) (1, 1) Chair { style: PanelBack, arms: true }",
];

#[test]
fn furnishing_makes_only_well_made_pieces() {
    for (kind, ruin, site) in furnished_levels(4) {
        for piece in site.level.furniture() {
            assert!(piece.is_well_made(), "{kind:?} ruin {ruin}: {piece:?}");
        }
    }
}
