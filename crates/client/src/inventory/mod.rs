//! What the character carries: grids in moveable panels, and picking things up,
//! holding them and putting them down. See `docs/context/client/inventory.md`.

mod bag_slots;
mod body;
mod ground;
mod ground_shown;
mod hands;
mod hold;
mod panels;
mod quest_drop;

use bevy::prelude::*;
use panels::OpenGrids;
use shared::item::Inventory;

use crate::common::{GaveItem, HeldOverGifts, HeldStock, PointerSystems, TookItem};
use crate::dom::DomInput;

// Split into files (`cargo xtask lint` keeps files small).
#[cfg(not(target_arch = "wasm32"))]
mod debug;
mod early_pictures;
mod filter;
mod finger;
mod hold_items;
mod holding;
mod server;
mod sort;
mod split;
mod strength_room;
mod ui_ids;
mod wear_note;

#[cfg(not(target_arch = "wasm32"))]
use debug::run_debug;
use finger::{item_cell_in_front, pick_up_by_finger};
pub use ground_shown::{GroundInReach, GroundShown};
use hold_items::{aim_items, drink_potion, eat_kind, hold_items};
use holding::{Hand, HeldBy, Holding, cancel_held};
pub use server::Stacks;
use server::read_server;
use split::Split;

/// A press on an item this long after the last one on it opens it.
const DOUBLE_PRESS_SECONDS: f64 = 0.35;

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Carried>()
            .init_resource::<crate::common::Trading>()
            .init_resource::<Stacks>()
            .init_resource::<Holding>()
            .init_resource::<bag_slots::EmptyIcon>()
            .init_resource::<Split>()
            .init_resource::<OpenGrids>()
            .init_resource::<ground::Ground>()
            .init_resource::<GroundShown>()
            .init_resource::<GroundInReach>()
            .init_resource::<HeldOverGifts>()
            .init_resource::<HeldStock>()
            .init_resource::<wear_note::WearNote>()
            .init_resource::<strength_room::StrengthRoom>()
            .init_resource::<quest_drop::QuestDrop>()
            .add_message::<TookItem>()
            .add_message::<OpenCharacter>()
            .add_message::<EatKind>()
            .add_message::<DrinkPotion>()
            .add_message::<GaveItem>()
            .add_message::<crate::common::OpenLantern>()
            .add_systems(
                Startup,
                (
                    panels::spawn_fixed_parts,
                    quest_drop::spawn_requester,
                    filter::make_icons,
                    sort::make_icon,
                ),
            )
            .add_systems(
                PreUpdate,
                (
                    // Before widgets stop a finger's drag on a panel, and before the
                    // menus see Esc.
                    (pick_up_by_finger, cancel_held)
                        .before(DomInput::Presses)
                        .after(PointerSystems),
                    // Once the split box has said whether it closed itself.
                    split::close_split
                        .after(crate::dom::ClosePopups)
                        .before(DomInput::Presses),
                    // Either side of `dom` moving what is held.
                    (
                        hold_items.before(crate::dom::HoldObjects),
                        aim_items.after(crate::dom::HoldObjects),
                    )
                        .after(DomInput::Modal),
                ),
            )
            .add_systems(
                Update,
                (
                    read_server,
                    panels::toggle_inventory,
                    panels::forget_closed,
                    panels::show_bins,
                    panels::show_drop,
                    (ground::read_server, filter::keep_filters).chain(),
                    (
                        ground::bind_section,
                        ground::show_section,
                        ground_shown::keep_ground_shown,
                    )
                        .chain(),
                    ground::show_aim,
                    (
                        (panels::bind_frames, panels::bind_pages),
                        (panels::show_grids, panels::show_weights),
                    )
                        .chain(),
                    (
                        body::bind_clothes,
                        body::show_clothes,
                        bag_slots::show_bag_slots,
                    )
                        .chain(),
                    strength_room::keep_strength_room,
                    (body::show_body_aim, wear_note::show_wear_note),
                    panels::show_aim,
                    panels::show_tooltip,
                    (panels::bind_button, panels::show_button).chain(),
                    split::move_split,
                    (split::show_split, split::bind_split).chain(),
                )
                    .chain(),
            );
        // Once the grids have drawn themselves.
        app.add_systems(
            PostUpdate,
            panels::mark_grids.after(crate::dom::DrawObjects),
        );
        app.add_systems(Update, early_pictures::ask_early);
        app.add_systems(Update, (eat_kind, drink_potion).before(panels::show_grids));
        app.add_systems(
            Update,
            (quest_drop::answer_requester, quest_drop::show_requester).chain(),
        );
        app.add_systems(Update, panels::switch_pages.after(panels::toggle_inventory));
        // Before the grids are drawn, so a sort shows at once.
        app.add_systems(Update, sort::press_sort.before(panels::show_grids));
        #[cfg(not(target_arch = "wasm32"))]
        app.add_message::<InventoryDebug>()
            .add_systems(Update, run_debug.before(panels::forget_closed));
    }
}

/// Opens the Character panel, or brings it to the front.
#[derive(Message, Clone, Copy, Debug)]
pub struct OpenCharacter;

/// Eat one of this kind (from the action bar): from the smallest carried
/// stack in the own grid, else the smallest in a bag
/// (`Inventory::stack_to_use`), as eating on the clothes panel does. Nothing
/// if none is carried, or the player's kind won't eat it.
#[derive(Message, Clone, Copy, Debug)]
pub struct EatKind(pub shared::item::ItemKind);

/// Drink a bottle of this potion (from the action bar): from the stack
/// `Inventory::potion_to_use` chooses, as letting go of it on the clothes
/// panel does. Nothing if none is carried.
#[derive(Message, Clone, Copy, Debug)]
pub struct DrinkPotion(pub shared::potion::PotionKey);

/// What the character carries, as last heard from the server plus any move
/// since, which it answers.
#[derive(Resource, Default)]
pub struct Carried(pub Inventory);

/// For looking at the panels (`dev_tools`).
#[cfg(not(target_arch = "wasm32"))]
#[derive(Message, Clone, Copy, Debug)]
pub enum InventoryDebug {
    OpenInventory,
    /// Opens every bag in the inventory.
    OpenBags,
    /// Picks up the item in the inventory at this index, until the program ends.
    PickUp(usize),
    /// Puts all but the first of what is in the inventory on the ground.
    DropSome,
}
