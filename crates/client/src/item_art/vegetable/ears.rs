//! The head at the top of each stalk in a sheaf: spikes of grain (bearded or
//! not), drooping panicles, dense heads, plumes, cattail spikes, fingers, and
//! buckwheat's clusters of three-cornered seeds.

use bevy::prelude::*;
use std::f32::consts::PI;

use super::Build;
use super::colours::colour;
use super::group::Ear;
use super::tube::{Round, cap, rings, tube};

/// Where a head starts and which way it points.
pub(super) struct Frame {
    pub(super) at: Vec3,
    pub(super) out: Vec3,
    /// Which stalk it tops.
    pub(super) n: usize,
}

impl Frame {
    /// Sideways from its way out, in the ground's plane.
    fn side(&self) -> Vec3 {
        Vec3::new(-self.out.z, 0.0, self.out.x)
    }

    fn yaw(&self) -> f32 {
        (-self.out.z).atan2(self.out.x)
    }
}

pub(super) fn head(build: &Build, ear: Ear, frame: &Frame, kit: &mut super::Kit) {
    match ear {
        Ear::Spike { awns } => spike(build, frame, awns, kit),
        Ear::Panicle => panicle(build, frame, kit),
        Ear::Dense => dense(build, frame, kit),
        Ear::Plume => plume(build, frame, kit),
        Ear::Cattail => cattail(build, frame, kit),
        Ear::Fingers => fingers(build, frame, kit),
        Ear::Buckwheat => buckwheat(build, frame, kit),
        Ear::Cob => {}
    }
}

/// Grains in two rows up the spike, narrowing to its tip, bearded if `awns`.
fn spike(build: &Build, frame: &Frame, awns: bool, kit: &mut super::Kit) {
    let grain = kit.material(colour(build.colours.skin), false);
    let beard = kit.material(colour(build.colours.skin * 0.92), false);
    let yaw = frame.yaw();
    for k in 0..10 {
        let t = k as f32 / 10.0;
        let side = if k % 2 == 0 { 1.0 } else { -1.0 };
        let narrow = 1.0 - 0.4 * t;
        let at = frame.at + frame.out * (t * 0.38) + frame.side() * side * 0.028 * narrow;
        let half = Vec3::new(0.042, 0.026, 0.026) * narrow;
        let turn = yaw - side * 0.3;
        kit.blob(&grain, at + Vec3::Y * 0.01, half, (turn, 0.0));
        if awns {
            let way = Quat::from_rotation_y(yaw - side * 0.12) * Vec3::X;
            kit.rod(&beard, at, at + way * 0.32, 0.003);
        }
    }
}

/// Branches of grains hanging off a nodding axis.
fn panicle(build: &Build, frame: &Frame, kit: &mut super::Kit) {
    let v = build.variety;
    let grain = kit.material(colour(build.colours.skin), false);
    let axis = kit.material(colour(build.colours.straw), false);
    let droop = if frame.n.is_multiple_of(2) { 1.0 } else { -1.0 } * 0.6;
    let way = Quat::from_rotation_y(-droop * 0.4) * frame.out;
    let end = frame.at + way * 0.45;
    kit.rod(&axis, frame.at, end, 0.006);
    for k in 0..6 {
        let t = 0.1 + k as f32 * 0.15;
        let from = frame.at.lerp(end, t);
        let side = if k % 2 == 0 { 1.0 } else { -1.0 };
        let branch = Quat::from_rotation_y(side * 0.7 + v.wobble(frame.n * 6 + k, 4) * 0.2) * way;
        let to = from + branch * (0.16 - t * 0.06);
        kit.rod(&axis, from, to, 0.004);
        for g in 0..3 {
            let on = from.lerp(to, 0.45 + g as f32 * 0.27) + Vec3::Y * 0.01;
            let yaw = (-branch.z).atan2(branch.x);
            kit.blob(&grain, on, Vec3::new(0.035, 0.015, 0.018), (yaw, 0.0));
        }
    }
}

