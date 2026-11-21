//! Rest clocks run by warmth (owner, 2026-11-21): too cold slows them, too
//! hot speeds them. Cooking only for now; yeast and casks may follow
//! (`todo-warmth-clocks`).

/// The warmth something carried rests at: about a body's or a room's, where
/// a clock runs at its usual pace.
pub const CARRIED_CELSIUS: i32 = 20;

/// How fast a rest clock runs at `celsius`, in percent of its usual pace:
/// stopped at freezing, rising 5 a degree to 100 at 20 °C, then on to 200 at
/// 40 °C and no faster. First guesses.
pub fn warmth_percent(celsius: i32) -> u32 {
    match celsius {
        ..=0 => 0,
        1..=20 => 5 * celsius as u32,
        21..=40 => 100 + 5 * (celsius - 20) as u32,
        _ => 200,
    }
}
