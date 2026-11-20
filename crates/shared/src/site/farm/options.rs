//! What a farm poi asks for, and how a field looks.

use serde::{Deserialize, Serialize};

use super::super::building::BuildingOptions;
use super::super::pen::PenLook;
use super::super::town::Rect;

/// Options for the farm generator.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FarmOptions {
    pub house: BuildingOptions,
    /// How many of the places round the house are used, an inclusive range.
    pub fields: (i64, i64),
}

/// What is needed to choose what grows in a field. Which stage it is at is
/// the client's choice: it is only looks.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct FieldLook {
    /// The cells sown, inside the fence.
    pub rect: Rect,
    /// Grazed rather than cropped.
    pub livestock: bool,
    /// A number for looks, so fields differ.
    pub variety: u32,
    /// Fruit plants grow in rows here (a server puts them in, by the place's
    /// warmth: `fruit::plant_orchards`), on grass rather than crops.
    #[serde(default)]
    pub orchard: bool,
    /// A pen for kept animals (a sty or a yard: `site::pen`), grazed bare
    /// rather than grassed; `None` for a field.
    #[serde(default)]
    pub pen: Option<PenLook>,
}

impl FieldLook {
    /// The same field `by` cells along (set into the wilderness), its pen's
    /// parts too.
    pub fn moved(&self, by: (i64, i64)) -> Self {
        let rect = |r: Rect| Rect {
            x: r.x + by.0,
            y: r.y + by.1,
            ..r
        };
        let pen = self.pen.map(|pen| PenLook {
            shelter: rect(pen.shelter),
            dovecote: pen.dovecote.map(|(x, y)| (x + by.0, y + by.1)),
            ..pen
        });
        Self {
            rect: rect(self.rect),
            pen,
            ..*self
        }
    }

    /// Whether a cell is in the field.
    pub fn contains(&self, x: i64, y: i64) -> bool {
        let Rect {
            x: left,
            y: top,
            width,
            height,
        } = self.rect;
        (left..left + width).contains(&x) && (top..top + height).contains(&y)
    }
}
