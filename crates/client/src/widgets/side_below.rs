//! A crafting panel's side (Start, the count, what is made) standing right
//! of its steps, or dropping below them once the steps grow wider than the
//! room left for them: the panel grows taller rather than the steps running
//! under the side (the Alchemy and Cooking panels).

use crate::dom::Dom;

/// The ids and widths of a panel's two halves.
pub struct SideBelow {
    /// The `span` holding both halves.
    pub body: &'static str,
    /// The steps' half, a `div`.
    pub steps: &'static str,
    /// The steps' half's usual width.
    pub steps_width: f32,
    /// The side's width.
    pub side_width: f32,
}

impl SideBelow {
    /// Whether steps `wide` need the side below them.
    pub fn below(&self, wide: f32) -> bool {
        wide > self.steps_width
    }

    /// Lays the halves out for steps `wide`: side by side as written, or
    /// the steps' half as wide as the steps and the `span` let wrap, at a
    /// width that leaves no room for the side beside them.
    pub fn place(&self, dom: &mut Dom, wide: f32) {
        if !self.below(wide) {
            dom.set(self.steps, "width", &whole(self.steps_width));
            dom.set_flag(self.body, "nowrap", true);
            dom.clear(self.body, "width");
            return;
        }
        dom.set(self.steps, "width", &whole(wide));
        dom.set_flag(self.body, "nowrap", false);
        let body = wide.max(self.steps_width + self.side_width);
        dom.set(self.body, "width", &whole(body));
    }
}

/// A width as markup writes it.
fn whole(width: f32) -> String {
    format!("{}", width.ceil() as u32)
}
