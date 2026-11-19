//! What a process shows before it runs and teaches when it works (Alchemy
//! plan): an effect of the product is known only when every herb that gave to
//! it has that effect known; a success teaches one effect of one herb.

use super::{Contribution, Planned, Product};
use crate::herb::HerbKnowledge;
use crate::potion::Dose;

/// One effect of the product as the preview shows it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Seen {
    Known(Dose),
    Unknown,
}

/// One herb's effect a success teaches.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Lesson {
    pub herb: String,
    /// Which of its effects (0 to 3), for `HerbKnowledge::learn_effect`.
    pub effect: usize,
}

/// The product's effects in `Stat`'s order, each `Unknown` unless the
/// character knows that effect on every herb that gave to it (a stored source's
/// doses are on the item, so known).
pub fn preview(planned: &Planned, known: &HerbKnowledge) -> Vec<Seen> {
    let doses = planned.product.contents().doses();
    doses
        .iter()
        .map(|&dose| match unknown(planned, known, dose).next() {
            None => Seen::Known(dose),
            Some(_) => Seen::Unknown,
        })
        .collect()
}

/// What a success teaches: the product's strongest effect with something
/// unknown (the first in `Stat`'s order on a tie), on the unknown herb that
/// gave most towards it (the first added on a tie); none when all is known.
pub fn lesson(planned: &Planned, known: &HerbKnowledge) -> Option<Lesson> {
    if planned.product == Product::Nothing {
        return None;
    }
    let mut doses: Vec<Dose> = planned.product.contents().doses().to_vec();
    doses.sort_by_key(|dose| std::cmp::Reverse(dose.step.abs()));
    doses.into_iter().find_map(|dose| {
        let towards = |c: &Contribution| c.steps * i32::from(dose.step.signum());
        let mut best: Option<&Contribution> = None;
        for c in unknown(planned, known, dose) {
            if best.is_none_or(|b| towards(c) > towards(b)) {
                best = Some(c);
            }
        }
        best.map(|c| Lesson {
            herb: c.herb.clone(),
            effect: c.effect,
        })
    })
}

/// The herbs' effects on this dose's stat that aren't known, in the order added.
fn unknown<'a>(
    planned: &'a Planned,
    known: &'a HerbKnowledge,
    dose: Dose,
) -> impl Iterator<Item = &'a Contribution> {
    (planned.contributions.iter())
        .filter(move |c| c.stat == dose.stat && !known.knows_effect(&c.herb, c.effect))
}
