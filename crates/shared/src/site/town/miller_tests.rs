use super::super::{Site, TOWN_VERSION, UNMILLED_TOWN_VERSION};
use super::*;
use crate::dungeon::{Cell, RoomId};
use crate::poi::PoiKind;
use crate::rng::Seed;

const KINDS: [PoiKind; 4] = [
    PoiKind::Hamlet,
    PoiKind::Village,
    PoiKind::Town,
    PoiKind::City,
];

/// Real towns of every size, from a few seeds.
fn towns() -> impl Iterator<Item = (PoiKind, Seed, SiteLevel)> {
    KINDS.into_iter().flat_map(|kind| {
        (0..16).map(move |n| {
            let seed = Seed::new(n * 977 + 13);
            (kind, seed, Site::new(kind, seed).level(0))
        })
    })
}

fn millers(site: &SiteLevel) -> Vec<&TownBuilding> {
    (site.buildings.iter())
        .filter(|b| b.look.kind == BuildingKind::Miller)
        .collect()
}

/// Most towns fit a miller (not everyone has flour), only towns and cities
/// more than one, and never more than three.
#[test]
fn most_towns_have_a_miller() {
    let mut with = 0;
    let mut all = 0;
    for (kind, _, site) in towns() {
        let count = millers(&site).len();
        let most = match kind {
            PoiKind::Town => 2,
            PoiKind::City => 3,
            _ => 1,
        };
        assert!(count <= most, "{kind:?}: {count}");
        with += usize::from(count > 0);
        all += 1;
    }
    assert!(with * 2 > all, "only {with} of {all}");
}

/// A mill is 9 × 9 with rounded corners, its door in the middle of a side
/// facing bare ground, with a cell of bare ground all round it.
#[test]
fn mills_are_round_towers_with_room_round_them() {
    for (_, _, site) in towns() {
        let level = &site.level;
        for mill in millers(&site) {
            let id = Some(RoomId(mill.look.id));
            let cells: Vec<(i64, i64)> = (mill.look.rects.iter())
                .flat_map(|r| {
                    (r.y..r.y + r.height)
                        .flat_map(move |y| (r.x..r.x + r.width).map(move |x| (x, y)))
                })
                .collect();
            assert_eq!(cells.len(), 81 - 4 * 3);
            let x0 = cells.iter().map(|c| c.0).min().unwrap();
            let y0 = cells.iter().map(|c| c.1).min().unwrap();
            for y in y0 - 1..y0 + 10 {
                for x in x0 - 1..x0 + 10 {
                    if level.room(x, y) != id {
                        assert!(level.cell(x, y).is_walkable(), "({x}, {y})");
                    }
                }
            }
            let (dx, dy) = mill.look.faces;
            let (x, y) = mill.look.door;
            assert_eq!(level.cell(x + dx, y + dy), Cell::Ground);
            assert_eq!((x - x0 - 4) * dy, 0);
            assert_eq!((y - y0 - 4) * dx, 0);
        }
    }
}

/// Millers come last, and everything else is as the town was before them:
/// the same buildings with the same ids, looks and numbers.
#[test]
fn millers_move_nothing_else() {
    for (kind, seed, site) in towns() {
        let old = Site::with_version(kind, seed, UNMILLED_TOWN_VERSION)
            .expect("saved towns still run")
            .level(0);
        assert!(millers(&old).is_empty());
        let others = site.buildings.len() - millers(&site).len();
        assert_eq!(&site.buildings[..others], &old.buildings[..]);
        assert!(
            millers(&site)
                .iter()
                .all(|m| usize::from(m.look.id) >= others)
        );
        for y in 0..old.level.bounds().3 {
            for x in 0..old.level.bounds().2 {
                let (x, y) = (x + old.level.bounds().0, y + old.level.bounds().1);
                let mill = millers(&site)
                    .iter()
                    .any(|m| site.level.room(x, y) == Some(RoomId(m.look.id)));
                // Yards come after millers, so may go elsewhere.
                let pen = [&site, &old].iter().any(|level| {
                    level.fields.iter().any(|f| {
                        let r = f.rect;
                        (r.x - 1..=r.x + r.width).contains(&x)
                            && (r.y - 1..=r.y + r.height).contains(&y)
                    })
                });
                if !mill && !pen {
                    assert_eq!(site.level.cell(x, y), old.level.cell(x, y), "{kind:?}");
                }
            }
        }
    }
    assert_ne!(TOWN_VERSION, UNMILLED_TOWN_VERSION);
}
