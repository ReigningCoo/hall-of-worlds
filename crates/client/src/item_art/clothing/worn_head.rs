//! What a wearer's head is like for what goes on it: horns that come out through the
//! cloth (bound slits, decision 14 of the clothing research), a snout or beak wanting
//! a longer face opening, a helmet over it whose bowl the cloth keeps inside, and how
//! far out what is worn on the head reaches, for a hat to sit over it.

use bevy::prelude::*;
use shared::body::{Layout, Snout};

use crate::item_art::Look;
use crate::item_art::body::horns_of;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub(in crate::item_art) struct WornHead {
    /// Each horn's root on the frame things are worn on, the way it leaves the head,
    /// and how thick it is there.
    pub horns: Option<[(Vec3, Vec3, f32); 2]>,
    /// A snout or beak in front of the face.
    pub snout: bool,
    /// A helmet's lower edge, sixteen heights from the front round to the left
    /// (`HeadForm::rim`'s points): cloth above it stays inside the helmet's bowl.
    pub helmet: Option<[f32; 16]>,
    /// How far off the head the clothes worn on it reach where a hat's band sits.
    pub under: Option<f32>,
}

impl WornHead {
    pub(super) fn of(layout: &Layout, worn: &[Look]) -> Self {
        let head = layout.upper_body().unwrap_or(*layout);
        let offset = layout.worn_frame().map_or(Vec3::ZERO, |(_, [x, y, z])| {
            Vec3::new(x as f32, y as f32, z as f32)
        });
        let paths = horns_of(&head);
        let horns = (paths.len() == 2).then(|| {
            std::array::from_fn(|at| {
                let [(root, thick), (next, _), _] = paths[at];
                (root - offset, (next - root).normalize_or(Vec3::Y), thick)
            })
        });
        let snout = head.parts.snout != Snout::Flat && head.parts.snout_length > 0;
        let frame = layout.worn_frame().map(|(frame, _)| frame);
        let mut helmet = None;
        let mut under = None;
        for look in worn {
            let rim = match (look, &frame) {
                (Look::Head { head, .. }, Some(frame)) => Some(head.form.rim(frame)),
                (Look::Set { set, .. }, Some(frame)) => {
                    set.form.head.as_ref().map(|form| form.rim(frame))
                }
                (Look::Clothing { clothing, .. }, _) => {
                    let garment = &clothing.form.garment;
                    if let Some(reach) = super::hat::reach_under(garment) {
                        under = Some(under.map_or(reach, |have: f32| have.max(reach)));
                    }
                    None
                }
                _ => None,
            };
            if let Some(rim) = rim {
                helmet = Some(std::array::from_fn(|at| rim[at % rim.len()][1] as f32));
            }
        }
        Self {
            horns,
            snout,
            helmet,
            under,
        }
    }

    /// The helmet's edge at this angle from the front (towards the left as it grows),
    /// if one is worn.
    pub fn helmet_at(&self, angle: f32) -> Option<f32> {
        let rim = self.helmet?;
        let at = angle.rem_euclid(std::f32::consts::TAU) / std::f32::consts::TAU * 16.0;
        let (first, along) = (at.floor() as usize % 16, at.fract());
        Some(rim[first] + (rim[(first + 1) % 16] - rim[first]) * along)
    }
}
