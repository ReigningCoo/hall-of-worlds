//! Trees chopped at and felled (woodcutting plan, `shared::tree`,
//! `shared::woodcutting`) on visits' levels: each tree's damage and the game
//! minute it was felled, kept by cell where its plants' harvests are
//! (`PlantKey`: the wilderness's by cell round the planet, a cutout's by its
//! site's); how far each felled tree has grown back, set on levels as they
//! are made and as world time passes; and telling everyone who can see one
//! when it changes.

mod cell_of;
mod grow;
mod keep;
mod kind;
mod share;

pub(in crate::sites) use cell_of::cell_of;
pub(crate) use grow::grow_trees;
pub(in crate::sites) use keep::put_felled;
pub(crate) use keep::{TreeChange, TreeNews, keep_tree, saved_tree, tidied};
pub(crate) use kind::{tree_here, tree_stage};
pub(crate) use share::share_trees;

#[cfg(test)]
mod tests;
