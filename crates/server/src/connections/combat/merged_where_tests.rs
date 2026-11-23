//! Wheres merged in one step (magic plan PR 10): Missile + Surround → Heal
//! heals the one the missile meets and whoever is round them.

use shared::creature::CreatureKind;
use shared::poi::PoiKind;
use shared::protocol::{ClientMessage, Landed, ServerMessage, SpellbookAsk};
use shared::skill::{SkillId, practice_for};
use shared::spell::{Effect, Filters, Spell, SpellKey, Step, Where, WherePart};

use super::tests::{received, run};
use crate::connections::tests::TestClient;

#[test]
fn missile_and_surround_heal_the_one_met_and_the_ring_round_them() {
    let mut test = TestClient::new();
    test.send(&[ClientMessage::DevEnterSite {
        kind: PoiKind::Castle,
        seed: 7,
        level: 0,
        ruin: 0,
    }]);
    let (x, y) = test.client.visit.as_ref().unwrap().room_to_walk_east(4);
    test.send(&[ClientMessage::DevTeleport { x, y }]);
    let population = test.world.populations.0.values_mut().next().unwrap();
    let near = (population.remake_for_test(0, CreatureKind::Dwarf, (x + 3, y))).id;
    let behind = (population.remake_for_test(1, CreatureKind::Dwarf, (x + 4, y))).id;
    // Surround unlocked (Magic at 5%), and skilled enough to afford it.
    for (path, percent) in [
        ("magic", 5),
        ("magic/form/missile", 100),
        ("magic/form/surround", 100),
        ("magic/divine/healing", 100),
    ] {
        let skill = SkillId::find(path).unwrap();
        test.client
            .sheet
            .skills
            .practise(skill, practice_for(percent));
    }
    let parts = [
        WherePart {
            place: Where::Missile,
            count: 1,
        },
        WherePart {
            place: Where::Surround,
            count: 1,
        },
    ];
    let spell = Spell::new(vec![
        Step::merged(&parts, Filters::NONE).unwrap(),
        Step::effects(&[(Effect::Heal, 1)]),
    ]);
    let skills = test.client.sheet.skills.clone();
    let book = &mut test.client.sheet.spellbook;
    book.store(spell.clone(), None, Some(&skills)).unwrap();
    // Aimed at the one behind: the missile meets the near one first.
    test.send(&[ClientMessage::Spellbook(SpellbookAsk::Cast {
        spell: SpellKey::of(&spell),
        target: Some(behind),
    })]);
    let mut healed = Vec::new();
    for _ in 0..80 {
        run(&mut test, 1, false);
        for message in received(&mut test) {
            if let ServerMessage::AbilityLanded {
                target: Some(target),
                landed: Landed::Healed { .. },
                ..
            } = message
            {
                healed.push(target);
            }
        }
    }
    healed.sort_unstable();
    let mut both = vec![near, behind];
    both.sort_unstable();
    assert_eq!(healed, both, "each healed once");
}
