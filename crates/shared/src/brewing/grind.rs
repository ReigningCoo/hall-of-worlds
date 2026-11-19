//! Grinding herbs and an oil into an ointment (decisions 6, 7 and 9 to 13).

use super::boost::{boosted, oil_suits, stat_row};
use super::cut::cut_unwanted;
use super::skill::{BrewSkills, cut_steps, most_doses, most_kinds, per_herb_cap, stat_cap};
use crate::herb::{Effect, HerbDef, Stat, potency};
use crate::item::Quality;
use crate::potion::{Contents, Dose, Oil};

/// One kind of herb in a mix, at the quality of the ones used.
#[derive(Clone, Copy, Debug)]
pub struct GroundHerb<'a> {
    pub def: &'a HerbDef,
    pub quality: Quality,
}

/// What grinding makes: `doses` of an ointment holding `contents` (which can
/// be empty when everything cancels out). Each dose used up one of every herb
/// in the mix; the one oil made the whole batch.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Ointment {
    pub contents: Contents,
    pub doses: u32,
}

/// Why a mix can't be ground.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Refused {
    /// No herbs at all.
    NoHerbs,
    /// The same herb twice: only one of each counts (decision 6).
    SameHerb,
    /// More kinds of herb than the brewer can handle (`most_kinds`).
    TooManyKinds,
    /// Doses asked for outside 1 to `most_doses`.
    Doses,
}

/// The steps one herb gives each of its effects (decision 9): `change ×
/// potency` (`herb::potency`, so quality only as far as Herbalism reaches),
/// rounded to the nearest whole step (Claude's choice, so a poor herb's
/// weakest effect still counts), at most `per_herb_cap` either way.
pub fn herb_steps(effect: Effect, quality: Quality, skills: BrewSkills) -> i32 {
    let percent = potency(quality, skills.herbalism) as i32;
    let size = (i32::from(effect.strength()) * percent + 50) / 100;
    let size = size.min(per_herb_cap(skills));
    if effect.raises() { size } else { -size }
}

/// Grinds these herbs (one of each kind, `doses` sets of them used) with an
/// oil: every herb's effects added per stat with opposites cancelling, the
/// oil's row boosted (`boosted`), each stat at most `stat_cap`, then
/// Herbalism's cuts (`cut_unwanted`). The caller checks the herbs and the oil
/// are carried and takes them away.
pub fn grind(
    herbs: &[GroundHerb],
    oil: Oil,
    doses: u32,
    skills: BrewSkills,
) -> Result<Ointment, Refused> {
    if herbs.is_empty() {
        return Err(Refused::NoHerbs);
    }
    let same = |(i, a): (usize, &GroundHerb)| herbs[..i].iter().any(|b| b.def.key == a.def.key);
    if herbs.iter().enumerate().any(same) {
        return Err(Refused::SameHerb);
    }
    if herbs.len() > most_kinds(skills) {
        return Err(Refused::TooManyKinds);
    }
    if !(1..=most_doses(skills)).contains(&doses) {
        return Err(Refused::Doses);
    }
    let contents = ground_doses(herbs, Some(oil), skills);
    Ok(Ointment { contents, doses })
}

/// The mortar's sums (decisions 9 to 13), with no checks: every herb's
/// effects added per stat with opposites cancelling, the oil's row boosted
/// (`boosted`; none without an oil), each stat at most `stat_cap`, then
/// Herbalism's cuts (`cut_unwanted`). `grind` and the Alchemy plan's Mortar
/// (`shared::alchemy`) share it.
pub fn ground_doses(herbs: &[GroundHerb], oil: Option<Oil>, skills: BrewSkills) -> Contents {
    let mut totals: Vec<(Stat, i32)> = Stat::ALL.iter().map(|&stat| (stat, 0)).collect();
    for herb in herbs {
        for &effect in &herb.def.effects {
            let at = Stat::ALL
                .iter()
                .position(|&s| s == effect.stat)
                .unwrap_or(0);
            totals[at].1 += herb_steps(effect, herb.quality, skills);
        }
    }
    let cap = stat_cap(skills);
    for (stat, step) in &mut totals {
        if Some(stat_row(*stat)) == oil.and_then(oil_suits) {
            *step = boosted(*step);
        }
        *step = (*step).clamp(-cap, cap);
    }
    cut_unwanted(&mut totals, cut_steps(skills));
    Contents::new(totals.into_iter().map(|(stat, step)| Dose {
        stat,
        step: step as i16,
    }))
}
