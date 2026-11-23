//! Games tables' boards (tavern furniture plan PR 6): a board on the table's
//! top, marked out for its game, with its men where they start
//! (`shared::board_game`). The frame's origin is the middle of the table's top,
//! facing south: the light player sits to the north, looking south. Sizes
//! and colours are first guesses.

use bevy::prelude::*;
use shared::board_game::{BoardGame, Man, Player};

use super::super::CELL_SIZE;
use super::colours::dark_wood;
use super::model::{Model, srgb};

const C: f32 = CELL_SIZE;
/// The board's side and thickness.
const BOARD: f32 = C * 0.72;
const THICK: f32 = C * 0.015;
/// How far squares are inlaid above the board.
const INLAY: f32 = C * 0.002;

fn light_square() -> Vec3 {
    srgb(0.88, 0.78, 0.56)
}

fn dark_square() -> Vec3 {
    srgb(0.36, 0.22, 0.13)
}

fn man_colour(player: Player) -> Vec3 {
    match player {
        Player::Light => srgb(0.94, 0.9, 0.78),
        Player::Dark => srgb(0.17, 0.12, 0.1),
    }
}

/// The board for `game`, with its men.
pub(super) fn board(model: &mut Model, game: BoardGame) {
    let half = BOARD / 2.0;
    let size = game.size();
    let square = BOARD / f32::from(size);
    model.block(
        Vec3::new(-half - C * 0.02, 0.0, -half - C * 0.02),
        Vec3::new(half + C * 0.02, THICK, half + C * 0.02),
        dark_wood(),
    );
    // A square's middle: file 0 on the light player's left (east, as they
    // look south), rank 0 nearest them (north).
    let middle = |(file, rank): (u8, u8)| {
        let x = (f32::from(size) - 1.0) / 2.0 - f32::from(file);
        let z = f32::from(rank) - (f32::from(size) - 1.0) / 2.0;
        Vec3::new(x * square, THICK, z * square)
    };
    let top = THICK + INLAY;
    let squares = (0..size).flat_map(|file| (0..size).map(move |rank| (file, rank)));
    for (file, rank) in squares {
        let colour = match game {
            BoardGame::Chess if (file + rank) % 2 == 0 => dark_square(),
            BoardGame::Chess => light_square(),
            BoardGame::Hnefatafl if game.marked((file, rank)) => dark_square(),
            BoardGame::Hnefatafl => light_square(),
        };
        // Hnefatafl's lines: the board between squares a little smaller.
        let gap = match game {
            BoardGame::Chess => 0.0,
            BoardGame::Hnefatafl => square * 0.08,
        };
        let at = middle((file, rank));
        let reach = square / 2.0 - gap;
        let low = Vec3::new(at.x - reach, THICK, at.z - reach);
        model.inlay(low, Vec3::new(at.x + reach, top, at.z + reach), colour);
    }
    let start = game.start();
    for ((file, rank), player, man) in start.men() {
        let foot = middle((file, rank)) + Vec3::Y * INLAY;
        self::man(model, foot, (player, man), square);
    }
}

/// An upright cylinder from `foot` up `height`, `radius` round.
fn post(model: &mut Model, foot: Vec3, (radius, height): (f32, f32), colour: Vec3) {
    let at = foot + Vec3::Y * height / 2.0;
    model.cylinder(at, (radius, height), Quat::IDENTITY, colour);
}

/// One man standing on `foot`, sized by the board's `square`.
fn man(model: &mut Model, foot: Vec3, (player, man): (Player, Man), square: f32) {
    let s = square;
    let colour = man_colour(player);
    // Light's knights look south, dark's north.
    let ahead = match player {
        Player::Light => 1.0,
        Player::Dark => -1.0,
    };
    let up = |height: f32| foot + Vec3::Y * height;
    if !matches!(man, Man::Defender | Man::Attacker) {
        post(model, foot, (0.32 * s, 0.08 * s), colour);
    }
    match man {
        Man::Pawn => {
            post(model, foot, (0.17 * s, 0.32 * s), colour);
            model.ball(up(0.38 * s), Vec3::splat(0.22 * s), colour);
        }
        Man::Rook => {
            post(model, foot, (0.22 * s, 0.5 * s), colour);
            post(model, up(0.5 * s), (0.27 * s, 0.12 * s), colour);
        }
        Man::Knight => {
            post(model, foot, (0.2 * s, 0.34 * s), colour);
            let turn = Quat::from_rotation_x(-0.5 * ahead);
            let head = up(0.48 * s) + Vec3::Z * 0.06 * s * ahead;
            model.cuboid(head, Vec3::new(0.16, 0.3, 0.38) * s, turn, colour);
        }
        Man::Bishop => {
            post(model, foot, (0.17 * s, 0.42 * s), colour);
            model.ball(up(0.52 * s), Vec3::new(0.26, 0.36, 0.26) * s, colour);
            model.ball(up(0.73 * s), Vec3::splat(0.08 * s), colour);
        }
        Man::Queen => {
            post(model, foot, (0.2 * s, 0.56 * s), colour);
            model.ball(up(0.6 * s), Vec3::splat(0.3 * s), colour);
            model.cone(up(0.8 * s), (0.12 * s, 0.14 * s), colour);
        }
        Man::King => {
            post(model, foot, (0.21 * s, 0.64 * s), colour);
            model.ball(up(0.68 * s), Vec3::splat(0.3 * s), colour);
            model.stick((up(0.78 * s), up(1.0 * s)), 0.05 * s, colour);
            let arm = Vec3::X * 0.08 * s;
            model.stick((up(0.92 * s) - arm, up(0.92 * s) + arm), 0.05 * s, colour);
        }
        Man::Defender | Man::Attacker => {
            post(model, foot, (0.3 * s, 0.22 * s), colour);
            model.ball(up(0.26 * s), Vec3::splat(0.2 * s), colour);
        }
    }
}
