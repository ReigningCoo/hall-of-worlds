use shared::herb::MINUTES_PER_DAY;
use shared::poi::ChunkPos;
use shared::tree::TreeStage;
use shared::woodcutting::Damage;

use super::{cell_of, tidied};
use crate::sites::Mirror;
use crate::storage::{SiteId, TreeSave};

const DAY: u64 = MINUTES_PER_DAY;

#[test]
fn damage_is_forgotten_a_day_after_the_last_blow() {
    let save = TreeSave {
        damage: Some(Damage {
            per_mille: 400,
            at: 10,
        }),
        felled: None,
    };
    assert_eq!(tidied(save, 10 + DAY - 1), (save, None));
    assert_eq!(tidied(save, 10 + DAY), (TreeSave::default(), None));
}

#[test]
fn a_felled_tree_is_kept_until_grown() {
    let save = TreeSave {
        damage: None,
        felled: Some(DAY),
    };
    assert_eq!(tidied(save, DAY), (save, Some(TreeStage::Stump)));
    assert_eq!(tidied(save, 3 * DAY), (save, Some(TreeStage::Cleared)));
    assert_eq!(tidied(save, 27 * DAY), (TreeSave::default(), None));
}

#[test]
fn kept_cells_are_found_round_the_planet_and_through_mirrors() {
    let wild = (SiteId::Wilderness, 0);
    let bounds = (990, 0, 20, 10);
    let key = |x, y| (SiteId::Wilderness, 0, (x, y));
    // The planet is 1000 cells round: kept at 5 is drawn at 1005 here.
    assert_eq!(
        cell_of(key(5, 3), wild, Some(1000), &[], bounds),
        Some((1005, 3))
    );
    assert_eq!(
        cell_of(key(995, 3), wild, Some(1000), &[], bounds),
        Some((995, 3))
    );
    assert_eq!(cell_of(key(500, 3), wild, Some(1000), &[], bounds), None);
    let site = SiteId::Poi(ChunkPos { x: 1, y: 2 });
    let mirror = Mirror {
        id: site,
        index: 0,
        offset: (-990, 0),
        bounds: (995, 0, 5, 5),
    };
    let in_site = (site, 0, (7, 2));
    assert_eq!(
        cell_of(in_site, wild, Some(1000), &[mirror], bounds),
        Some((997, 2))
    );
    let outside = (site, 0, (20, 2));
    assert_eq!(cell_of(outside, wild, Some(1000), &[mirror], bounds), None);
}
