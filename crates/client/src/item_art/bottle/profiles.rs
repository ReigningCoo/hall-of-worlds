//! Each shape's side outline, turned on a lathe: radius and height, in units of
//! about a millimetre, from the middle of the bottom up to the mouth. Every
//! outline ends neck, lip, lip, mouth, middle of the mouth, which `Profile`
//! reads the stopper's measurements from. The heart's body is not lathed: only
//! its neck is here (`heart.rs`).

use bevy::prelude::*;

use super::look::Shape;

/// What the rings round a shape are like below `until` high, fading to round
/// above it so necks are always round.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) enum Section {
    Round,
    /// Flat sides, this many, a corner facing the front.
    Sides(u32),
    /// Pressed flat front to back, to this share of its width.
    Flat(f32),
}

/// A shape's outline and its section.
pub(super) struct Profile {
    pub(super) points: Vec<Vec2>,
    pub(super) section: Section,
    /// Where the section fades back to round, and the neck (more tinted) starts.
    pub(super) shoulder: f32,
}

impl Profile {
    /// The top of the mouth.
    pub(super) fn top(&self) -> f32 {
        self.points[self.points.len() - 1].y
    }

    /// The mouth's radius, which the stopper fills.
    pub(super) fn mouth(&self) -> f32 {
        self.points[self.points.len() - 2].x
    }

    /// The lip's radius and the height of its bottom.
    pub(super) fn lip(&self) -> (f32, f32) {
        let lip = self.points[self.points.len() - 4];
        (lip.x, lip.y)
    }

    /// The neck's radius just under the lip.
    pub(super) fn neck(&self) -> f32 {
        self.points[self.points.len() - 5].x
    }
}

/// The neck, lip and mouth every outline ends with, from `neck` radius at
/// height `from` up to a mouth of `mouth`, the lip `lip` across.
fn mouth_end(to: f32, neck: f32, lip: f32, mouth: f32) -> [Vec2; 5] {
    [
        Vec2::new(neck, to),
        Vec2::new(lip, to + 1.0),
        Vec2::new(lip, to + 4.0),
        Vec2::new(mouth, to + 5.0),
        Vec2::new(0.0, to + 5.0),
    ]
}

fn outline(body: &[(f32, f32)], neck_to: f32, neck: f32, lip: f32, mouth: f32) -> Vec<Vec2> {
    let mut points: Vec<Vec2> = body.iter().map(|&(r, y)| Vec2::new(r, y)).collect();
    points.extend(mouth_end(neck_to, neck, lip, mouth));
    points
}

/// A shape's outline, if it is lathed (all but the heart's body).
pub(super) fn profile(shape: Shape) -> Profile {
    let (body, neck_to, neck, section, shoulder) = data(shape);
    let lip = neck + 2.0;
    let mouth = neck - 1.5;
    Profile {
        points: smooth(&outline(body, neck_to, neck, lip, mouth)),
        section,
        shoulder,
    }
}

