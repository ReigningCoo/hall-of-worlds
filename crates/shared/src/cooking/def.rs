//! `IngredientDef`: one raw food's culinary data, a row of `ingredients.ron`.

use serde::{Deserialize, Serialize};

use super::{CookingNeed, Flavour, Function, Method, Role, Source, Texture, Usage};

/// The strongest an ingredient's flavour can be (1 is faint).
pub const MAX_STRENGTH: u8 = 5;

/// What one raw food is to a cook. Diet flags are false unless written.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct IngredientDef {
    pub source: Source,
    /// Its key in the source's table (`Herbs`, `Fruits`, `Livestock`, ...).
    pub key: String,
    pub usage: Usage,
    /// Every role it can take, the likeliest first; none when inedible.
    pub roles: Vec<Role>,
    /// The role it usually takes, one of `roles`.
    #[serde(default)]
    pub main: Option<Role>,
    pub cooking: CookingNeed,
    pub texture: Texture,
    #[serde(default)]
    pub functions: Vec<Function>,
    #[serde(default)]
    pub methods: Vec<Method>,
    #[serde(default)]
    pub flavours: Vec<Flavour>,
    /// How strongly it tastes, 1 to `MAX_STRENGTH`.
    pub strength: u8,
    /// The most of a dish it should be, in percent by weight: past this a
    /// dish loses balance. 0 for what should never be in one.
    pub max_share: u8,
    /// Flesh of an animal (not vegetarian).
    #[serde(default)]
    pub meat: bool,
    /// From an animal (not vegan); true for all meat.
    #[serde(default)]
    pub animal_product: bool,
    /// Wheat, barley, rye and what is made of them; oats count as free of it
    /// (owner, 2026-11-21).
    #[serde(default)]
    pub gluten: bool,
    /// Milk or made from it (not lactose free).
    #[serde(default)]
    pub dairy: bool,
    /// Counts as a tree nut for allergies, as labels list them: chestnut and
    /// acorn do, coconut does not (owner, 2026-11-21).
    #[serde(default)]
    pub nut_allergen: bool,
}

impl IngredientDef {
    /// Whether the row makes sense.
    pub fn check(&self) -> Result<(), String> {
        let name = format!("{:?} {}", self.source, self.key);
        if self.key.trim().is_empty() {
            return Err(format!("{name}: no key"));
        }
        if !(1..=MAX_STRENGTH).contains(&self.strength) {
            return Err(format!("{name}: strength {}", self.strength));
        }
        if self.max_share > 100 {
            return Err(format!("{name}: max share {}%", self.max_share));
        }
        if self.main.is_some_and(|main| !self.roles.contains(&main)) {
            return Err(format!("{name}: its main role is not one of its roles"));
        }
        if self.meat && !self.animal_product {
            return Err(format!("{name}: meat that is no animal product"));
        }
        Ok(())
    }
}
