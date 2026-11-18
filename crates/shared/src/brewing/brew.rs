//! Brewing an ointment with a water into a potion (decisions 2, 14 and 15).

use super::boost::boosted;
use crate::potion::{Contents, Dose, Water};

/// A potion forms when any dose reaches this many steps, either way.
pub const POTION_AT: i16 = 4;
/// The weakest dose a potion keeps; weaker ones are dropped.
pub const KEPT_FROM: i16 = 2;

/// What a water does to an ointment's doses (decision 2): Cave Water a quarter
/// more on harmful doses, Moon Water on helpful ones (`boosted`), Salt Water
/// takes away the strongest helpful dose (the first in `Stat`'s order on a
/// tie), plain Water nothing.
pub fn watered(ointment: &Contents, water: Water) -> Contents {
    let doses = ointment.doses().iter().copied();
    let strengthen = |helpful: bool| {
        Contents::new(doses.clone().map(|dose| match (dose.step > 0) == helpful {
            true => Dose {
                step: boosted(i32::from(dose.step)) as i16,
                ..dose
            },
            false => dose,
        }))
    };
    match water {
        Water::Plain => ointment.clone(),
        Water::Cave => strengthen(false),
        Water::Moon => strengthen(true),
        Water::Salt => {
            let strongest = doses
                .clone()
                .filter(|dose| dose.step > 0)
                .reduce(|best, dose| if dose.step > best.step { dose } else { best });
            Contents::new(doses.filter(|dose| Some(*dose) != strongest))
        }
    }
}

/// The potion an ointment and a water make (decision 14), or `None` when no
/// dose reaches `POTION_AT` after the water: then the ingredients are still
/// used up and practice still given (decision 15, the caller's). The potion
/// keeps the doses of `KEPT_FROM` or more, either way.
pub fn brew(ointment: &Contents, water: Water) -> Option<Contents> {
    let watered = watered(ointment, water);
    let doses = watered.doses();
    if !doses.iter().any(|dose| dose.step.abs() >= POTION_AT) {
        return None;
    }
    let kept = doses.iter().filter(|dose| dose.step.abs() >= KEPT_FROM);
    Some(Contents::new(kept.copied()))
}
