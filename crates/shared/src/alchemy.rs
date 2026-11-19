//! The Alchemy rules (Alchemy plan, owner, 2026-11-19: `docs/context/alchemy.md`):
//! a process of apparatus steps, each with its own sub-skill; its chance, the
//! product (dilution only at the end, doses kept in hundredths until then), its
//! quality, what the preview shows and what a success teaches. Pure functions
//! on whole numbers, so the client's preview matches the server. The Mortar
//! reuses brewing's rules (`brewing::ground_doses`). Not the panel or the
//! server's side.

mod apparatus;
mod chance;
mod learn;
mod mix;
mod mortar;
mod outcome;
mod plan;
mod process;
mod quality;
mod retort;
mod skills;

pub use apparatus::{Apparatus, CALCINING, CIRCULATING, DISTILLING, FILTERING, GRINDING, INFUSING};
pub use chance::{CERTAIN, chance, square_root_up, steps_allowed, succeeds};
pub use learn::{Lesson, Seen, lesson, preview};
pub use mix::{HUNDREDTHS, Mix, boosted};
pub use mortar::Contribution;
pub use outcome::{Outcome, outcome};
pub use plan::{Planned, Product, plan};
pub use process::{Form, Process, RETORT_UNITS, Refused, Solvent, Step, Stored};
pub use quality::{LUCKY_ONCE, LUCKY_TWICE, least_quality, luck_steps, moved, skill_steps};
pub use skills::AlchemySkills;

#[cfg(test)]
mod tests;
