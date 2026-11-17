//! `named_weapons` and `generic_weapons`: what names there are and the lengths
//! each covers, for the players' wiki (`cargo xtask wiki`).

use super::table::rows;
use super::{Build, CELLS, Family, overlap};

/// A name a weapon may go by, with the lengths it covers in millimetres: from
/// the first, up to but not including the last. A head at the side's `blade` is
/// its cutting edge.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WeaponName {
    pub name: &'static str,
    pub family: Family,
    pub build: Build,
    pub blade: (u16, u16),
    pub handle: (u16, u16),
}

/// Every specific weapon (not a generic name), in table order: family by
/// family, best known first. Its lengths are those its row can be made in.
pub fn named_weapons() -> Vec<WeaponName> {
    rows()
        .filter(|row| !row.generic)
        .map(|row| {
            let places: Vec<_> = row.places().collect();
            WeaponName {
                name: row.name,
                family: row.family,
                build: row.build,
                blade: reach(places.iter().map(|(blade, _)| *blade)),
                handle: reach(places.iter().map(|(_, handle)| *handle)),
            }
        })
        .collect()
}

/// The generic names a weapon of this family may be given, by the size grid's
/// cells its rows reach, with the lengths of those cells they reach. A name on
/// several cells of one build comes once, covering them all. A short blade on a
/// grip is named "Knife" here: `BladeForm::name` calls it "Dagger" unless it
/// has one edge.
pub fn generic_weapons(family: Family) -> Vec<WeaponName> {
    let family_rows: Vec<_> = rows().filter(|row| row.family == family).collect();
    let mut names: Vec<WeaponName> = Vec::new();
    for cell in &CELLS {
        let Some(name) = cell.generic else { continue };
        let reached: Vec<_> = family_rows
            .iter()
            .filter(|row| row.build == cell.build)
            .filter_map(|row| {
                Some((
                    overlap(row.blade, cell.blade)?,
                    overlap(row.handle, cell.handle)?,
                ))
            })
            .collect();
        if reached.is_empty() {
            continue;
        }
        let blade = reach(reached.iter().map(|(blade, _)| *blade));
        let handle = reach(reached.iter().map(|(_, handle)| *handle));
        let same = |known: &&mut WeaponName| known.name == name && known.build == cell.build;
        match names.iter_mut().find(same) {
            Some(known) => {
                known.blade = (known.blade.0.min(blade.0), known.blade.1.max(blade.1));
                known.handle = (known.handle.0.min(handle.0), known.handle.1.max(handle.1));
            }
            None => names.push(WeaponName {
                name,
                family,
                build: cell.build,
                blade,
                handle,
            }),
        }
    }
    names
}

/// The shortest start and the longest end of these spans.
fn reach(spans: impl Iterator<Item = (u16, u16)> + Clone) -> (u16, u16) {
    let start = spans.clone().map(|span| span.0).min().unwrap_or(0);
    let end = spans.map(|span| span.1).max().unwrap_or(0);
    (start, end)
}
