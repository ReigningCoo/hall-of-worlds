use super::*;
use crate::dungeon::Cell;
use crate::furniture::Holder;
use crate::inn::{CORNER_TIER, GROUND, OPEN_TIER, RoomKind, level, rooms};

#[test]
fn a_room_a_tier_each_larger() {
    let own = own_rooms();
    let tiers: Vec<u8> = own.iter().map(|room| room.tier).collect();
    assert_eq!(tiers, (1..=CORNER_TIER).collect::<Vec<_>>());
    let area = |room: &OwnRoom| room.rect.width * room.rect.height;
    assert!(own.windows(2).all(|two| area(&two[0]) < area(&two[1])));
    // They are the layout's own rooms, with the same doors.
    let marked: Vec<_> = (rooms(UPSTAIRS).into_iter())
        .filter_map(|room| match room.kind {
            RoomKind::OwnRoom { tier } => Some((tier, room.rects[0], room.door)),
            _ => None,
        })
        .collect();
    let listed: Vec<_> = own.iter().map(|r| (r.tier, r.rect, Some(r.door))).collect();
    assert_eq!(marked, listed);
}

#[test]
fn only_the_open_tier_is_unlocked() {
    let up = level(UPSTAIRS).unwrap().level;
    for room in own_rooms() {
        let (x, y) = room.door;
        assert_eq!(up.cell(x, y), Cell::Door);
        assert_eq!(up.is_locked(x, y), room.tier > OPEN_TIER, "{room:?}");
    }
    for room in rooms(UPSTAIRS) {
        if let RoomKind::Bedroom { .. } = room.kind {
            let (x, y) = room.door.unwrap();
            assert!(up.is_locked(x, y), "{room:?}");
        }
    }
}

#[test]
fn cells_tell_their_room() {
    for room in own_rooms() {
        let (x, y) = (room.rect.x, room.rect.y);
        assert_eq!(own_room_at(UPSTAIRS, (x, y)), Some(room.tier));
        assert_eq!(own_room_at(GROUND, (x, y)), None);
        assert_eq!(own_room_at(UPSTAIRS, room.door), None);
    }
    assert_eq!(own_room_at(UPSTAIRS, crate::inn::spots::STAIRS), None);
}

#[test]
fn each_has_a_bed_chests_and_a_wardrobe_to_be_used() {
    let up = level(UPSTAIRS).unwrap().level;
    for room in own_rooms() {
        let pieces: Vec<_> = (up.furniture())
            .filter(|piece| piece.cells().any(|(x, y)| room.rect.contains(x, y)))
            .collect();
        for piece in &pieces {
            assert!(
                piece.cells().all(|(x, y)| room.rect.contains(x, y)),
                "{piece:?}"
            );
            // Its front is open floor, so it can be reached.
            if Holder::of(piece.furniture).is_some() {
                let (dx, dy) = match piece.facing {
                    Side::North => (0, -1),
                    Side::East => (1, 0),
                    Side::South => (0, 1),
                    Side::West => (-1, 0),
                };
                for (x, y) in piece.cells() {
                    let front = (x + dx, y + dy);
                    let free = up.cell(front.0, front.1) == Cell::Floor
                        && up.furniture_at(front.0, front.1).is_none();
                    assert!(free || piece.cells().any(|cell| cell == front), "{piece:?}");
                }
            }
        }
        let count = |f: fn(&Furniture) -> bool| pieces.iter().filter(|p| f(&p.furniture)).count();
        assert_eq!(count(|f| matches!(f, Furniture::Bed { .. })), 1);
        assert_eq!(count(|f| matches!(f, Furniture::Wardrobe { .. })), 1);
        let chests = count(|f| matches!(f, Furniture::Chest { .. }));
        assert_eq!(chests, if room.tier > 2 { 2 } else { 1 });
        // Nothing in front of the door.
        let (x, y) = room.door;
        let inside = [(0, -1), (1, 0), (0, 1), (-1, 0)]
            .map(|(dx, dy)| (x + dx, y + dy))
            .into_iter()
            .find(|&(x, y)| room.rect.contains(x, y))
            .unwrap();
        assert!(up.furniture_at(inside.0, inside.1).is_none());
    }
}
