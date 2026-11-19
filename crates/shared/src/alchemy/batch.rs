//! `Batch`: what a process holds between steps (Alchemy plan): the doses at
//! full amount (`Mix`), what form it is, its volume in 100 ml units, the
//! alcohol and salt in it, and the salt the Alembic has left behind. Then
//! what it finishes as (`finished`), dilution coming only there.

use super::{Form, Mix, Product};
use crate::brewing::{KEPT_FROM, POTION_AT};
use crate::potion::Contents;

/// Millilitres of alcohol in the batch each bottle of pure alcohol takes
/// (Claude's first guess, for the owner to tune): 500 ml of 6% cider holds
/// 30 ml, enough for 3 bottles once reduced.
pub const ALCOHOL_A_BOTTLE: u32 = 10;

/// What a process holds between steps.
#[derive(Clone, Debug, Default)]
pub(super) struct Batch {
    pub mix: Mix,
    /// None until a Mortar or a stored source; liquids alone have none.
    pub form: Option<Form>,
    /// Its volume in 100 ml units: none for a Paste or an Ointment.
    pub units: u32,
    /// Millilitres of pure alcohol in it.
    pub alcohol: u32,
    /// 100 ml units of Salt Water in it, their salt not yet distilled out.
    pub salt: u32,
    /// Salt the Alembic has left behind, one for each unit of Salt Water.
    pub salt_made: u32,
    /// Whether its last step was an Alembic that reduced it: only then is
    /// what holds alcohol and no doses pure alcohol.
    pub distilled: bool,
}

impl Batch {
    /// The product: with liquid in it, diluted over its units into a potion
    /// if a dose reaches `POTION_AT` (keeping those of `KEPT_FROM` or more);
    /// distilled alcohol with no doses, pure alcohol; with no liquid, the
    /// mixture as it is.
    pub fn finished(&self) -> Product {
        if self.units == 0 {
            let contents = self.mix.diluted(1);
            return match (self.form, contents.is_empty()) {
                (Some(form), false) => Product::Mixture { form, contents },
                _ => Product::Nothing,
            };
        }
        if self.mix.is_empty() {
            let bottles = self.units.min(self.alcohol / ALCOHOL_A_BOTTLE);
            return match self.distilled && bottles > 0 {
                true => Product::Alcohol { bottles },
                false => Product::Nothing,
            };
        }
        let diluted = self.mix.diluted(self.units);
        if !diluted.doses().iter().any(|d| d.step.abs() >= POTION_AT) {
            return Product::Nothing;
        }
        let kept = diluted.doses().iter().filter(|d| d.step.abs() >= KEPT_FROM);
        Product::Potion {
            contents: Contents::new(kept.copied()),
            bottles: self.units,
        }
    }
}
