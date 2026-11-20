use bevy::prelude::*;
use shared::crafting_bench::Craft;

use super::{REACH, TALLEST, tool_shell};

/// Every tool a bench takes stands on its foot within the room it has, and is
/// simple enough for the map.
#[test]
fn every_tool_is_small_and_stands_on_its_foot() {
    let (mut meshes, mut materials) = (Assets::<Mesh>::default(), Assets::default());
    for kind in Craft::Alchemy.tools() {
        let shell = tool_shell(kind, &mut meshes, &mut materials).expect("a model");
        let low = (shell.positions.iter()).fold(f32::MAX, |low, p| low.min(p.y));
        let high = (shell.positions.iter()).fold(f32::MIN, |high, p| high.max(p.y));
        let across =
            (shell.positions.iter()).fold(0.0f32, |most, p| most.max(p.x.abs().max(p.z.abs())));
        assert!(low.abs() < 0.3, "{kind:?} stands at {low}");
        assert!(
            high <= TALLEST + 0.3 && high > TALLEST * 0.3,
            "{kind:?} is {high} tall"
        );
        assert!(across <= REACH + 0.3, "{kind:?} reaches {across}");
        assert!(shell.len() < 3000, "{kind:?}: {} triangles", shell.len());
    }
}
