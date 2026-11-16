//! Other floors of a building (owner's design, 2026-11-16): the floors below,
//! each lowered by whole storeys and darker the further down, and the floors
//! above, raised, with a wedge towards the camera left out; drawn from what the
//! client has of them: what is seen of them now (`shared::floor_sight`), and
//! their remembered slices in grey. See `docs/context/client/dungeon_map.md`.

mod aim;
mod draw;
mod floors;
mod look;
mod one;
mod which;

use super::sight_lines::SurfaceMaterial;
pub(super) use aim::aim_wedge;
pub(super) use draw::draw_others;
pub(super) use floors::Floors;
pub(super) use look::follow_materials;
pub(super) use which::has_floors_below;
