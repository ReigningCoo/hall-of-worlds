//! Every land biome grows something even at its very hottest and coldest
//! (owner, 2026-11-17: the hottest deserts had nothing).

use super::*;
use crate::climate::{Biome, CELSIUS_AT_ZERO, CELSIUS_PER_UNIT};
use crate::mob::Habitat;

/// The coldest and hottest whole degrees a biome can have, as
/// `Wilderness::habitat_at` rounds them: the climate's temperature runs from 0
/// to 1 (clamped), and `Biome::of` decides the biome from it and moisture.
fn extremes(biome: Biome) -> (i32, i32) {
    let mut found: Option<(i32, i32)> = None;
    for t in 0..=1000 {
        let temperature = f64::from(t) / 1000.0;
        let here = (0..=100).any(|m| Biome::of(temperature, f64::from(m) / 100.0) == biome);
        if !here {
            continue;
        }
        let celsius = (CELSIUS_AT_ZERO + temperature * CELSIUS_PER_UNIT).round() as i32;
        found = Some(found.map_or((celsius, celsius), |(low, high)| {
            (low.min(celsius), high.max(celsius))
        }));
    }
    found.expect("every biome has some temperature")
}

#[test]
fn every_biome_grows_a_herb_at_its_hottest_and_coldest() {
    let herbs = Herbs::built_in();
    let mut bare = Vec::new();
    for biome in Biome::ALL {
        let (coldest, hottest) = extremes(biome);
        for celsius in [coldest, hottest] {
            let habitat = Habitat {
                biome,
                metres: 0,
                celsius,
            };
            if !herbs.iter().any(|def| def.suits(&habitat)) {
                bare.push(format!("{biome:?} at {celsius} °C"));
            }
        }
    }
    assert!(bare.is_empty(), "nothing grows in {bare:?}");
}

#[test]
fn the_hottest_place_is_forty_degrees() {
    // The herbs' table was widened to just above this: if the climate's scale
    // changes, look at `herbs.ron`'s hot desert herbs again.
    assert_eq!(extremes(Biome::Desert).1, 40);
}
