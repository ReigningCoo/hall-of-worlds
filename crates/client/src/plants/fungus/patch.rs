//! Laying out one fungus patch, a cell's worth, by its form: a few caps
//! clustering (more of the small ones), brackets on a stub of wood, jelly and
//! crusts on a fallen stick, lichens in a mat, beards hanging from a dead
//! snag's twigs, and a truffle only as a mound of disturbed earth. Each
//! fungus is `item_art::fungus_one`, stood up as it is; sizes grow by stage.

use std::f32::consts::TAU;

use bevy::prelude::*;
use shared::fungus::{Form, FungusDef, Unit};
use shared::rng::{Rng, Seed};

use super::super::plan::METRE;
use super::ground::{linear, mound, rod, stick, stub};
use super::shell::Shell;
use crate::dungeon_map::CELL_SIZE;
use crate::item_art::fungus_one;

/// For the patches' own random numbers: the same everywhere, so every client
/// draws a patch alike.
const LOOKS: Seed = Seed::new(0x6675_6e67_6921);

/// The fungus's own variants (`item_art`'s pictures have 16).
const ONE_VARIANTS: u8 = 16;

/// How far from the cell's middle anything reaches, in world units.
pub const REACH: f32 = CELL_SIZE * 0.48;

/// How fine the maps' models are kept at least, in world units
/// (`Shell::simplify`); larger patches more coarsely.
const GRID: f32 = 0.3;

/// How much of the usual outline's width a patch of `def` gets: lichens'
/// fine lobes and strands would be lost in a full one.
pub fn outline_share(def: &FungusDef) -> f32 {
    match def.look.form {
        Form::LichenBeard | Form::LichenPlates | Form::LichenTufts => 0.4,
        _ => 1.0,
    }
}

/// How big one is at stages 1 to 4 against full grown.
pub fn growth(stage: u8) -> f32 {
    match stage {
        0 | 1 => 0.35,
        2 => 0.6,
        3 => 0.85,
        _ => 1.0,
    }
}

/// How far across a full-grown one of `def` is drawn, in metres: small ones
/// larger than life so they show, the largest a little smaller.
pub fn drawn_metres(def: &FungusDef) -> f32 {
    let metres = f32::from(def.look.largest) / 1000.0;
    (0.2 + 0.9 * metres.powf(0.6)).min(1.1)
}

/// How many stand in a full-grown cluster of `def`.
fn cluster_count(def: &FungusDef) -> usize {
    match def.look.largest {
        _ if def.unit == Unit::Piece => 1,
        0..=40 => 5,
        41..=100 => 3,
        _ => 2,
    }
}

/// A patch of `def` grown to `stage`, in variant `variant`.
pub fn build(def: &FungusDef, stage: u8, variant: u8) -> Shell {
    let rng = Seed::new(LOOKS.derive(&def.key).at(&[i64::from(variant)])).rng();
    let size = drawn_metres(def) * METRE * growth(stage);
    let mut patch = Patch {
        def,
        shell: Shell::default(),
        rng,
        size,
        variant,
    };
    match def.look.form {
        Form::Truffle => patch.mound(stage),
        Form::Bracket => patch.on_stub(stage),
        Form::LichenBeard => patch.hanging(stage),
        Form::Jelly | Form::Crust if def.feeds_on_wood() => patch.on_stick(stage),
        Form::LichenTufts | Form::LichenPlates | Form::Crust => patch.mat(stage),
        _ => patch.cluster(stage),
    }
    patch.shell.fit(REACH);
    let extent = (patch.shell.positions.iter())
        .map(|p| p.x.abs().max(p.z.abs()).max(p.y.abs() / 2.0))
        .fold(0.0f32, f32::max);
    patch.shell.simplify(GRID.max(extent / 10.0));
    patch.shell
}

/// A patch being laid out.
struct Patch<'a> {
    def: &'a FungusDef,
    shell: Shell,
    rng: Rng,
    /// One full-grown fungus across at this stage, in world units.
    size: f32,
    variant: u8,
}

