//! Where fungi grow and picking them: patches' growth, underground placing by
//! niche, depth and daylight, and felled trees.

use super::*;
use crate::climate::Biome;
use crate::dungeon::{Cell, Level};
use crate::herb::{STAGES, growth_stage_minutes};
use crate::mob::Habitat;
use crate::poi::PoiKind;
use crate::rng::Seed;
use crate::site::Site;
use crate::tree::TreeStage;

fn habitat(celsius: i32) -> Habitat {
    Habitat {
        biome: Biome::SeasonalForest,
        metres: 100,
        celsius,
    }
}

/// A patch grows to full and is picked only then, once a stage since.
#[test]
fn patches_are_picked_full_grown() {
    let fungi = Fungi::built_in();
    let def = fungi.get("oyster_mushroom").unwrap();
    let mut patch = FungusPatch {
        fungus: def.key.clone(),
        phase: 0,
        picked: None,
    };
    let step = growth_stage_minutes(def.growth);
    assert_eq!(patch.stage(def, 0), 1);
    assert!(!patch.can_pick(def, 0));
    let grown = step * u64::from(STAGES - 1);
    assert_eq!(patch.stage(def, grown), STAGES);
    assert!(patch.can_pick(def, grown));
    patch.picked = Some(grown);
    assert!(!patch.can_pick(def, grown + 1));
}

/// A large piece is one item; others as many as a herb patch gives.
#[test]
fn harvests_count_as_herbs() {
    let fungi = Fungi::built_in();
    let mut rng = Seed::new(3).rng();
    let piece = (fungi.iter()).find(|def| def.unit == Unit::Piece).unwrap();
    assert_eq!(harvest(piece, 10_000, &mut rng), 1);
    let one = fungi.get("oyster_mushroom").unwrap();
    for _ in 0..50 {
        assert!((1..=3).contains(&harvest(one, 0, &mut rng)));
    }
}

/// About one floor cell in 60 of a cave grows a fungus suiting it: never above
/// ground only, dim ones near daylight, Dreamcap on the deepest level alone.
#[test]
fn caves_grow_fungi_by_their_cells() {
    let fungi = Fungi::built_in();
    let (mut floors, mut patches, mut kinds) = (0, 0, std::collections::BTreeSet::new());
    for n in 0..12 {
        let site = Site::new(PoiKind::RockCave, Seed::new(n));
        for index in 0..site.levels() {
            let level = site.level(index);
            let kind = Underground::Cave {
                depth: index,
                levels: site.levels(),
                crypt: false,
            };
            let grown = grow_underground(&level, fungi, kind, &habitat(10), Seed::new(n));
            assert_eq!(
                grown,
                grow_underground(&level, fungi, kind, &habitat(10), Seed::new(n))
            );
            let (min_x, min_y, width, height) = level.level.bounds();
            for y in min_y..min_y + height {
                for x in min_x..min_x + width {
                    floors += usize::from(level.level.cell(x, y) == Cell::Floor);
                }
            }
            for ((x, y), patch) in &grown {
                assert_eq!(level.level.cell(*x, *y), Cell::Floor);
                let def = fungi.get(&patch.fungus).unwrap();
                assert_ne!(def.grows, Where::Surface, "{}", def.key);
                assert!(def.temperature.suits(10), "{}", def.key);
                if def.key == "dreamcap" {
                    assert_eq!(index + 1, site.levels());
                }
                if def.grows == Where::Dim {
                    assert_eq!(index, 0);
                }
                kinds.insert(def.key.clone());
            }
            patches += grown.len();
        }
    }
    let expected = floors / PATCH_ONE_IN as usize;
    assert!(
        patches * 2 > expected && patches < expected * 2,
        "{patches} of {floors}"
    );
    assert!(kinds.len() > 5, "{kinds:?}");
}

/// What a cell offers follows what is round it.
#[test]
fn cells_offer_niches_by_their_neighbours() {
    let mut level = Level::new(0, 0, 12, 12);
    for y in 1..11 {
        for x in 1..11 {
            level.set(x, y, Cell::Floor);
        }
    }
    level.set(5, 5, Cell::Water);
    level.set(8, 8, Cell::Lava);
    let cave = Underground::Cave {
        depth: 1,
        levels: 3,
        crypt: false,
    };
    let at = |at| underground::spot_at(&level, cave, &[], at);
    assert!(at((4, 4)).niches.contains(&Niche::Water));
    assert!(!at((2, 9)).niches.contains(&Niche::Water));
    assert!(at((7, 7)).niches.contains(&Niche::Lava));
    assert!(at((1, 5)).niches.contains(&Niche::Rock));
    assert!(!at((3, 3)).niches.contains(&Niche::Remains));
    assert!(!at((3, 3)).dim);
    let deepest = Underground::Cave {
        depth: 2,
        levels: 3,
        crypt: false,
    };
    let spot = underground::spot_at(&level, deepest, &[], (3, 3));
    assert!(spot.deepest && spot.niches.contains(&Niche::Remains));
    let top = Underground::Cave {
        depth: 0,
        levels: 3,
        crypt: false,
    };
    assert!(underground::spot_at(&level, top, &[(2, 2)], (4, 4)).dim);
    assert!(!underground::spot_at(&level, top, &[(2, 2)], (9, 9)).dim);
    let basement = underground::spot_at(&level, Underground::Basement, &[], (5, 5));
    assert_eq!(basement.niches, [Niche::Wood, Niche::Soil]);
    assert!(!basement.earth && !basement.dim);
}

/// Rarer fungi are likelier deeper; very rare ones never on the top level.
#[test]
fn rarer_fungi_gain_with_depth() {
    use crate::vegetable::Abundance;
    assert_eq!(weight(Abundance::VeryRare, 0), 0);
    assert!(weight(Abundance::Rare, 4) > weight(Abundance::Rare, 0));
    assert_eq!(weight(Abundance::Common, 0), weight(Abundance::Common, 5));
}

/// A felled tree's fungus feeds on wood and suits the land, and shows only
/// while the tree is cleared.
#[test]
fn felled_trees_grow_wood_feeders_once_cleared() {
    let fungi = Fungi::built_in();
    let land = habitat(10);
    let found: Vec<_> = (0..200)
        .filter_map(|x| felled_patch(fungi, &land, Seed::new(7), (x, 0)))
        .collect();
    assert!(found.len() > 40 && found.len() < 120, "{}", found.len());
    for patch in &found {
        let def = fungi.get(&patch.fungus).unwrap();
        assert!(def.feeds_on_wood() && def.biomes.contains(&land.biome));
    }
    let mut level = Level::new(0, 0, 3, 3);
    level.set(1, 1, Cell::Tree);
    level.set_fungus(1, 1, found[0].clone());
    assert!(level.fungus_at(1, 1).is_none());
    level.set_felled(1, 1, Some(TreeStage::Stump));
    assert!(level.fungus_at(1, 1).is_none());
    level.set_felled(1, 1, Some(TreeStage::Cleared));
    assert!(level.fungus_at(1, 1).is_some());
    assert_eq!(level.fungus_patches().count(), 1);
    level.set_felled(1, 1, Some(TreeStage::Sapling));
    assert!(level.fungus_at(1, 1).is_none());
}
