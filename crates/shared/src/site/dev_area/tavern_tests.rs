//! The Tavern Dev Area.

use std::collections::BTreeSet;

use super::tests::{area, level_of};
use super::*;
use crate::bar::BarKind;
use crate::furniture::{Furniture, Holder, Piece};
use crate::site::{Generator, Site, SiteLevel};

fn tavern(size: u16) -> SiteLevel {
    level_of(area(size, DevLayout::Tavern, DevMobs::None))
}

/// Every cell a 4-way fill reaches from the stairs, every door open, over
/// cells no piece stands on (a bar's flap is walked through).
fn reached(site: &SiteLevel) -> BTreeSet<(i64, i64)> {
    let open = site.level.with_all_doors_open();
    let start = (site.stairs[0].x, site.stairs[0].y);
    let mut reached = BTreeSet::new();
    let mut queue = vec![start];
    while let Some((x, y)) = queue.pop() {
        let piece = site.level.furniture_at(x, y).map(|p| p.furniture);
        let flap = matches!(piece, Some(Furniture::BarCounter { counter }) if counter.kind == BarKind::Flap);
        let blocked = piece.is_some() && !flap;
        if !open.is_walkable(x, y) || blocked || !reached.insert((x, y)) {
            continue;
        }
        queue.extend([(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)]);
    }
    reached
}

fn beside(piece: &Piece) -> impl Iterator<Item = (i64, i64)> + '_ {
    piece
        .cells()
        .flat_map(|(x, y)| [(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)])
}

#[test]
fn its_pieces_are_well_made_and_its_decor_fits() {
    let site = tavern(32);
    let floor: Vec<&Piece> = site.level.furniture().collect();
    assert!(floor.len() > 60, "{}", floor.len());
    for piece in &floor {
        assert!(piece.is_well_made(), "{piece:?}");
    }
    // Two spirit racks on tables, candles on the bar and on two tables.
    assert_eq!(site.level.furniture_on_top().count(), 6);
    assert!(site.level.decor().count() > 25);
    for decor in site.level.decor() {
        assert!(decor.fits(&site.level), "{decor:?}");
    }
}

#[test]
fn everything_in_it_can_be_reached() {
    let site = tavern(32);
    let reached = reached(&site);
    for piece in site.level.furniture() {
        let used = Holder::of(piece.furniture).is_some()
            || matches!(
                piece.furniture,
                Furniture::Bed { .. } | Furniture::Chair { .. }
            );
        if used {
            assert!(beside(piece).any(|c| reached.contains(&c)), "{piece:?}");
        }
    }
    // Behind the bar, through its flap.
    let behind = (site.level.furniture())
        .find(|p| matches!(p.furniture, Furniture::CaskRack { .. }))
        .expect("a cask rack");
    assert!(beside(behind).any(|c| reached.contains(&c)));
}

#[test]
fn it_is_the_same_in_every_size_and_has_no_cellar() {
    let small = tavern(16);
    let large = tavern(64);
    assert_eq!(
        small.level.furniture().count(),
        large.level.furniture().count()
    );
    let tavern = area(32, DevLayout::Tavern, DevMobs::None);
    let generator = Generator::DevArea(tavern);
    let site = Site::with_generator(DEV_AREA_KIND, Seed::new(7), generator.version(), generator)
        .expect("made");
    assert!(site.cellar_casks(0, &site.level(0)).is_empty());
    assert_eq!(DevLayout::named("tavern"), Some(DevLayout::Tavern));
}
