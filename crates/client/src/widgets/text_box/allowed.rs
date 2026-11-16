//! What a box keeps of what is typed, pasted or set: line breaks only in a
//! box of several lines, digits only in a box for them, and no more than its
//! most.

use super::{TextBox, units};

/// What of `typed` a box keeps: line breaks only in a box of several lines
/// (`\r\n` and `\r` as `\n`), and only digits in a box for them.
pub(in crate::widgets) fn allowed(typed: &str, digits_only: bool, multiline: bool) -> String {
    let typed = typed.replace("\r\n", "\n").replace('\r', "\n");
    let keeps = |c: char| match c {
        '\n' => multiline && !digits_only,
        _ => !c.is_control() && (!digits_only || c.is_ascii_digit()),
    };
    typed.chars().filter(|&c| keeps(c)).collect()
}

impl TextBox {
    /// What of `typed` this box keeps, to the most it may hold.
    pub(in crate::widgets) fn keeps(&self, typed: &str) -> String {
        let kept = allowed(typed, self.digits_only, self.multiline);
        match self.max_length {
            Some(most) => units::units(&kept, self.accepts_items)
                .get(most)
                .map_or(kept.clone(), |unit| kept[..unit.start].to_owned()),
            None => kept,
        }
    }
}