/// The body's outline up to the neck, how high the neck goes and how wide it is,
/// the section and the shoulder.
type Data = (&'static [(f32, f32)], f32, f32, Section, f32);

#[rustfmt::skip]
fn data(shape: Shape) -> Data {
    use Section::*;
    match shape {
            Shape::RoundFlask => (
                &[(0., 0.), (18., 1.), (32., 8.), (38., 22.), (36., 36.), (26., 48.), (12., 56.), (9., 62.)],
                80., 8., Round, 52.,
            ),
            Shape::ConeFlask => (
                &[(0., 0.), (34., 0.), (36., 2.), (34., 5.), (12., 50.), (10., 54.)],
                72., 10., Round, 50.,
            ),
            Shape::LongNeck => (
                &[(0., 0.), (20., 1.), (30., 10.), (32., 22.), (26., 34.), (10., 42.), (6., 48.)],
                92., 6., Round, 40.,
            ),
            Shape::Vial => (
                &[(0., 0.), (8., 1.), (11., 5.), (12., 10.), (12., 70.)],
                80., 12., Round, 70.,
            ),
            Shape::Ampoule => (
                &[(0., 0.), (9., 2.), (14., 10.), (15., 30.), (13., 44.), (6., 52.), (4., 58.), (6., 64.)],
                72., 5., Round, 48.,
            ),
            Shape::Onion => (
                &[(0., 0.), (30., 0.), (42., 8.), (44., 16.), (36., 28.), (18., 38.), (10., 44.)],
                60., 9., Round, 36.,
            ),
            Shape::Gourd => (
                &[(0., 0.), (20., 1.), (28., 10.), (29., 20.), (24., 30.), (14., 36.), (16., 42.), (20., 50.), (18., 58.), (10., 64.)],
                72., 8., Round, 62.,
            ),
            Shape::Teardrop => (
                &[(0., 0.), (14., 2.), (24., 12.), (26., 24.), (22., 38.), (14., 54.), (8., 66.)],
                74., 7., Round, 58.,
            ),
            Shape::Jar => (
                &[(0., 0.), (28., 0.), (30., 2.), (30., 48.), (26., 52.), (18., 54.)],
                58., 17., Round, 52.,
            ),
            Shape::Ringed => (
                &[(0., 0.), (20., 1.), (22., 4.), (22., 14.), (25., 17.), (22., 20.), (22., 34.), (25., 37.), (22., 40.), (22., 54.), (18., 60.), (9., 64.)],
                76., 8., Round, 60.,
            ),
            Shape::Bell => (
                &[(0., 0.), (34., 0.), (36., 3.), (30., 8.), (22., 18.), (16., 32.), (14., 46.), (10., 56.)],
                66., 8., Round, 50.,
            ),
            Shape::Tall => (
                &[(0., 0.), (18., 0.), (20., 3.), (20., 56.), (17., 64.), (8., 70.)],
                80., 7., Round, 64.,
            ),
            Shape::Square => (
                &[(0., 0.), (24., 0.), (26., 3.), (26., 52.), (22., 60.), (9., 66.)],
                76., 8., Sides(4), 60.,
            ),
            Shape::Hexagonal => (
                &[(0., 0.), (22., 0.), (24., 3.), (24., 46.), (18., 56.), (9., 62.)],
                72., 8., Sides(6), 56.,
            ),
            Shape::HipFlask => (
                &[(0., 0.), (26., 1.), (36., 8.), (38., 24.), (36., 40.), (28., 48.), (12., 54.), (9., 58.)],
                68., 8., Flat(0.42), 54.,
            ),
            // Only the neck: it rises from inside the heart's dip.
            Shape::Heart => (&[(0., 40.), (8., 40.)], 74., 8., Round, 40.),
    }
}

/// Points between each pair along a centripetal Catmull-Rom curve, so the
/// outline is smooth but still passes through every point given, never rising
/// and falling again (which would fold the glass). The last five (the mouth) are
/// kept as they are, so the stopper's measurements stay exact.
fn smooth(points: &[Vec2]) -> Vec<Vec2> {
    const STEPS: usize = 4;
    let last = points.len() as isize - 1;
    // Past either end, carried straight on.
    let at = |i: isize| match i {
        -1 => 2.0 * points[0] - points[1],
        i if i > last => 2.0 * points[last as usize] - points[last as usize - 1],
        i => points[i as usize],
    };
    let kept = points.len() - 5;
    let mut out: Vec<Vec2> = Vec::new();
    for i in 0..kept as isize {
        let segment = [at(i - 1), at(i), at(i + 1), at(i + 2)];
        for step in 0..STEPS {
            let mut point = centripetal(segment, step as f32 / STEPS as f32);
            point.x = point.x.max(0.0);
            if let Some(below) = out.last() {
                point.y = point.y.max(below.y);
            }
            out.push(point);
        }
    }
    out.extend_from_slice(&points[kept..]);
    out
}

/// A point `u` of the way from `p[1]` to `p[2]` (Barry and Goldman's way).
fn centripetal(p: [Vec2; 4], u: f32) -> Vec2 {
    let gap = |a: Vec2, b: Vec2| a.distance(b).sqrt().max(1e-3);
    let t1 = gap(p[0], p[1]);
    let t2 = t1 + gap(p[1], p[2]);
    let t3 = t2 + gap(p[2], p[3]);
    let t = t1 + u * (t2 - t1);
    let lerp = |a: Vec2, b: Vec2, from: f32, to: f32| a + (b - a) * ((t - from) / (to - from));
    let a1 = lerp(p[0], p[1], 0.0, t1);
    let a2 = lerp(p[1], p[2], t1, t2);
    let a3 = lerp(p[2], p[3], t2, t3);
    let b1 = lerp(a1, a2, 0.0, t2);
    let b2 = lerp(a2, a3, t1, t3);
    lerp(b1, b2, t1, t2)
}
