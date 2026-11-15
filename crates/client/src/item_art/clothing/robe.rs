//! Robes: long and open down the front, wrapped one side over the other, meeting, or
//! with the deel's curved flap; how they are held, their sleeves, collar, lining, and
//! contrasting edges or embroidery.

use shared::clothing::{
    Collar, Fastening, Front, Length, Lining, Robe, RobeDecoration, RobeLength, RobeSleeves,
};

use super::shape::{FITTED, FRONT, LOOSE, Wear};
use super::strips::{Ease, On};
use crate::item_art::armour::{Model, Stuff};
use crate::item_art::body::blob;

impl Wear<'_> {
    pub(super) fn robe(&self, model: &mut Model, robe: &Robe) {
        let (shoulder, waist) = (self.shoulder(), self.waist());
        let hem = self.length(match robe.length {
            RobeLength::Knee => Length::Knee,
            RobeLength::Calf => Length::Calf,
            RobeLength::Ankle => Length::Ankle,
        });
        let body = match robe.lining {
            Lining::Padded | Lining::Fur => LOOSE + 8.0,
            _ => LOOSE + 2.0,
        };
        let ease = move |height: f32, _: f32| body + (waist - height).max(0.0) * 0.1;
        self.trunk(model, Stuff::Main, &|_| shoulder, &|_| hem, &ease);
        self.yoke(model, Stuff::Main, body, 12.0);
        if robe.lining == Lining::Padded {
            let rows = ((shoulder - hem) / 70.0) as usize;
            for row in 1..rows {
                let height = shoulder - row as f32 * 70.0;
                self.band(model, Stuff::Shade, height, 2.5, ease(height, 0.0));
            }
        }
        let edge = self.front_edge(robe.front, hem);
        let cuff = self.robe_sleeves(model, robe.sleeves);
        // The front edge: a band or a fur facing, contrasting, or a darker line.
        let (stuff, wide) = match (robe.lining, robe.collar, robe.decoration) {
            (Lining::Fur, _, _) => (Stuff::Bone, 40.0),
            (_, Collar::Band, _) => (Stuff::Cloth, 32.0),
            (_, _, RobeDecoration::ContrastEdges) => (Stuff::Cloth, 22.0),
            _ => (Stuff::Shade, 4.0),
        };
        if wide > 10.0 {
            self.strip(model, stuff, &edge, &ease, wide);
            self.neck_ring(model, stuff, 12.0, body, wide * 0.5);
            self.band(model, stuff, hem + wide / 2.0, wide, ease(hem, 0.0));
            if cuff > 0.0 {
                self.cuffs(model, stuff, cuff - wide / 2.0, wide, FITTED);
            }
        } else {
            self.line(model, stuff, &edge, &ease, wide);
        }
        match robe.collar {
            Collar::Standing => self.collar(model, Stuff::Main, 45.0, 12.0, body),
            Collar::Round => self.neck_ring(model, Stuff::Shade, 12.0, body, 4.0),
            Collar::Band | Collar::None => {}
        }
        self.fastening(model, robe.fastening, &edge, &ease);
        if robe.decoration == RobeDecoration::Embroidery {
            for side in [-1.0, 1.0] {
                let angle = FRONT + side * 0.55;
                let down = [(self.chest() + 50.0, angle), (self.chest() - 70.0, angle)];
                self.strip(model, Stuff::Field, &down, &ease, 55.0);
            }
        }
    }

    /// Where the front's edge runs, from the neck down to the hem.
    fn front_edge(&self, front: Front, hem: f32) -> Vec<On> {
        let (shoulder, chest, waist) = (self.shoulder(), self.chest(), self.waist());
        match front {
            Front::Meeting => vec![(shoulder, FRONT), (hem, FRONT)],
            // The left side over the right: its edge runs to the body's right.
            Front::WrapRight | Front::WrapLeft => {
                let side = if front == Front::WrapRight { 1.0 } else { -1.0 };
                vec![
                    (shoulder, FRONT + side * 0.15),
                    (waist, FRONT + side * 0.7),
                    (hem, FRONT + side * 0.75),
                ]
            }
            Front::CurvedFlap => vec![
                (shoulder, FRONT + 0.1),
                (chest, FRONT + 0.75),
                (Self::between(waist, chest, 0.5), FRONT + 1.1),
                (hem, FRONT + 1.15),
            ],
        }
    }

    /// The sleeves, and how far down the arm they end (0 for none).
    fn robe_sleeves(&self, model: &mut Model, sleeves: RobeSleeves) -> f32 {
        let (elbow, wrist) = (self.elbow(), self.wrist());
        match sleeves {
            RobeSleeves::Narrow => {
                self.shoulder_sleeves(model, Stuff::Main, wrist - 10.0, &|_| LOOSE * 0.7);
                wrist - 10.0
            }
            RobeSleeves::Wide => {
                let bell = move |down: f32| 14.0 + 60.0 * (down / wrist).powi(2);
                self.shoulder_sleeves(model, Stuff::Main, wrist, &bell);
                wrist
            }
            RobeSleeves::SquareHanging => {
                self.shoulder_sleeves(model, Stuff::Main, wrist - 30.0, &|_| 25.0);
                self.hanging(model, Stuff::Main, elbow - 80.0, 270.0, 210.0);
                wrist - 30.0
            }
            RobeSleeves::LongHanging => {
                self.shoulder_sleeves(model, Stuff::Main, elbow, &|_| 20.0);
                let long = (self.waist() - self.knee()).max(200.0) + 150.0;
                self.hanging(model, Stuff::Main, 40.0, long, 70.0);
                elbow
            }
        }
    }

    /// How the front is held: a sash at the waist, ties, buttons, hooks or frogs on
    /// the edge.
    fn fastening(&self, model: &mut Model, fastening: Fastening, edge: &[On], ease: Ease) {
        let (shoulder, waist) = (self.shoulder(), self.waist());
        // The edge's angle at a height above the waist.
        let angle_at = |height: f32| {
            edge.windows(2)
                .find(|pair| height <= pair[0].0 && height >= pair[1].0)
                .map_or(edge[0].1, |pair| {
                    let t = (pair[0].0 - height) / (pair[0].0 - pair[1].0).max(1.0);
                    Self::between(pair[0].1, pair[1].1, t)
                })
        };
        let along = |apart: f32| -> Vec<On> {
            let count = ((shoulder - 30.0 - waist) / apart).max(1.0) as usize;
            (0..=count)
                .map(|at| {
                    let height = Self::between(shoulder - 30.0, waist, at as f32 / count as f32);
                    (height, angle_at(height))
                })
                .collect()
        };
        match fastening {
            Fastening::Sash => {
                self.band(model, Stuff::Cloth, waist, 70.0, ease(waist, 0.0) + 2.0);
                let (point, out) = self.at(waist, FRONT - 0.5, ease(waist, 0.0) + 10.0);
                blob(
                    model.of(Stuff::Cloth),
                    point + out * 6.0,
                    bevy::prelude::Vec3::splat(16.0),
                );
            }
            Fastening::Ties => {
                let top = shoulder - 30.0;
                for (height, angle) in [(top, angle_at(top)), (waist, angle_at(waist))] {
                    let (point, out) = self.at(height, angle, ease(height, angle) + 4.0);
                    blob(
                        model.of(Stuff::Shade),
                        point + out * 3.0,
                        bevy::prelude::Vec3::splat(7.0),
                    );
                }
            }
            Fastening::Buttons | Fastening::SideHooks => {
                for (height, angle) in along(40.0) {
                    self.button(model, Stuff::Rivet, (height, angle), ease, 4.0);
                }
            }
            Fastening::Frogs => {
                for (height, angle) in along(45.0) {
                    let across = [(height, angle - 0.15), (height, angle + 0.15)];
                    self.line(model, Stuff::Backing, &across, ease, 4.0);
                }
            }
        }
    }
}
