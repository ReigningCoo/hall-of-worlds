//! Reading an `Item`: the same fields, then any counted kind's old stack (from
//! before counted stacks, 2026-11-12: one item with an id for each coin) folded
//! into its count, and a form for a necklace saved before necklaces had one. Works
//! the same for messages, which never have such stacks or necklaces.

use serde::Deserialize;

use crate::id::Id;
use crate::item::item::one;
use crate::item::{Favour, Item, ItemKind, Known, Made, MadeAt, Placed, Quality};
use crate::item_form::{ItemForm, RollRequest, give_form_to};
use crate::quest::QuestId;

/// An `Item` as written: see `Item` for each field.
#[derive(Deserialize)]
pub(super) struct Saved {
    id: Id,
    kind: ItemKind,
    contents: Vec<Placed>,
    made: Made,
    #[serde(default)]
    quality: Option<Quality>,
    #[serde(default)]
    made_at: Option<MadeAt>,
    #[serde(default)]
    name: Option<String>,
    #[serde(default)]
    set: bool,
    #[serde(default)]
    form: Option<ItemForm>,
    #[serde(default)]
    favour: Option<Favour>,
    #[serde(default)]
    known: Known,
    #[serde(default)]
    above: Vec<Item>,
    #[serde(default = "one")]
    count: u32,
    #[serde(default)]
    quest: Option<QuestId>,
}

impl From<Saved> for Item {
    fn from(saved: Saved) -> Self {
        let mut item = Item {
            id: saved.id,
            kind: saved.kind,
            contents: saved.contents,
            made: saved.made,
            quality: saved.quality,
            made_at: saved.made_at,
            name: saved.name,
            set: saved.set,
            form: saved.form,
            favour: saved.favour,
            known: saved.known,
            above: saved.above,
            count: saved.count,
            quest: saved.quest,
        };
        if item.kind.def().counted {
            // The bottom's id and `made_at` stay the stack's.
            let above: u32 = item.above.drain(..).map(|one| one.count).sum();
            item.count = item.count.max(1) + above;
        } else {
            item.count = 1;
        }
        if item.kind == ItemKind::Necklace && item.form.is_none() {
            give_old_necklace_form(&mut item);
        }
        item
    }
}

/// A form for a necklace saved before they had one (2026-11-15), rolled from its id
/// as a new one would be, but without settings: gems are items, and only the
/// server makes ids, so a stone can't be made while reading.
fn give_old_necklace_form(item: &mut Item) {
    let request = RollRequest {
        settings: Some(0),
        ..RollRequest::NONE
    };
    let size = Some(item.made.size);
    let seed = item.id.seed();
    let made = give_form_to(item, size, seed, &request, &mut || Id::NOBODY);
    made.expect("a necklace without settings can always be made");
}
