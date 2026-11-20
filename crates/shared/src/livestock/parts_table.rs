//! `Parts`: the organs, fats and other parts butchering gives, built in from
//! `parts.ron`, each naming animals in the livestock table.

use std::sync::LazyLock;

use serde::{Deserialize, Serialize};

use super::{Animals, Fat, Livestock, LivestockDef, Organ, Part};

/// The parts built into the game, as written.
const BUILT_IN: &str = include_str!("parts.ron");

static BUILT_IN_PARTS: LazyLock<Parts> = LazyLock::new(|| {
    Parts::read(BUILT_IN, Livestock::built_in())
        .unwrap_or_else(|error| panic!("the built-in parts: {error}"))
});

/// Every organ, fat and other part.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct Parts {
    pub organs: Vec<Organ>,
    pub fats: Vec<Fat>,
    pub others: Vec<Part>,
}

impl Parts {
    /// The parts built into the game, shared.
    pub fn built_in() -> &'static Self {
        &BUILT_IN_PARTS
    }

    /// The parts written in a RON file, checked against these animals: keys
    /// used once, named and from animals in the table, chances at most 100%.
    pub fn read(text: &str, animals: &Livestock) -> Result<Self, String> {
        let parts: Self = ron::from_str(text).map_err(|error| error.to_string())?;
        let none: Vec<String> = Vec::new();
        let organs = parts
            .organs
            .iter()
            .map(|o| (&o.key, &o.name, &o.from, &o.except));
        let fats = parts.fats.iter().map(|f| (&f.key, &f.name, &f.from, &none));
        let others = parts
            .others
            .iter()
            .map(|p| (&p.key, &p.name, &p.from, &none));
        let mut keys = Vec::new();
        for (key, name, from, except) in organs.chain(fats).chain(others) {
            if keys.contains(&key) || name.trim().is_empty() || from.is_empty() {
                return Err(format!("{key}: written twice, unnamed or from nothing"));
            }
            keys.push(key);
            let named = from.iter().filter_map(|from| match from {
                Animals::Animal(animal) => Some(animal),
                _ => None,
            });
            if let Some(missing) = named.chain(except).find(|a| animals.get(a).is_none()) {
                return Err(format!("{key}: no animal {missing}"));
            }
        }
        for organ in &parts.organs {
            let wrong = |e: &crate::herb::Effect| !(1..=3).contains(&e.change.unsigned_abs());
            if organ.base == 0 || organ.base > 100 || organ.effects.iter().any(wrong) {
                return Err(format!("{}: its chance or an effect is wrong", organ.key));
            }
        }
        for fat in &parts.fats {
            if let Some(from) = &fat.rendered_from
                && !parts.fats.iter().any(|other| other.key == *from)
            {
                return Err(format!("{}: no fat {from} to render", fat.key));
            }
        }
        Ok(parts)
    }

    /// The organs this animal may give.
    pub fn organs_of<'a>(&'a self, def: &'a LivestockDef) -> impl Iterator<Item = &'a Organ> {
        self.organs
            .iter()
            .filter(move |organ| organ.comes_from(def))
    }

    /// The fats this animal gives.
    pub fn fats_of<'a>(&'a self, def: &'a LivestockDef) -> impl Iterator<Item = &'a Fat> {
        self.fats.iter().filter(move |fat| fat.comes_from(def))
    }

    /// The other parts this animal gives.
    pub fn others_of<'a>(&'a self, def: &'a LivestockDef) -> impl Iterator<Item = &'a Part> {
        self.others.iter().filter(move |part| part.comes_from(def))
    }
}