impl Patch<'_> {
    /// 0 to 1.
    fn share(&mut self) -> f32 {
        self.rng.next_f64() as f32
    }

    /// The `n`th fungus, scaled `scale` and placed by `place`.
    fn fungus(&mut self, n: usize, place: Transform) {
        let variant = (usize::from(self.variant) * 5 + n * 3) % usize::from(ONE_VARIANTS);
        for piece in fungus_one(self.def, variant as u8) {
            let grey = Vec3::splat(0.5);
            (self.shell).add(&piece.mesh, place * piece.transform, grey);
        }
    }

    /// How many of `full` show at `stage`: fewer while young.
    fn count(full: usize, stage: u8) -> usize {
        match stage {
            0 | 1 => full.div_ceil(2),
            2 => (full * 3).div_ceil(4),
            _ => full,
        }
    }

    /// Standing ones in a cluster, the first near the middle, the rest round
    /// it, each a little smaller and leaning out.
    fn cluster(&mut self, stage: u8) {
        let count = Self::count(cluster_count(self.def), stage);
        let start = self.share() * TAU;
        for n in 0..count {
            let scale = self.size * (1.0 - 0.12 * n.min(3) as f32) * (0.9 + 0.2 * self.share());
            let angle = start + n as f32 * 2.399_963;
            let out = match n {
                0 => self.size * 0.15 * self.share(),
                _ => self.size * (0.45 + 0.2 * (n as f32).sqrt()),
            };
            let out = out.min(REACH - scale * 0.5).max(0.0);
            let lean = if n == 0 {
                0.0
            } else {
                0.12 + 0.1 * self.share()
            };
            let place = Transform::from_xyz(angle.cos() * out, 0.0, angle.sin() * out)
                .with_rotation(
                    Quat::from_rotation_y(self.share() * TAU)
                        * Quat::from_axis_angle(Vec3::new(angle.sin(), 0.0, -angle.cos()), -lean),
                )
                .with_scale(Vec3::splat(scale));
            self.fungus(n, place);
        }
    }

    /// Brackets on a stub of wood, a tier up its side.
    fn on_stub(&mut self, stage: u8) {
        let full = drawn_metres(self.def) * METRE;
        let radius = (0.12 * METRE + full * 0.15).min(REACH * 0.4);
        let height = 0.35 * METRE + full * 0.9;
        let (mesh, place) = stub(radius, height);
        self.shell.add(&mesh, place, linear(Vec3::splat(0.4)));
        // A shelf reaches about 0.65 of its size out from the wood.
        self.size = self.size.min((REACH - radius) / 0.65);
        let tiers = if self.def.unit == Unit::Handful { 3 } else { 2 };
        let count = Self::count(tiers, stage);
        let start = self.share() * TAU;
        for n in 0..count {
            let turn = start + n as f32 * 1.3 + 0.3 * self.share();
            let up = height * (0.35 + 0.22 * n as f32).min(0.85);
            let dir = Vec3::new(turn.sin(), 0.0, turn.cos());
            let scale = self.size * (1.0 - 0.15 * n as f32);
            let place = Transform::from_translation(dir * radius * 0.85 + Vec3::Y * up)
                .with_rotation(Quat::from_rotation_y(turn))
                .with_scale(Vec3::splat(scale));
            self.fungus(n, place);
        }
    }

    /// Jelly or a crust along a fallen stick.
    fn on_stick(&mut self, stage: u8) {
        let length = (self.size.max(0.3 * METRE) * 2.4).min(REACH * 1.8);
        let radius = 0.07 * METRE + self.size * 0.08;
        let turn = self.share() * TAU;
        let along = Quat::from_rotation_y(turn);
        let (mesh, place) = stick(radius, length);
        let place = Transform::from_rotation(along) * place;
        self.shell.add(&mesh, place, linear(Vec3::splat(0.4)));
        let count = Self::count(3, stage);
        for n in 0..count {
            let x = (n as f32 - (count as f32 - 1.0) / 2.0) * length * 0.28;
            let side = (n % 2) as f32 * 2.0 - 1.0;
            let tilt = Quat::from_rotation_x(side * 0.5);
            let at = along * Vec3::new(x, radius * 1.4, side * radius * 0.4);
            let scale = self.size * (0.8 + 0.2 * self.share());
            let place = Transform::from_translation(at)
                .with_rotation(along * tilt)
                .with_scale(Vec3::splat(scale));
            self.fungus(n, place);
        }
    }

    /// Lichen (or a crust) as a mat on the ground, many spread out.
    fn mat(&mut self, stage: u8) {
        // Tufts are each a cushion of many sprigs: fewer of them.
        let full = if self.def.look.form == Form::LichenTufts {
            4
        } else {
            7
        };
        let count = Self::count(full, stage);
        let spread = REACH * 0.75;
        for n in 0..count {
            let angle = self.share() * TAU;
            let out = spread * self.share().sqrt();
            let scale = self.size * (0.6 + 0.4 * self.share());
            let out = out.min(REACH - scale * 0.5).max(0.0);
            let place = Transform::from_xyz(angle.cos() * out, 0.0, angle.sin() * out)
                .with_rotation(Quat::from_rotation_y(self.share() * TAU))
                .with_scale(Vec3::splat(scale));
            self.fungus(n, place);
        }
    }

    /// Beards hanging from the twigs of a dead snag.
    fn hanging(&mut self, stage: u8) {
        let full = drawn_metres(self.def) * METRE;
        let height = 0.5 * METRE + full * 1.1;
        let radius = 0.05 * METRE;
        let lean = Vec3::new(self.share() - 0.5, 0.0, self.share() - 0.5) * 0.2 * METRE;
        let (mesh, place) = rod(Vec3::ZERO, Vec3::Y * height + lean, radius);
        self.shell.add(&mesh, place, linear(Vec3::splat(0.4)));
        let count = Self::count(2, stage);
        let start = self.share() * TAU;
        for n in 0..count {
            let turn = start + n as f32 * 2.6;
            let up = height * (0.95 - 0.2 * n as f32);
            let from = Vec3::Y * up + lean * (up / height);
            let out = Vec3::new(turn.cos(), 0.25, turn.sin()) * self.size.max(0.3 * METRE) * 0.8;
            let (mesh, place) = rod(from, from + out, radius * 0.6);
            self.shell.add(&mesh, place, linear(Vec3::splat(0.4)));
            // The beard's twig runs along x: turned to lie along this one.
            let place = Transform::from_translation(from + out * 0.55)
                .with_rotation(Quat::from_rotation_y(-turn))
                .with_scale(Vec3::splat(self.size * 0.8));
            self.fungus(n, place);
        }
    }

    /// The mound over a truffle, larger as it grows.
    fn mound(&mut self, stage: u8) {
        let radius = (0.18 * METRE + self.size * 0.4) * growth(stage).sqrt();
        let seed = self.rng.next_u64();
        for (mesh, place) in mound(radius.min(REACH * 0.6), seed) {
            self.shell.add(&mesh, place, linear(Vec3::splat(0.4)));
        }
    }
}
