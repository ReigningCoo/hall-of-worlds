//! What a process shows before it runs and teaches when it works (Alchemy
//! plan): an effect of the product is known only when every herb that gave to
//! it has that effect known; a success teaches one effect of one herb. Fungi
//! used like herbs are known and taught the same, in `FungusKnowledge`.

use super::{Contribution, Planned, Product, made_from};
use crate::brewing::Ingredient;
use crate::fungus::FungusKnowledge;
use crate::herb::HerbKnowledge;
use crate::potion::Dose;

/// One effect of the product as the preview shows it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Seen {
    Known(Dose),
    Unknown,
}

/// What a character knows of the herbs and fungi a process uses.
#[derive(Clone, Copy, Debug)]
pub struct Known<'a> {
    pub herbs: &'a HerbKnowledge,
    pub fungi: &'a FungusKnowledge,
}

impl Contribution {
    /// Whether this effect of its herb (or fungus) is known.
    pub fn is_known(&self, known: Known) -> bool {
        match self.ingredient {
            Ingredient::Herb => known.herbs.knows_effect(&self.herb, self.effect),
            Ingredient::Fungus => known.fungi.knows_effect(&self.herb, self.effect),
        }
    }
}

/// One herb's (or fungus's) effect a success teaches.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Lesson {
    pub ingredient: Ingredient,
    pub herb: String,
    /// Which of its effects (0 to 3), for `learn_effect`.
    pub effect: usize,
}

impl Lesson {
    /// Learns it, in the knowledge of its table: whether it was new.
    pub fn learn(&self, herbs: &mut HerbKnowledge, fungi: &mut FungusKnowledge) -> bool {
        match self.ingredient {
            Ingredient::Herb => herbs.learn_effect(&self.herb, self.effect),
            Ingredient::Fungus => fungi.learn_effect(&self.herb, self.effect),
        }
    }
}

/// The product's effects in `Stat`'s order, each `Unknown` unless the
/// character knows that effect on every herb that gave to it (a stored source's
/// doses are on the item, so known).
pub fn preview(planned: &Planned, known: Known) -> Vec<Seen> {
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
pub fn lesson(planned: &Planned, known: Known) -> Option<Lesson> {
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
            ingredient: c.ingredient,
            herb: c.herb.clone(),
            effect: c.effect,
        })
    })
}

/// The herbs' effects on this dose's stat that aren't known, in the order
/// added: for a transformation's effect, those on the stat it was made from.
fn unknown<'a>(
    planned: &'a Planned,
    known: Known<'a>,
    dose: Dose,
) -> impl Iterator<Item = &'a Contribution> {
    let stat = made_from(dose.stat);
    (planned.contributions.iter()).filter(move |c| c.stat == stat && !c.is_known(known))
}
