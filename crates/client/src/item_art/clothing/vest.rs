//! Vests and jerkins: sleeveless, to the waist or the hip, with a V at the neck over
//! broad shoulders; buttoned, laced, hooked or worn open; cloth, leather, buff or lined
//! with fur showing at the edges; embroidery over the heart, braid or contrasting
//! edges.

use shared::clothing::{Vest, VestDecoration, VestFastening, VestLength, VestMake};

use super::shape::{FITTED, FRONT, Wear};
use super::strips::On;
use crate::item_art::armour::{Model, Stuff};

/// How far the front is open either side when worn open.
const OPEN: f32 = 0.16;

impl Wear<'_> {
    pub(super) fn vest(&self, model: &mut Model, vest: &Vest) {
        let shoulder = self.shoulder();
        let hem = self.hem(match vest.length {
            VestLength::Waist => self.waist() - 20.0,
            VestLength::Hip => self.hip() + 10.0,
        });
        let body = match vest.make {
            VestMake::FurLined => FITTED + 6.0,
            VestMake::Leather | VestMake::Buff => FITTED + 2.0,
            VestMake::Cloth => FITTED,
        };
        let ease = move |_: f32, _: f32| body;
        // A V down the front to the chest.
        let deep = shoulder - self.chest() + 30.0;
        let top =
            move |angle: f32| shoulder - deep * (((angle - FRONT).cos() - 0.6) / 0.4).max(0.0);
        let bottom = move |_: f32| hem;
        let open = vest.fastening == VestFastening::Open;
        if open {
            self.open_front(model, Stuff::Main, &top, &bottom, &ease, OPEN);
        } else {
            self.trunk(model, Stuff::Main, &top, &bottom, &ease);
        }
        self.straps(
            model,
            Stuff::Main,
            &[1.0, -1.0],
            top(FRONT - 0.5),
            75.0,
            body,
        );
        // The edges: down the V and the front, and the hem.
        let gap = if open { OPEN } else { 0.02 };
        let edges: Vec<Vec<On>> = [-1.0, 1.0]
            .into_iter()
            .map(|side: f32| {
                let mut edge: Vec<On> = (0..=6)
                    .map(|at| {
                        let angle = FRONT + side * (0.93 - at as f32 / 6.0 * (0.93 - gap));
                        (top(angle) - 2.0, angle)
                    })
                    .collect();
                edge.push((hem + 2.0, FRONT + side * gap));
                edge
            })
            .collect();
        let trim = match (vest.make, vest.decoration) {
            (VestMake::FurLined, _) => Some((Stuff::Bone, 26.0)),
            (_, VestDecoration::ContrastEdging) => Some((Stuff::Cloth, 14.0)),
            (_, VestDecoration::Braid) => Some((Stuff::Field, 8.0)),
            _ => None,
        };
        match trim {
            Some((stuff, wide)) => {
                for edge in &edges {
                    self.strip(model, stuff, edge, &ease, wide);
                }
                self.band(model, stuff, hem + wide / 2.0, wide, body);
            }
            None => {
                for edge in &edges {
                    self.line(model, Stuff::Shade, edge, &ease, 3.0);
                }
            }
        }
        let span = (top(FRONT) - 10.0, hem + 15.0);
        match vest.fastening {
            VestFastening::Open => {}
            VestFastening::Buttons => {
                self.buttons(model, Stuff::Rivet, FRONT, span, &ease, 32.0, 4.5)
            }
            VestFastening::Lacing => self.lacing(model, FRONT, span, &ease),
            VestFastening::Hooks => {
                self.buttons(model, Stuff::Rivet, FRONT, span, &ease, 45.0, 2.5)
            }
        }
        if vest.decoration == VestDecoration::Embroidery {
            let heart = FRONT - 0.5;
            let patch = [(self.chest() + 20.0, heart), (self.chest() - 50.0, heart)];
            self.strip(model, Stuff::Field, &patch, &ease, 50.0);
        }
    }
}
