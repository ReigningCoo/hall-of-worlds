//! What the fuel panel says.

use shared::lantern::{FOUNT_ML, LanternForm};
use shared::pour::BOTTLE_ML;

/// How much oil is in it.
pub(super) fn oil(fount: LanternForm) -> String {
    match fount.ml {
        0 => format!("Empty: it holds {FOUNT_ML} ml of oil."),
        ml => format!("{ml} ml of oil, of {FOUNT_ML} ml."),
    }
}

/// How long it burns for, by playtime, held in a hand or tied to the belt.
pub(super) fn left(fount: LanternForm) -> String {
    if !fount.has_oil() {
        return "Without oil it gives no light.".to_owned();
    }
    let minutes = fount.seconds_left().div_ceil(60);
    let time = match (minutes / 60, minutes % 60) {
        (0, m) => format!("{m} min"),
        (h, 0) => format!("{h} h"),
        (h, m) => format!("{h} h {m} min"),
    };
    format!("Burns for {time} more, held in a hand or tied to the belt.")
}

/// Beside the space to drop a bottle on.
pub(super) fn fill(fount: LanternForm) -> String {
    match fount.room() {
        0 => "Full.".to_owned(),
        room if room < BOTTLE_ML => {
            format!("Drop a bottle of oil here to fill it: {room} ml goes in, the rest is spilt.")
        }
        _ => "Drop a bottle of oil here to fill it.".to_owned(),
    }
}
