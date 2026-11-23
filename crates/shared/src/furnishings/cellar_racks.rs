//! The cellar's racks: each cellar cask (`shared::cellar`) onto a rack or
//! cradle of its size where one fits against its wall (a Tun onto its
//! stand, which also needs the next cell along the wall), and perhaps a wine
//! rack against another wall of the same room.

use super::Setting;
use super::furnisher::Furnisher;
use crate::bottle_rack::{BottleRack, BottleRackKind};
use crate::cask::CaskSize;
use crate::cask_rack::CaskRack;
use crate::cellar::CellarCask;
use crate::dungeon::RoomId;
use crate::furniture::Furniture;

/// Purpose numbers for the hashes.
const CASK: i64 = 1;
const WINE: i64 = 2;
const WINE_LENGTH: i64 = 3;

/// Puts racks under `casks`, and perhaps a wine rack in their room: the
/// cellar room, if there is one.
pub(super) fn place_racks(
    furnisher: &mut Furnisher,
    setting: &Setting,
    casks: &[CellarCask],
) -> Option<RoomId> {
    let first = casks.first()?;
    let room = furnisher.site.level.room(first.at.0, first.at.1);
    for cask in casks {
        rack_under(furnisher, cask);
    }
    let room = room?;
    if furnisher.roll(&[WINE]) % 100 < setting.wine_rack_percent {
        wine_rack(furnisher, room);
    }
    Some(room)
}

/// A rack, cradle or stand of `cask`'s size with the cask's cell in it, if
/// one fits: the cask then lies on it.
fn rack_under(furnisher: &mut Furnisher, cask: &CellarCask) {
    let (x, y) = cask.at;
    let rack = match cask.size {
        CaskSize::Tun => CaskRack {
            size: cask.size,
            length: 2,
        },
        size => CaskRack { size, length: 1 },
    };
    // A stand along the wall from the cask's cell, or ending on it.
    let anchors = match rack.length {
        1 => vec![(x, y)],
        _ => vec![(x, y), (x - 1, y), (x, y - 1)],
    };
    let free = |cell: (i64, i64)| cell == cask.at || furnisher.is_free(cell);
    let spots = furnisher.spots(&anchors, (rack.length, 1), free, CASK);
    let covers = |spot: &super::wall::WallSpot| {
        let (w, h) = (i64::from(spot.size.0), i64::from(spot.size.1));
        (spot.at.0..spot.at.0 + w).contains(&x) && (spot.at.1..spot.at.1 + h).contains(&y)
    };
    let spots = spots.into_iter().filter(covers).collect();
    let wood = furnisher.wood(&[x, y, CASK]);
    furnisher.place_first(spots, Furniture::CaskRack { rack }, wood);
}

/// A wine rack 3 to 1 long against a wall of `room`, the longest that fits.
fn wine_rack(furnisher: &mut Furnisher, room: RoomId) {
    let cells = furnisher.cells_of(room);
    let most = 1 + (furnisher.roll(&[WINE_LENGTH]) % 3) as u8;
    for length in (1..=most).rev() {
        let free = |cell| furnisher.is_free(cell);
        let spots = furnisher.spots(&cells, (length, 1), free, WINE);
        let rack = BottleRack {
            kind: BottleRackKind::Wine,
            length,
        };
        let wood = furnisher.wood(&[WINE]);
        if (furnisher.place_first(spots, Furniture::BottleRack { rack }, wood)).is_some() {
            return;
        }
    }
}
