//! No fighting anywhere in The Way Inn: the magic that has always kept it
//! stops every blow (attacks, kicks, kicking doors, trampling:
//! `Ability::harms`), on both sides; Heal and the rest work as anywhere.

use shared::ability::Ability;

use crate::connections::Client;

/// Whether `ability` is refused to this player where they are: anything
/// harmful, in the inn.
pub(crate) fn refuses_harm(client: &Client, ability: Ability) -> bool {
    ability.harms() && super::in_inn(client)
}
