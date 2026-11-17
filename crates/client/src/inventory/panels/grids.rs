//! Opening grids in panels.

use bevy::prelude::*;
use shared::item::{Holder, Inventory};

use super::super::ui_ids::grid_id;
use super::frame::spawn_bag;
use super::pages::CHARACTER;
use super::shown::{grows, shown_size};
use super::{FIRST_AT, NEXT_AT, OpenGrids};
use crate::dom::{Dom, document, top_strip};

impl OpenGrids {
    /// Opens this grid's panel, or brings it to the front if it is open. The
    /// panel is its document's (`inventory/panel`, `bag#<id>/panel`).
    pub(in crate::inventory) fn open(
        &mut self,
        holder: Holder,
        carried: &Inventory,
        commands: &mut Commands,
        dom: &mut Dom,
    ) {
        let panel = grid_id(Some(holder), "panel").to_string();
        if let Some(index) = self.open.iter().position(|(open, ..)| *open == holder) {
            dom.raise(&panel);
            // The latest raised is last, for quick moves (`bags_open`).
            let raised = self.open.remove(index);
            self.open.push(raised);
            return;
        }
        let Some(size) = shown_size(carried, holder) else {
            return;
        };
        let at = FIRST_AT + NEXT_AT * (self.opened % 8) as f32;
        self.opened += 1;
        let document = match holder {
            // Tabs for what they are like, what they wear and carry, and what
            // they can do, on the page last shown.
            Holder::Inventory => {
                self.page.show(dom);
                // (On a strip: its own node takes no room over the view.)
                let mut character = commands.spawn(document(&CHARACTER));
                character.insert(top_strip()).id()
            }
            Holder::Inside(bag) => {
                let name = carried.find(bag).map_or("", |item| item.kind.def().name);
                dom.set(&panel, "title", name);
                spawn_bag(commands, bag, grows(carried, holder))
            }
        };
        // (Made at the end of the frame, once the panel is built.)
        dom.start_at(&panel, at);
        self.open.push((holder, document, size));
    }

    /// The bags with panels open, the one opened or raised last at the end.
    pub(in crate::inventory) fn bags_open(&self) -> Vec<shared::id::Id> {
        let bag = |(holder, ..): &(Holder, Entity, (u8, u8))| match *holder {
            Holder::Inside(bag) => Some(bag),
            Holder::Inventory => None,
        };
        self.open.iter().filter_map(bag).collect()
    }

    /// Whether the inventory's panel is open.
    pub(super) fn inventory_open(&self) -> bool {
        self.open
            .iter()
            .any(|(open, ..)| *open == Holder::Inventory)
    }

    pub(super) fn toggle_inventory(
        &mut self,
        carried: &Inventory,
        commands: &mut Commands,
        dom: &mut Dom,
    ) {
        let open = self
            .open
            .iter()
            .position(|(open, ..)| *open == Holder::Inventory);
        match open {
            Some(index) => {
                // Its panel goes with its document.
                let (_, document, _) = self.open.remove(index);
                commands.entity(document).despawn();
            }
            None => self.open(Holder::Inventory, carried, commands, dom),
        }
    }
}
