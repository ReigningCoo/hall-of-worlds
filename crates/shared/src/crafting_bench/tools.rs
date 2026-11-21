//! What lies on a bench: one place per kind of tool its craft takes, a
//! Range's storage space, and a hearth's fire.

use serde::{Deserialize, Serialize};

use super::Craft;
use crate::campfire::Campfire;
use crate::id::Id;
use crate::item::{Item, ItemKind};

/// How many things a Range's storage space holds (each a stack as put),
/// Claude's first guess.
pub const STORAGE_PLACES: usize = 8;

/// A bench's tools, the server's to keep: at most one of each kind its craft
/// takes, in the craft's order; for a Range, food left in its storage; for a
/// hearth, whether it is always lit and its fire of kindling otherwise.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct BenchTools {
    pub craft: Craft,
    tools: Vec<Item>,
    /// A Range's storage, in the order put (`STORAGE_PLACES` at most).
    #[serde(default)]
    storage: Vec<Item>,
    /// A hearth lit by the household for good (towers', castles'), needing
    /// no kindling (`hearth::always_lit`).
    #[serde(default)]
    pub always_lit: bool,
    /// A hearth's fire of kindling (`campfire::Campfire`'s burning; no Ash).
    #[serde(default)]
    pub fire: Option<Campfire>,
}

/// Why a bench won't take an item. Only ever this one answer, so trying
/// things never tells a player which kinds a craft takes (owner's).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NotTaken;

impl BenchTools {
    /// An empty bench for `craft`, unlit.
    pub fn new(craft: Craft) -> Self {
        Self {
            craft,
            tools: Vec::new(),
            storage: Vec::new(),
            always_lit: false,
            fire: None,
        }
    }

    /// Its tools, in the craft's order.
    pub fn tools(&self) -> &[Item] {
        &self.tools
    }

    /// What lies in its storage space, in the order put.
    pub fn storage(&self) -> &[Item] {
        &self.storage
    }

    /// Its storage, for food's clocks to run (`food`'s
    /// `settle_lying_food`).
    pub fn storage_mut(&mut self) -> &mut [Item] {
        &mut self.storage
    }

    /// Its tools, then its storage: everything on it.
    pub fn items(&self) -> impl Iterator<Item = &Item> {
        self.tools.iter().chain(&self.storage)
    }

    /// The tool of `kind` on it, if any.
    pub fn tool(&self, kind: ItemKind) -> Option<&Item> {
        self.tools.iter().find(|tool| tool.kind == kind)
    }

    /// Whether it holds a tool of `kind`.
    pub fn has(&self, kind: ItemKind) -> bool {
        self.tool(kind).is_some()
    }

    /// Whether it gives a hearth's heat at `now` (server seconds): a hearth
    /// always lit, or one whose fire still burns.
    pub fn is_lit(&self, now: u64) -> bool {
        self.craft.is_hearth()
            && (self.always_lit || self.fire.as_ref().is_some_and(|fire| fire.is_lit(now)))
    }

    /// Puts `item` in its kind's place, giving back the tool that was there
    /// (a swap); food (a Dish or intermediate) goes in a Range's storage
    /// while there is room. Anything else is refused: a kind the craft
    /// doesn't take, more than one tool at once, or a full storage.
    pub fn put(&mut self, item: Item) -> Result<Option<Item>, NotTaken> {
        let order = self.craft.tools();
        let Some(place) = order.iter().position(|&kind| kind == item.kind) else {
            return self.store(item).map(|()| None);
        };
        if item.count() != 1 {
            return Err(NotTaken);
        }
        let old = self.tools.iter().position(|tool| tool.kind == item.kind);
        let old = old.map(|at| self.tools.remove(at));
        let rank = |tool: &Item| order.iter().position(|&kind| kind == tool.kind);
        let at = self
            .tools
            .iter()
            .take_while(|tool| rank(tool) < Some(place));
        let at = at.count();
        self.tools.insert(at, item);
        Ok(old)
    }

    /// Puts food into the storage space, if it has one with room.
    fn store(&mut self, item: Item) -> Result<(), NotTaken> {
        let food = matches!(item.kind, ItemKind::Dish | ItemKind::Intermediate);
        if !food || !self.craft.has_storage() || self.storage.len() >= STORAGE_PLACES {
            return Err(NotTaken);
        }
        self.storage.push(item);
        Ok(())
    }

    /// Takes the tool or stored thing `id` off, if it is on it.
    pub fn take(&mut self, id: Id) -> Option<Item> {
        if let Some(at) = self.tools.iter().position(|tool| tool.id == id) {
            return Some(self.tools.remove(at));
        }
        let at = self.storage.iter().position(|item| item.id == id)?;
        Some(self.storage.remove(at))
    }
}
