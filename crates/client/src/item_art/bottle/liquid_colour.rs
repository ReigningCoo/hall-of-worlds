//! The liquid's colours: each stat its own, grouped by attribute row as the
//! creation screen's triangles are (Body warm, Mind blue, Spirit green), Health
//! crimson and Mana blue; made paler or darker by how much it helps or harms,
//! and less see-through the stronger it is.

use bevy::prelude::*;
use shared::herb::{Rgb, Stat};
use shared::potion::{Oil, Water};

use super::liquid::{Base, LiquidLook, MOST_NET, MOST_STRENGTH};

/// How far towards white the most helpful liquid goes, and towards black the
/// most harmful.
const PALEST: f32 = 0.4;
const DARKEST: f32 = 0.6;
/// How see-through the weakest and strongest liquids are.
const WEAKEST_ALPHA: f32 = 0.72;
const STRONGEST_ALPHA: f32 = 0.97;
/// How see-through waters and oils are: water nearly clear, oil less so.
const WATER_ALPHA: f32 = 0.4;
const OIL_ALPHA: f32 = 0.82;
/// How see-through juice is, and cordial (half water).
const JUICE_ALPHA: f32 = 0.78;
const CORDIAL_ALPHA: f32 = 0.6;
/// How far cordial goes towards water's colour.
const CORDIAL_WATER: f32 = 0.45;

/// A water's colour: faintly tinted, as water in glass is.
fn water_colour(water: Water) -> Color {
    match water {
        Water::Plain => Color::srgb(0.72, 0.86, 0.95),
        // Mineral: a grey green.
        Water::Cave => Color::srgb(0.62, 0.74, 0.7),
        // Silvery, a touch of violet.
        Water::Moon => Color::srgb(0.82, 0.84, 0.98),
        // Sea green, a little cloudy.
        Water::Salt => Color::srgb(0.6, 0.82, 0.78),
        // Clear, a touch cooler than plain water.
        Water::Rain => Color::srgb(0.78, 0.88, 0.98),
    }
}

/// An oil's colour: golden to amber.
fn oil_colour(oil: Oil) -> Color {
    match oil {
        // Pale straw, as a refined neutral oil.
        Oil::Neutral => Color::srgb(0.96, 0.9, 0.6),
        // Golden, as wheat germ oil.
        Oil::Grain => Color::srgb(0.95, 0.78, 0.2),
        Oil::Nut => Color::srgb(0.78, 0.5, 0.16),
        // Greenish gold, like hemp or pumpkin seed oil.
        Oil::Seed => Color::srgb(0.66, 0.7, 0.18),
    }
}

/// The colour a base starts from.
fn base_colour(base: Base) -> Color {
    match base {
        Base::Stat(stat) => stat_colour(stat),
        Base::Water(water) => water_colour(water),
        Base::Oil(oil) => oil_colour(oil),
        Base::Juice { colour, cordial } => {
            let Rgb(r, g, b) = colour;
            let juice = Color::srgb_u8(r, g, b);
            match cordial {
                true => juice.mix(&water_colour(Water::Plain), CORDIAL_WATER),
                false => juice,
            }
        }
    }
}

/// A stat's own colour.
pub(super) fn stat_colour(stat: Stat) -> Color {
    match stat {
        // Body: reds, oranges and yellows.
        Stat::Strength => Color::srgb(0.86, 0.18, 0.1),
        Stat::Agility => Color::srgb(0.96, 0.82, 0.12),
        Stat::Toughness => Color::srgb(0.88, 0.48, 0.1),
        // Mind: blues, violet and cyan.
        Stat::Intellect => Color::srgb(0.42, 0.3, 0.95),
        Stat::Perception => Color::srgb(0.1, 0.78, 0.88),
        Stat::Will => Color::srgb(0.62, 0.16, 0.72),
        // Spirit: greens.
        Stat::Presence => Color::srgb(0.08, 0.66, 0.3),
        Stat::Empathy => Color::srgb(0.58, 0.86, 0.26),
        Stat::Aura => Color::srgb(0.18, 0.78, 0.62),
        Stat::Health => Color::srgb(0.74, 0.04, 0.16),
        Stat::Mana => Color::srgb(0.14, 0.38, 1.0),
        // A deep night blue with a glint of green, like eyes in the dark.
        Stat::Darkvision => Color::srgb(0.1, 0.3, 0.26),
    }
}

/// A colour made paler (`net` above 0) or darker (below) by how far it helps or
/// harms.
pub(super) fn shaded(colour: Color, net: i16) -> Color {
    let share = net as f32 / MOST_NET as f32;
    if share >= 0.0 {
        colour.mix(&Color::WHITE, PALEST * share)
    } else {
        colour.mix(&Color::BLACK, DARKEST * -share)
    }
}

/// How solid the liquid is, from its strength.
pub(super) fn alpha(look: &LiquidLook) -> f32 {
    match look.main {
        Base::Water(_) => return WATER_ALPHA,
        Base::Oil(_) => return OIL_ALPHA,
        Base::Juice { cordial: false, .. } => return JUICE_ALPHA,
        Base::Juice { cordial: true, .. } => return CORDIAL_ALPHA,
        Base::Stat(_) => {}
    }
    let share = look.strength as f32 / MOST_STRENGTH as f32;
    WEAKEST_ALPHA + (STRONGEST_ALPHA - WEAKEST_ALPHA) * share
}

/// The main colour, shaded.
pub(in crate::item_art) fn main_colour(look: &LiquidLook) -> Color {
    shaded(base_colour(look.main), look.net)
}

/// The second colour, shaded: the second strongest stat's, or the main one
/// darker (or paler, if it is already dark) when there is only one.
pub(in crate::item_art) fn second_colour(look: &LiquidLook) -> Color {
    match look.second {
        Some(stat) => shaded(stat_colour(stat), look.net),
        None if look.net < 0 => main_colour(look).mix(&Color::WHITE, 0.35),
        None => main_colour(look).mix(&Color::BLACK, 0.35),
    }
}

/// The blobs' colour: their stat's, shaded the other way from the liquid.
pub(super) fn blob_colour(look: &LiquidLook, stat: Stat) -> Color {
    let net = if look.net > 0 {
        -MOST_NET / 2
    } else {
        MOST_NET / 2
    };
    shaded(stat_colour(stat), net)
}
