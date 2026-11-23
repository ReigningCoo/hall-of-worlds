use super::Setting;
use crate::cask::CaskSize;
use crate::decor::{Decor, Hanging};
use crate::door::Side;
use crate::dungeon::Cell;
use crate::furniture::{Furniture, Piece};
use crate::poi::PoiKind;
use crate::rng::Seed;
use crate::site::{Generator, Site, SiteLevel};

/// Every furnished kind's sites, intact and ruined, for `count` seeds each:
/// the site, its ruin and each of its levels.
fn furnished(count: u64) -> Vec<(Site, u8, u32, SiteLevel)> {
    let mut found = Vec::new();
    for kind in PoiKind::ALL
        .into_iter()
        .filter(|&k| Setting::for_kind(k).is_some())
    {
        for n in 0..count {
            for ruin in [0, 3] {
                if ruin > 0 && !Generator::can_be_ruined(kind) {
                    continue;
                }
                let generator = Generator::for_poi(kind).with_ruin(ruin);
                let seed = Seed::new(n * 6151 + 17);
                let Some(site) = Site::with_generator(kind, seed, generator.version(), generator)
                else {
                    continue;
                };
                for index in 0..site.levels() {
                    let level = site.level(index);
                    found.push((site.clone(), ruin, index, level));
                }
            }
        }
    }
    found
}

fn count(levels: &[(Site, u8, u32, SiteLevel)], wanted: impl Fn(&Piece) -> bool) -> usize {
    (levels.iter())
        .map(|(_, _, _, site)| site.level.furniture().filter(|p| wanted(p)).count())
        .sum()
}

#[test]
fn castles_and_towers_get_every_furnishing() {
    let levels = furnished(12);
    let beds = count(&levels, |p| matches!(p.furniture, Furniture::Bed { .. }));
    let chests = count(&levels, |p| matches!(p.furniture, Furniture::Chest { .. }));
    let wardrobes = count(&levels, |p| {
        matches!(p.furniture, Furniture::Wardrobe { .. })
    });
    let fireplaces = count(&levels, |p| {
        matches!(p.furniture, Furniture::Fireplace { .. })
    });
    let casks = count(&levels, |p| {
        matches!(p.furniture, Furniture::CaskRack { .. })
    });
    let wine = count(&levels, |p| {
        matches!(p.furniture, Furniture::BottleRack { .. })
    });
    let decor: usize = levels
        .iter()
        .map(|(_, _, _, s)| s.level.decor().count())
        .sum();
    let candles: usize = (levels.iter())
        .map(|(_, _, _, s)| s.level.furniture_on_top().count())
        .sum();
    let found = [
        beds, chests, wardrobes, fireplaces, casks, wine, decor, candles,
    ];
    assert!(found.iter().all(|&n| n > 3), "{found:?}");
}

#[test]
fn furnishings_are_well_made_and_beds_have_their_heads_to_a_wall() {
    for (site, _, _, level) in furnished(8) {
        for piece in level.level.furniture() {
            assert!(piece.is_well_made(), "{:?} {piece:?}", site.kind());
            if let Furniture::Bed { .. } = piece.furniture {
                let head: Vec<_> = piece
                    .cells()
                    .filter(|&c| !piece.covers(piece.facing.opposite().beside(c)))
                    .collect();
                for cell in head {
                    let (x, y) = piece.facing.opposite().beside(cell);
                    assert_eq!(level.level.cell(x, y), Cell::Wall, "{piece:?}");
                }
            }
        }
        for decor in level.level.decor() {
            assert!(decor.fits(&level.level), "{decor:?}");
        }
    }
}

#[test]
fn cellar_casks_lie_where_they_did_and_on_racks_of_their_size() {
    let mut racked = 0;
    let mut stands = 0;
    for (site, _, index, level) in furnished(16) {
        for cask in site.cellar_casks(index, &level) {
            let Some(piece) = level.level.furniture_at(cask.at.0, cask.at.1) else {
                continue;
            };
            let Furniture::CaskRack { rack } = piece.furniture else {
                panic!("{cask:?} under {piece:?}");
            };
            assert_eq!(rack.size, cask.size);
            racked += 1;
            stands += usize::from(cask.size == CaskSize::Tun);
        }
    }
    assert!(racked > 20, "{racked}");
    assert!(stands > 0);
}

#[test]
fn only_lived_in_places_are_lit() {
    for (site, ruin, _, level) in furnished(8) {
        let lit = Setting::for_kind(site.kind()).is_some_and(|s| s.lit) && ruin == 0;
        let lights = (level.level.decor()).any(|decor| {
            matches!(
                decor,
                Decor::Chandelier { .. }
                    | Decor::Wall {
                        hanging: Hanging::Sconce,
                        ..
                    }
            )
        });
        assert!(lit || !lights, "{:?} ruin {ruin}", site.kind());
        assert!(lit || level.level.furniture_on_top().count() == 0);
    }
}

#[test]
fn furnishings_are_the_same_every_time() {
    for kind in [PoiKind::Castle, PoiKind::StoneTower] {
        let site = Site::new(kind, Seed::new(4242));
        for index in 0..site.levels() {
            let (once, again) = (site.level(index), site.level(index));
            let pieces = |l: &SiteLevel| l.level.furniture().copied().collect::<Vec<_>>();
            let decor = |l: &SiteLevel| l.level.decor().copied().collect::<Vec<_>>();
            assert_eq!(pieces(&once), pieces(&again));
            assert_eq!(decor(&once), decor(&again));
        }
    }
}

#[test]
fn against_wall_faces_away_from_it() {
    let bed = Furniture::Bed {
        bed: crate::furniture::Bed::Double,
    };
    let wood = crate::furniture::Material::Wood(crate::furniture::Wood::Oak);
    let piece = super::against_wall(bed, (3, 4), (2, 2), Side::West, wood);
    assert_eq!(piece.facing, Side::East);
    assert!(piece.is_well_made());
}
