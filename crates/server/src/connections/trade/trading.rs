//! What a client has to do with trading: the shop trade or the trade with a
//! character it has open, what it asked of other characters this tick, and
//! (saved) its Trading setting and receive box.

use shared::id::Id;
use shared::protocol::TradeAsk;
use shared::trade::{Received, ShopTrade, Side, TradingSetting};

use crate::storage::TradeSave;

/// A client's trading.
#[derive(Debug, Default)]
pub(crate) struct Trading {
    /// The shop trade open, if any: not saved.
    pub(super) shop: Option<ShopTrade>,
    /// The trade with a character open, asked or being asked: not saved.
    pub(super) dealing: Option<Dealing>,
    /// What it asked this tick of trades with characters, done once every
    /// client's messages have been read (`deals.rs`), as only then can both
    /// sides be reached.
    pub(super) asked: Vec<DealAsk>,
    /// Saved with the character.
    pub(super) saved: TradeSave,
}

/// A trade with a character: who with, how far it has got, and this side.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct Dealing {
    /// The other player's entity id, or the NPC's.
    pub(super) with: Id,
    /// With an NPC, whose side is always empty and locked in.
    pub(super) npc: bool,
    pub(super) state: DealState,
    pub(super) side: Side,
}

/// How far a trade with a character has got.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum DealState {
    /// This side asked, and waits for the other to Allow or Deny.
    Asking,
    /// This side is being asked.
    Asked,
    /// Open on both sides.
    Open,
}

/// Something asked of a trade with a character, done once messages are read.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) enum DealAsk {
    /// Opening one with this player or NPC.
    Open(Id),
    /// A change to it, answering it, locking in or closing it.
    Change(TradeAsk),
}

impl Trading {
    /// Starting as saved.
    pub(crate) fn loaded(saved: TradeSave) -> Self {
        Self {
            saved,
            ..Self::default()
        }
    }

    /// What is saved of it.
    pub(crate) fn to_save(&self) -> TradeSave {
        self.saved.clone()
    }

    /// The character's Trading setting.
    pub(crate) fn setting(&self) -> TradingSetting {
        self.saved.setting
    }

    /// Their receive box.
    pub(super) fn received(&self) -> &Received {
        &self.saved.received
    }

    /// Whether a trade may start: nothing waits in the receive box.
    pub(super) fn may_start(&self) -> bool {
        self.saved.received.is_empty()
    }
}

impl Dealing {
    /// A new one with `with`, at `state`, nothing offered.
    pub(super) fn new(with: Id, npc: bool, state: DealState) -> Self {
        Self {
            with,
            npc,
            state,
            side: Side::default(),
        }
    }
}
