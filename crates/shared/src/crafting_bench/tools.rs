//! What lies on a bench: one place per kind of tool its craft takes.

use serde::{Deserialize, Serialize};

use super::Craft;
use crate::id::Id;
use crate::item::{Item, ItemKind};

/// A bench's tools, the server's to keep: at most one of each kind its craft
/// takes, in the craft's order.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct BenchTools {
    pub craft: Craft,
    tools: Vec<Item>,
}

/// Why a bench won't take an item. Only ever this one answer, so trying
/// things never tells a player which kinds a craft takes (owner's).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NotTaken;

impl BenchTools {
    /// An empty bench for `craft`.
    pub fn new(craft: Craft) -> Self {
        Self {
            craft,
            tools: Vec::new(),
        }
    }

    /// Its tools, in the craft's order.
    pub fn tools(&self) -> &[Item] {
        &self.tools
    }

    /// The tool of `kind` on it, if any.
    pub fn tool(&self, kind: ItemKind) -> Option<&Item> {
        self.tools.iter().find(|tool| tool.kind == kind)
    }

    /// Whether it holds a tool of `kind`.
    pub fn has(&self, kind: ItemKind) -> bool {
        self.tool(kind).is_some()
    }

    /// Puts `item` in its kind's place, giving back the tool that was there
    /// (a swap), or refuses it: a kind the craft doesn't take, or more than
    /// one at once.
    pub fn put(&mut self, item: Item) -> Result<Option<Item>, NotTaken> {
        let order = self.craft.tools();
        let Some(place) = order.iter().position(|&kind| kind == item.kind) else {
            return Err(NotTaken);
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

    /// Takes the tool `id` off, if it is on it.
    pub fn take(&mut self, id: Id) -> Option<Item> {
        let at = self.tools.iter().position(|tool| tool.id == id)?;
        Some(self.tools.remove(at))
    }
}
