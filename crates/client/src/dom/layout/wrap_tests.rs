//! A row that just fits a panel sized to fit it never wraps, whatever the
//! interface's size: laid out by Bevy's own layout, headless (as `bevy_ui`'s
//! tests set it up).

use bevy::app::{HierarchyPropagatePlugin, PropagateSet};
use bevy::camera::{ComputedCameraValues, RenderTargetInfo, Viewport};
use bevy::transform::systems::{
    mark_dirty_trees, propagate_parent_transforms, sync_simple_transforms,
};
use bevy::ui::ui_surface::UiSurface;
use bevy::ui::update::propagate_ui_target_cameras;
use bevy::ui::{
    ComputedUiRenderTargetInfo, ComputedUiTargetCamera, sync_font_size_to_em_size, ui_layout_system,
};

use super::*;
use crate::dom::theme::Theme;

/// The window's own scale, as on a laptop at 150%.
const WINDOW_SCALE: f32 = 1.5;

fn layout_app() -> App {
    let mut app = App::new();
    app.add_plugins(TaskPoolPlugin::default());
    app.add_plugins(HierarchyPropagatePlugin::<ComputedUiTargetCamera>::new(
        PostUpdate,
    ));
    app.add_plugins(HierarchyPropagatePlugin::<ComputedUiRenderTargetInfo>::new(
        PostUpdate,
    ));
    app.init_resource::<UiScale>();
    app.init_resource::<UiSurface>();
    app.init_resource::<bevy::text::TextPipeline>();
    app.init_resource::<bevy::text::FontCx>();
    app.init_resource::<bevy::text::RemSize>();
    app.init_resource::<bevy::text::ScaleCx>();
    app.init_resource::<bevy::transform::StaticTransformOptimizations>();
    app.add_systems(
        PostUpdate,
        (
            ApplyDeferred,
            propagate_ui_target_cameras,
            sync_font_size_to_em_size,
            ui_layout_system,
            mark_dirty_trees,
            sync_simple_transforms,
            propagate_parent_transforms,
        )
            .chain(),
    );
    for set in [
        PropagateSet::<ComputedUiTargetCamera>::default().intern(),
        PropagateSet::<ComputedUiRenderTargetInfo>::default().intern(),
    ] {
        app.configure_sets(
            PostUpdate,
            set.after(propagate_ui_target_cameras)
                .before(ui_layout_system),
        );
    }
    let size = UVec2::new(3000, 1600);
    app.world_mut().spawn((
        Camera2d,
        Camera {
            computed: ComputedCameraValues {
                target_info: Some(RenderTargetInfo {
                    physical_size: size,
                    scale_factor: WINDOW_SCALE,
                }),
                ..default()
            },
            viewport: Some(Viewport {
                physical_size: size,
                ..default()
            }),
            ..default()
        },
    ));
    app
}

/// A panel like a moveable one (absolute, fitting what is in it, with an edge
/// and padding) holding a block holding a `span` of three 44 px buttons, as
/// the movement pad has: returns the first and last buttons.
fn spawn_pad_row(world: &mut World) -> (Entity, Entity) {
    let look = Theme::parchment().look(Tag::Span, None);
    let around = Around {
        look: &look,
        spot: None,
        row_hidden: false,
        line: 20.0,
    };
    let span = node_for(Tag::Span, &Attrs::default(), &around);
    let button = || Node {
        width: px(44),
        height: px(44),
        flex_shrink: 0.0,
        ..default()
    };
    let first = world.spawn(button()).id();
    let middle = world.spawn(button()).id();
    let last = world.spawn(button()).id();
    let span = world.spawn(span).add_children(&[first, middle, last]).id();
    let block = world
        .spawn(Node {
            flex_direction: FlexDirection::Column,
            ..default()
        })
        .add_child(span)
        .id();
    world
        .spawn(Node {
            position_type: PositionType::Absolute,
            left: px(1700),
            top: px(900),
            flex_direction: FlexDirection::Column,
            border: UiRect::all(px(PANEL_EDGE)),
            padding: UiRect::all(px(8)),
            ..default()
        })
        .add_child(block);
    (first, last)
}

#[test]
fn a_row_that_just_fits_never_wraps() {
    let mut app = layout_app();
    let (first, last) = spawn_pad_row(app.world_mut());
    let mut wrapped = Vec::new();
    // Every Auto size a window might give, a thousandth apart.
    for step in 500..=1500 {
        let scale = step as f32 / 1000.0;
        app.world_mut().resource_mut::<UiScale>().0 = scale;
        app.update();
        let top = |entity| {
            app.world()
                .get::<UiGlobalTransform>(entity)
                .unwrap()
                .translation
                .y
        };
        if (top(first) - top(last)).abs() > 0.5 {
            wrapped.push(scale);
        }
    }
    assert!(wrapped.is_empty(), "wrapped at {wrapped:?}");
}
