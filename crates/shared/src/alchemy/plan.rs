//! Working a process out (Alchemy plan): every step in order at full amount,
//! dilution only at the end, the chance, and the least quality. The same on
//! both sides: the client's preview and the server's result.

use super::alembic::alembic;
use super::batch::Batch;
use super::crucible::{crucible, filter};
use super::mortar::mortar;
use super::pelican::pelican;
use super::retort::retort;
use super::worked_quality;
use super::{
    AlchemySkills, Apparatus, Contribution, Form, Mix, Process, Refused, Step, chance,
    least_quality, skill_steps,
};
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
    /// A Paste or an Ointment from a Mortar last, or a Powder from a
    /// Crucible (a Filter after either keeping it).
    Mixture { form: Form, contents: Contents },
    /// A potion in `bottles` bottles, one per 100 ml, each holding `contents`.
    Potion { contents: Contents, bottles: u32 },
    /// Pure alcohol in `bottles` bottles: drinks distilled with nothing else
    /// in them (`ALCOHOL_A_BOTTLE`).
    Alcohol { bottles: u32 },
}

impl Product {
    /// Its doses; none for nothing.
    pub fn contents(&self) -> &Contents {
        match self {
            Self::Nothing | Self::Alcohol { .. } => &NO_CONTENTS,
            Self::Mixture { contents, .. } | Self::Potion { contents, .. } => contents,
        }
    }

    /// Whether it has a quality: a mixture's or a potion's, not pure
    /// alcohol's (a liquid as plain as water) or nothing's.
    pub fn has_quality(&self) -> bool {
        matches!(self, Self::Mixture { .. } | Self::Potion { .. })
    }
}

/// A process worked out, before the server's rolls.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Planned {
    pub product: Product,
    /// In hundredths of a percent (`CERTAIN` is 100%).
    pub chance: u32,
    /// The least it can be (`least_quality`); none for nothing or pure
    /// alcohol.
    pub quality: Option<Quality>,
    /// The apparatus whose sub-skills are practised, a step each: every step
    /// but a Retort holding liquids alone.
    pub practised: Vec<Apparatus>,
    /// What each herb gave, for learning and the preview.
    pub contributions: Vec<Contribution>,
    /// Salt left behind in an Alembic, one for each 100 ml of Salt Water
    /// distilled: given with the product on success.
    pub salt: u32,
}

/// Works out what `process` makes.
pub fn plan(process: &Process, skills: AlchemySkills) -> Result<Planned, Refused> {
    if process.steps.is_empty() {
        return Err(Refused::Empty);
    }
    let source = process.source.as_ref();
    let mut batch = Batch {
        mix: source.map_or(Mix::default(), |stored| Mix::of(&stored.contents)),
        form: source.map(|stored| stored.form),
        ..Batch::default()
    };
    let mut extra_kinds = 0;
    let mut contributions = Vec::new();
    let mut practised = Vec::new();
    for (at, step) in process.steps.iter().enumerate() {
        match step {
            Step::Mortar { herbs, solvent } => {
                if at > 0 || source.is_some() {
                    return Err(Refused::MortarNotFirst);
                }
                let ground = mortar(herbs, *solvent, skills.brew)?;
                (batch.mix, batch.form) = (ground.mix, Some(ground.form));
                if ground.form == Form::Tincture {
                    // The pure alcohol it was ground in: 100 ml of liquid.
                    (batch.units, batch.alcohol) = (1, 100);
                }
                (contributions, extra_kinds) = (ground.contributions, ground.extra_kinds);
                practised.push(Apparatus::Mortar);
            }
            Step::Retort { liquids, solid } => {
                retort(&mut batch, liquids, *solid)?;
                if batch.form.is_some() {
                    practised.push(Apparatus::Retort);
                }
            }
            Step::Alembic { fed } => {
                alembic(&mut batch, fed.as_ref())?;
                practised.push(Apparatus::Alembic);
            }
            Step::Crucible => {
                crucible(&mut batch)?;
                practised.push(Apparatus::Crucible);
            }
            Step::Filter { powder } => {
                filter(&mut batch, powder.as_ref())?;
                practised.push(Apparatus::Filter);
            }
            Step::Pelican => {
                pelican(&mut batch)?;
                practised.push(Apparatus::Pelican);
            }
        }
    }
    let product = batch.finished();
    let steps = process.steps.len() as u32;
    let chance = chance(skills.alchemy_percent(), steps, extra_kinds);
    let used: Vec<Apparatus> = process.steps.iter().map(Step::apparatus).collect();
    let old = source.is_some_and(|stored| stored.form.is_old());
    let quality = product.has_quality().then(|| {
        let purity = worked_quality(product.contents(), batch.circulated, process.at_bench);
        least_quality(purity, chance, skill_steps(steps, &used, skills), old)
    });
    Ok(Planned {
        product,
        chance,
        quality,
        practised,
        contributions,
        salt: batch.salt_made,
    })
}
