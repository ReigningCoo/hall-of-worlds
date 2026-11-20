//! What the wild forms of kept animals look like where their own looks are
//! kept for them: an aurochs (wild cattle) has an aurochs's coat, bulls an
//! eel stripe, cows plain. Kept and feral cattle never roll these
//! (`looks::WEIGHTS`).

use crate::creature::CreatureKind;
use crate::gender::Gender;
use crate::sheet::{Appearance, Trait};

/// A wild form's looks: its own choices set over what was rolled.
pub(super) fn wild_looks(kind: CreatureKind, gender: Gender, rolled: Appearance) -> Appearance {
    let mut looks = rolled;
    if kind == CreatureKind::Cattle {
        let markings = if gender == Gender::Male {
            "Eel stripe"
        } else {
            "Plain"
        };
        set(&mut looks, kind, gender, Trait::CoatColour, "Aurochs");
        set(&mut looks, kind, gender, Trait::Markings, markings);
    }
    looks
}

/// Sets a trait to the choice of this name that suits the gender, if there is one.
fn set(looks: &mut Appearance, kind: CreatureKind, gender: Gender, which: Trait, name: &str) {
    let choices = which.choices_for(kind);
    let found = choices
        .iter()
        .position(|choice| choice.name == name && choice.suits(gender));
    if let Some(place) = found {
        looks.0.insert(which, place as u8);
    }
}
