//! The size a worn thing was made in, when it isn't the viewer's own: "Size:
//! Large (you: Medium)". Clothes, armour, footwear and rings come in sizes
//! (`ItemKind::comes_in_sizes` with a `wear` def); loose blades and gems are
//! made in sizes too, but aren't worn, so they say nothing.

use shared::creature::Size;
use shared::item::Item;
use shared::lang::{Args, Lang};

use crate::common::written;

/// The line about the size `item` was made in against the viewer's `yours`,
/// or nothing when it is theirs or the kind isn't worn in sizes. A stack's is
/// its top's.
pub(super) fn size_line(item: &Item, yours: Size, lang: &Lang) -> Option<String> {
    let item = item.top();
    let sized = item.kind.comes_in_sizes() && item.kind.def().wear.is_some();
    if !sized || item.made.size == yours {
        return None;
    }
    let args = Args::new()
        .text("size", size_words(item.made.size, lang))
        .text("yours", size_words(yours, lang));
    Some(written(lang.render("item-size", &args)))
}

/// "Large", through `item.ftl`.
fn size_words(size: Size, lang: &Lang) -> String {
    let args = Args::new().text("size", size.name().to_lowercase());
    written(lang.render("item-size-name", &args))
}

#[cfg(test)]
mod tests;
