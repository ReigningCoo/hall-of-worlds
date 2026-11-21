//! Drawing the panel into its canvases: along the top, every tool
//! (`ToolNode`, red when not to hand); under it, the grid: each row's source
//! space, then its places, each an empty place (`PlaceNode`) or a step's box
//! (`StepNode`, a join two rows tall) with its drop spaces under it
//! (`SpaceNode`); and the culture's space. Drawn again whenever the process,
//! what is carried, the skills, the kitchen or a drag's aim changes.

use bevy::prelude::*;
use shared::alchemy::steps_allowed;
use shared::cooking::{CookingSkills, Tool};
use shared::id::Id;
use shared::item::Inventory;

use super::bench::{Bench, ROWS};
use super::boxes::{Picture, id, space_shown, spawn_box, spawn_place, spawn_running, tool_item};
use super::drag::{BenchAim, Dragging};
use super::grid_nodes::{at, place_id, row_node, spaces_node, water_shown};
use super::kitchen::{KitchenNow, Kitchens};
use super::spaces::{ML_STEP, Place, SpaceIcons, in_place, places, tip};
use crate::character::CharacterSheet;
use crate::dom::Solid;
use crate::inventory::Carried;
use crate::widgets::HoverTip;

/// The top row's canvas.
#[derive(Component)]
pub(super) struct ToolRow;

/// The grid's canvas.
#[derive(Component)]
pub(super) struct GridCanvas;

/// The culture space's canvas.
#[derive(Component)]
pub(super) struct CultureCanvas;

/// A tool in the top row.
#[derive(Component, Clone, Copy)]
pub(super) struct ToolNode(pub(super) Tool);

/// A step's box in the grid.
#[derive(Component, Clone, Copy)]
pub(super) struct StepNode(pub(super) usize);

/// An empty place in the grid: its column and row.
#[derive(Component, Clone, Copy)]
pub(super) struct PlaceNode(pub(super) u8, pub(super) u8);

/// A drop space.
#[derive(Component, Clone, Copy)]
pub(super) struct SpaceNode(pub(super) Place);

/// A canvas just built.
type NewCanvas = Or<(Added<ToolRow>, Added<GridCanvas>, Added<CultureCanvas>)>;

/// What drawing reads.
type Reads<'w> = (
    Res<'w, Carried>,
    Res<'w, CharacterSheet>,
    Res<'w, Kitchens>,
    Res<'w, BenchAim>,
    Res<'w, SpaceIcons>,
);

#[allow(clippy::type_complexity)]
pub(super) fn draw_grid(
    mut bench: ResMut<Bench>,
    (carried, sheet, kitchens, aim, icons): Reads,
    canvases: Query<
        (Entity, Has<ToolRow>, Has<GridCanvas>),
        Or<(With<ToolRow>, With<GridCanvas>, With<CultureCanvas>)>,
    >,
    added: Query<(), NewCanvas>,
    mut commands: Commands,
) {
    let kitchen = &kitchens.0;
    if carried.is_changed() || kitchens.is_changed() {
        let there = |id: Id| carried.0.find(id).is_some() || kitchen.stored(id).is_some();
        if bench.uses().iter().any(|(id, ..)| !there(*id))
            || bench.culture.is_some_and(|id| !there(id))
        {
            bench.keep_there(there);
        }
    }
    let changed = bench.is_changed() || carried.is_changed() || sheet.is_changed();
    if !(changed || kitchens.is_changed() || aim.is_changed() || !added.is_empty()) {
        return;
    }
    let skills = CookingSkills::of(&sheet.0.skills);
    let allowed = steps_allowed(skills.cooking_percent()) as usize;
    let draw = Draw {
        bench: &bench,
        carried: &carried.0,
        kitchen,
        aim: &aim,
        icons: &icons,
    };
    for (canvas, tools, grid) in &canvases {
        commands.entity(canvas).despawn_related::<Children>();
        commands.entity(canvas).with_children(|within| {
            if tools {
                draw.tools(within);
            } else if grid {
                draw.grid(within, allowed);
            } else {
                draw.space(within, Place::Culture);
            }
        });
    }
}

/// What the canvases are drawn from.
struct Draw<'a> {
    bench: &'a Bench,
    carried: &'a Inventory,
    kitchen: &'a KitchenNow,
    aim: &'a BenchAim,
    icons: &'a SpaceIcons,
}

