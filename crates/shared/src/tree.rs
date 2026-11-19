//! Trees that can be felled (woodcutting plan, `docs/context/woodcutting.md`):
//! the wild kinds and where each grows (`TreeKind::roll`), every tree's wood
//! and logs, fruit trees' too, and a felled tree growing back. See
//! `docs/context/shared/tree.md`.

mod fruit_wood;
mod kind;
mod roll;
mod stage;
mod standing;
mod table;

pub use fruit_wood::{fruit_kindling, fruit_wood};
pub use kind::{Commonness, Group, TreeKind};
pub use roll::Setting;
pub use stage::{
    HALF_GROWN_COST_PER_MILLE, REGROW_MINUTES, STAGE_MINUTES, STUMP_COST_PER_MILLE, STUMP_MINUTES,
    TreeStage,
};
pub use standing::Tree;
pub use table::{TREES, TreeDef};

#[cfg(test)]
mod tests;
