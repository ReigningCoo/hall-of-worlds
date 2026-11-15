//! Laying a pair of hands down for a picture: fingers down, backs to the front, thumbs
//! towards each other (owner's choice), tipped back a little. Shared by gauntlets and
//! clothes' gloves.

use bevy::prelude::*;
use shared::armour::hands::Hand;
use shared::body::{Frame, Side};

use super::super::Model;
use super::{Axes, PICTURE_GAP, TILT};

/// Both hands as `build` makes each on `frame`, laid side by side, not yet fitted to
/// a picture's cells.
pub(in crate::item_art) fn lay_pair(
    frame: &Frame,
    digits: u8,
    build: &dyn Fn(Side) -> Model,
) -> Model {
    let mut picture = Model::default();
    for side in Side::BOTH {
        let mut hand = build(side);
        let axes = Axes::of(&Hand::of(frame, side, digits));
        // Down stays down, out comes towards the camera, and the thumb's way goes
        // across: right for the right hand, left for the left.
        let turn = Mat3::from_cols(axes.out.cross(axes.down), -axes.down, axes.out).transpose();
        hand.map(|point| turn * (point - axes.wrist));
        let (low, high) = hand.bounds();
        // Thumbs towards each other: the right hand on the left.
        let shift = match side {
            Side::Right => -PICTURE_GAP / 2.0 - high.x,
            Side::Left => PICTURE_GAP / 2.0 - low.x,
        };
        hand.map(|point| point + Vec3::X * shift);
        picture.join(hand);
    }
    let tilt = Quat::from_rotation_x(-TILT);
    picture.map(|point| tilt * point);
    picture
}
