//! Animals from the creature research (`docs/context/research-creatures.md`): beasts
//! on four legs and birds on two, each as a few measurements their layout is worked
//! out from (`beasts/build.rs`). Unlike a biped's frame, a beast's layout is at its own
//! usual size, in millimetres: it fills its picture whatever its size.

use super::Layout;
use super::measured::Measured;
use super::measured_heads::MeasuredHead;
use super::parts::Parts;
use crate::creature::CreatureKind;
use chicken_head::CHICKEN_HEAD;
pub use sexes::{Sex, Sexes};
pub use table::BEASTS;

mod build;
mod chicken_head;
mod drawn_heads;
mod livestock;
mod livestock_sexes;
mod sexes;
mod table;

/// A beast standing on four legs, level-backed.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Fourfoot {
    /// The top of the back over the shoulders, from the ground.
    pub withers: i32,
    /// Along the back from the hip joints to the shoulder joints.
    pub body: i32,
    /// The chest's half width and half depth.
    pub barrel: (i32, i32),
    /// The forelegs' and hind legs' thickness as radii, where each hangs from and at
    /// each joint down to the last above the ground, measured with the trunk.
    pub fore: [i32; 4],
    pub hind: [i32; 4],
    /// How long a foot is along the ground.
    pub foot: i32,
    /// The neck's length, its lean forwards from upright in degrees, and its radius
    /// where it meets the head (it widens to the trunk's last ring).
    pub neck: (i32, i32, i32),
    /// The head: its outline (`HeadShape`) scaled to this long from the back of the
    /// skull to the nose, leaning this many degrees nose down.
    pub head: (i32, i32, &'static MeasuredHead),
    /// How much thicker the coat makes the trunk all round than the measured shape.
    pub coat: i32,
    /// The trunk's shape, as measured, scaled to this size.
    pub measured: &'static Measured,
}

/// A bird on two legs, its back leaning forwards.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Bird {
    /// How high its hips are.
    pub hips: i32,
    /// Along the back from the hips to the neck, and how far the back leans forwards
    /// from upright, in degrees.
    pub body: (i32, i32),
    pub barrel: (i32, i32),
    pub leg: i32,
    pub neck: (i32, i32, i32),
    /// As a beast's: its outline's length, its lean and its outline.
    pub head: (i32, i32, &'static MeasuredHead),
}

/// How a beast's body is put together.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Plan {
    Fourfoot(Fourfoot),
    Bird(Bird),
}

/// An animal, by name, with its body's measurements and what it has beyond a plain
/// trunk and head.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Beast {
    pub name: &'static str,
    pub plan: Plan,
    pub parts: Parts,
    /// How its males and females differ, if they do: `parts` is a neutral one's.
    pub sexes: Option<Sexes>,
}

impl Beast {
    /// The animal of this kind, by its name.
    pub fn of(kind: CreatureKind) -> Option<Beast> {
        BEASTS
            .into_iter()
            .find(|beast| beast.name == kind.def().name)
    }

    /// Its body's layout, at its own usual size.
    pub fn layout(&self) -> Layout {
        match self.plan {
            Plan::Fourfoot(four) => build::fourfoot(&four, self.parts),
            Plan::Bird(bird) => build::bird(&bird, self.parts),
        }
    }
}
