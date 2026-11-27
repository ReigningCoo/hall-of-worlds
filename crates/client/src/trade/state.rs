//! What the trade window keeps while open: its document, who is near enough
//! to trade with, the shop's whole stock and the two grids, and what it has
//! to say. The trade itself is `common::Trading`, which the inventory reads.

use bevy::prelude::*;
use shared::id::Id;
use shared::item::Item;
use std::collections::BTreeMap;

use super::markup::{OWN_COLUMNS, STOCK_COLUMNS};
use crate::common::own_side;
use crate::widgets::StockGrid;

/// A shop near enough to trade with, as the dropdown lists it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct Shop {
    pub(super) keeper: Id,
    pub(super) name: String,
}

/// The window.
#[derive(Resource)]
pub(super) struct TradeWindow {
    /// Its document, while it is open.
    pub(super) document: Option<Entity>,
    /// It was opened this frame or since: choose the nearest shop.
    pub(super) just_opened: bool,
    /// Who keeps each shop on this level, by building.
    pub(super) keepers: BTreeMap<u16, Id>,
    /// The shops in the dropdown, nearest first.
    pub(super) shops: Vec<Shop>,
    /// The shop's whole stock, as last heard, things being bought included.
    pub(super) stock: Vec<Item>,
    /// The shop's side (its stock but what is being bought) and the own side
    /// (what is being sold), with the grids they are drawn in once built.
    pub(super) theirs: Option<StockGrid>,
    pub(super) own: StockGrid,
    pub(super) their_holder: Option<Entity>,
    pub(super) own_holder: Option<Entity>,
    /// What the window says under the sides: a refusal, or how a trade ended.
    pub(super) said: Option<String>,
    /// A refusal was just said: the state after it doesn't clear it.
    pub(super) just_refused: bool,
    /// The middle and bottom want writing again.
    pub(super) dirty: bool,
}

impl Default for TradeWindow {
    fn default() -> Self {
        Self {
            document: None,
            just_opened: false,
            keepers: BTreeMap::new(),
            shops: Vec::new(),
            stock: Vec::new(),
            theirs: None,
            own: own_grid(),
            their_holder: None,
            own_holder: None,
            said: None,
            just_refused: false,
            dirty: true,
        }
    }
}

impl TradeWindow {
    /// Forgets the trade's grids and stock (its document and the shops stay).
    pub(super) fn forget_trade(&mut self) {
        self.stock.clear();
        self.theirs = None;
        self.own = own_grid();
        self.dirty = true;
    }

    /// The shop's grid, for `with`'s stock: a new one for another shop.
    pub(super) fn their_grid(&mut self, with: Id) -> &mut StockGrid {
        if self.theirs.as_ref().is_none_or(|grid| grid.with != with) {
            self.theirs = Some(StockGrid::new(with, STOCK_COLUMNS, &[]));
        }
        self.theirs.as_mut().expect("just made")
    }
}

/// The own side: 8 × 8 however little is in it.
fn own_grid() -> StockGrid {
    StockGrid::new(own_side(), OWN_COLUMNS, &[]).at_least(OWN_COLUMNS)
}
