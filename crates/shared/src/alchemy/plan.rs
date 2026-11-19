//! Working a process out (Alchemy plan): every step in order at full amount,
//! dilution only at the end, the chance, and the least quality. The same on
//! both sides: the client's preview and the server's result.

use super::mortar::mortar;
use super::retort::retort;
use super::{
    AlchemySkills, Apparatus, Contribution, Form, Mix, Process, Refused, Step, chance,
    least_quality, skill_steps,
};
use crate::brewing::{KEPT_FROM, POTION_AT, quality};
use crate::item::Quality;
use crate::potion::Contents;

/// Nothing's doses.
static NO_CONTENTS: Contents = Contents::EMPTY;

/// What a process makes, if it works.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Product {
    /// Nothing worth keeping: no dose left, or a liquid with none reaching
    /// `POTION_AT` once diluted. The ingredients are still used up.
    Nothing,
    /// A Paste or an Ointment, from a Mortar last.
    Mixture { form: Form, contents: Contents },
    /// A potion in `bottles` bottles, one per 100 ml, each holding `contents`.
    Potion { contents: Contents, bottles: u32 },
}

impl Product {
    /// Its doses; none for nothing.
    pub fn contents(&self) -> &Contents {
        match self {
            Self::Nothing => &NO_CONTENTS,
            Self::Mixture { contents, .. } | Self::Potion { contents, .. } => contents,
        }
    }
}

/// A process worked out, before the server's rolls.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Planned {
    pub product: Product,
    /// In hundredths of a percent (`CERTAIN` is 100%).
    pub chance: u32,
    /// The least it can be (`least_quality`); none for nothing.
    pub quality: Option<Quality>,
    /// The apparatus whose sub-skills are practised, a step each: every step
    /// but a Retort holding liquids alone.
    pub practised: Vec<Apparatus>,
    /// What each herb gave, for learning and the preview.
    pub contributions: Vec<Contribution>,
}

/// Works out what `process` makes.
pub fn plan(process: &Process, skills: AlchemySkills) -> Result<Planned, Refused> {
    if process.steps.is_empty() {
        return Err(Refused::Empty);
    }
    let source = process.source.as_ref();
    let mut mix = source.map_or(Mix::default(), |stored| Mix::of(&stored.contents));
    let mut form = source.map(|stored| stored.form);
    let (mut units, mut extra_kinds) = (0, 0);
    let mut contributions = Vec::new();
    let mut practised = Vec::new();
    for (at, step) in process.steps.iter().enumerate() {
        match step {
            Step::Mortar { herbs, solvent } => {
                if at > 0 || source.is_some() {
                    return Err(Refused::MortarNotFirst);
                }
                let ground = mortar(herbs, *solvent, skills.brew)?;
                (mix, form) = (ground.mix, Some(ground.form));
                (contributions, extra_kinds) = (ground.contributions, ground.extra_kinds);
                practised.push(Apparatus::Mortar);
            }
            Step::Retort { liquids } => {
                units += retort(&mut mix, liquids)?;
                if form.is_some() {
                    practised.push(Apparatus::Retort);
                }
            }
            other => return Err(Refused::NotBuilt(other.apparatus())),
        }
    }
    let product = finished(&mix, form, units);
    let steps = process.steps.len() as u32;
    let chance = chance(skills.alchemy_percent(), steps, extra_kinds);
    let used: Vec<Apparatus> = process.steps.iter().map(Step::apparatus).collect();
    let old = source.is_some_and(|stored| stored.form.is_old());
    let quality = (product != Product::Nothing).then(|| {
        let purity = quality(product.contents());
        least_quality(purity, chance, skill_steps(steps, &used, skills), old)
    });
    Ok(Planned {
        product,
        chance,
        quality,
        practised,
        contributions,
    })
}

/// The product: with liquid in it, diluted over its 100 ml units into a
/// potion if a dose reaches `POTION_AT` (keeping those of `KEPT_FROM` or
/// more); otherwise the mixture as it is.
fn finished(mix: &Mix, form: Option<Form>, units: u32) -> Product {
    if units == 0 {
        let contents = mix.diluted(1);
        return match (form, contents.is_empty()) {
            (Some(form), false) => Product::Mixture { form, contents },
            _ => Product::Nothing,
        };
    }
    let diluted = mix.diluted(units);
    if !diluted.doses().iter().any(|d| d.step.abs() >= POTION_AT) {
        return Product::Nothing;
    }
    let kept = diluted.doses().iter().filter(|d| d.step.abs() >= KEPT_FROM);
    Product::Potion {
        contents: Contents::new(kept.copied()),
        bottles: units,
    }
}
