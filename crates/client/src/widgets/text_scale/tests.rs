use bevy::ecs::system::SystemId;
use bevy::prelude::*;
use bevy::ui::ComputedUiRenderTargetInfo;

use super::follow_scale;

/// Whether the text has changed since this last looked, as Bevy's text
/// measuring sees it.
#[derive(Resource, Default)]
struct Marked(bool);

fn note_marked(texts: Query<Ref<Text>>, mut marked: ResMut<Marked>) {
    marked.0 = texts.iter().any(|text| text.is_changed());
}

/// A world with one piece of text, after a first frame, so nothing about it
/// counts as changed any more; and a frame of `follow_scale` then the check.
fn settled() -> (World, Entity, [SystemId; 2]) {
    let mut world = World::new();
    world.init_resource::<Marked>();
    let text = world
        .spawn((Text::new("Combat"), ComputedUiRenderTargetInfo::default()))
        .id();
    let systems = [
        world.register_system(follow_scale),
        world.register_system(note_marked),
    ];
    frame(&mut world, systems);
    (world, text, systems)
}

fn frame(world: &mut World, systems: [SystemId; 2]) -> bool {
    for system in systems {
        world.run_system(system).unwrap();
    }
    world.resource::<Marked>().0
}

#[test]
fn text_is_measured_again_when_its_scale_arrives_after_it() {
    let (mut world, text, systems) = settled();
    // Text set at run time under an interface already on show: the scale
    // reaches it after it was first measured.
    world
        .get_mut::<ComputedUiRenderTargetInfo>(text)
        .unwrap()
        .set_changed();
    assert!(frame(&mut world, systems));
}

#[test]
fn text_whose_scale_stays_is_left_alone() {
    let (mut world, _, systems) = settled();
    assert!(!frame(&mut world, systems));
}
