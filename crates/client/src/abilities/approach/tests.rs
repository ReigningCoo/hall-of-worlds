use shared::ability::{Ability, BarThing};
use shared::combat::{Body, Weapon, Wielded, reach_between};
use shared::creature::Size;
use shared::sheet::Sheet;
use shared::spell::{MISSILE_RANGE, SpellKey, heal_spell, healing_missile};
use shared::spot::Spot;

use super::*;

fn body(x: i64, size: Size) -> Body {
    Body {
        at: Spot::new(x, 0),
        height: 0,
        size,
    }
}

/// A weapon this many units long.
fn weapon(reach: u64) -> Weapon {
    Weapon {
        length_mm: (reach * u64::from(shared::combat::MM_PER_UNIT)) as u32,
        ..Weapon::UNARMED
    }
}

#[test]
fn dual_wielding_reaches_as_far_as_the_shorter_weapon() {
    let spear = Wielded {
        main: weapon(300),
        off: None,
    };
    assert_eq!(weapon_reach(&spear), 300);
    let and_dagger = Wielded {
        off: Some(weapon(40)),
        ..spear
    };
    assert_eq!(weapon_reach(&and_dagger), 40);
}

#[test]
fn attack_and_kick_reach_by_the_weapon_and_barefoot() {
    let sheet = Sheet::default();
    let spear = Wielded {
        main: weapon(300),
        off: None,
    };
    let (me, them) = (body(0, Size::Medium), body(2000, Size::Large));
    let needed = |thing: BarThing| reach_needed(thing, (&sheet, &spear), &me, &them);
    let attack = needed(Ability::Attack.into());
    assert_eq!(attack, Some(reach_between(Size::Medium, 300, Size::Large)));
    let kick = needed(Ability::Kick.into());
    assert_eq!(kick, Some(reach_between(Size::Medium, 0, Size::Large)));
    assert!(kick < attack);
    // Others aren't walked into reach for.
    assert_eq!(needed(Ability::FellTree.into()), None);
}

#[test]
fn a_spell_reaches_by_its_first_where_and_one_on_the_caster_by_none() {
    let sheet = Sheet::default();
    let wielded = Wielded {
        main: Weapon::UNARMED,
        off: None,
    };
    let (me, them) = (body(0, Size::Medium), body(3000, Size::Medium));
    let needed = |spell| reach_needed(BarThing::Spell(spell), (&sheet, &wielded), &me, &them);
    let missile = SpellKey::of(&healing_missile());
    assert_eq!(needed(missile), Some(MISSILE_RANGE));
    assert_eq!(needed(SpellKey::of(&heal_spell())), None);
}
