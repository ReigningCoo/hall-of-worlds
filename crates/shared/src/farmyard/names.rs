//! What milk, eggs and honey are called, by their animal's key. English for
//! now, as liquids' names are.

use crate::livestock::Livestock;

/// "Cow's Milk", "Mare's Milk", "Camel Milk"; "{Animal} Milk" for one not
/// named here, "Milk" for an animal the table lacks.
pub fn milk_name(animal: &str) -> String {
    let name = match animal {
        "cattle" => "Cow's Milk",
        "highland_cattle" => "Highland Cow's Milk",
        "water_buffalo" => "Buffalo Milk",
        "sheep" => "Ewe's Milk",
        "goat" => "Goat's Milk",
        "horse" => "Mare's Milk",
        "dromedary" => "Camel Milk",
        "bactrian_camel" => "Bactrian Camel Milk",
        _ => return of_animal(animal, "Milk"),
    };
    name.to_owned()
}

/// "Hen's Egg", "Duck Egg"; "{Animal} Egg" for one not named here, "Egg"
/// for an animal the table lacks.
pub fn egg_name(animal: &str) -> String {
    match animal {
        "chicken" => "Hen's Egg".to_owned(),
        _ => of_animal(animal, "Egg"),
    }
}

/// "Honey" (from honeybees); "{Animal} Honey" from anything else.
pub fn honey_name(animal: &str) -> String {
    match animal {
        "honeybee" => "Honey".to_owned(),
        _ => of_animal(animal, "Honey"),
    }
}

/// "{Animal} {what}", each word of the animal's name capitalised.
fn of_animal(animal: &str, what: &str) -> String {
    let Some(def) = Livestock::built_in().get(animal) else {
        return what.to_owned();
    };
    let words: Vec<String> = def.name.split(' ').map(capital).collect();
    format!("{} {what}", words.join(" "))
}

fn capital(word: &str) -> String {
    let mut chars = word.chars();
    match chars.next() {
        Some(first) => first.to_uppercase().chain(chars).collect(),
        None => String::new(),
    }
}
