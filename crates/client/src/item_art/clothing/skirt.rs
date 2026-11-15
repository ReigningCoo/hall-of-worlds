//! Skirts: a cone from the waist (or up to the chest) flaring by how it is made:
//! gathered and pleated full, gored, a tube, wrapped round with an overlap, the dhoti's
//! folds, or fibres hanging from a band; an apron front, pleats and gathers as lines,
//! and the waist by how it is held. Worn under a tunic it starts at the tunic's hem;
//! a tail comes out through a slit, and a taur's upper body has it only to the hips.

use std::f32::consts::TAU;

use shared::clothing::{Skirt, SkirtFront, SkirtLength, SkirtMake, SkirtWaist};

use super::shape::{CLOSE, FITTED, FRONT, LEFT, Wear};
use super::strips::Ease;
use crate::item_art::armour::{Model, Stuff};
use crate::item_art::body::limb;

impl Wear<'_> {
    pub(super) fn skirt(&self, model: &mut Model, skirt: &Skirt) {
        let (waist, knee, ankle) = (self.waist(), self.knee(), self.ankle());
        let hem = self.hem(match skirt.length {
            SkirtLength::Knee => knee - 20.0,
            SkirtLength::Calf => Self::between(knee, ankle, 0.5),
            SkirtLength::Ankle => ankle + 30.0,
            SkirtLength::Floor => 0.0,
        });
        let (base, flare) = match skirt.make {
            SkirtMake::Gathered => (FITTED, 0.3),
            SkirtMake::Pleated => (FITTED, 0.22),
            SkirtMake::ManyPleats => (FITTED, 0.38),
            SkirtMake::Gored => (FITTED, 0.24),
            SkirtMake::Tube => (CLOSE, 0.03),
            SkirtMake::Wrapped => (FITTED, 0.06),
            SkirtMake::BetweenLegs => (FITTED, 0.08),
            SkirtMake::Fibres => (FITTED, 0.14),
        };
        let wanted = match skirt.waist {
            SkirtWaist::Chest => Self::between(waist, self.chest(), 0.85),
            _ => waist + 15.0,
        };
        // Under a tunic it is hidden above the tunic's hem, and flares from there.
        let under = self
            .wearer()
            .and_then(|wearer| wearer.torso_hem)
            .map_or(f32::MAX, |hem| hem + 30.0);
        let top = wanted.min(under);
        if top <= hem + 10.0 {
            return;
        }
        let from = waist.min(top);
        let ease = move |height: f32, _: f32| base + (from - height).max(0.0) * flare;
        if skirt.make == SkirtMake::Fibres {
            self.fibres(model, (top, hem), &ease);
        } else {
            self.trunk(model, Stuff::Main, &|_| top, &|_| hem, &ease);
            self.skirt_lines(model, skirt, (top, hem), &ease);
        }
        if top >= wanted {
            self.skirt_waist(model, skirt.waist, top, base);
        }
    }

    /// Pleats, gathers, gores, an apron front, a wrap's overlap and the dhoti's folds.
    fn skirt_lines(&self, model: &mut Model, skirt: &Skirt, (top, hem): (f32, f32), ease: Ease) {
        let apron = skirt.front == SkirtFront::Apron;
        let (every, from) = match skirt.make {
            SkirtMake::Pleated => (0.24, self.hip()),
            SkirtMake::ManyPleats => (0.13, top - 20.0),
            SkirtMake::Gathered => (0.18, top),
            SkirtMake::Gored => (TAU / 6.0, top),
            _ => (0.0, top),
        };
        if every > 0.0 {
            let count = (TAU / every) as usize;
            for at in 0..count {
                let angle = at as f32 * TAU / count as f32;
                // An apron front lies flat over the pleats.
                let from_front = (angle - FRONT).abs();
                if apron && from_front < 0.55 {
                    continue;
                }
                // Gathers show only near the waist.
                let low = match skirt.make {
                    SkirtMake::Gathered => from - 90.0,
                    _ => hem + 4.0,
                };
                let line = [(from - 4.0, angle), (low.max(hem + 4.0), angle)];
                self.line(model, Stuff::Shade, &line, ease, 2.0);
            }
        }
        if apron {
            let panel = [(top - 4.0, FRONT), (hem + 2.0, FRONT)];
            let over = |height: f32, angle: f32| ease(height, angle) + 2.0;
            self.strip(model, Stuff::Main, &panel, &over, 170.0);
        }
        match skirt.make {
            SkirtMake::Wrapped => {
                // The overlap's edge, down the front a little to the left.
                let edge = [(top, LEFT + 1.1), (hem + 2.0, LEFT + 1.25)];
                self.line(model, Stuff::Shade, &edge, ease, 3.0);
            }
            SkirtMake::BetweenLegs => {
                // The dhoti's pleated front, tucked at the waist.
                for side in [-1.0, 1.0] {
                    let fold = [(top - 10.0, FRONT + side * 0.06), (hem, FRONT + side * 0.1)];
                    self.line(model, Stuff::Shade, &fold, ease, 2.5);
                }
                let gather = [(top - 10.0, FRONT), (hem + 60.0, FRONT)];
                let over = |height: f32, angle: f32| ease(height, angle) + 3.0;
                self.strip(model, Stuff::Main, &gather, &over, 50.0);
            }
            _ => {}
        }
    }

    /// Grass or tufts hanging in two rows from the band.
    fn fibres(&self, model: &mut Model, (top, hem): (f32, f32), ease: Ease) {
        let count = 56;
        for row in 0..2 {
            for at in 0..count {
                let angle = (at as f32 + row as f32 * 0.5) / count as f32 * TAU;
                let from = top - 20.0;
                if self.in_slit(hem + 1.0, angle) {
                    continue;
                }
                let strand = |height: f32| {
                    let (point, out) =
                        self.at(height, angle, ease(height, angle) + row as f32 * 3.0);
                    point + out * 2.5
                };
                let path = [
                    (strand(from), 3.0),
                    (strand(Self::between(from, hem, 0.5)), 2.6),
                    (strand(hem), 1.6),
                ];
                limb(model.of(Stuff::Main), &path);
            }
        }
    }

    /// How a skirt is held at the top.
    fn skirt_waist(&self, model: &mut Model, waist: SkirtWaist, top: f32, ease: f32) {
        let over = move |_: f32, _: f32| ease + 2.0;
        match waist {
            SkirtWaist::Drawstring => {
                self.band(model, Stuff::Shade, top - 8.0, 14.0, ease);
                for side in [-1.0, 1.0] {
                    let cord = [
                        (top - 8.0, FRONT + side * 0.04),
                        (top - 80.0, FRONT + side * 0.07),
                    ];
                    self.line(model, Stuff::Backing, &cord, &over, 2.5);
                }
            }
            SkirtWaist::Ties => {
                self.band(model, Stuff::Shade, top - 15.0, 30.0, ease);
                let tie = [(top - 15.0, LEFT + 0.2), (top - 110.0, LEFT + 0.15)];
                self.line(model, Stuff::Shade, &tie, &over, 5.0);
            }
            SkirtWaist::Tucked => self.band(model, Stuff::Main, top - 10.0, 22.0, ease + 2.0),
            SkirtWaist::Belt => {
                self.band(model, Stuff::Backing, top - 12.0, 24.0, ease);
                self.button(model, Stuff::Rivet, (top - 12.0, FRONT), &over, 7.0);
            }
            SkirtWaist::Chest => self.band(model, Stuff::Cloth, top - 18.0, 36.0, ease),
        }
    }
}
