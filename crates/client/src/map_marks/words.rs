//! The popups' words, through `map-mark.ftl` (a mark's own name is `common::mark_name`).

use shared::lang::Args;
use shared::map_mark::MOST_HAND_MARKS;

use crate::common::{Wording, written};

/// What the placing popup says instead of its icons when no more may be placed by hand.
pub(super) fn too_many(wording: &Wording) -> String {
    let args = Args::new().count("most", MOST_HAND_MARKS as i64);
    written(wording.0.render("map-mark-too-many", &args))
}
