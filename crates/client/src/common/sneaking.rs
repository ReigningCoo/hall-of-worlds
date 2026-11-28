//! Whether the player is sneaking (`shared::ability::Ability::Sneak`, a toggle).

use bevy::prelude::*;

/// Whether the player sneaks, as the server last said: `abilities` keeps it
/// (turned on and off from the action bar), `player` paces steps on levels by
/// it (`shared::energy::sneak_step_energy`).
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Sneaking(pub bool);

/// Whether a mob is Alert to or Aware of the player, as the server says
/// (`ServerMessage::MobAlert`, mob behaviour plan PR 7): `sneak_eye` keeps it,
/// and the eye opens wide.
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct MobAlert(pub bool);
