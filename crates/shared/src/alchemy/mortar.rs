//! The Mortar step (Alchemy plan): the mortar's values and decisions as
//! brewing had them (`brewing::ground_doses`), herbs alone making a Paste and
//! with an oil an Ointment, with pure alcohol a Tincture (no row boost, every
//! dose +5%); more kinds than Alchemy allows halve the chance instead of being
//! refused.

use super::{Form, Mix, Refused, Solvent};
use crate::brewing::{BrewSkills, GroundHerb, Ingredient, ground_doses, herb_steps, most_kinds};
use crate::herb::Stat;

/// What one herb's effect gave a mixture, before the oil, the caps and the
/// cuts: what learning weighs (`lesson`) and what the preview checks is known.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Contribution {
    /// Which table the herb is from: a herb or a fungus used like one.
    pub ingredient: Ingredient,
    /// The herb (or fungus), by key.
    pub herb: String,
    /// Which of its effects (0 to 3).
    pub effect: usize,
    pub stat: Stat,
    /// Its steps, either way (`brewing::herb_steps`).
    pub steps: i32,
}

/// A tincture's doses in percent of the dry grind's.
pub const TINCTURE_PERCENT: i32 = 105;

/// What the Mortar makes.
pub(super) struct Ground {
    pub mix: Mix,
    pub form: Form,
    pub contributions: Vec<Contribution>,
    /// Herb kinds over `most_kinds`, each halving the chance.
    pub extra_kinds: u32,
}

/// Grinds one of each of these herbs with the solvent.
pub(super) fn mortar(
    herbs: &[GroundHerb],
    solvent: Solvent,
    skills: BrewSkills,
) -> Result<Ground, Refused> {
    if herbs.is_empty() {
        return Err(Refused::NoHerbs);
    }
    let same = |(i, a): (usize, &GroundHerb)| herbs[..i].iter().any(|b| b.same_kind(a));
    if herbs.iter().enumerate().any(same) {
        return Err(Refused::SameHerb);
    }
    let (oil, form) = match solvent {
        Solvent::Dry => (None, Form::Paste),
        Solvent::Oil(oil) => (Some(oil), Form::Ointment),
        Solvent::Alcohol => (None, Form::Tincture),
    };
    let mut contributions = Vec::new();
    for herb in herbs {
        for (effect, &e) in herb.effects.iter().enumerate() {
            contributions.push(Contribution {
                ingredient: herb.ingredient,
                herb: herb.key.to_owned(),
                effect,
                stat: e.stat,
                steps: herb_steps(e, herb.quality, skills),
            });
        }
    }
    let mut mix = Mix::of(&ground_doses(herbs, oil, skills));
    if form == Form::Tincture {
        mix.scale(TINCTURE_PERCENT);
    }
    Ok(Ground {
        mix,
        form,
        contributions,
        extra_kinds: herbs.len().saturating_sub(most_kinds(skills)) as u32,
    })
}
