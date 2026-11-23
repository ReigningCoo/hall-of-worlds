//! `Places`: where the steps' row and its steps are on screen, to tell the
//! place in the row something let go at a point would go.

use bevy::prelude::*;
use bevy::ui::UiGlobalTransform;

use super::rows::StepsRow;
use super::steps::{EndNode, StepNode};
use crate::widgets::logical_rect;

/// How far outside the row a drop still counts as on it.
const MARGIN: f32 = 12.0;

/// Where the steps' row and its steps are.
#[derive(bevy::ecs::system::SystemParam)]
pub(super) struct Places<'w, 's> {
    row: Query<'w, 's, (&'static ComputedNode, &'static UiGlobalTransform), With<StepsRow>>,
    steps: Query<
        'w,
        's,
        (
            &'static StepNode,
            &'static ComputedNode,
            &'static UiGlobalTransform,
        ),
    >,
    end: Query<'w, 's, (&'static ComputedNode, &'static UiGlobalTransform), With<EndNode>>,
}

impl Places<'_, '_> {
    /// The place in the row a step let go at `at` (interface pixels) goes:
    /// before the first step whose middle is right of it; none off the row.
    pub(super) fn insert_at(&self, at: Vec2) -> Option<usize> {
        let (node, transform) = self.row.iter().next()?;
        let (corner, size) = logical_rect(node, transform);
        let (end_corner, end_size) = (self.end.iter().next())
            .map_or((corner, size), |(node, transform)| {
                logical_rect(node, transform)
            });
        let right = (end_corner.x + end_size.x).max(corner.x + size.x);
        let inside = at.x >= corner.x - MARGIN
            && at.x <= right + MARGIN
            && at.y >= corner.y - MARGIN
            && at.y <= corner.y + size.y + MARGIN;
        if !inside {
            return None;
        }
        let before = self.steps.iter().filter(|(_, node, transform)| {
            let (corner, size) = logical_rect(node, transform);
            corner.x + size.x / 2.0 < at.x
        });
        Some(before.count())
    }
}
