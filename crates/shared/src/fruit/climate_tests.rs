//! Every land biome grows some fruit even at its very hottest and coldest, as
//! herbs do (`herb::climate_tests`).

use super::*;
use crate::climate::Biome;
use crate::herb::climate_tests::extremes;

#[test]
fn every_biome_grows_a_fruit_at_its_hottest_and_coldest() {
    let mut bare = Vec::new();
    for biome in Biome::ALL {
        let (coldest, hottest) = extremes(biome);
        for celsius in [coldest, hottest] {
            let grows = Fruits::built_in()
                .items()
                .any(|def| def.biomes.contains(&biome) && def.temperature.suits(celsius as i16));
            if !grows {
                bare.push(format!("{biome:?} at {celsius} °C"));
            }
        }
    }
    assert!(bare.is_empty(), "no fruit grows in {bare:?}");
}
