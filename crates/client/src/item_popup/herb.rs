//! A herb's popup (owner: knowing a herb only changes what is shown): its name if
//! the character knows it, else "Unknown herb"; the part used ("Root"); and a
//! line for each of its four effects, worded if known ("Slightly raises
//! Toughness"), else "?". A herb the server's table lacks says only that.

use shared::bonus;
use shared::herb::{Effect, HerbKnowledge, Herbs, Stat};
use shared::item::Item;
use shared::lang::{Args, Lang};
use shared::vitals::Vital;

use crate::common::{stat_words, written};

/// What a herb's popup says in place of the usual name, description and bonuses.
#[derive(Default)]
pub(super) struct HerbWords {
    pub(super) name: String,
    pub(super) part: Option<String>,
    pub(super) effects: Vec<String>,
}

/// What to say of this item, if it is a herb: by its top, as stacks are named.
pub(super) fn herb_words(
    item: &Item,
    herbs: &Herbs,
    known: &HerbKnowledge,
    lang: &Lang,
) -> Option<HerbWords> {
    let form = item.top().herb()?;
    let unknown = || written(lang.render("herb-unknown", &Args::new()));
    let Some(def) = herbs.get(&form.herb) else {
        return Some(HerbWords {
            name: unknown(),
            ..HerbWords::default()
        });
    };
    let name = match known.knows_name(&def.key) {
        true => def.name.clone(),
        false => unknown(),
    };
    let effects =
        def.effects
            .iter()
            .enumerate()
            .map(|(n, effect)| match known.knows_effect(&def.key, n) {
                true => effect_words(effect, lang),
                false => written(lang.render("herb-effect-unknown", &Args::new())),
            });
    Some(HerbWords {
        name,
        part: Some(def.part_name.clone()),
        effects: effects.collect(),
    })
}

/// "Slightly raises Toughness", "Greatly lowers Mana".
pub(super) fn effect_words(effect: &Effect, lang: &Lang) -> String {
    let stat = match effect.stat {
        Stat::Health => bonus::Stat::Restore(Vital::Health),
        Stat::Mana => bonus::Stat::Restore(Vital::Mana),
        stat => match stat.attribute() {
            Some(attribute) => bonus::Stat::Attribute(attribute),
            None => return String::new(),
        },
    };
    let way = if effect.raises() { "raises" } else { "lowers" };
    let args = Args::new()
        .text("way", way)
        .count("strength", i64::from(effect.strength()))
        .text("stat", stat_words(stat, lang));
    written(lang.render("herb-effect", &args))
}

#[cfg(test)]
mod tests;
