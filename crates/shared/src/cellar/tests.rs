use super::*;
use crate::dungeon::Cell;
use crate::furniture::Furniture;
use crate::item::Quality;
use crate::poi::PoiKind;
use crate::potion::Liquid;
use crate::rng::Seed;
use crate::site::Site;

const KINDS: [PoiKind; 3] = [PoiKind::Castle, PoiKind::StoneTower, PoiKind::WoodenTower];

/// Each kind's sites with cellars among `count` seeds, with their casks.
fn cellars(count: u64) -> Vec<(PoiKind, u64, Site, u32, Vec<CellarCask>)> {
    let mut found = Vec::new();
    for kind in KINDS {
        for n in 0..count {
            let site = Site::new(kind, Seed::new(n * 7919 + 13));
            for index in 0..site.levels() {
                let casks = site.cellar_casks(index, &site.level(index));
                if !casks.is_empty() {
                    found.push((kind, n, site.clone(), index, casks));
                }
            }
        }
    }
    found
}

#[test]
fn only_castles_and_towers_have_cellars() {
    for kind in PoiKind::ALL {
        let has = (0..20).any(|n| {
            let site = Site::new(kind, Seed::new(n));
            cellar_level(kind, Seed::new(n), site.levels(), site.has_basement()).is_some()
        });
        assert_eq!(has, KINDS.contains(&kind), "{kind:?}");
    }
}

#[test]
fn at_most_one_level_and_its_basement_if_any() {
    let found = cellars(12);
    assert!(found.len() >= 12, "only {} cellars", found.len());
    for (kind, n, site, index, _) in &found {
        let same_site = found.iter().filter(|(k, m, ..)| k == kind && m == n);
        assert_eq!(same_site.count(), 1, "{kind:?} one level");
        if site.has_basement() {
            assert_eq!(*index, site.levels() - 1, "{kind:?}");
        } else {
            assert_eq!(*index, 0, "{kind:?}");
        }
    }
}

#[test]
fn casks_stand_in_one_room_against_walls_apart() {
    for (kind, _, site, index, casks) in cellars(12) {
        let level = site.level(index);
        let stairs: Vec<_> = level.stairs.iter().map(|s| (s.x, s.y)).collect();
        assert!(casks.len() <= 8, "{kind:?}");
        let room = level.level.room(casks[0].at.0, casks[0].at.1);
        for (n, cask) in casks.iter().enumerate() {
            let (x, y) = cask.at;
            assert_eq!(level.level.cell(x, y), Cell::Floor);
            assert_eq!(level.level.room(x, y), room, "{kind:?} one room");
            // Nothing but its own rack (`shared::furnishings`).
            let under = level.level.furniture_at(x, y).map(|piece| piece.furniture);
            assert!(
                matches!(under, None | Some(Furniture::CaskRack { .. })),
                "{under:?}"
            );
            let walls = [(1, 0), (-1, 0), (0, 1), (0, -1)]
                .iter()
                .any(|&(dx, dy)| level.level.cell(x + dx, y + dy) == Cell::Wall);
            assert!(walls, "{kind:?} against a wall");
            assert!(
                stairs
                    .iter()
                    .all(|&(sx, sy)| (sx - x).abs() > 1 || (sy - y).abs() > 1)
            );
            for other in &casks[n + 1..] {
                let apart = (other.at.0 - x).abs() > 1 || (other.at.1 - y).abs() > 1;
                assert!(apart, "{kind:?} casks a cell apart");
            }
        }
    }
}

#[test]
fn the_same_every_time() {
    let site = Site::new(PoiKind::Castle, Seed::new(5));
    for index in 0..site.levels() {
        let level = site.level(index);
        assert_eq!(
            site.cellar_casks(index, &level),
            site.cellar_casks(index, &level)
        );
    }
}

#[test]
fn contents_vary_and_keep_to_the_rules() {
    let cask = CellarCask {
        at: (0, 0),
        size: crate::cask::CaskSize::Hogshead,
        wood: crate::material::Material::Oak,
    };
    let mut rng = Seed::new(3).rng();
    let (mut empty, mut vinegar, mut sealed, mut opened) = (0, 0, 0, 0);
    for _ in 0..400 {
        let form = stock(cask, &mut rng);
        assert!(form.ml <= form.most_ml());
        match &form.liquid {
            None => empty += 1,
            Some(Liquid::Vinegar(_)) => {
                assert!(form.opened && form.ml > 0);
                vinegar += 1;
            }
            Some(Liquid::Drink(drink)) => {
                assert!(form.time.made.is_some());
                assert!(drink.quality <= Quality::Legendary);
                if form.opened {
                    assert!(form.ml < form.most_ml());
                    opened += 1;
                } else {
                    assert_eq!(form.ml, form.most_ml());
                    sealed += 1;
                }
            }
            other => panic!("unexpected {other:?}"),
        }
    }
    for (what, count) in [
        ("empty", empty),
        ("vinegar", vinegar),
        ("sealed", sealed),
        ("opened", opened),
    ] {
        assert!(count > 20, "{what}: {count}");
    }
}
