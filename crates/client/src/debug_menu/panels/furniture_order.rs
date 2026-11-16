//! What Place Furniture puts down (no Bevy systems, so it can be tested
//! alone): the panel's choices, and the piece a click makes. Clicking the same
//! cell again with the same choices gives the next variation (every style of
//! the kind that fits, chairs without and then with arms), wrapping round; a
//! click elsewhere starts a new piece in the variation the last one ended on.

use bevy::prelude::*;
use shared::door::Side;
use shared::furniture::{
    BenchStyle, ChairStyle, Furniture, Material, Piece, TableStyle, footprint,
};

/// Which kind of piece.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Kind {
    Table,
    Chair,
    Bench,
}

impl Kind {
    pub(super) const ALL: [Self; 3] = [Self::Table, Self::Chair, Self::Bench];

    pub(super) fn name(self) -> &'static str {
        match self {
            Self::Table => "Table",
            Self::Chair => "Chair",
            Self::Bench => "Bench",
        }
    }

    /// The sizes it comes in, long side by short.
    pub(super) fn sizes(self) -> &'static [(u8, u8)] {
        match self {
            Self::Table => &[(1, 1), (2, 1), (3, 1), (2, 2), (3, 2)],
            Self::Chair => &[(1, 1)],
            Self::Bench => &[(1, 1), (2, 1), (3, 1)],
        }
    }

    /// A size as the dropdown says it: short side first, as "2 × 3".
    pub(super) fn size_name(self, (long, short): (u8, u8)) -> String {
        match self {
            Self::Bench => long.to_string(),
            _ => format!("{short} × {long}"),
        }
    }

    /// Every variation of a piece of this size: each style that fits, of the
    /// material's kind (wood or stone; either for `None`, Random).
    fn variations(self, size: (u8, u8), material: Option<Material>) -> Vec<Furniture> {
        let all: Vec<Furniture> = match self {
            Self::Table => (TableStyle::ALL.into_iter())
                .filter(|style| style.fits(size))
                .map(|style| Furniture::Table { style })
                .collect(),
            Self::Chair => (ChairStyle::ALL.into_iter())
                .flat_map(|style| [false, true].map(|arms| Furniture::Chair { style, arms }))
                .filter(|&chair| !(chair_arms(chair) && chair.is_stone()))
                .collect(),
            Self::Bench => (BenchStyle::ALL.into_iter())
                .map(|style| Furniture::Bench { style })
                .collect(),
        };
        let fits = |piece: &Furniture| material.is_none_or(|m| m.is_stone() == piece.is_stone());
        all.into_iter().filter(fits).collect()
    }
}

fn chair_arms(furniture: Furniture) -> bool {
    matches!(furniture, Furniture::Chair { arms: true, .. })
}

/// The panel's choices: kind, size, facing and material.
type Choices = (Kind, usize, Side, Option<Material>);

/// What the panel has chosen, and the last piece put down.
#[derive(Resource, Clone, Debug, PartialEq)]
pub(super) struct FurnitureOrder {
    pub(super) kind: Kind,
    /// Its place in `kind.sizes()`.
    pub(super) size: usize,
    pub(super) facing: Side,
    /// `None` for Random: rolled each click, of the style's kind.
    pub(super) material: Option<Material>,
    /// The cell last clicked and the choices then, to know a click again.
    last: Option<((i64, i64), Choices)>,
    /// Which variation the last piece was.
    variation: usize,
}

impl Default for FurnitureOrder {
    fn default() -> Self {
        Self {
            kind: Kind::Table,
            size: 4,
            facing: Side::East,
            material: None,
            last: None,
            variation: 0,
        }
    }
}

impl FurnitureOrder {
    /// The piece a click on `cell` puts down (its north-west cell), `roll`
    /// choosing a Random material.
    pub(super) fn click(&mut self, cell: (i64, i64), roll: u64) -> Piece {
        let sizes = self.kind.sizes();
        let (long, short) = sizes[self.size.min(sizes.len() - 1)];
        let choices = (self.kind, self.size, self.facing, self.material);
        let again = self.last == Some((cell, choices));
        let variations = self.kind.variations((long, short), self.material);
        self.variation = (self.variation + usize::from(again)) % variations.len();
        self.last = Some((cell, choices));
        let furniture = variations[self.variation];
        let facing = match self.kind {
            // A table runs east–west or north–south, east when square.
            Kind::Table if long > short && matches!(self.facing, Side::North | Side::South) => {
                Side::South
            }
            Kind::Table => Side::East,
            Kind::Chair | Kind::Bench => self.facing,
        };
        let material = self.material.unwrap_or_else(|| {
            let of_kind = Material::ALL.into_iter();
            let of_kind: Vec<Material> = of_kind
                .filter(|m| m.is_stone() == furniture.is_stone())
                .collect();
            of_kind[(roll % of_kind.len() as u64) as usize]
        });
        Piece {
            at: cell,
            size: footprint(furniture, (long, short), facing),
            facing,
            material,
            furniture,
        }
    }
}

#[cfg(test)]
mod tests;
