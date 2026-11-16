//! Other floors of a building, seen from an upper one (owner's design,
//! 2026-11-16): the floors below, each lowered by whole storeys and darker the
//! further down, drawn from what the client has of them: what is seen of them
//! now (`shared::floor_sight`), and their remembered slices in grey. See
//! `docs/context/client/dungeon_map.md`.

mod below;
mod draw;
mod look;
mod which;

use super::sight_lines::SurfaceMaterial;
pub(super) use below::Below;
pub(super) use draw::draw_below;
pub(super) use look::follow_materials;
pub(super) use which::has_floors_below;
