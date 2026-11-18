//! The mixing panel's words: a herb by name as far as the character knows it,
//! doses ("+3 Strength, −1 Will") and what a choice would make.

use shared::brewing::{BrewSkills, GrindRefused, HerbStack, Ointment, Recipe, Refused, most_kinds};
use shared::herb::{HerbKnowledge, Herbs, Stat};
use shared::potion::Contents;

/// What a herb is called for the character: its name once known.
pub(super) fn herb_name(key: &str, herbs: &Herbs, known: &HerbKnowledge) -> String {
    match herbs.get(key) {
        Some(def) if known.knows_name(key) => def.name.clone(),
        _ => "Unknown herb".to_owned(),
    }
}

/// A herb stack's row: "Yarrow, Common (12)".
pub(super) fn herb_row(stack: &HerbStack, herbs: &Herbs, known: &HerbKnowledge) -> String {
    let name = herb_name(&stack.herb, herbs, known);
    format!("{name}, {:?} ({})", stack.quality, stack.count)
}

/// A stat's name: "Strength", "Health".
fn stat_name(stat: Stat) -> &'static str {
    match stat {
        Stat::Health => "Health",
        Stat::Mana => "Mana",
        stat => stat.attribute().map_or("?", |attribute| attribute.name()),
    }
}

/// Doses in words: "+3 Strength, −1 Will", or "nothing".
pub(in crate::abilities) fn doses(contents: &Contents) -> String {
    let words: Vec<String> = contents
        .doses()
        .iter()
        .map(|dose| {
            let sign = if dose.step < 0 { "\u{2212}" } else { "+" };
            format!(
                "{sign}{} {}",
                dose.step.unsigned_abs(),
                stat_name(dose.stat)
            )
        })
        .collect();
    match words.is_empty() {
        true => "nothing".to_owned(),
        false => words.join(", "),
    }
}

/// A recipe's row: "Yarrow, Sage with Nut Oil: +3 Strength", and what it last
/// brewed into ("; with Moon Water, +4 Strength").
pub(super) fn recipe_row(recipe: &Recipe, herbs: &Herbs, known: &HerbKnowledge) -> String {
    let names: Vec<String> = (recipe.herbs.iter())
        .map(|key| herb_name(key, herbs, known))
        .collect();
    let made = doses(&recipe.made);
    let brewed = match &recipe.brewed {
        Some(brewed) => {
            let potion = brewed.potion.as_ref().map_or("no potion".to_owned(), doses);
            format!("; with {}, {potion}", brewed.water.name())
        }
        None => String::new(),
    };
    let oil = recipe.oil.name();
    format!("{} with {oil}: {made}{brewed}", names.join(", "))
}

/// What the choice would make, or why it can't be ground. `foreseen` is
/// whether the character knows every effect of the herbs ticked: otherwise
/// what it would hold isn't told.
pub(super) fn makes(
    made: &Result<Ointment, GrindRefused>,
    foreseen: bool,
    skills: BrewSkills,
) -> String {
    match made {
        Ok(ointment) if !foreseen => format!(
            "Makes {} Ointment, of what you can't yet tell.",
            ointment.doses
        ),
        Ok(ointment) if ointment.contents.is_empty() => {
            "These cancel each other out: it would come to nothing.".to_owned()
        }
        Ok(ointment) => format!(
            "Makes {} Ointment: {}.",
            ointment.doses,
            doses(&ointment.contents)
        ),
        Err(GrindRefused::Rules(Refused::NoHerbs)) => "Tick at least one herb.".to_owned(),
        Err(GrindRefused::Rules(Refused::TooManyKinds)) => format!(
            "You can grind at most {} kinds of herb together.",
            most_kinds(skills)
        ),
        Err(GrindRefused::Oil) => "It needs a bottle of oil.".to_owned(),
        Err(_) => "Not enough of each herb for so many doses.".to_owned(),
    }
}
