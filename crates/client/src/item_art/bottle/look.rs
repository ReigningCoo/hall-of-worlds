//! `BottleLook`: a bottle's shape, glass and stopper, picked on the client from
//! the bottle's look seed (owner, 2026-11-17: shapes are the client's alone).

use shared::potion::BottleForm;
use shared::rng::Seed;

/// A bottle's outline. Most are turned on a lathe from a side outline; the
/// square, hexagonal, hip flask and heart are not round.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Shape {
    RoundFlask,
    ConeFlask,
    LongNeck,
    Vial,
    Ampoule,
    Onion,
    Gourd,
    Teardrop,
    Jar,
    Ringed,
    Bell,
    Tall,
    Square,
    Hexagonal,
    HipFlask,
    Heart,
}

impl Shape {
    pub const ALL: [Self; 16] = [
        Self::RoundFlask,
        Self::ConeFlask,
        Self::LongNeck,
        Self::Vial,
        Self::Ampoule,
        Self::Onion,
        Self::Gourd,
        Self::Teardrop,
        Self::Jar,
        Self::Ringed,
        Self::Bell,
        Self::Tall,
        Self::Square,
        Self::Hexagonal,
        Self::HipFlask,
        Self::Heart,
    ];
}

/// The glass's shade, seen mostly at its edges and neck so a liquid inside
/// reads the same in any of them.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Glass {
    Clear,
    PaleGreen,
    Amber,
    Cobalt,
    Smoky,
    Violet,
    Rose,
}

impl Glass {
    pub const ALL: [Self; 7] = [
        Self::Clear,
        Self::PaleGreen,
        Self::Amber,
        Self::Cobalt,
        Self::Smoky,
        Self::Violet,
        Self::Rose,
    ];
}

/// What closes it: there is always one.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Stopper {
    Cork,
    /// A cork sealed over with red wax, dripping down the neck.
    Waxed,
    /// A stopper of the bottle's own glass, with a ball on top.
    Glass,
    /// A wooden bung with a cloth tied over it.
    Cloth,
}

impl Stopper {
    pub const ALL: [Self; 4] = [Self::Cork, Self::Waxed, Self::Glass, Self::Cloth];
}

/// Everything a bottle's picture shows of the bottle itself. The liquid (potion
/// bottles PR 3) goes beside it in `Look::Bottle`, from the contents alone.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct BottleLook {
    pub shape: Shape,
    pub glass: Glass,
    pub stopper: Stopper,
}

impl BottleLook {
    /// Picked from the bottle's look seed: any shape alike, clear and pale green
    /// glass and plain corks the commonest.
    pub fn of(form: &BottleForm) -> Self {
        let mut rng = Seed::new(form.look).derive("bottle look").rng();
        let shape = Shape::ALL[rng.range_u64(0, Shape::ALL.len() as u64) as usize];
        let glass = rng.pick_weighted(&[
            (Glass::Clear, 4),
            (Glass::PaleGreen, 3),
            (Glass::Amber, 2),
            (Glass::Cobalt, 2),
            (Glass::Smoky, 1),
            (Glass::Violet, 1),
            (Glass::Rose, 1),
        ]);
        let stopper = rng.pick_weighted(&[
            (Stopper::Cork, 4),
            (Stopper::Waxed, 2),
            (Stopper::Glass, 2),
            (Stopper::Cloth, 2),
        ]);
        Self {
            shape,
            glass,
            stopper,
        }
    }
}
