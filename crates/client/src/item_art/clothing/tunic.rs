//! Tunics: a body flaring below the waist by its cut, a yoke, the neckline, sleeves,
//! how it closes, an overfold, and bands, clavi, embroidery, smocking or brocaded
//! panels.

use shared::clothing::{Closure, Cut, Length, Neck, Sleeves, Tunic, TunicDecoration};

use super::shape::{BACK, FITTED, FRONT, FULL, LEFT, LOOSE, RIGHT, Wear};
use super::strips::Ease;
use crate::item_art::armour::{Model, Stuff};

impl Wear<'_> {
    /// Where a tunic's or robe's hem falls.
    pub(super) fn length(&self, length: Length) -> f32 {
        let hem = match length {
            Length::Hip => self.hip(),
            Length::Thigh => Self::between(self.hip(), self.knee(), 0.5),
            Length::Knee => self.knee() - 20.0,
            Length::Calf => Self::between(self.knee(), self.ankle(), 0.5),
            Length::Ankle => self.ankle() + 30.0,
            Length::Floor => 0.0,
        };
        self.hem(hem)
    }

    pub(super) fn tunic(&self, model: &mut Model, tunic: &Tunic) {
        let (shoulder, waist) = (self.shoulder(), self.waist());
        let hem = self.length(tunic.length);
        let (body, flare) = match tunic.cut {
            Cut::Rectangles => (LOOSE, 0.07),
            Cut::Gored => (LOOSE, 0.2),
            Cut::Fitted => (FITTED, 0.24),
            Cut::Draped => (FULL * 0.7, 0.06),
        };
        let ease = move |height: f32, _: f32| body + (waist - height).max(0.0) * flare;
        let one = tunic.sleeves == Sleeves::OneShoulder;
        // Draped over the left shoulder only: under the right arm.
        let top = move |angle: f32| match one {
            true => shoulder - 130.0 * ((angle - RIGHT).cos() + 0.3).clamp(0.0, 1.0),
            false => shoulder,
        };
        self.trunk(model, Stuff::Main, &top, &|_| hem, &ease);
        let open = if tunic.neck == Neck::Pinned {
            30.0
        } else {
            10.0
        };
        if one {
            self.straps(model, Stuff::Main, &[1.0], shoulder - 120.0, 110.0, body);
            self.brooches(model, &[1.0], body);
        } else {
            self.yoke(model, Stuff::Main, body, open);
            self.tunic_neck(model, tunic.neck, open, body);
        }
        if tunic.overfold {
            let fold = Self::between(waist, shoulder, 0.3);
            let over = move |height: f32, angle: f32| ease(height, angle) + 4.0;
            self.trunk(model, Stuff::Main, &top, &|_| fold, &over);
        }
        let cuff = self.tunic_sleeves(model, tunic);
        self.closure(model, tunic.closure, hem, &ease);
        self.tunic_decoration(model, tunic.decoration, hem, cuff, open, &ease);
    }

    fn tunic_neck(&self, model: &mut Model, neck: Neck, open: f32, ease: f32) {
        match neck {
            Neck::Slit => self.slit(model, FRONT, 110.0, open, ease),
            Neck::ButtonedSlit => {
                self.slit(model, FRONT, 120.0, open, ease);
                let shoulder = self.shoulder();
                let span = (shoulder - 10.0, shoulder - 110.0);
                self.buttons(
                    model,
                    Stuff::Rivet,
                    FRONT + 0.05,
                    span,
                    &|_, _| ease,
                    25.0,
                    3.5,
                );
            }
            Neck::OffCentre => self.slit(model, FRONT - 0.35, 120.0, open, ease),
            Neck::Round => self.neck_ring(model, Stuff::Shade, open, ease, 4.0),
            Neck::Square => self.front_neck(model, 90.0, true, open, ease),
            Neck::V => self.front_neck(model, 120.0, false, open, ease),
            Neck::Pinned => self.brooches(model, &[1.0, -1.0], ease),
        }
    }

    /// The sleeves, and how far down the arm they end (0 for none).
    fn tunic_sleeves(&self, model: &mut Model, tunic: &Tunic) -> f32 {
        let (elbow, wrist) = (self.elbow(), self.wrist());
        let fit = if tunic.cut == Cut::Fitted {
            FITTED
        } else {
            LOOSE * 0.6
        };
        let plain = move |_: f32| fit;
        match tunic.sleeves {
            Sleeves::None | Sleeves::OneShoulder => return 0.0,
            Sleeves::Short => self.shoulder_sleeves(model, Stuff::Main, elbow * 0.45, &plain),
            Sleeves::Elbow => self.shoulder_sleeves(model, Stuff::Main, elbow + 15.0, &plain),
            Sleeves::Wrist => self.shoulder_sleeves(model, Stuff::Main, wrist - 10.0, &plain),
            Sleeves::Wide => {
                let end = elbow + 80.0;
                let bell = move |down: f32| 12.0 + 45.0 * (down / end).powi(2);
                self.shoulder_sleeves(model, Stuff::Main, end, &bell);
                return end;
            }
            Sleeves::Hanging => {
                self.shoulder_sleeves(model, Stuff::Main, wrist - 10.0, &|_| FITTED);
                let long = (self.waist() - self.knee()).max(200.0);
                self.hanging(model, Stuff::Main, elbow - 20.0, long, 110.0);
            }
            Sleeves::LacedOn => {
                self.shoulder_sleeves(model, Stuff::Main, wrist - 10.0, &|_| FITTED);
                self.cuffs(model, Stuff::Shade, 0.0, 4.0, FITTED);
                self.sleeve_ties(model, FITTED);
            }
            Sleeves::Pinned => {
                let end = elbow + 20.0;
                self.shoulder_sleeves(model, Stuff::Main, end, &|_| 22.0);
                return end;
            }
        }
        match tunic.sleeves {
            Sleeves::Short => elbow * 0.45,
            Sleeves::Elbow => elbow + 15.0,
            _ => wrist - 10.0,
        }
    }

    /// Lacing or buttons, from the neck down.
    pub(super) fn closure(&self, model: &mut Model, closure: Closure, hem: f32, ease: Ease) {
        let (shoulder, waist) = (self.shoulder(), self.waist());
        match closure {
            Closure::PulledOn => {}
            Closure::LacedFront => {
                self.lacing(model, FRONT, (shoulder - 15.0, shoulder - 190.0), ease)
            }
            Closure::LacedSide => self.lacing(model, LEFT, (shoulder - 70.0, waist), ease),
            Closure::LacedBack => self.lacing(model, BACK, (shoulder - 15.0, waist), ease),
            Closure::Buttoned => {
                let seam = [(shoulder, FRONT), (hem, FRONT)];
                self.line(model, Stuff::Shade, &seam, ease, 2.5);
                let span = (shoulder - 15.0, waist.max(hem));
                self.buttons(model, Stuff::Rivet, FRONT + 0.06, span, ease, 30.0, 4.0);
            }
        }
    }

    fn tunic_decoration(
        &self,
        model: &mut Model,
        decoration: TunicDecoration,
        hem: f32,
        cuff: f32,
        open: f32,
        ease: Ease,
    ) {
        let shoulder = self.shoulder();
        let hem_ease = ease(hem, 0.0);
        match decoration {
            TunicDecoration::None => {}
            TunicDecoration::Bands | TunicDecoration::Embroidery => {
                let (stuff, high) = match decoration {
                    TunicDecoration::Bands => (Stuff::Cloth, 30.0),
                    _ => (Stuff::Field, 14.0),
                };
                self.band(model, stuff, hem + high / 2.0 + 15.0, high, hem_ease);
                if cuff > 0.0 {
                    self.cuffs(model, stuff, cuff - high / 2.0 - 5.0, high, FITTED);
                }
                self.neck_ring(model, stuff, open, ease(shoulder, 0.0), 7.0);
            }
            TunicDecoration::Clavi => {
                for angle in [FRONT - 0.35, FRONT + 0.35, BACK - 0.35, BACK + 0.35] {
                    let down = [(shoulder - 5.0, angle), (hem + 5.0, angle)];
                    self.strip(model, Stuff::Cloth, &down, ease, 18.0);
                }
            }
            TunicDecoration::Smocking => {
                for row in 0..6 {
                    let height = shoulder - 40.0 - row as f32 * 18.0;
                    let across = [(height, FRONT - 0.7), (height, FRONT + 0.7)];
                    self.line(model, Stuff::Shade, &across, ease, 3.0);
                }
            }
            TunicDecoration::BrocadedPanels => {
                let down = [(shoulder - 25.0, FRONT), (self.waist() + 20.0, FRONT)];
                self.strip(model, Stuff::Field, &down, ease, 110.0);
            }
        }
    }
}
