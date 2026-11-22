//! A crafting tool's outline on the Alchemy and Cooking panels: solid red
//! when it isn't to hand (or a step is past those allowed), else the quality
//! colour of the tool that will be used; plain on an empty box.

use bevy::prelude::*;
use shared::item::Quality;

use super::theme::GRID_LINE;
use crate::common::quality_colour;

/// Solid red: a tool not to hand.
const MISSING: Color = Color::srgb(0.85, 0.12, 0.12);

/// A box's border: how wide and what colour.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ToolEdge {
    pub width: f32,
    pub colour: Color,
}

impl ToolEdge {
    /// An empty box's thin line.
    pub const PLAIN: Self = Self {
        width: 1.0,
        colour: GRID_LINE,
    };

    /// Not to hand, or past the steps allowed.
    pub const MISSING: Self = Self {
        width: 3.0,
        colour: MISSING,
    };

    /// The tool used, at `quality`; `None` when there is none to hand.
    pub fn of(quality: Option<Quality>) -> Self {
        match quality {
            Some(quality) => Self {
                width: 2.0,
                colour: quality_colour(quality),
            },
            None => Self::MISSING,
        }
    }
}