impl Draw<'_> {
    /// Every tool, red when neither carried nor in the kitchen.
    fn tools(&self, parent: &mut ChildSpawnerCommands) {
        parent.spawn(row_node()).with_children(|row| {
            for tool in Tool::ALL {
                let to_hand = self.carried.has_cooking_tool(tool, self.kitchen.kitchen());
                let tip = match to_hand {
                    true => format!("{}: drag it down, or click or tap it", tool.name()),
                    false if tool == Tool::Oven => "Oven (only at a lit hearth)".to_owned(),
                    false => format!("{} (not to hand)", tool.name()),
                };
                let marks = (ToolNode(tool), id("tool", tool.index()));
                spawn_box(row, (self.picture(tool), !to_hand, 1.0), tip, marks, |_| {});
            }
        });
    }

    /// A tool's picture.
    fn picture(&self, tool: Tool) -> Picture {
        match tool_item(tool) {
            Some(kind) => Picture::Item(kind),
            None => Picture::Icon(self.icons.tool(tool)),
        }
    }

    /// The grid: sources, then the places of each row.
    fn grid(&self, parent: &mut ChildSpawnerCommands, allowed: usize) {
        let columns = self.bench.columns_shown();
        let node = Node {
            display: Display::Grid,
            grid_template_columns: RepeatedGridTrack::auto(u16::from(columns) + 1),
            grid_template_rows: RepeatedGridTrack::auto(ROWS as u16),
            column_gap: px(8),
            row_gap: px(8),
            justify_content: JustifyContent::Start,
            justify_items: JustifyItems::Start,
            ..default()
        };
        // Steps past the allowed, in the order they run.
        let order = self.bench.in_order();
        parent.spawn(node).with_children(|grid| {
            for row in 0..ROWS as u8 {
                grid.spawn(at(0, row, 1)).with_children(|cell| {
                    self.space(cell, Place::Source(row));
                });
                for column in 0..columns {
                    match self.bench.step_at(column, row) {
                        Some(step) if self.bench.steps[step].row == row => {
                            let over =
                                order.iter().position(|&s| s == step).unwrap_or(0) >= allowed;
                            self.step(grid, step, over);
                        }
                        Some(_) => {}
                        None => {
                            let aimed =
                                self.aim.place.filter(|p| p.0 == (column, row)).map(|p| p.1);
                            grid.spawn(at(column + 1, row, 1)).with_children(|cell| {
                                let marks = (
                                    PlaceNode(column, row),
                                    id("place", usize::from(column) * 10 + usize::from(row)),
                                );
                                spawn_place(cell, aimed, marks);
                            });
                        }
                    }
                }
            }
        });
    }

    /// A step's box and spaces, in its place.
    fn step(&self, grid: &mut ChildSpawnerCommands, step: usize, over: bool) {
        let station = &self.bench.steps[step];
        let dragged = self.aim.dragging == Some(Dragging::Step(step));
        let join = station.height > 1;
        let name = station.tool.name();
        let tip = match (over, join) {
            (true, _) => format!("{name}: past the steps allowed, halving the chance"),
            (false, true) => format!("{name}, joining both rows: click or tap to split"),
            (false, false) => format!("{name}: click or tap to join both rows"),
        };
        let picture = if dragged {
            Picture::None
        } else {
            self.picture(station.tool)
        };
        let column = at(station.column + 1, station.row, station.height);
        grid.spawn(column).with_children(|column| {
            let height = if join { 2.0 } else { 1.0 };
            let marks = (StepNode(step), id("step", step));
            spawn_box(column, (picture, over, height), tip, marks, |inside| {
                spawn_running(inside, step)
            });
            column.spawn(spaces_node()).with_children(|spaces| {
                for place in places(step, station, self.kitchen.water_faced) {
                    self.space(spaces, place);
                }
            });
        });
    }

    /// One drop space.
    fn space(&self, parent: &mut ChildSpawnerCommands, place: Place) {
        let faced = match place {
            Place::Faced(step) => self.bench.steps.get(step).map_or(0, |s| s.faced),
            _ => 0,
        };
        let water = (faced > 0).then(water_shown);
        let stack = in_place(self.bench, place).and_then(|(id, n)| {
            let item = self.carried.find(id).or_else(|| self.kitchen.stored(id))?;
            let shown = match place {
                Place::Liquid(..) => n / ML_STEP,
                _ => n,
            };
            Some((item, shown))
        });
        let stack = stack.or(water.as_ref().map(|item| (item, faced / ML_STEP)));
        let aimed = self
            .aim
            .space
            .filter(|(on, _)| *on == place)
            .map(|(_, ok)| ok);
        let shown = space_shown(stack, self.icons.of(place), aimed);
        let tip = HoverTip(tip(place).to_owned());
        let (name, at) = place_id(place);
        parent.spawn((shown, SpaceNode(place), tip, Solid, id(name, at)));
    }
}
