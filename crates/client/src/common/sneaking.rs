//! Whether the player is sneaking (`shared::ability::Ability::Sneak`, a toggle).

use bevy::prelude::*;

/// Whether the player sneaks, as the server last said: `abilities` keeps it
/// (turned on and off from the action bar), `player` paces steps on levels by
/// it (`shared::energy::sneak_step_energy`).
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Sneaking(pub bool);

/// Whether a mob is Alert to the player, as the server says: the sneaking eye
/// opens wide (`sneak_eye`). Nothing sets it until alertness comes (mob
/// behaviour plan PR 7), which keeps it from the server's word.
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct MobAlert(pub bool);
