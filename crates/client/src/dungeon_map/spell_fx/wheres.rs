//! Draws what the queue holds: each where as it happened, a burst where a
//! missile struck, and sparkles rising where effects landed.

use bevy::prelude::*;
use shared::spell::{Look, Shown, Where};
use shared::spot::Spot;

use super::super::CELL_SIZE;
use super::colours::look_colours;
use super::fade::Grow;
use super::news::{FxQueue, Queued};
use super::paint::Paint;
use super::place::{CHEST, spot_drawn};
use super::rays::{beams, ring, touch_arc};
use crate::common::DrawnGround;

/// Sparkles in a burst, and how big, in cells.
const BURST: usize = 14;
const SPARK: f32 = 0.04;

/// A flash's size (in cells) and life (in seconds).
const FLASH: f32 = 0.3;
const FLASH_LIFE: f32 = 0.35;

/// Draws everything waiting in the queue.
pub(crate) fn spawn_fx(mut paint: Paint, mut queue: ResMut<FxQueue>, ground: Res<DrawnGround>) {
    for queued in std::mem::take(&mut queue.pending) {
        match queued {
            Queued::Where(shown) => draw_where(&mut paint, &ground, &shown),
            Queued::Struck(at, look) => {
                let colours = look_colours(&look);
                let place = spot_drawn(&ground, at, CHEST);
                flash(&mut paint, place, colours[0], FLASH * 0.7);
                let speed = (1.2 * CELL_SIZE, 0.0);
                paint.burst(place, &colours, BURST, speed, SPARK * CELL_SIZE);
            }
            Queued::Landed(on, look) => landed(&mut paint, &ground, &on, &look),
            Queued::Clouds(cells, seconds) => {
                for cell in cells {
                    super::clouds::cloud(&mut paint, &ground, cell, seconds);
                }
            }
        }
    }
}

/// One where, by its kind, with a little burst on each thing it hit.
fn draw_where(paint: &mut Paint, ground: &DrawnGround, shown: &Shown) {
    let colours = look_colours(&shown.look);
    match shown.place {
        Where::Touch => touch_arc(paint, ground, shown, &colours),
        Where::Surround => ring(paint, ground, shown, &colours),
        Where::Beam => beams(paint, ground, shown, &colours),
        Where::Point => {
            for &hit in &shown.hits {
                let place = spot_drawn(ground, hit, CHEST);
                flash(paint, place, colours[0], FLASH);
                let speed = (1.6 * CELL_SIZE, 0.0);
                paint.burst(place, &colours, BURST, speed, SPARK * CELL_SIZE);
            }
            return;
        }
        // Flying missiles are bolts (`draw_bolts`).
        Where::Missile => return,
    }
    for &hit in &shown.hits {
        let place = spot_drawn(ground, hit, CHEST);
        let speed = (0.8 * CELL_SIZE, 0.0);
        paint.burst(place, &colours, BURST / 2, speed, SPARK * CELL_SIZE);
    }
}

/// Sparkles rising round each one an effect landed on, and a soft glow.
fn landed(paint: &mut Paint, ground: &DrawnGround, on: &[Spot], look: &Look) {
    let colours = look_colours(look);
    for &spot in on {
        let place = spot_drawn(ground, spot, CHEST * 0.6);
        flash(paint, place, colours[0] * 0.5, FLASH * 1.4);
        for n in 0..BURST + 6 {
            let way = paint.random_way();
            let round = Vec3::new(way.x, 0.0, way.z).normalize_or_zero() * 0.3 * CELL_SIZE;
            let rise = Vec3::Y * (0.8 + paint.random()) * CELL_SIZE;
            let at = place + round + Vec3::Y * (paint.random() - 0.5) * 0.6 * CELL_SIZE;
            let colour = colours[n % colours.len()];
            paint.sparkle(at, rise, colour, SPARK * CELL_SIZE);
        }
    }
}

/// A ball of light at `place` swelling and fading.
fn flash(paint: &mut Paint, place: Vec3, colour: LinearRgba, size: f32) {
    let ball = paint.shared().ball;
    let transform = Transform::from_translation(place).with_scale(Vec3::splat(size * CELL_SIZE));
    paint.fading(ball, transform, colour, (FLASH_LIFE, Grow::Spread));
}
