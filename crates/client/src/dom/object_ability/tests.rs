use shared::ability::Ability;

use super::*;
use crate::widgets::AbilityLook;

fn app() -> App {
    let mut app = App::new();
    app.add_systems(Update, (draw_ability_grids, draw_ability_slots));
    app
}

fn thing(ability: Ability) -> AbilityThing {
    let look = AbilityLook {
        icon: Handle::default(),
        name: ability.name().to_string(),
        passive: !ability.is_active(),
        usable: true,
        colour: Color::WHITE,
        item: None,
    };
    AbilityThing {
        thing: ability.into(),
        look,
    }
}

fn cells(app: &mut App) -> Vec<AbilityCell> {
    let mut cells = app.world_mut().query::<&AbilityCell>();
    cells.iter(app.world()).copied().collect()
}

fn parts(app: &mut App) -> Vec<SlotPart> {
    let mut parts = app.world_mut().query::<&SlotPart>();
    parts.iter(app.world()).copied().collect()
}

#[test]
fn a_list_draws_each_ability_as_its_own() {
    let mut app = app();
    let shown = AbilityGridShown {
        columns: 8,
        things: vec![
            (thing(Ability::Attack), AbilityMarks::default()),
            (
                thing(Ability::Heal),
                AbilityMarks {
                    on: true,
                    ..default()
                },
            ),
        ],
    };
    let list = app.world_mut().spawn(shown).id();
    app.update();
    let drawn = cells(&mut app);
    assert_eq!(drawn.len(), 2);
    assert!(drawn.iter().all(|cell| cell.of == list));
    assert!(drawn.iter().any(|cell| cell.thing == Ability::Heal.into()));
    // Marked as a slot would be: only the one that is on.
    assert_eq!(parts(&mut app), vec![SlotPart::On(list)]);
}

#[test]
fn a_slot_draws_its_ability_and_how_it_is_used() {
    let mut app = app();
    let slot = app.world_mut().spawn(AbilitySlotShown::default()).id();
    app.update();
    // Empty: a frame, and nothing to find.
    assert!(cells(&mut app).is_empty());
    assert!(parts(&mut app).is_empty());

    let shown = AbilitySlotShown {
        thing: Some(thing(Ability::Attack)),
        key: "1".to_string(),
        marks: AbilityMarks {
            on: true,
            queued: true,
            ..default()
        },
        shade: true,
        ..default()
    };
    *app.world_mut().get_mut::<AbilitySlotShown>(slot).unwrap() = shown;
    app.update();
    let drawn = cells(&mut app);
    assert_eq!(drawn.len(), 1);
    assert_eq!(
        (drawn[0].of, drawn[0].thing),
        (slot, Ability::Attack.into())
    );
    let found = parts(&mut app);
    assert!(found.contains(&SlotPart::On(slot)));
    assert!(found.contains(&SlotPart::Queued(slot)));
    assert!(found.contains(&SlotPart::Shade(slot)));
}
