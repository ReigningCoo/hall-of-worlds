//! The Alchemy rules (Alchemy plan, owner, 2026-11-19: `docs/context/alchemy.md`):
//! a process of apparatus steps, each with its own sub-skill; its chance, the
//! product (dilution only at the end, doses kept in hundredths until then), its
//! quality, what the preview shows and what a success teaches. Pure functions
//! on whole numbers, so the client's preview matches the server. The Mortar
//! reuses brewing's rules (`brewing::ground_doses`). Also what the server runs
//! (PR 3): the order as asked for, the Paste and Powder items, timing, practice and the
//! sub-skills learned on carrying an apparatus. Not the panel.

mod alembic;
mod apparatus;
mod batch;
mod chance;
mod crucible;
mod learn;
mod mix;
mod mortar;
mod order;
mod outcome;
mod paste;
mod pelican;
mod plan;
mod process;
mod quality;
mod retort;
mod run;
mod salt;
mod skills;

pub use alembic::{STRENGTH_A_DIVISION, reduced_by};
pub use apparatus::{Apparatus, CALCINING, CIRCULATING, DISTILLING, FILTERING, GRINDING, INFUSING};
pub use batch::ALCOHOL_A_BOTTLE;
pub use chance::{CERTAIN, chance, square_root_up, steps_allowed, succeeds};
pub use crucible::POWDER_PERCENT;
pub use learn::{Lesson, Seen, lesson, preview};
pub use mix::{HUNDREDTHS, Mix, boosted};
pub use mortar::{Contribution, TINCTURE_PERCENT};
pub use order::{AlchemyOrder, Checked, MOST_REPEATS, OrderRefused, OrderStep};
pub use outcome::{Outcome, outcome};
pub use paste::{PASTE_GRAMS, POWDER_GRAMS, new_mixture};
pub use pelican::{CIRCULATION_PERCENT, circulated_quality};
pub use plan::{Planned, Product, plan};
pub use process::{Form, Process, RETORT_UNITS, Refused, Solvent, Step, Stored};
pub use quality::{LUCKY_ONCE, LUCKY_TWICE, least_quality, luck_steps, moved, skill_steps};
pub use run::{HERB_PRACTICE, STEP_PRACTICE, learn_sub_skills, run_seconds, run_ticks};
pub use salt::{SALT_GRAMS, new_salt};
pub use skills::AlchemySkills;

#[cfg(test)]
mod alembic_tests;
#[cfg(test)]
mod crucible_tests;
#[cfg(test)]
mod order_tests;
#[cfg(test)]
mod pelican_tests;
#[cfg(test)]
mod tests;
