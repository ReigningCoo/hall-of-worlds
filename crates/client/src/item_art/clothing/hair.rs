//! How much of the hair the clothes on the head hide: a lowest edge round the head,
//! as a helmet's rim is, so hair above it isn't drawn and long hair hangs below it.

use std::f32::consts::TAU;

use shared::body::Frame;
use shared::clothing::{ClothingForm, Garment, HeadclothForm, HeaddressForm, HeaddressVeil, Reach};

use super::shape::{Fit, Wear};

/// The heights of a piece's lower edge on the head, sixteen from the front round to the
/// left, on this frame; `None` for one not on the head.
pub(in crate::item_art) fn hair_rim(form: &ClothingForm, frame: &Frame) -> Option<[f32; 16]> {
    let wear = Wear {
        frame,
        fit: Fit::Alone,
        thick: 1.0,
    };
    let level = |down: f32| wear.head_y(down);
    let neck = frame.neck_base as f32;
    // From `front` at the front to `back` behind.
    let round = |front: f32, back: f32| -> [f32; 16] {
        std::array::from_fn(|at| {
            let behind = (1.0 - (at as f32 / 16.0 * TAU).cos()) / 2.0;
            front + (back - front) * behind
        })
    };
    Some(match &form.garment {
        Garment::Hood(hood) if hood.chaperon => round(level(0.42), level(0.42)),
        Garment::Hood(_) => round(level(1.0) - 15.0, neck),
        Garment::Cap(cap) if cap.ear_flaps => round(level(0.42), level(0.6)),
        Garment::Cap(_) => round(level(0.42), level(0.45)),
        Garment::Headcloth(cloth) => match (cloth.form, cloth.reach) {
            (HeadclothForm::Turban | HeadclothForm::HeadTie, _) => round(level(0.46), level(0.5)),
            (_, Reach::Head) => round(level(0.3), level(0.62)),
            _ => round(level(0.3), neck),
        },
        Garment::Headdress(dress) => {
            let back = match (dress.form, dress.veil) {
                (_, HeaddressVeil::Behind) => neck,
                (HeaddressForm::FrenchHood | HeaddressForm::Gable, _) => level(0.62),
                _ => level(0.5),
            };
            round(level(0.32), back)
        }
        Garment::Hat(_) => round(level(0.38), level(0.38)),
        _ => return None,
    })
}
