use shared::fungus::{Form, Fungi};

use super::super::models::VARIANTS;
use super::patch::{REACH, build, growth};

/// Every fungus builds at every stage and variant, staying within its cell
/// and under a sensible number of triangles.
#[test]
fn every_fungus_builds_within_its_cell() {
    let mut most = 0;
    for def in Fungi::built_in().iter() {
        for stage in 1..=4 {
            for variant in 0..VARIANTS {
                let shell = build(def, stage, variant);
                assert!(!shell.is_empty(), "{} {stage} {variant}", def.key);
                most = most.max(shell.len());
                for point in &shell.positions {
                    let out = point.x.hypot(point.z);
                    assert!(out <= REACH * 1.001, "{} reaches {out} at {stage}", def.key);
                    assert!(point.is_finite(), "{}", def.key);
                }
            }
        }
    }
    assert!(most <= 7_000, "{most} triangles in one patch");
}

/// A young patch is smaller than a full-grown one.
#[test]
fn young_ones_are_smaller() {
    assert!(growth(1) < growth(2) && growth(2) < growth(3) && growth(3) < growth(4));
    for def in Fungi::built_in().iter() {
        if def.look.form == Form::Truffle {
            continue;
        }
        let top = |stage| {
            (build(def, stage, 0).positions.iter())
                .map(|p| p.y.abs() + p.x.hypot(p.z))
                .fold(0.0f32, f32::max)
        };
        // Wood they grow on is the same size at every stage.
        assert!(top(1) <= top(4) + 0.01, "{}", def.key);
    }
}

/// Outlines and memory keep the fill's points and triangles.
#[test]
fn three_meshes_match() {
    let def = Fungi::built_in().iter().next().expect("a fungus");
    let shell = build(def, 4, 0);
    for mesh in [shell.fill(), shell.outline(1.0), shell.memory()] {
        assert_eq!(mesh.count_vertices(), shell.positions.len());
        assert_eq!(mesh.indices().map(|i| i.len()), Some(shell.indices.len()));
    }
}
