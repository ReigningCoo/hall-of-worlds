//! The tunic's tables of weighted choices, each going by the choices before it.

use super::{Closure, Cut, Length, Neck, Sleeves, Tunic, TunicDecoration};
use crate::underclothes::table::when;

pub(super) const LENGTHS: [(Length, u64); 6] = [
    (Length::Hip, 10),
    (Length::Thigh, 18),
    (Length::Knee, 30),
    (Length::Calf, 20),
    (Length::Ankle, 18),
    (Length::Floor, 4),
];

pub(super) fn cuts(length: Length) -> [(Cut, u64); 4] {
    [
        (Cut::Rectangles, 50),
        (Cut::Gored, when(length >= Length::Knee, 25)),
        (Cut::Fitted, 15),
        (Cut::Draped, when(length >= Length::Thigh, 10)),
    ]
}

pub(super) fn sleeves(length: Length, cut: Cut) -> [(Sleeves, u64); 9] {
    use Sleeves::*;
    if cut == Cut::Draped {
        let short = length <= Length::Knee;
        return [
            (None, 30),
            (Short, 0),
            (Elbow, 0),
            (Wrist, 0),
            (Wide, 0),
            (Hanging, 0),
            (LacedOn, 0),
            (Pinned, 50),
            (OneShoulder, when(short, 20)),
        ];
    }
    let fitted = cut == Cut::Fitted;
    [
        (None, 8),
        (Short, 12),
        (Elbow, 12),
        (Wrist, 50),
        (Wide, when(cut == Cut::Rectangles, 8)),
        (Hanging, when(fitted && length >= Length::Ankle, 6)),
        (LacedOn, when(fitted, 4)),
        (Pinned, 0),
        (OneShoulder, 0),
    ]
}

pub(super) fn closures(length: Length, cut: Cut, sleeves: Sleeves) -> [(Closure, u64); 5] {
    use Closure::*;
    if cut != Cut::Fitted {
        return [
            (PulledOn, 1),
            (LacedFront, 0),
            (LacedSide, 0),
            (LacedBack, 0),
            (Buttoned, 0),
        ];
    }
    let hanging = sleeves == Sleeves::Hanging;
    [
        (PulledOn, 0),
        (LacedFront, when(!hanging, 30)),
        (LacedSide, 30),
        (LacedBack, when(!hanging, 15)),
        (Buttoned, when(!hanging && length <= Length::Knee, 25)),
    ]
}

pub(super) fn necks(cut: Cut, closure: Closure) -> [(Neck, u64); 7] {
    use Neck::*;
    if cut == Cut::Draped {
        return [
            (Slit, 0),
            (ButtonedSlit, 0),
            (Round, 0),
            (Square, 0),
            (V, 0),
            (OffCentre, 0),
            (Pinned, 1),
        ];
    }
    let loose = closure == Closure::PulledOn;
    [
        (Slit, 30),
        (ButtonedSlit, when(loose, 8)),
        (Round, 30),
        (Square, when(cut == Cut::Fitted, 10)),
        (V, 8),
        (OffCentre, when(loose, 5)),
        (Pinned, 0),
    ]
}

pub(super) fn decorations(tunic: &Tunic) -> [(TunicDecoration, u64); 6] {
    use TunicDecoration::*;
    let rectangles = tunic.cut == Cut::Rectangles;
    let plain_neck = matches!(tunic.neck, Neck::Slit | Neck::Round);
    [
        (None, 50),
        (Bands, 25),
        (
            Clavi,
            when(rectangles && plain_neck && tunic.length >= Length::Knee, 6),
        ),
        (Embroidery, 15),
        (
            Smocking,
            when(
                rectangles && tunic.sleeves == Sleeves::Wrist && tunic.length >= Length::Knee,
                3,
            ),
        ),
        (
            BrocadedPanels,
            when(rectangles && tunic.sleeves == Sleeves::None, 8),
        ),
    ]
}
