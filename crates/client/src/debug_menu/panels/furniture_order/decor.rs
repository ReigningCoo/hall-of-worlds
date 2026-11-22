//! What Place Furniture puts down beyond single pieces: a booth's three
//! pieces, and decor (`shared::decor`). Something to hang goes on a wall's
//! face: clicking a wall hangs it on the face turned the way chosen (or any
//! with floor in front), clicking floor hangs it on the wall behind, facing the
//! way chosen (or any wall beside it). Clicking again goes through its
//! variations: every hanging, every chandelier, every rug pattern (a rug's
//! colour rolled, its length running the way chosen).

use shared::decor::{ChandelierStyle, Decor, Hanging, Rug, RugColour, RugPattern};
use shared::door::Side;
use shared::dungeon::{Cell, Level};
use shared::furniture::{Material, Piece, Wood, booth, takes_furniture};

use super::{FurnitureOrder, Kind};

/// What a click puts down.
#[derive(Clone, Debug, PartialEq)]
pub(in crate::debug_menu) enum Order {
    Pieces(Vec<Piece>),
    Decor(Decor),
}

const SIDES: [Side; 4] = [Side::North, Side::East, Side::South, Side::West];

impl FurnitureOrder {
    /// What a click on `cell` of `level` puts down, `roll` choosing what is
    /// random; `None` for something to hang with no wall there.
    pub(in crate::debug_menu) fn order(
        &mut self,
        cell: (i64, i64),
        roll: u64,
        level: &Level,
    ) -> Option<Order> {
        let sizes = self.kind.sizes();
        let (long, short) = sizes[self.size.min(sizes.len() - 1)];
        let across = matches!(self.facing, Side::North | Side::South);
        Some(match self.kind {
            Kind::Booth => {
                self.vary(cell, 1);
                let wood = match self.material {
                    Some(Material::Wood(wood)) => wood,
                    _ => rolled_wood(roll),
                };
                Order::Pieces(booth(cell, long, short, across, wood).to_vec())
            }
            Kind::WallDecoration => {
                let (wall, face) = wall_face(level, cell, self.facing)?;
                let hanging = Hanging::ALL[self.vary(cell, Hanging::ALL.len())];
                Order::Decor(Decor::Wall {
                    wall,
                    face,
                    hanging,
                })
            }
            Kind::Chandelier => {
                let style = ChandelierStyle::ALL[self.vary(cell, ChandelierStyle::ALL.len())];
                Order::Decor(Decor::Chandelier { at: cell, style })
            }
            Kind::Rug => {
                let pattern = RugPattern::ALL[self.vary(cell, RugPattern::ALL.len())];
                let colour = RugColour::ALL[(roll % RugColour::ALL.len() as u64) as usize];
                Order::Decor(Decor::Rug {
                    at: cell,
                    // Its length runs the way chosen.
                    size: if across { (short, long) } else { (long, short) },
                    rug: Rug { pattern, colour },
                })
            }
            _ => Order::Pieces(vec![self.click(cell, roll)]),
        })
    }
}

/// A wood rolled from `roll`.
fn rolled_wood(roll: u64) -> Wood {
    let woods: Vec<Wood> = (Material::ALL.into_iter())
        .filter_map(|material| match material {
            Material::Wood(wood) => Some(wood),
            Material::Stone(_) => None,
        })
        .collect();
    woods[(roll % woods.len() as u64) as usize]
}

/// The wall and face something clicked at `cell` hangs on, facing `facing`
/// where it can.
fn wall_face(level: &Level, cell: (i64, i64), facing: Side) -> Option<((i64, i64), Side)> {
    let at = |(x, y): (i64, i64)| level.contains(x, y).then(|| level.cell(x, y));
    let floor = |c: (i64, i64)| at(c).is_some_and(takes_furniture);
    let wall = |c: (i64, i64)| at(c) == Some(Cell::Wall);
    if wall(cell) {
        let faces = std::iter::once(facing).chain(SIDES);
        return faces
            .into_iter()
            .find(|side| floor(side.beside(cell)))
            .map(|side| (cell, side));
    }
    let behind = std::iter::once(facing.opposite()).chain(SIDES);
    let side = behind.into_iter().find(|side| wall(side.beside(cell)))?;
    Some((side.beside(cell), side.opposite()))
}
