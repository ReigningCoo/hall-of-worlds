//! How an effect looks on the bar: which effects show at all, their icons
//! (game-icons.net, listed in `xtask/icons.txt`) and the colour behind them.

use bevy::prelude::*;
use shared::alcohol::Stage;
use shared::bonus::{Amount, Effect, Source, Stat};
use shared::sheet::Sheet;

/// Behind an effect that helps: a calm blue-green.
const HELPFUL: Color = Color::srgb(0.16, 0.38, 0.40);
/// Behind one that harms (drains Health or Mana, or takes anything away): a
/// sickly yellow-green.
const HARMFUL: Color = Color::srgb(0.42, 0.46, 0.10);

/// Every effect on the sheet, then those coming later that it was sent (only
/// ones recognised, `Sheet::as_sent`), earliest first: one per source, the
/// first found, so a poison's next phase shows once the one before has gone.
pub(super) fn listed(sheet: &Sheet) -> Vec<Effect> {
    let mut later: Vec<&Effect> = sheet.later.0.iter().map(|p| &p.effect).collect();
    later.sort_by_key(|effect| effect.start);
    let mut listed: Vec<Effect> = Vec::new();
    for effect in sheet.effects.0.iter().chain(later) {
        if listed.iter().all(|e| e.source != effect.source) {
            listed.push(effect.clone());
        }
    }
    listed
}

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
        less || matches!(bonus.stat, Stat::Drain(_) | Stat::DeadlyDrain(_))
    })
}

/// The colour behind its icon.
pub(super) fn colour(effect: &Effect) -> Color {
    if is_harmful(effect) { HARMFUL } else { HELPFUL }
}

/// Where its icon is, in the assets: every food shares one.
pub(super) fn icon_path(source: Source) -> &'static str {
    match source {
        Source::Eaten(_)
        | Source::EatenFruit(_)
        | Source::EatenVegetable(_)
        | Source::EatenFungus(_) => "icons/delapouite/eating.png",
        Source::FruitEffects(_) => "icons/lorc/shiny-apple.png",
        Source::VegetableEffects(_) => "icons/delapouite/carrot.png",
        Source::FungusEffects(_) | Source::FungusWindow(_) => "icons/lorc/mushroom.png",
        Source::FungusPoison(_) => "icons/lorc/spotted-mushroom.png",
        Source::Poison => "icons/lorc/poison-bottle.png",
        Source::Dev => "icons/lorc/magic-swirl.png",
        Source::RawHerb => "icons/delapouite/herbs-bundle.png",
        Source::Ointment => "icons/delapouite/covered-jar.png",
        Source::Potion(shared::herb::Stat::Darkvision) => "icons/delapouite/night-vision.png",
        Source::Potion(_) => "icons/lorc/fizzing-flask.png",
        Source::Juice(_) | Source::Cordial(_) | Source::Drink => "icons/delapouite/drinking.png",
        Source::Alcohol(stage) => match stage {
            Stage::Tipsy => "icons/lorc/wine-glass.png",
            Stage::Drunk => "icons/lorc/beer-stein.png",
            Stage::VeryDrunk => "icons/delapouite/beer-bottle.png",
            Stage::Collapsed => "icons/delapouite/knocked-out-stars.png",
            Stage::Poisoned => "icons/lorc/vomiting.png",
            Stage::Hangover => "icons/lorc/brain-freeze.png",
        },
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
