//! `Mix`: what a process holds between steps, each stat's dose in hundredths
//! of a step at full amount and strength. Dilution comes only at the end
//! (`diluted`), so the result is rounded once and the client's preview matches
//! the server's.

use crate::brewing::stat_row;
use crate::herb::Stat;
use crate::potion::{Contents, Dose};
use crate::sheet::Row;

/// A step in hundredths.
pub const HUNDREDTHS: i32 = 100;

/// Each stat's dose in hundredths of a step, in `Stat::ALL`'s order.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Mix(pub [i32; Stat::ALL.len()]);

impl Mix {
    /// Whole doses as a mix.
    pub fn of(contents: &Contents) -> Self {
        let mut mix = Self::default();
        for dose in contents.doses() {
            mix.0[dose.stat as usize] += i32::from(dose.step) * HUNDREDTHS;
        }
        mix
    }

    /// Whether every dose is 0.
    pub fn is_empty(&self) -> bool {
        self.0.iter().all(|&dose| dose == 0)
    }

    /// Every dose made `percent`% of what it is, towards 0 (a tincture's 105:
    /// exact on whole steps).
    pub fn scale(&mut self, percent: i32) {
        for dose in &mut self.0 {
            *dose = *dose * percent / 100;
        }
    }

    /// A quarter more on every dose in `row` (an oil's), at least a whole
    /// step, either way (`brewing::boosted`'s rule in hundredths).
    pub fn boost_row(&mut self, row: Row) {
        for (at, dose) in self.0.iter_mut().enumerate() {
            if stat_row(Stat::ALL[at]) == row {
                *dose = boosted(*dose);
            }
        }
    }

    /// A quarter more on every helpful (or harmful) dose: Moon (or Cave) Water.
    pub fn boost_way(&mut self, helpful: bool) {
        for dose in &mut self.0 {
            if *dose != 0 && (*dose > 0) == helpful {
                *dose = boosted(*dose);
            }
        }
    }

    /// The strongest helpful (or harmful) dose taken away, the first in
    /// `Stat`'s order on a tie: Salt (or Rain) Water.
    pub fn remove_strongest(&mut self, helpful: bool) {
        let mut strongest: Option<usize> = None;
        for (at, &dose) in self.0.iter().enumerate() {
            let way = dose != 0 && (dose > 0) == helpful;
            if way && strongest.is_none_or(|best| dose.abs() > self.0[best].abs()) {
                strongest = Some(at);
            }
        }
        if let Some(at) = strongest {
            self.0[at] = 0;
        }
    }

    /// Filtered through `powder` (the Filter): each stat in both adds
    /// together, either way; a stat in only one is taken out.
    pub fn filtered(&self, powder: &Mix) -> Mix {
        let mut out = Mix::default();
        for (at, dose) in out.0.iter_mut().enumerate() {
            let (a, b) = (self.0[at], powder.0[at]);
            if a != 0 && b != 0 {
                *dose = a + b;
            }
        }
        out
    }

    /// The doses spread over `units` bottles (at least 1), each rounded to
    /// the nearest whole step, halves away from 0: dose ÷ units.
    pub fn diluted(&self, units: u32) -> Contents {
        let per = i64::from(HUNDREDTHS) * i64::from(units.max(1));
        Contents::new(self.0.iter().enumerate().map(|(at, &dose)| {
            let size = (i64::from(dose).abs() * 2 + per) / (per * 2);
            let step = size.min(i64::from(i16::MAX)) as i16;
            Dose {
                stat: Stat::ALL[at],
                step: if dose < 0 { -step } else { step },
            }
        }))
    }
}

/// A dose in hundredths made a quarter stronger, at least a whole step:
/// 400 → 500, 200 → 300, -250 → -350, 0 stays 0.
pub fn boosted(dose: i32) -> i32 {
    let more = (dose.abs() / 4).max(HUNDREDTHS);
    match dose {
        0 => 0,
        d if d > 0 => d + more,
        d => d - more,
    }
}
