use super::*;
use shared::fruit::Fruits;

const GROWING: [TreeStage; 4] = [
    TreeStage::Sapling,
    TreeStage::Young,
    TreeStage::HalfGrown,
    TreeStage::Grown,
];

/// How tall a model stands, and how far it reaches from its middle.
fn extent(pieces: &Pieces) -> (f32, f32) {
    let points = pieces.fill.positions.iter();
    points.fold((0.0f32, 0.0f32), |(high, far), p| {
        (high.max(p[1]), far.max(Vec2::new(p[0], p[2]).length()))
    })
}

/// Every wild kind has a model at every stage and in every variant, a stump
/// and nothing once cleared, each outlined, kept within what a click on the
/// grown tree takes in, and small enough to draw many.
#[test]
fn every_wild_tree_is_built_at_every_stage() {
    for kind in TreeKind::all() {
        for variant in 0..VARIANTS {
            assert!(wild(kind, TreeStage::Cleared, variant).is_none());
            let stump = wild(kind, TreeStage::Stump, variant).expect("a stump");
            assert!(!stump.outline.is_empty());
            let (height, reach) = tree_size(kind, TreeStage::Stump).expect("a size");
            let (high, far) = extent(&stump);
            assert!(high <= height && far <= reach, "{kind:?}'s stump");
            let mut last = 0.0;
            for stage in GROWING {
                let pieces = wild(kind, stage, variant).expect("a tree");
                assert!(!pieces.outline.is_empty(), "{kind:?} {stage:?}");
                assert!(pieces.fill.len() <= 800, "{kind:?}: {}", pieces.fill.len());
                let (high, far) = extent(&pieces);
                let (height, reach) = tree_size(kind, stage).expect("a size");
                assert!(high > last, "{kind:?} {stage:?} grows");
                assert!(high <= height && far <= reach, "{kind:?} {stage:?}");
                last = high;
            }
            assert!(extent(&stump).0 < last * 0.3, "{kind:?}'s stump is low");
        }
    }
}

/// Every fruit plant with a trunk has a stump and grows back through smaller
/// models of itself; nothing is drawn while cleared.
#[test]
fn felled_fruit_trees_have_stumps_and_grow_back() {
    for def in Fruits::built_in()
        .iter()
        .filter(|def| def.plant.has_trunk())
    {
        assert!(fruit(def, TreeStage::Cleared, 0).is_none());
        let stump = fruit(def, TreeStage::Stump, 0).expect("a stump");
        assert!(!stump.outline.is_empty(), "{}", def.key);
        let mut last = extent(&stump).0 * 0.5;
        for stage in &GROWING[..3] {
            let pieces = fruit(def, *stage, 0).expect("a tree");
            let high = extent(&pieces).0;
            assert!(high > last, "{} {stage:?}", def.key);
            last = high;
        }
    }
}

/// A grey copy keeps every corner and has only greys.
#[test]
fn memory_is_a_grey_copy() {
    let pieces = wild(TreeKind::Oak, TreeStage::Grown, 0).expect("an oak");
    let grey = greyed(&pieces.fill);
    assert_eq!(grey.positions, pieces.fill.positions);
    assert!(grey.colours.iter().all(|c| c[0] == c[1] && c[1] == c[2]));
}
