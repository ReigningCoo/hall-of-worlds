//! How likely butchering gives an organ (owner, decision 9).

/// Chances are out of this: hundredths of a percent.
pub const OUT_OF: u32 = 10_000;

/// The chance, out of `OUT_OF`, that butchering gives an organ with this base
/// percent, by the butcher's Butchery percent (0 to 100): base + (95 − base) ×
/// Butchery%, never less than the base. A rare stone (gallstone, bezoar, os
/// cordis) stays rare instead: base × 0.5 at no skill to base × 1.5 at 100%.
pub fn organ_chance(base: u8, stone: bool, butchery: u8) -> u32 {
    let base = u32::from(base.min(100));
    let skill = u32::from(butchery.min(100));
    if stone {
        base * (50 + skill)
    } else {
        base * 100 + 95u32.saturating_sub(base) * skill
    }
}
