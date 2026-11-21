//! `DishDef`: one real dish as a pattern, a row of `dishes.ron`: its tool
//! steps, how wet it is, the roles it needs and must not have, and the foods
//! it is made of. What goes into a pattern: `Wanted`, `Wetness`, `DishForm`.

use serde::{Deserialize, Serialize};

use super::{Role, Source, Tool};

/// A food a dish asks for: any of a source (any egg, any milk), or one by
/// its key.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Wanted {
    Any(Source),
    One(Source, String),
}

impl Wanted {
    /// Whether the food with this source and key is it.
    pub fn matches(&self, source: Source, key: &str) -> bool {
        match self {
            Self::Any(any) => *any == source,
            Self::One(one, wanted) => *one == source && wanted == key,
        }
    }
}

/// How wet a dish is: the share of it that is liquid (`Wetness::of`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Wetness {
    /// Under 5%.
    Dry,
    /// Under 25%.
    Moist,
    /// Under 50%.
    Wet,
    /// Half or more: a soup.
    Soupy,
}

impl Wetness {
    /// The wetness of `liquid` in `total`, both in grams (a millilitre of
    /// water a gram).
    pub fn of(liquid: u32, total: u32) -> Self {
        let percent = u64::from(liquid) * 100 / u64::from(total.max(1));
        match percent {
            0..5 => Self::Dry,
            5..25 => Self::Moist,
            25..50 => Self::Wet,
            _ => Self::Soupy,
        }
    }

    /// How many bands apart two wetnesses are.
    pub fn apart(self, other: Self) -> u32 {
        (self as i32 - other as i32).unsigned_abs()
    }
}

/// What a dish is served as: the shape of its item and model (plan PRs 6
/// and 11).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum DishForm {
    Bowl,
    Plate,
    Jar,
    Pudding,
    Sauce,
    EggDish,
    Flatbread,
    Noodles,
    Dumplings,
    Cake,
    Pie,
    Paste,
    Loaf,
    Ball,
    Block,
    Strips,
    Sausage,
    Pat,
    Other,
}

/// One real dish (`dishes.ron`). Saves and messages name it by `key`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DishDef {
    pub key: String,
    pub name: String,
    pub cuisine: String,
    /// How well known it is, 1 to 5: the better known wins a tie.
    pub known: u8,
    pub form: DishForm,
    /// Its tool steps, the last one last (rests left out).
    pub tools: Vec<Tool>,
    /// How many of them join rows.
    #[serde(default)]
    pub joins: u8,
    pub liquid: Wetness,
    /// The roles it is made of.
    pub needs: Vec<Role>,
    /// Whether it needs a liquid in it (a soup's, a sauce's).
    #[serde(default)]
    pub wants_liquid: bool,
    /// Roles it must not have (beyond seasonings' leeway).
    #[serde(default)]
    pub forbids: Vec<Role>,
    /// The foods it must have, each group any of its foods.
    #[serde(default)]
    pub specific: Vec<Vec<Wanted>>,
    /// Foods it usually has: each one present (or missing) makes it likelier
    /// (or less likely) than a dish of the same shape.
    #[serde(default)]
    pub with: Vec<Vec<Wanted>>,
    /// Made to go into other dishes (stock, butter, dough).
    #[serde(default)]
    pub intermediate: bool,
}

impl DishDef {
    /// Whether the row makes sense.
    pub fn check(&self) -> Result<(), String> {
        let name = &self.key;
        let plain = |c: char| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_';
        if name.is_empty() || !name.chars().all(plain) || self.name.trim().is_empty() {
            return Err(format!("dish {name:?}: its key or name"));
        }
        if !(1..=5).contains(&self.known) {
            return Err(format!("{name}: known {}", self.known));
        }
        if self.tools.is_empty() || usize::from(self.joins) > self.tools.len() {
            return Err(format!("{name}: its tools and joins"));
        }
        if self.needs.iter().any(|role| self.forbids.contains(role)) {
            return Err(format!("{name}: needs a role it forbids"));
        }
        if self.specific.iter().chain(&self.with).any(Vec::is_empty) {
            return Err(format!("{name}: an empty group of foods"));
        }
        Ok(())
    }

    /// The tool its last step is.
    pub fn last(&self) -> Option<Tool> {
        self.tools.last().copied()
    }
}
