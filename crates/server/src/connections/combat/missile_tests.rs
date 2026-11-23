//! Missiles in flight (magic plan PR 6): a Healing Missile flies to a mob a
//! few cells off, everyone on the level is told as it is launched and as it
//! lands, and the Heal lands on whoever it met.

use shared::creature::CreatureKind;
use shared::id::Id;
use shared::poi::PoiKind;
use shared::protocol::{ClientMessage, Landed, MissileNews, ServerMessage, SpellbookAsk};
use shared::skill::{SkillId, practice_for};
use shared::spell::{SpellKey, healing_missile};

use super::tests::{received, run};
use crate::connections::tests::TestClient;

/// A player in a castle with a dwarf four cells east.
fn four_cells_off() -> (TestClient, Id) {
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
    let id = (population.remake_for_test(0, CreatureKind::Dwarf, (x + 4, y))).id;
    // Missile unlocked (Magic at 2%).
    let magic = SkillId::find("magic").unwrap();
    test.client.sheet.skills.practise(magic, practice_for(2));
    (test, id)
}

#[test]
fn a_healing_missile_flies_then_heals_the_one_it_meets() {
    let (mut test, mob) = four_cells_off();
    assert!(test.client.sheet.spellbook.knows(&healing_missile()));
    test.send(&[ClientMessage::Spellbook(SpellbookAsk::Cast {
        spell: SpellKey::of(&healing_missile()),
        target: Some(mob),
    })]);
    let (mut launched, mut landed) = (None, None);
    let mut healed = false;
    for tick in 0..80 {
        run(&mut test, 1, false);
        for message in received(&mut test) {
            match message {
                ServerMessage::Missile(MissileNews::Flying { path, .. }) => {
                    assert!(path.length > 3 * shared::spot::CELL_SPAN as u64);
                    launched = Some(tick);
                }
                ServerMessage::Missile(MissileNews::Landed { .. }) => landed = Some(tick),
                ServerMessage::AbilityLanded {
                    target: Some(target),
                    landed: Landed::Healed { .. },
                    ..
                } if target == mob => {
                    assert!(landed.is_some(), "healed once it has landed");
                    healed = true;
                }
                _ => {}
            }
        }
    }
    let (launched, landed) = (launched.unwrap(), landed.unwrap());
    // About 3.5 cells at 16 cells a second, 20 ticks a second.
    assert!(
        (3..=6).contains(&(landed - launched)),
        "{launched} {landed}"
    );
    assert!(healed);
}
