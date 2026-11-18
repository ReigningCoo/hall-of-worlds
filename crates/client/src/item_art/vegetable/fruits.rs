//! Fruits eaten as vegetables: a tomato with its green star, a ribbed
//! pumpkin and its woody stalk, a bumpy cucumber, a glossy aubergine in its
//! calyx, a bottle gourd and a wrinkled chayote.

use bevy::prelude::*;
use std::f32::consts::{PI, TAU};

use super::Build;
use super::colours::colour;
use super::group::Fruit;
use super::leaf::{Blade, Leaf, leaf};
use super::tube::{Round, cap, rings, tube};

pub(super) fn fruit(build: &Build, fruit: Fruit, kit: &mut super::Kit) {
    match fruit {
        Fruit::Tomato => round(build, 6, 0.04, 0.42, kit),
        Fruit::Pumpkin => round(build, 10, 0.07, 0.35, kit),
        Fruit::Cucumber => cucumber(build, kit),
        Fruit::Aubergine => aubergine(build, kit),
        Fruit::Gourd => gourd(build, kit),
        Fruit::Chayote => chayote(build, kit),
    }
}

/// A ribbed globe seen from above, `tall` against its width, stalk on top.
fn round(build: &Build, ribs: u32, depth: f32, tall: f32, kit: &mut super::Kit) {
    let c = build.colours;
    let skin = c.skin;
    let at = |t: f32| Vec3::new(0.0, t * tall * 2.0 * 0.5, 0.0);
    let half = |t: f32| Vec2::splat(0.5 * (PI * t).sin().max(0.0).powf(0.6));
    let round = Round {
        ribs,
        depth,
        stripes: Some((skin * 0.85, 0.12)),
    };
    let tomato = ribs < 8;
    tube(kit, &rings(24, at, half, |_| skin), round, tomato);
    let top = at(1.0) - Vec3::Y * 0.01;
    let green = c.accent_or(c.leaf);
    if tomato {
        for k in 0..5 {
            let yaw = k as f32 / 5.0 * TAU + build.variety.wobble(k, 1) * 0.2;
            let one = Leaf {
                cup: -0.2,
                arch: 0.02,
                ..Leaf::new(Blade::Long, top, yaw, 0.2, 0.07).coloured(green, green)
            };
            leaf(kit, &one);
        }
        let stalk = kit.material(colour(green * 0.85), false);
        kit.rod(&stalk, top, top + Vec3::new(0.02, 0.07, 0.0), 0.02);
    } else {
        let stalk = kit.material(colour(green), false);
        kit.blob(&stalk, top, Vec3::new(0.09, 0.03, 0.09), (0.0, 0.0));
        let end = top + Vec3::new(0.04, 0.1, -0.03);
        kit.rod(&stalk, top, end, 0.045);
        kit.blob(&stalk, end, Vec3::splat(0.05), (0.0, 0.0));
    }
}

fn cucumber(build: &Build, kit: &mut super::Kit) {
    let c = build.colours;
    let skin = c.skin;
    let r = (build.width_share() / 2.0).clamp(0.08, 0.2);
    let bend = 0.04 + build.variety.wobble(0, 2) * 0.03;
    let at = |t: f32| Vec3::new(t - 0.5, r, (PI * t).sin() * bend);
    let half = |t: f32| Vec2::splat(r * cap(t, 0.12).min(cap(1.0 - t, 0.1)));
    let round = Round {
        ribs: 5,
        depth: 0.015,
        stripes: Some((skin.lerp(Vec3::ONE, 0.3), 0.12)),
    };
    tube(kit, &rings(24, at, half, |_| skin), round, true);
    let bump = kit.material(colour(skin * 0.75), false);
    for k in 0..12 {
        let t = 0.12 + k as f32 * 0.065;
        let a = build.variety.wobble(k, 3) * 1.3;
        let spot = at(t) + Vec3::new(0.0, a.cos() * r, a.sin() * r);
        kit.blob(&bump, spot, Vec3::splat(r * 0.12), (0.0, 0.0));
    }
}

fn aubergine(build: &Build, kit: &mut super::Kit) {
    let c = build.colours;
    let skin = c.skin;
    let r = (build.width_share() / 2.0).clamp(0.15, 0.3);
    let at = |t: f32| Vec3::new(t * 0.9 - 0.5, r, 0.0);
    let half = |t: f32| Vec2::splat(r * (1.0 - 0.45 * t) * cap(t, 0.15).min(cap(1.0 - t, 0.08)));
    tube(kit, &rings(24, at, half, |_| skin), Round::default(), true);
    let green = c.accent_or(c.leaf);
    let calyx = kit.material(colour(green), false);
    let top = at(0.97);
    kit.blob(&calyx, top, Vec3::new(0.06, r * 0.5, r * 0.5), (0.0, 0.0));
    for k in 0..4 {
        let a = PI + (k as f32 - 1.5) * 0.5;
        let one = Leaf {
            cup: -0.3,
            arch: 0.12,
            ..Leaf::new(Blade::Long, top + Vec3::Y * r * 0.4, a, 0.22, 0.1).coloured(green, green)
        };
        leaf(kit, &one);
    }
    kit.rod(&calyx, top, top + Vec3::new(0.16, 0.02, 0.02), 0.025);
}

fn gourd(build: &Build, kit: &mut super::Kit) {
    let c = build.colours;
    let skin = c.skin;
    let bulb = |t: f32, from: f32, to: f32| {
        let s = (t - from) / (to - from);
        if (0.0..=1.0).contains(&s) {
            (PI * s).sin().powf(0.7)
        } else {
            0.0
        }
    };
    let at = |t: f32| Vec3::new(t - 0.5, 0.22, 0.0);
    let half = |t: f32| {
        let r = (0.22 * bulb(t, 0.0, 0.62))
            .max(0.13 * bulb(t, 0.5, 1.0))
            .max(0.07 * bulb(t, 0.03, 0.97));
        Vec2::splat(r)
    };
    let shade = move |t: f32| skin * (1.0 + 0.06 * ((t * 40.0).sin()));
    tube(kit, &rings(40, at, half, shade), Round::default(), false);
    let stalk = kit.material(colour(c.leaf * 0.8), false);
    kit.rod(
        &stalk,
        at(0.99),
        at(0.99) + Vec3::new(0.08, 0.02, 0.0),
        0.02,
    );
}

fn chayote(build: &Build, kit: &mut super::Kit) {
    let c = build.colours;
    let skin = c.skin;
    let at = |t: f32| Vec3::new(t - 0.5, 0.3, 0.0);
    let half = |t: f32| {
        let pear = 0.34 * (1.0 - 0.4 * t) * cap(t, 0.2).min(cap(1.0 - t, 0.15));
        Vec2::new(pear, pear * 0.85)
    };
    let round = Round {
        ribs: 5,
        depth: 0.12,
        stripes: Some((skin * 0.82, 0.15)),
    };
    tube(kit, &rings(24, at, half, |_| skin), round, false);
}
