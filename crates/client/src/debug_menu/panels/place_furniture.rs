//! The Place Furniture panel (owner's request, 2026-11-16): what piece to put
//! down, and Place, a toggle that turns Furniture Mode on: clicks then put it
//! on the cell clicked, and clicks on the same cell go through its variations
//! (`furniture_order`).

use bevy::prelude::*;
use shared::door::Side;
use shared::furniture::{Material, Stone, Wood};

use super::furniture_order::Kind;
use crate::dom::Template;
use crate::html;
use crate::widgets::UiId;

pub(super) const FORM: &str = "debug.place_furniture";

/// The ways to face, by the dropdown's order.
pub(super) const FACINGS: [(&str, Side); 4] = [
    ("North", Side::North),
    ("East", Side::East),
    ("South", Side::South),
    ("West", Side::West),
];

/// Which of the panel's dropdowns it is.
#[derive(Component, Clone, Copy, PartialEq, Eq)]
pub(in crate::debug_menu) enum FurnitureChoice {
    Kind,
    Size,
    Facing,
    Material,
}

/// Place: on while Furniture Mode is.
#[derive(Component)]
pub(in crate::debug_menu) struct FurniturePlace;

/// "Click to place", shown while Furniture Mode is on.
#[derive(Component)]
pub(in crate::debug_menu) struct FurnitureClickToPlace;

/// The panel's contents, as markup; `start_furniture_panel` gives the
/// dropdowns their choices and sets them to the order kept.
pub(super) static PLACE_FURNITURE: Template = html!(
    "debug_menu.place_furniture",
    r#"
<form id="debug.place_furniture">
  <panel id="panel" moveable>
    <span nowrap valign="middle">
      <label for="kind">Kind</label>
      <select id="kind" width="100"><option>Table</option></select>
      <label for="size">Size</label>
      <select id="size" width="90"><option>2 × 3</option></select>
      <label for="facing">Facing</label>
      <select id="facing" width="90"><option>East</option></select>
      <label for="material">Material</label>
      <select id="material" width="120"><option>Random</option></select>
    </span>
    <span nowrap align="right" valign="middle">
      <p id="click_to_place" class="warning">Click to place, again to vary</p>
      <button id="place">Place</button>
    </span>
  </panel>
</form>
"#
);

/// A material's name in the dropdown.
fn material_name(material: Material) -> &'static str {
    match material {
        Material::Wood(wood) => match wood {
            Wood::Oak => "Oak",
            Wood::Walnut => "Walnut",
            Wood::Ash => "Ash",
            Wood::Pine => "Pine",
            Wood::Elm => "Elm",
            Wood::Beech => "Beech",
            Wood::Cherry => "Cherry",
            Wood::Ebony => "Ebony",
        },
        Material::Stone(stone) => match stone {
            Stone::Granite => "Granite",
            Stone::Sandstone => "Sandstone",
            Stone::Limestone => "Limestone",
            Stone::Marble => "Marble",
            Stone::Slate => "Slate",
        },
    }
}

impl FurnitureChoice {
    pub(super) const ALL: [Self; 4] = [Self::Kind, Self::Size, Self::Facing, Self::Material];

    /// Its dropdown's name in the markup.
    pub(super) fn ui_name(self) -> &'static str {
        match self {
            Self::Kind => "kind",
            Self::Size => "size",
            Self::Facing => "facing",
            Self::Material => "material",
        }
    }

    /// What it offers, in order, for pieces of `kind`.
    pub(super) fn options(self, kind: Kind) -> Vec<String> {
        match self {
            Self::Kind => Kind::ALL.map(|kind| kind.name().to_owned()).to_vec(),
            Self::Size => (kind.sizes().iter())
                .map(|&size| kind.size_name(size))
                .collect(),
            Self::Facing => FACINGS.map(|(name, _)| name.to_owned()).to_vec(),
            Self::Material => std::iter::once("Random")
                .chain(Material::ALL.map(material_name))
                .map(str::to_owned)
                .collect(),
        }
    }
}

/// What a built part of the panel is known by, from its id.
pub(super) fn bind(bound: &mut EntityCommands, id: &UiId) {
    let name = id.parts().1;
    if let Some(choice) = FurnitureChoice::ALL
        .into_iter()
        .find(|it| it.ui_name() == name)
    {
        bound.insert(choice);
        return;
    }
    match name {
        "place" => bound.insert(FurniturePlace),
        // Shown only while Furniture Mode is on.
        "click_to_place" => bound.insert((FurnitureClickToPlace, Visibility::Hidden)),
        _ => bound,
    };
}
