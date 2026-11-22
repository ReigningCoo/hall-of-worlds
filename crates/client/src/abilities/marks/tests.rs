use bevy::ecs::system::RunSystemOnce;
use bevy::prelude::*;
use shared::ability::{Ability, ActionBar, BarSlot, BarThing};

use super::Marks;
use crate::abilities::manual::Manual;
use crate::common::{OpenPanels, Sneaking};
use crate::dom::AbilityMarks;

#[test]
fn the_bar_and_the_page_share_how_each_ability_is_used() {
    let mut bar = ActionBar::default();
    bar.slots[1] = Some(BarSlot {
        thing: BarThing::Ability(Ability::Attack),
        auto: true,
    });
    let mut manual = Manual::default();
    manual.press(Ability::Heal);
    let mut world = World::new();
    world.init_resource::<OpenPanels>();
    world
        .run_system_once(|mut panels: ResMut<OpenPanels>| {
            OpenPanels::tell(&mut panels, Ability::Alchemy, true);
        })
        .unwrap();
    let panels = world.resource::<OpenPanels>();
    let marks = Marks::now(&bar, &manual, Sneaking(true), panels);
    assert_eq!(
        marks.of(Ability::Attack),
        AbilityMarks {
            auto: true,
            ..default()
        }
    );
    assert!(marks.of(Ability::Heal).queued);
    // A toggle that is on, and a panel open.
    assert!(marks.of(Ability::Sneak).on);
    assert!(marks.of(Ability::Alchemy).on);
    assert_eq!(marks.of(Ability::Cook), AbilityMarks::default());

    let marks = Marks::now(&bar, &manual, Sneaking(false), &OpenPanels::default());
    assert!(!marks.of(Ability::Sneak).on);
    assert!(!marks.of(Ability::Alchemy).on);
}
