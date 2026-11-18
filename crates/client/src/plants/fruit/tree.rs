//! Broadleaf trees: a trunk forking into branches under a crown of leaf
//! masses, round, spreading or upright by how wide it grows against how tall
//! (owner's: from height and spread).

use bevy::prelude::*;
use shared::fruit::FruitPosition;
use shared::rng::Rng;

use super::super::pieces::Pieces;
use super::super::plan::{WOOD, between};
use super::{Dress, Look, Show, around, out};

/// A broadleaf tree.
pub(super) fn tree(pieces: &mut Pieces, look: &Look, rng: &mut Rng, dress: &mut Dress) {
    // How far up the trunk forks, and how deep the crown is, by how wide it
    // grows in life.
    let (fork, deep) = match look.wide {
        w if w > 1.0 => (0.38, 0.34),
        w if w < 0.6 => (0.25, 0.55),
        _ => (0.32, 0.45),
    };
    let thick = (look.height * 0.045).clamp(0.3, 1.6);
    let fork = Vec3::Y * look.height * fork;
    let half = look.height * deep / 2.0;
    let middle = Vec3::Y * (look.height - half);
    pieces.stem(Vec3::ZERO, fork, (thick, thick * 0.75), WOOD);
    let branches = 3 + (rng.next_u64() % 3) as usize;
    let mut masses = vec![(middle + Vec3::Y * half * 0.3, 0.75)];
    for i in 0..branches {
        let angle = around(i, branches, rng);
        let tip = middle + out(angle, 0.0) * look.radius * between(rng, 0.45, 0.6)
            - Vec3::Y * half * between(rng, 0.0, 0.3);
        pieces.stem(fork, tip, (thick * 0.55, thick * 0.25), WOOD);
        masses.push((tip, between(rng, 0.55, 0.7)));
    }
    let size = Vec3::new(look.radius, half, look.radius);
    let mut balls = Vec::new();
    for (at, share) in masses {
        let own = size * share * between(rng, 0.9, 1.1);
        let turn = between(rng, 0.0, 1.6);
        pieces.ball(at, own, turn, look.leaf * between(rng, 0.85, 1.1));
        balls.push((at, own));
    }
    if look.fruits == FruitPosition::Trunk && matches!(look.show, Show::Fruit(_)) {
        on_trunk(pieces, fork.y, thick, dress);
        return;
    }
    let count = match (look.show, look.fruits) {
        (Show::Flowers, _) => 6,
        (_, FruitPosition::Clusters) => 2,
        _ => 4,
    };
    for (at, own) in balls {
        dress.over(pieces, at, own, count);
    }
}

/// Fruit on the trunk itself (cacao, jackfruit), from near the foot to where
/// it forks, standing out from the bark.
fn on_trunk(pieces: &mut Pieces, up_to: f32, thick: f32, dress: &mut Dress) {
    let count = 5;
    let size = dress.look.fruit_size;
    for i in 0..count {
        let angle = around(i, count, &mut dress.rng);
        let up = up_to * dress.between(0.3, 0.95);
        let at = Vec3::Y * up + out(angle, 0.0) * (thick * 0.8 + size * 0.8);
        dress.at(pieces, at + Vec3::Y * size);
    }
}
