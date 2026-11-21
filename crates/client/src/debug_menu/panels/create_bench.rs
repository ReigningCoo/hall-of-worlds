//! The Create Crafting Bench panel (owner's request, 2026-11-20): the kind of
//! bench, a dropdown per tool it takes ("None", then each quality), and
//! Create, a toggle for Bench Mode: a ghost of the bench follows the pointer
//! (`dungeon_map`), right click or Rotate turns it, and a click puts it down
//! for good (`BenchAsk::Dev`), turning Create off. Where it can't be used,
//! Create is greyed and the panel says why (`create_bench_where`).

use bevy::prelude::*;
use shared::crafting_bench::Craft;
use shared::door::Side;
use shared::furniture::{Furniture, Material, Piece, Stone, Wood, footprint};
use shared::item::Quality;
use shared::protocol::DevBench;

use crate::dom::{Dom, Template};
use crate::html;
use crate::widgets::UiId;

pub(super) const FORM: &str = "debug.create_crafting_bench";

/// What the panel has set up, kept while the game runs.
#[derive(Resource, Clone, Debug, PartialEq, Eq)]
pub(in crate::debug_menu) struct BenchOrder {
    pub(super) craft: Craft,
    /// The way its front faces (away from the wall behind it).
    pub(super) facing: Side,
    /// Each of the craft's tools (`Craft::tools`, in order): its quality, or
    /// none.
    pub(super) tools: Vec<Option<Quality>>,
}

impl Default for BenchOrder {
    fn default() -> Self {
        Self::of(Craft::Alchemy, Side::South)
    }
}

impl BenchOrder {
    /// A bench of `craft` with no tools, facing `facing`.
    pub(super) fn of(craft: Craft, facing: Side) -> Self {
        let tools = vec![None; craft.tools().len()];
        Self {
            craft,
            facing,
            tools,
        }
    }

    /// The bench and its tools, its north-west cell at `at`.
    pub(super) fn bench(&self, at: (i64, i64)) -> DevBench {
        let furniture = Furniture::CraftingBench { craft: self.craft };
        let piece = Piece {
            at,
            size: footprint(furniture, (self.craft.length(), 1), self.facing),
            facing: self.facing,
            // Hearths are of stone (`Piece::is_well_made`).
            material: match self.craft.is_hearth() {
                true => Material::Stone(Stone::Granite),
                false => Material::Wood(Wood::Oak),
            },
            furniture,
        };
        let kinds = self.craft.tools().into_iter().zip(&self.tools);
        let tools = kinds.filter_map(|(kind, quality)| Some((kind, (*quality)?)));
        DevBench {
            piece,
            tools: tools.collect(),
        }
    }

    /// Turned a quarter turn clockwise, seen from above.
    pub(super) fn turned(&self) -> Self {
        let facing = match self.facing {
            Side::North => Side::East,
            Side::East => Side::South,
            Side::South => Side::West,
            Side::West => Side::North,
        };
        Self {
            facing,
            ..self.clone()
        }
    }
}

/// A craft's name in the Type dropdown.
pub(super) fn craft_name(craft: Craft) -> &'static str {
    match craft {
        Craft::Alchemy => "Alchemy",
        hearth => hearth.name(),
    }
}

/// The Type dropdown.
#[derive(Component)]
pub(in crate::debug_menu) struct BenchType;

/// The dropdown of the tool at this place in the craft's list.
#[derive(Component, Clone, Copy)]
pub(in crate::debug_menu) struct BenchTool(pub(super) usize);

/// The list the tools' rows are copied into.
#[derive(Component)]
pub(in crate::debug_menu) struct BenchToolList;

/// Create: on while Bench Mode is.
#[derive(Component)]
pub(in crate::debug_menu) struct BenchCreate;

/// Rotate: a quarter turn, for touch (right click does it too).
#[derive(Component)]
pub(in crate::debug_menu) struct BenchRotate;

/// "Click to place", shown while Bench Mode is on.
#[derive(Component)]
pub(in crate::debug_menu) struct BenchClickToPlace;

/// The panel's contents, as markup; each tool's row is a copy of the
/// template, numbered by its place in `Craft::tools`.
pub(super) static CREATE_CRAFTING_BENCH: Template = html!(
    "debug_menu.create_crafting_bench",
    r#"
<form id="debug.create_crafting_bench">
  <panel id="panel" moveable>
    <span nowrap valign="middle">
      <label for="type">Type</label>
      <select id="type" width="120"><option>Alchemy</option></select>
    </span>
    <div id="tools" class="tight"/>
    <p id="why_not" class="warning" max-width="320" hidden>.</p>
    <span nowrap align="right" valign="middle">
      <p id="click_to_place" class="warning">Click to place, right click to turn</p>
      <button id="rotate">Rotate</button>
      <button id="create">Create</button>
    </span>
    <template id="tool">
      <span nowrap valign="middle">
        <select id="quality" width="120"><option>None</option></select>
        <p id="name"/>
      </span>
    </template>
  </panel>
</form>
"#
);

/// Fills the list with a row for each of `craft`'s tools.
pub(super) fn fill_tools(dom: &mut Dom, craft: Craft) {
    let list = format!("{FORM}/tools");
    dom.empty(&list);
    for (at, kind) in craft.tools().into_iter().enumerate() {
        dom.copy_template(&format!("{FORM}/tool"), &list, at);
        dom.set_text(&format!("{FORM}/name.{at}"), kind.def().name);
    }
}

/// What a built part of the panel is known by, from its id.
pub(super) fn bind(bound: &mut EntityCommands, id: &UiId) {
    match (id.parts().1, id.index()) {
        ("type", _) => bound.insert(BenchType),
        ("tools", _) => bound.insert(BenchToolList),
        ("quality", Some(at)) => bound.insert(BenchTool(at)),
        ("create", _) => bound.insert(BenchCreate),
        ("rotate", _) => bound.insert(BenchRotate),
        // Shown only while Bench Mode is on.
        ("click_to_place", _) => bound.insert((BenchClickToPlace, Visibility::Hidden)),
        _ => bound,
    };
}
