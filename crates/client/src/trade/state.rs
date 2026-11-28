//! What the trade window keeps while open: its document, who is near enough
//! to trade with, the other side's things and the grids, the receive box, and
//! what it has to say. The trade itself is `common::Trading`, which the
//! inventory reads.

use bevy::prelude::*;
use shared::id::Id;
use shared::item::Item;
use shared::protocol::Keeper;
use shared::trade::BUYING_ROWS;
use std::collections::BTreeMap;

use super::markup::{OWN_COLUMNS, STOCK_COLUMNS};
use crate::common::{buying_box, own_side, receive_box};
use crate::widgets::{ItemMark, StockGrid};

/// Someone near enough to trade with, as the dropdown lists them: a shop's
/// keeper, a player or an NPC.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct Partner {
    pub(super) with: Id,
    pub(super) name: String,
    /// A player set never to trade: greyed out, not to be chosen.
    pub(super) never: bool,
}

/// The window.
#[derive(Resource)]
pub(super) struct TradeWindow {
    /// Its document, while it is open.
    pub(super) document: Option<Entity>,
    /// It was opened this frame or since: choose the nearest shop.
    pub(super) just_opened: bool,
    /// Who keeps each shop on this level, by building.
    pub(super) keepers: BTreeMap<u16, Keeper>,
    /// Who the dropdown lists: players, shops, NPCs, each nearest first.
    pub(super) shops: Vec<Partner>,
    /// The shop's whole stock, as last heard, things being bought included;
    /// or what a character offers.
    pub(super) stock: Vec<Item>,
    /// The shop's side (its stock but what is being bought) and the own side
    /// (what is being sold), with the grids they are drawn in once built.
    pub(super) theirs: Option<StockGrid>,
    pub(super) own: StockGrid,
    pub(super) their_holder: Option<Entity>,
    pub(super) own_holder: Option<Entity>,
    /// A shop trade's Buying box, where the server says things lie in it.
    pub(super) buying: StockGrid,
    pub(super) buying_holder: Option<Entity>,
    /// A character's side (seen, not touched), drawn where they put things.
    pub(super) between_holder: Option<Entity>,
    /// The receive box: what a settled trade gave, until taken out.
    pub(super) received: Vec<Item>,
    pub(super) receive: StockGrid,
    pub(super) receive_holder: Option<Entity>,
    /// Who asks to trade, and their name, while the requester is up.
    pub(super) asked: Option<(Id, String)>,
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
            buying: buying_grid(),
            buying_holder: None,
            between_holder: None,
            received: Vec::new(),
            receive: receive_grid(),
            receive_holder: None,
            asked: None,
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
        self.buying = buying_grid();
        self.dirty = true;
    }

    /// Whether the receive box shows in place of the trade: something is in it.
    pub(super) fn receiving(&self) -> bool {
        !self.received.is_empty()
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
    marked(StockGrid::new(own_side(), OWN_COLUMNS, &[]).exactly(OWN_COLUMNS))
}

/// The Buying box: 8 wide, 4 rows until the server says it has grown.
fn buying_grid() -> StockGrid {
    marked(StockGrid::new(buying_box(), OWN_COLUMNS, &[]).exactly(BUYING_ROWS))
}

/// Everything on a side of the trade shows the bag (trading tweaks PR 3).
fn marked(mut grid: StockGrid) -> StockGrid {
    grid.mark = ItemMark::Selling;
    grid
}

/// The receive box: 8 × 8 however little is in it.
fn receive_grid() -> StockGrid {
    StockGrid::new(receive_box(), OWN_COLUMNS, &[]).at_least(OWN_COLUMNS)
}
