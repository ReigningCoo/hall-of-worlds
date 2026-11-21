//! Which kept animals give milk, eggs or honey, and the one Give Item rolls.

use crate::livestock::{Livestock, LivestockDef, Product};
use crate::potion::Liquid;
use crate::rng::{Rng, Seed};

/// Every animal giving `product`, in the table's order.
pub fn givers(product: Product) -> impl Iterator<Item = &'static LivestockDef> {
    (Livestock::built_in().iter()).filter(move |def| def.gives(product))
}

/// The animal giving `product` with this key, or with none, one rolled from
/// `seed`, any alike: Give Item's, chosen the same way on both sides. `None`
/// for no such animal.
pub fn dev_animal(
    product: Product,
    key: Option<&str>,
    seed: Seed,
) -> Option<&'static LivestockDef> {
    if let Some(key) = key {
        return givers(product).find(|def| def.key == key);
    }
    pick(product, &mut seed.derive("dev farmyard").rng())
}

/// Some animal's milk, rolled, as Give Item's bottles, vessels and casks hold.
pub fn rolled_milk(rng: &mut Rng) -> Liquid {
    let def = pick(Product::Milk, rng).expect("the built-in milk animals");
    Liquid::Milk(def.key.clone())
}

/// Honey, its animal rolled (only honeybees so far).
pub fn rolled_honey(rng: &mut Rng) -> Liquid {
    let def = pick(Product::Honey, rng).expect("the built-in honey animals");
    Liquid::Honey(def.key.clone())
}

fn pick(product: Product, rng: &mut Rng) -> Option<&'static LivestockDef> {
    let count = givers(product).count() as u64;
    if count == 0 {
        return None;
    }
    givers(product).nth(rng.range_u64(0, count) as usize)
}

impl LivestockDef {
    /// Whether it gives this product.
    pub fn gives(&self, product: Product) -> bool {
        self.produce
            .iter()
            .any(|produce| produce.product == product)
    }

    /// What one of its eggs weighs, in grams; 0 if it lays none.
    pub fn egg_grams(&self) -> u32 {
        (self.produce.iter())
            .find(|produce| produce.product == Product::Egg)
            .map_or(0, |produce| produce.grams)
    }
}
