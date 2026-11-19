//! The pieces the rows are drawn from: an apparatus's box two cells across
//! with its picture (and, on a step, the colour fill and the wash while
//! running), a drop space, and the insertion mark.

use bevy::prelude::*;
use shared::id::Id;
use shared::item::{Item, ItemKind, Rotation};

use crate::dom::{SlotLook, SlotShown, SlotThing, Solid};
use crate::item_art::look_of;
use crate::widgets::{
    CELL, GRID_COLOUR, GRID_LINE, HoverTip, PICTURE_PIXELS_PER_CELL, REFUSED, UiId, item_picture,
    picture_in_square,
};

/// Gold: the insertion mark, and the third colour of the fill.
pub(super) const GOLD: Color = Color::srgb(0.85, 0.65, 0.13);

/// An apparatus box's side: two cells.
pub(super) const BOX: f32 = CELL * 2.0;

/// An empty space's picture, faint in the ink of the bin's.
const EMPTY_TINT: Color = Color::srgba(0.24, 0.16, 0.08, 0.55);
/// Over a step's picture while it runs: washed out, as black and white.
pub(super) const WASH: Color = Color::srgba(0.9, 0.9, 0.88, 0.7);

/// The colour fill along a step while it runs (`run::show_fill`).
#[derive(Component)]
pub(super) struct Fill(pub(super) usize);

/// The wash over a step's picture while it runs.
#[derive(Component)]
pub(super) struct Wash;

/// A box two cells across: its border red when `over` (past the steps
/// allowed), its tip, the picture of `kind`, and whatever `inside` adds.
pub(super) fn spawn_box(
    parent: &mut ChildSpawnerCommands,
    (kind, over): (Option<ItemKind>, bool),
    tip: String,
    marks: impl Bundle,
    inside: impl FnOnce(&mut ChildSpawnerCommands),
) {
    let node = Node {
        width: px(BOX),
        height: px(BOX),
        border: UiRect::all(px(if over { 3.0 } else { 1.0 })),
        ..default()
    };
    let border = BorderColor::from(if over { REFUSED } else { GRID_LINE });
    let mut entity = parent.spawn((node, border, BackgroundColor(GRID_COLOUR), Solid));
    entity.insert((HoverTip(tip), marks));
    entity.with_children(|within| {
        if let Some(kind) = kind {
            spawn_picture(within, &Item::new(Id::stand_in(0), kind));
        }
        inside(within);
    });
}

/// A step's fill and wash, over its picture.
pub(super) fn spawn_running(parent: &mut ChildSpawnerCommands, step: usize) {
    let over = |width: f32| Node {
        position_type: PositionType::Absolute,
        left: px(0),
        top: px(0),
        width: percent(width),
        height: percent(100.0),
        ..default()
    };
    parent.spawn((over(100.0), BackgroundColor(Color::NONE), Wash));
    parent.spawn((over(0.0), BackgroundColor(Color::NONE), Fill(step)));
}

/// An item's picture filling its box.
pub(super) fn spawn_picture(parent: &mut ChildSpawnerCommands, item: &Item) {
    let Some(look) = look_of(item) else {
        return;
    };
    let filling = Node {
        position_type: PositionType::Absolute,
        width: percent(100.0),
        height: percent(100.0),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };
    let pixels = look.longest() as u32 * PICTURE_PIXELS_PER_CELL * 2;
    let node = picture_in_square(&look);
    parent
        .spawn(filling)
        .with_child(item_picture(look, pixels, node, Rotation::None));
}

/// A drop space: the stack in it, or its faint picture.
pub(super) fn space_shown(
    stack: Option<&Item>,
    icon: Handle<Image>,
    aim: Option<bool>,
) -> SlotShown {
    SlotShown {
        look: SlotLook::Worn,
        icon: stack.is_none().then_some(icon),
        icon_tint: EMPTY_TINT,
        thing: stack.map(|item| SlotThing {
            item: item.clone(),
            count: item.count() as usize,
            faded: false,
            pressable: false,
        }),
        aim,
    }
}

/// Where a dragged step would go: a gold bar as tall as a box.
pub(super) fn spawn_insert_mark(parent: &mut ChildSpawnerCommands) {
    let node = Node {
        width: px(4.0),
        height: px(BOX),
        ..default()
    };
    parent.spawn((node, BackgroundColor(GOLD)));
}

/// A widget id for the debug API: `alchemy/<name>.<at>`.
pub(super) fn id(name: &'static str, at: usize) -> UiId {
    UiId::new("alchemy", name).at(at)
}
