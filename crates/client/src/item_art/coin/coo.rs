//! The coins' emboss, in place of a ruler's head: the owner's "Reigning Coo", a
//! shaggy highland cow's head from the front wearing a crown, traced by hand from
//! `spare_assets/coin/reigning-coo.webp` into flat outlines with heights, lowest
//! first. Bold and simple, as a coin is shown at about 32 pixels; the horns are
//! drawn in so their tips stay inside the rim.

use super::outline::{Outline, Raise};

/// Every outline of the coo, lowest first, in the face's units (radius 1, y up).
pub(super) fn coo() -> Vec<Outline> {
    let mut outlines = Vec::new();
    outlines.extend(ears());
    outlines.push(head());
    outlines.extend(horns());
    outlines.extend(eyes());
    outlines.extend(fringe());
    outlines.extend(muzzle());
    outlines.extend(crown());
    outlines
}

/// The ears, shaggy lobes out to the sides under the horns.
fn ears() -> [Outline; 2] {
    let right = Outline::new(
        &[
            (0.34, 0.30),
            (0.56, 0.30),
            (0.70, 0.24),
            (0.80, 0.17),
            (0.74, 0.12),
            (0.82, 0.06),
            (0.72, 0.02),
            (0.76, -0.05),
            (0.62, -0.06),
            (0.66, -0.12),
            (0.50, -0.10),
            (0.36, -0.04),
        ],
        Raise::To(0.38),
    );
    [right.mirrored(), right]
}

/// The head: broad at the brow, narrowing in shaggy steps to the chin.
fn head() -> Outline {
    Outline::symmetric(
        &[
            (0.0, 0.42),
            (0.20, 0.44),
            (0.34, 0.38),
            (0.40, 0.20),
            (0.42, 0.0),
            (0.44, -0.16),
            (0.47, -0.24),
            (0.40, -0.30),
            (0.43, -0.40),
            (0.35, -0.46),
            (0.37, -0.56),
            (0.29, -0.62),
            (0.28, -0.72),
            (0.18, -0.76),
            (0.0, -0.78),
        ],
        Raise::To(0.5),
    )
}

/// The wide horns, out from under the crown and sweeping up at the tips.
fn horns() -> [Outline; 2] {
    let right = Outline::new(
        &[
            (0.26, 0.36),
            (0.40, 0.31),
            (0.55, 0.33),
            (0.66, 0.39),
            (0.73, 0.48),
            (0.75, 0.57),
            (0.70, 0.66),
            (0.68, 0.57),
            (0.63, 0.49),
            (0.54, 0.44),
            (0.42, 0.42),
            (0.28, 0.44),
        ],
        Raise::To(0.62),
    );
    [right.mirrored(), right]
}

/// Eyes peering out beside the fringe.
fn eyes() -> [Outline; 2] {
    [-1.0, 1.0].map(|side| Outline::oval((0.37 * side, -0.07), (0.04, 0.035), Raise::To(0.64)))
}

/// The fringe hanging over the eyes: one mass with a strand's point at the
/// bottom of each lock, the middle one longest and a little off to the left,
/// and shallow grooves between the locks.
fn fringe() -> [Outline; 5] {
    let fringe = Outline::new(
        &[
            (-0.34, 0.42),
            (0.34, 0.42),
            (0.38, 0.26),
            (0.31, 0.02),
            (0.24, 0.17),
            (0.16, -0.05),
            (0.09, 0.10),
            (0.0, -0.21),
            (-0.08, 0.08),
            (-0.18, -0.09),
            (-0.24, 0.15),
            (-0.32, 0.04),
            (-0.39, 0.24),
        ],
        Raise::To(0.74),
    );
    let groove = |top: (f32, f32), bottom: (f32, f32)| {
        Outline::new(
            &[
                (top.0 - 0.012, top.1),
                (top.0 + 0.012, top.1),
                (bottom.0 + 0.004, bottom.1),
                (bottom.0 - 0.004, bottom.1),
            ],
            Raise::Dent(0.22),
        )
    };
    [
        fringe,
        groove((0.21, 0.42), (0.24, 0.20)),
        groove((0.08, 0.42), (0.09, 0.13)),
        groove((-0.09, 0.42), (-0.08, 0.11)),
        groove((-0.22, 0.42), (-0.24, 0.18)),
    ]
}

/// The broad muzzle, its nostrils pressed in.
fn muzzle() -> [Outline; 3] {
    let muzzle = Outline::symmetric(
        &[
            (0.0, -0.30),
            (0.14, -0.31),
            (0.24, -0.38),
            (0.28, -0.50),
            (0.24, -0.62),
            (0.14, -0.68),
            (0.0, -0.69),
        ],
        Raise::To(0.86),
    );
    let nostril = |side: f32| Outline::oval((0.12 * side, -0.49), (0.065, 0.045), Raise::Dent(0.4));
    [muzzle, nostril(-1.0), nostril(1.0)]
}

/// The crown: five points over a curved band, a ball on each point, and three
/// stones set in the band.
fn crown() -> Vec<Outline> {
    let mut crown = vec![
        Outline::new(
            &[
                (-0.35, 0.50),
                (-0.36, 0.64),
                (-0.26, 0.56),
                (-0.18, 0.70),
                (-0.09, 0.58),
                (0.0, 0.77),
                (0.09, 0.58),
                (0.18, 0.70),
                (0.26, 0.56),
                (0.36, 0.64),
                (0.35, 0.50),
            ],
            Raise::To(0.9),
        ),
        Outline::new(
            &[
                (-0.34, 0.40),
                (0.0, 0.43),
                (0.34, 0.40),
                (0.35, 0.53),
                (0.0, 0.56),
                (-0.35, 0.53),
            ],
            Raise::To(1.0),
        ),
    ];
    let tips = [
        (-0.36, 0.66),
        (-0.18, 0.72),
        (0.0, 0.80),
        (0.18, 0.72),
        (0.36, 0.66),
    ];
    crown.extend(tips.map(|tip| Outline::oval(tip, (0.038, 0.038), Raise::To(1.0))));
    let stones = [(-0.2, 0.475), (0.0, 0.495), (0.2, 0.475)];
    crown.extend(stones.map(|at| Outline::oval(at, (0.032, 0.032), Raise::Dent(0.3))));
    crown
}
