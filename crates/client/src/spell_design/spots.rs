//! What of the panel is at a point (`Spot`), found by `DesignSpots` from the
//! node in front: a piece along the top, a step, a where, effect or filter in
//! a step, or the end box.

use bevy::prelude::*;

use super::nodes::FilterNode;
use super::piece::{FilterKind, Piece};
use super::rows::PieceNode;
use super::steps::{EndNode, EntryNode, StepNode};
use crate::dom::Finder;

/// What is at a point of the panel.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Spot {
    Piece(Piece),
    Step(usize),
    Entry(usize, usize),
    Filter(FilterKind),
    End,
}

/// Finds what of the panel is at a point, or a node is in.
#[derive(bevy::ecs::system::SystemParam)]
pub(super) struct DesignSpots<'w, 's> {
    finder: Finder<'w, 's>,
    parents: Query<'w, 's, &'static ChildOf>,
    pieces: Query<'w, 's, &'static PieceNode>,
    steps: Query<'w, 's, &'static StepNode>,
    entries: Query<'w, 's, &'static EntryNode>,
    filters: Query<'w, 's, &'static FilterNode>,
    ends: Query<'w, 's, (), With<EndNode>>,
}

impl DesignSpots<'_, '_> {
    pub(super) fn at(&self, at: Vec2) -> Option<Spot> {
        self.of(self.finder.target_at(at)?)
    }

    /// The innermost part of the panel a node is, or is in.
    pub(super) fn of(&self, front: Entity) -> Option<Spot> {
        let up = std::iter::once(front).chain(self.parents.iter_ancestors(front));
        up.into_iter().find_map(|entity| {
            if let Ok(node) = self.pieces.get(entity) {
                Some(Spot::Piece(node.0))
            } else if let Ok(node) = self.filters.get(entity) {
                Some(Spot::Filter(node.0))
            } else if let Ok(node) = self.entries.get(entity) {
                Some(Spot::Entry(node.0, node.1))
            } else if let Ok(node) = self.steps.get(entity) {
                Some(Spot::Step(node.0))
            } else {
                self.ends.get(entity).ok().map(|()| Spot::End)
            }
        })
    }
}

impl Spot {
    /// The step it is in, if any.
    pub(super) fn step(self) -> Option<usize> {
        match self {
            Self::Step(at) | Self::Entry(at, _) => Some(at),
            Self::Filter(_) => Some(0),
            Self::Piece(_) | Self::End => None,
        }
    }
}
