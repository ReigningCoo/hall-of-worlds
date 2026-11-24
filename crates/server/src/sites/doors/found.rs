//! Identify finding secret doors (magic plan PR 13): every secret door within
//! `count` cells of where it lands is known to its caster from then on, as
//! one seen open is (`remember_doors`), and its cells are sent again.

use shared::dungeon::Cell;

use super::super::update_sight;
use super::{changed_at, remember_doors_found};
use crate::connections::Client;

/// The caster finds the secret doors within `count` cells (at most 5) of
/// `cells` on their level. Whether they found any they didn't know.
pub(crate) fn find_secret_doors(client: &mut Client, cells: &[(i64, i64)], count: u32) -> bool {
    let Some(mut visit) = client.visit.take() else {
        return false;
    };
    let reach = i64::from(count.clamp(1, 5));
    let mut found = Vec::new();
    for &(cx, cy) in cells {
        for y in cy - reach..=cy + reach {
            for x in cx - reach..=cx + reach {
                let level = &visit.level.level;
                let secret = level.contains(x, y) && level.cell(x, y) == Cell::SecretDoor;
                if secret && visit.doors.contains_key(&(x, y)) && !found.contains(&(x, y)) {
                    found.push((x, y));
                }
            }
        }
    }
    let mut learnt = false;
    for &(x, y) in &found {
        if remember_doors_found(client, &visit, (x, y)) {
            changed_at(&mut visit, x, y);
            learnt = true;
        }
    }
    client.visit = Some(visit);
    if learnt {
        // Gone: noticed when its messages are next read.
        let _ = update_sight(client);
    }
    learnt
}
