//! How an effect looks on the bar: which effects show at all, their icons
//! (game-icons.net, listed in `xtask/icons.txt`) and the colour behind them.

use bevy::prelude::*;
use shared::bonus::{Amount, Effect, Source, Stat};

/// Behind an effect that helps: a calm blue-green.
const HELPFUL: Color = Color::srgb(0.16, 0.38, 0.40);
/// Behind one that harms (drains Health or Mana, or takes anything away): a
/// sickly yellow-green.
const HARMFUL: Color = Color::srgb(0.42, 0.46, 0.10);

/// The effects that show, in the order they are kept: those with an end not
/// yet reached at tick `now`.
pub(super) fn timed(effects: &[Effect], now: f64) -> impl Iterator<Item = &Effect> {
    effects
        .iter()
        .filter(move |effect| effect.end.is_some_and(|end| now < end as f64))
}

/// Whether it does harm: draining, or less of anything.
pub(super) fn is_harmful(effect: &Effect) -> bool {
    effect.bonuses.iter().any(|bonus| {
        let less = match bonus.amount {
            Amount::Flat(milli) | Amount::Percent(milli) => milli < 0,
        };
        less || matches!(bonus.stat, Stat::Drain(_))
    })
}

/// The colour behind its icon.
pub(super) fn colour(effect: &Effect) -> Color {
    if is_harmful(effect) { HARMFUL } else { HELPFUL }
}

/// Where its icon is, in the assets: every food shares one.
pub(super) fn icon_path(source: Source) -> &'static str {
    match source {
        Source::Eaten(_) | Source::EatenFruit(_) | Source::EatenVegetable(_) => {
            "icons/delapouite/eating.png"
        }
        Source::FruitEffects(_) => "icons/lorc/shiny-apple.png",
        Source::VegetableEffects(_) => "icons/delapouite/carrot.png",
        Source::Poison => "icons/lorc/poison-bottle.png",
        Source::Dev => "icons/lorc/magic-swirl.png",
        Source::RawHerb => "icons/delapouite/herbs-bundle.png",
        Source::Ointment => "icons/delapouite/covered-jar.png",
        Source::Potion(_) => "icons/lorc/fizzing-flask.png",
    }
}

/// How much of its time has gone at tick `now`: 0 when it began, 1 at its end.
pub(super) fn gone(effect: &Effect, now: f64) -> f32 {
    let Some(end) = effect.end else {
        return 0.0;
    };
    let whole = end.saturating_sub(effect.start) as f64;
    if whole <= 0.0 {
        return 1.0;
    }
    ((now - effect.start as f64) / whole).clamp(0.0, 1.0) as f32
}
