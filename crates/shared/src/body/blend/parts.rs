//! Blending what two bodies have beyond a person's: sizes move from one to the other;
//! a snout, horns or a coat of another kind shrink to nothing by halfway and the new
//! kind grows from there; feet, wings, a sting, the count of humps and the shapes of
//! ears and tail change at halfway.

use super::super::Parts;
use super::{HALF, WHOLE, mix};

pub(super) fn blend(a: &Parts, b: &Parts, t: i32) -> Parts {
    let (snout, snout_length) = swapped((a.snout, a.snout_length), (b.snout, b.snout_length), t);
    let (horns, horn_length) = swapped((a.horns, a.horn_length), (b.horns, b.horn_length), t);
    let (coat_shape, coat_length) = swapped(
        (a.coat_shape, a.coat_length),
        (b.coat_shape, b.coat_length),
        t,
    );
    let late = t >= HALF;
    Parts {
        snout,
        snout_length,
        ears: mix(a.ears, b.ears, t),
        ear_shape: either(late, a.ear_shape, b.ear_shape),
        horns,
        horn_length,
        tusks: mix(a.tusks, b.tusks, t),
        tail: mix(a.tail, b.tail, t),
        tail_shape: either(late, a.tail_shape, b.tail_shape),
        sting: either(late, a.sting, b.sting),
        abdomen: mix(a.abdomen, b.abdomen, t),
        feet: either(late, a.feet, b.feet),
        wings: either(late, a.wings, b.wings),
        mane: mix(a.mane, b.mane, t),
        tail_hair: mix(a.tail_hair, b.tail_hair, t),
        dewlap: mix(a.dewlap, b.dewlap, t),
        udder: mix(a.udder, b.udder, t),
        comb: mix(a.comb, b.comb, t),
        fringe: mix(a.fringe, b.fringe, t),
        antlers: mix(a.antlers, b.antlers, t),
        tines: mix(a.tines, b.tines, t),
        humps: either(late, a.humps, b.humps),
        hump: mix(a.hump, b.hump, t),
        hump_along: mix(a.hump_along, b.hump_along, t),
        beard: mix(a.beard, b.beard, t),
        wattles: mix(a.wattles, b.wattles, t),
        snood: mix(a.snood, b.snood, t),
        caruncles: mix(a.caruncles, b.caruncles, t),
        knob: mix(a.knob, b.knob, t),
        spurs: mix(a.spurs, b.spurs, t),
        coat_shape,
        coat_length,
    }
}

/// A part of some kind and size: the same kind blends its size; another kind shrinks
/// to nothing by halfway, where the new kind takes its place and grows.
fn swapped<K: PartialEq>(a: (K, i32), b: (K, i32), t: i32) -> (K, i32) {
    if a.0 == b.0 {
        let size = mix(a.1, b.1, t);
        return (a.0, size);
    }
    if t < HALF {
        (a.0, mix(a.1, 0, t * 2))
    } else {
        (b.0, mix(0, b.1, (t - HALF) * 2))
    }
}

/// Every size of these parts at this many thousandths.
pub(in crate::body) fn scaled(parts: &Parts, share: i32) -> Parts {
    let size = |x: i32| x * share / WHOLE;
    Parts {
        snout_length: size(parts.snout_length),
        ears: size(parts.ears),
        horn_length: size(parts.horn_length),
        tusks: size(parts.tusks),
        tail: size(parts.tail),
        abdomen: size(parts.abdomen),
        mane: size(parts.mane),
        tail_hair: size(parts.tail_hair),
        dewlap: size(parts.dewlap),
        udder: size(parts.udder),
        comb: size(parts.comb),
        fringe: size(parts.fringe),
        antlers: size(parts.antlers),
        hump: size(parts.hump),
        beard: size(parts.beard),
        wattles: size(parts.wattles),
        snood: size(parts.snood),
        caruncles: size(parts.caruncles),
        knob: size(parts.knob),
        spurs: size(parts.spurs),
        coat_length: size(parts.coat_length),
        ..*parts
    }
}

/// The first body's before halfway, the second's from then on.
fn either<T>(late: bool, a: T, b: T) -> T {
    if late { b } else { a }
}