/// A dense egg of tiny grains.
fn dense(build: &Build, frame: &Frame, kit: &mut super::Kit) {
    let skin = build.colours.skin;
    let body = kit.material(colour(skin * 0.85), false);
    let grain = kit.material(colour(skin), true);
    let middle = frame.at + frame.out * 0.17 + Vec3::Y * 0.03;
    kit.blob(
        &body,
        middle,
        Vec3::new(0.17, 0.07, 0.08),
        (frame.yaw(), 0.0),
    );
    for k in 0..16 {
        let t = (k as f32 / 16.0) * 2.0 - 1.0;
        let side = build.variety.wobble(frame.n * 16 + k, 5);
        let at = middle + frame.out * t * 0.14 + frame.side() * side * 0.06 * (1.0 - t * t).sqrt();
        kit.blob(&grain, at + Vec3::Y * 0.05, Vec3::splat(0.025), (0.0, 0.0));
    }
}

/// A plume tapering to a nodding tip, with a few side plumes.
fn plume(build: &Build, frame: &Frame, kit: &mut super::Kit) {
    let skin = build.colours.skin;
    let bend = if frame.n.is_multiple_of(2) {
        0.12
    } else {
        -0.12
    };
    let side = frame.side();
    let main = |t: f32| frame.at + frame.out * t * 0.5 + side * bend * t * t + Vec3::Y * 0.05;
    let half = |t: f32| Vec2::splat(0.045 * (1.0 - 0.7 * t) * cap(t, 0.05));
    let bumpy = Round {
        ribs: 7,
        depth: 0.15,
        stripes: Some((skin * 0.8, 0.2)),
    };
    tube(kit, &rings(16, main, half, |_| skin), bumpy, false);
    for k in 0..2 {
        let from = main(0.25 + k as f32 * 0.25);
        let way = side * if k == 0 { 1.0 } else { -1.0 };
        let branch = |t: f32| from + (frame.out + way) * 0.6 * t * 0.25;
        let small = |t: f32| Vec2::splat(0.03 * (1.0 - 0.7 * t) * cap(t, 0.05));
        tube(kit, &rings(8, branch, small, |_| skin), bumpy, false);
    }
}

/// A thick cylinder of grains, like a cattail.
fn cattail(build: &Build, frame: &Frame, kit: &mut super::Kit) {
    let skin = build.colours.skin;
    let at = |t: f32| frame.at + frame.out * t * 0.38 + Vec3::Y * 0.04;
    let half = |t: f32| Vec2::splat(0.04 * cap(t, 0.08).min(cap(1.0 - t, 0.12)));
    let grainy = Round {
        ribs: 9,
        depth: 0.06,
        stripes: Some((skin * 0.85, 0.25)),
    };
    let shade = move |t: f32| skin * (1.0 - 0.1 * f32::from((t * 16.0).fract() < 0.3));
    tube(kit, &rings(16, at, half, shade), grainy, false);
}

/// Four or five finger-like spikes spreading from the top.
fn fingers(build: &Build, frame: &Frame, kit: &mut super::Kit) {
    let skin = build.colours.skin;
    let count = 4 + frame.n % 2;
    for k in 0..count {
        let a = (k as f32 / (count - 1) as f32 - 0.5) * 1.3;
        let way = Quat::from_rotation_y(a) * frame.out;
        let curl = frame.side() * a.signum() * 0.04;
        let at = |t: f32| frame.at + way * t * 0.22 + curl * t * t + Vec3::Y * 0.03;
        let half = |t: f32| Vec2::splat(0.018 * cap(1.0 - t, 0.2));
        let grainy = Round {
            ribs: 5,
            depth: 0.12,
            stripes: Some((skin * 0.8, 0.2)),
        };
        tube(kit, &rings(8, at, half, |_| skin), grainy, false);
    }
}

/// A cluster of dark three-cornered seeds.
fn buckwheat(build: &Build, frame: &Frame, kit: &mut super::Kit) {
    let seed = kit.material(colour(build.colours.skin), true);
    for k in 0..7 {
        let a = k as f32 / 7.0 * PI * 2.0 + build.variety.wobble(frame.n * 7 + k, 6) * 0.3;
        let off = (frame.out * a.cos() + frame.side() * a.sin()) * 0.05;
        let at = frame.at + frame.out * 0.06 + off + Vec3::Y * 0.03;
        kit.blob(&seed, at, Vec3::new(0.035, 0.025, 0.03), (a, 0.5));
    }
}
