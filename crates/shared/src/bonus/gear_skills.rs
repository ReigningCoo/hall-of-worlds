//! The skills a quality bonus on an item may raise: only those fitting it
//! (a sword its own Swords nodes and Melee, never Axes), and only those the
//! rules read with bonuses so far (`combat::Fighter`): fighting skills.

use crate::combat::Weapon;
use crate::creature::{CreatureKind, Location};
use crate::item::{Item, ItemCategory};
use crate::skill::SkillId;

/// Fighting with every weapon.
const MELEE: &str = "combat/melee";
/// Defending with anything (armour helps it).
const DEFENCE: &str = "combat/melee/defence";
/// Kicking (footwear and foot armour help it).
const KICK: &str = "combat/melee/kick";

/// The skills this item may raise, in a fixed order: a weapon its own node,
/// its type, defending with its type, and Melee; armour Defence; anything on
/// the feet Kick.
pub fn gear_skills(item: &Item) -> Vec<SkillId> {
    let mut skills = Vec::new();
    if let Some(weapon) = Weapon::of(item) {
        // A family's attack skill never depends on the kind holding it.
        let attack = weapon.attack_skill(CreatureKind::default());
        skills.extend(weapon.skill);
        skills.extend(attack.filter(|skill| Some(*skill) != weapon.skill));
        skills.extend(weapon.defence_skill());
        skills.extend(SkillId::find(MELEE));
    }
    let locations = item.worn_locations();
    if item.kind.category() == ItemCategory::Armour {
        skills.extend(SkillId::find(DEFENCE));
    }
    if locations.contains(&Location::Feet) {
        skills.extend(SkillId::find(KICK));
    }
    skills
}
