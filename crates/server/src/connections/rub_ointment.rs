//! Rubbing ointment on oneself (through `consume.rs`, brewing decision 8): a
//! quarter of each dose (`brewing::skin_effect`), attributes for as long as
//! its quality says, Health and Mana given or taken at once.

use shared::brewing::skin_effect;
use shared::item::Item;
use shared::vitals::{Points, Vital};

use super::Client;

/// Rubs on `used`, one ointment already taken from where it was, at `now`.
pub(super) fn rub_ointment(client: &mut Client, used: &Item, now: u64) {
    let Some(form) = used.ointment() else {
        return;
    };
    let applied = skin_effect(&form.contents, now);
    let limits = client.limits(now);
    let played = client.activity.played;
    let (sheet, situation) = client.sheet_and_situation();
    if let Some(effect) = applied.effect {
        sheet.add_effect(effect, situation, now);
    }
    for (vital, amount) in applied.vitals {
        let size = Points(amount.unsigned_abs());
        match (vital, amount > 0) {
            (_, true) => sheet.vitals.restore(vital, size, limits, now),
            (Vital::Health, false) => sheet.vitals.hurt(size, limits, now, played),
            (Vital::Mana, false) => {
                sheet.vitals.spend_mana(size, limits, now, played);
            }
        }
    }
    client.unsaved = true;
    client.refresh_attributes(now);
}
