use shared::alchemy::{Apparatus, CERTAIN};

use super::bench::Bench;
use super::markup::ALCHEMY;
use super::run::stage_colour;
use super::spaces::{MOST_HERBS, Place, places, put};
use super::words::chance_words;
use bevy::prelude::*;
use shared::id::Id;

#[test]
fn markup_is_good() {
    ALCHEMY.assert_good();
}

#[test]
fn steps_go_in_move_and_come_out() {
    let mut bench = Bench::default();
    bench.insert(0, Apparatus::Retort);
    bench.insert(0, Apparatus::Mortar);
    bench.insert(9, Apparatus::Retort);
    let order = |bench: &Bench| bench.steps.iter().map(|s| s.apparatus).collect::<Vec<_>>();
    use Apparatus::{Mortar, Retort};
    assert_eq!(order(&bench), [Mortar, Retort, Retort]);
    // The Mortar dragged to the end.
    bench.shift(0, 3);
    assert_eq!(order(&bench), [Retort, Retort, Mortar]);
    // And back before the first.
    bench.shift(2, 0);
    assert_eq!(order(&bench), [Mortar, Retort, Retort]);
    bench.remove(1);
    assert_eq!(order(&bench), [Mortar, Retort]);
}

#[test]
fn spaces_grow_as_they_fill() {
    let mut bench = Bench::default();
    bench.insert(0, Apparatus::Mortar);
    bench.insert(1, Apparatus::Retort);
    assert_eq!(
        places(0, &bench.steps[0]),
        [Place::Herb(0, 0), Place::Oil(0)]
    );
    assert_eq!(
        places(1, &bench.steps[1]),
        [Place::Liquid(1, 0), Place::Solid(1)]
    );
    for n in 0..MOST_HERBS + 2 {
        put(
            &mut bench,
            Place::Herb(0, n),
            Some(Id::stand_in(n as u64 + 1)),
        );
    }
    assert_eq!(places(0, &bench.steps[0]).len(), MOST_HERBS + 1);
    // Taking one out closes the gap.
    put(&mut bench, Place::Herb(0, 0), None);
    assert_eq!(bench.steps[0].herbs[0], Id::stand_in(2));
    // A liquid may be listed twice: two 100 ml from one stack.
    put(&mut bench, Place::Liquid(1, 0), Some(Id::stand_in(50)));
    put(&mut bench, Place::Liquid(1, 1), Some(Id::stand_in(50)));
    let order = bench.order(1);
    let shared::alchemy::OrderStep::Retort { liquids, .. } = &order.steps[1] else {
        panic!("a Retort");
    };
    assert_eq!(liquids, &[Id::stand_in(50), Id::stand_in(50)]);
}

#[test]
fn chance_in_words() {
    assert_eq!(chance_words(CERTAIN), "Chance 100%");
    assert_eq!(chance_words(CERTAIN / 8), "Chance 12.5%");
    assert_eq!(chance_words(CERTAIN * 3 / 2), "Chance 150%");
}

#[test]
fn colour_runs_black_white_gold_product() {
    let product = Color::srgb(0.0, 0.0, 1.0);
    assert_eq!(
        stage_colour(0.0, product).to_srgba(),
        Color::BLACK.to_srgba()
    );
    let white = stage_colour(1.0 / 3.0, product).to_srgba();
    assert!(white.red > 0.99 && white.blue > 0.99);
    assert_eq!(stage_colour(1.0, product).to_srgba(), product.to_srgba());
}
