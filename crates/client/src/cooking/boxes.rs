//! The pieces the panel is drawn from: a tool's box two cells across with
//! its picture (an item's, or the Oven's icon), and on a step the colour fill
//! and the wash while running; a drop space; an empty place in the grid.

use bevy::prelude::*;
use shared::cooking::Tool;
use shared::id::Id;
use shared::item::{Item, ItemKind, Rotation};

use crate::dom::{SlotLook, SlotShown, SlotThing, Solid};
use crate::item_art::look_of;
use crate::widgets::{
    CELL, GRID_COLOUR, GRID_LINE, HoverTip, PICTURE_PIXELS_PER_CELL, REFUSED, ToolEdge, UiId,
    item_picture, picture_in_square,
};

/// Gold: an aimed place, and the third colour of the fill.
pub(super) const GOLD: Color = Color::srgb(0.85, 0.65, 0.13);

/// A tool box's side: two cells.
pub(super) const BOX: f32 = CELL * 2.0;

/// An empty space's picture, faint in the ink of the bin's.
const EMPTY_TINT: Color = Color::srgba(0.24, 0.16, 0.08, 0.55);
/// Over a step's picture while it runs: washed out, as black and white.
pub(super) const WASH: Color = Color::srgba(0.9, 0.9, 0.88, 0.7);
/// An empty place in the grid: barely there.
const EMPTY_PLACE: Color = Color::srgba(0.24, 0.16, 0.08, 0.08);

/// The colour fill along a step while it runs (`run::show_fill`).
#[derive(Component)]
pub(super) struct Fill(pub(super) usize);

/// The wash over a step's picture while it runs.
#[derive(Component)]
pub(super) struct Wash;

/// The item a tool is drawn as; the Oven has none, and a knife's picture
/// needs its rolled blade (their icons instead: `SpaceIcons::tool`).
pub(super) fn tool_item(tool: Tool) -> Option<ItemKind> {
    match tool {
        Tool::Pan => Some(ItemKind::Pan),
        Tool::Cauldron => Some(ItemKind::Cauldron),
        Tool::Bowl => Some(ItemKind::Bowl),
        Tool::Sieve => Some(ItemKind::Sieve),
        Tool::Mortar => Some(ItemKind::MortarAndPestle),
        Tool::Knife | Tool::Oven => None,
    }
}

/// What a box shows: an item's picture or an icon.
pub(super) enum Picture {
    Item(ItemKind),
    Icon(Handle<Image>),
    None,
}

/// A box two cells across (`height` boxes tall): its border (`ToolEdge`:
/// red when not to hand or past the steps allowed, else the quality of the
/// tool used), its tip, its picture, and whatever `inside` adds.
pub(super) fn spawn_box(
    parent: &mut ChildSpawnerCommands,
    (picture, edge, height): (Picture, ToolEdge, f32),
    tip: String,
    marks: impl Bundle,
    inside: impl FnOnce(&mut ChildSpawnerCommands),
) {
    let node = Node {
        width: px(BOX),
        height: px(BOX * height),
        border: UiRect::all(px(edge.width)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };
    let border = BorderColor::from(edge.colour);
    let mut entity = parent.spawn((node, border, BackgroundColor(GRID_COLOUR), Solid));
    entity.insert((HoverTip(tip), marks));
    entity.with_children(|within| {
        match picture {
            Picture::Item(kind) => spawn_picture(within, &Item::new(Id::stand_in(0), kind)),
            Picture::Icon(icon) => {
                let node = Node {
                    width: px(BOX * 0.8),
                    height: px(BOX * 0.8),
                    ..default()
                };
                within.spawn((
                    node,
                    ImageNode::new(icon).with_color(EMPTY_TINT.with_alpha(1.0)),
                ));
            }
            Picture::None => {}
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

/// A drop space: the stack in it (with `count` shown), or its faint picture.
pub(super) fn space_shown(
    stack: Option<(&Item, u32)>,
    icon: Handle<Image>,
    aim: Option<bool>,
) -> SlotShown {
    SlotShown {
        look: SlotLook::Worn,
        icon: stack.is_none().then_some(icon),
        icon_tint: EMPTY_TINT,
        thing: stack.map(|(item, count)| SlotThing {
            item: item.clone(),
            count: count as usize,
            faded: false,
            pressable: false,
        }),
        aim,
    }
}

/// An empty place in the grid, a box across, gold when a step would go
/// there, red when it can't.
pub(super) fn spawn_place(
    parent: &mut ChildSpawnerCommands,
    aim: Option<bool>,
    marks: impl Bundle,
) {
    let (width, colour) = match aim {
        Some(true) => (3.0, GOLD),
        Some(false) => (3.0, REFUSED),
        None => (1.0, GRID_LINE.with_alpha(0.3)),
    };
    let node = Node {
        width: px(BOX),
        height: px(BOX),
        border: UiRect::all(px(width)),
        ..default()
    };
    let tip = HoverTip("Drop a tool here".to_owned());
    parent.spawn((
        node,
        BorderColor::from(colour),
        BackgroundColor(EMPTY_PLACE),
        Solid,
        tip,
        marks,
    ));
}

/// A widget id for the debug API: `cooking/<name>.<at>`.
pub(super) fn id(name: &'static str, at: usize) -> UiId {
    UiId::new("cooking", name).at(at)
}
