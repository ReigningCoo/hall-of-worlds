use super::*;
use crate::bonus::Raised;
use crate::creature::CreatureKind;
use crate::rng::Seed;
use crate::sheet::Attributes;
use crate::skill::Skills;

#[test]
fn a_kick_is_slower_than_two_punches_and_weaker_than_one() {
    let attributes = Attributes::all(100);
    let skills = Skills::default();
    let fighter = Fighter {
        kind: CreatureKind::Human,
        attributes: &attributes,
        skills: Some(&skills),
        raised: Raised::default(),
        wielded: Wielded {
            main: Weapon::UNARMED,
            off: None,
        },
        guard: Guard::NONE,
    };
    let punch = fighter.swing(0);
    let kick = fighter.kick();
    assert!(kick.cost > 2 * punch.cost);
    assert_eq!(kick.wind_up, wind_up(kick.cost));
    assert!(kick_skill().is_some());
    for seed in 0..50 {
        let luck = || Seed::new(seed).rng();
        let blow = fighter.harm(&punch, &fighter, &mut luck()).0;
        let shoved = fighter.kick_harm(&fighter, true, &mut luck()).0;
        let held = fighter.kick_harm(&fighter, false, &mut luck()).0;
        assert!(held < blow, "seed {seed}: {held} against {blow}");
        assert!(shoved * 2 <= held + 1 && held <= shoved * 2 + 1);
        // Against a thing it does more than a punch would.
        assert!(fighter.kick_force(&mut luck()).0 > blow);
    }
}

#[test]
fn people_and_beasts_with_legs_know_kick() {
    use crate::ability::{Ability, known_by_kind};
    assert!(crate::body::has_legs(CreatureKind::Human));
    assert!(known_by_kind(CreatureKind::Human).contains(&Ability::Kick));
    let carried = crate::item::Inventory::default();
    assert!(Ability::Kick.can_use(&carried, &crate::sheet::Sheet::new(CreatureKind::Human)));
}
