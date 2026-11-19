//! A process's chance of working (Alchemy plan): 100% + Alchemy%, halved for
//! each step beyond those allowed and each herb kind over the mortar's limit.
//! Kept in hundredths of a percent, whole numbers only.

/// A chance of 100%, in hundredths of a percent.
pub const CERTAIN: u32 = 10_000;

/// The whole square root of `n`, rounded up: 0 → 0, 1 → 1, 2 → 2, 4 → 2, 5 → 3.
pub fn square_root_up(n: u32) -> u32 {
    let root = crate::brewing::square_root(n);
    if root * root < n { root + 1 } else { root }
}

/// How many steps a process may have before its chance halves:
/// ⌈√Alchemy%⌉, at least 1 (1 untrained, 10 at 100%).
pub fn steps_allowed(alchemy_percent: u32) -> u32 {
    square_root_up(alchemy_percent.min(100)).max(1)
}

/// The chance, in hundredths of a percent (`CERTAIN` is 100%), of a process of
/// `steps` steps with `extra_kinds` herb kinds over the mortar's limit:
/// 100% + Alchemy%, halved (rounded down) once per step beyond
/// `steps_allowed` and once per extra kind.
pub fn chance(alchemy_percent: u32, steps: u32, extra_kinds: u32) -> u32 {
    let alchemy = alchemy_percent.min(100);
    let halvings = steps.saturating_sub(steps_allowed(alchemy)) + extra_kinds;
    (CERTAIN + alchemy * 100).checked_shr(halvings).unwrap_or(0)
}

/// Whether a process with this chance works, for the server's `roll` from
/// 0 to `CERTAIN` − 1.
pub fn succeeds(chance: u32, roll: u32) -> bool {
    roll < chance
}
