//! Logs as items: `ItemKind::Log` with a `LogForm` saying its wood.

use serde::{Deserialize, Serialize};

use crate::id::Id;
use crate::item::{Item, ItemKind, MadeAt, Shape};
use crate::item_form::ItemForm;
use crate::material::Material;

/// A log's size in cubic centimetres: 30 × 30 × 100 (owner).
pub const LOG_CM3: u32 = 30 * 30 * 100;

/// Which wood a log is.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct LogForm {
    pub wood: Material,
}

impl LogForm {
    /// What it is called: "Oak Log".
    pub fn name(&self) -> String {
        format!("{} Log", self.wood.name())
    }

    /// What it weighs: its size of the wood, seasoned (`Material::density`,
    /// tenths of a gram a cm³).
    pub fn grams(&self) -> u32 {
        LOG_CM3 * self.wood.density() / 10
    }

    /// The cells it takes up: 8 long, 3 across.
    pub fn shape(&self) -> Shape {
        Shape::rectangle(8, 3)
    }
}

/// A new log of `wood`, with id `id`.
pub fn new_log(wood: Material, id: Id, made_at: Option<MadeAt>) -> Item {
    let mut item = Item::new(id, ItemKind::Log);
    item.made_at = made_at;
    item.form = Some(ItemForm::Log(LogForm { wood }));
    item
}

impl Item {
    /// Which log it is, if it is one.
    pub fn log(&self) -> Option<&LogForm> {
        match &self.form {
            Some(ItemForm::Log(log)) => Some(log),
            _ => None,
        }
    }
}
