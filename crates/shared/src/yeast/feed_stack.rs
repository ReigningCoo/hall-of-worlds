//! Feeding a stack of yeast (owner: it keeps feeding until every yeast has
//! fed, the ones most in need first): which yeasts the flour, water and empty
//! bottles carried go round, and feeding them. Each feed takes a sack of flour
//! and 100 ml of fresh water; a split takes them twice and an empty bottle for
//! the second yeast (Claude's first guesses).

use crate::id::Id;
use crate::item::{Inventory, Item};

use super::feed::{Fed, Window, feed};

/// What is carried to feed with: sacks of flour in the stack chosen, 100 ml
/// portions of fresh water, and empty bottles.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Supplies {
    pub flour: u32,
    pub water: u32,
    pub bottles: u32,
}

/// Why nothing in a stack could be fed.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FeedRefused {
    /// Not a stack of yeast, or not a stack of flour carried.
    NotYeast,
    NotFlour,
    /// Every one a starter still developing, or split and not yet fed on time.
    NothingToFeed,
    NoFlour,
    NoWater,
    /// A split (fed under 12 hours) needs an empty bottle for the new yeast.
    NoBottle,
}

impl FeedRefused {
    /// Why, in English, for logs and the client's red line.
    pub fn words(self) -> &'static str {
        match self {
            Self::NotYeast => "only yeast is fed with flour",
            Self::NotFlour => "yeast is fed with flour",
            Self::NothingToFeed => "it can't be fed yet",
            Self::NoFlour => "there isn't enough flour",
            Self::NoWater => "feeding needs fresh water carried",
            Self::NoBottle => "fed this early it splits: carry an empty bottle",
        }
    }
}

/// The yeasts a feed reaches, neediest first, with their windows, and what
/// they take of the supplies.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct FeedPlan {
    pub yeasts: Vec<(Id, Window)>,
    pub took: Supplies,
}

/// What feeding `stack` with `supplies` would reach: every yeast that can be
/// fed, the longest unfed first, while the supplies last.
pub fn plan_feeding(stack: &Item, supplies: Supplies) -> Result<FeedPlan, FeedRefused> {
    let mut yeasts: Vec<&Item> = (stack.members().rev())
        .filter(|one| one.yeast().is_some())
        .collect();
    if yeasts.is_empty() {
        return Err(FeedRefused::NotYeast);
    }
    yeasts.sort_by_key(|one| std::cmp::Reverse(one.yeast().map_or(0, |yeast| yeast.unfed)));
    let mut plan = FeedPlan::default();
    let mut short = FeedRefused::NothingToFeed;
    for one in yeasts {
        let Some(yeast) = one.yeast().filter(|yeast| yeast.feedable()) else {
            continue;
        };
        let window = yeast.window();
        let (each, bottles) = if window == Window::Early {
            (2, 1)
        } else {
            (1, 0)
        };
        let took = plan.took;
        let missing = [
            (took.flour + each > supplies.flour, FeedRefused::NoFlour),
            (took.water + each > supplies.water, FeedRefused::NoWater),
            (
                took.bottles + bottles > supplies.bottles,
                FeedRefused::NoBottle,
            ),
        ];
        if let Some(&(_, why)) = missing.iter().find(|(lacking, _)| *lacking) {
            if short == FeedRefused::NothingToFeed {
                short = why;
            }
            continue;
        }
        plan.took = Supplies {
            flour: took.flour + each,
            water: took.water + each,
            bottles: took.bottles + bottles,
        };
        plan.yeasts.push((one.id, window));
    }
    match plan.yeasts.is_empty() {
        true => Err(short),
        false => Ok(plan),
    }
}

impl Inventory {
    /// Feeds the yeasts `plan` reaches in the carried stack `stack`, by a
    /// feeder with `fermenting` (`Skills::total`), `roll` giving 0 to 99 for
    /// each on-time feed: the second yeasts of splits (to be given ids and put
    /// with the stack), and how many stepped up.
    pub fn feed_yeasts(
        &mut self,
        stack: Id,
        plan: &FeedPlan,
        fermenting: u32,
        roll: &mut impl FnMut() -> u32,
    ) -> (Vec<Item>, u32) {
        let (mut splits, mut stepped) = (Vec::new(), 0);
        let Some(found) = self.find_mut(stack) else {
            return (splits, stepped);
        };
        for &(id, _) in &plan.yeasts {
            let one = match found.id == id {
                true => &mut *found,
                false => match found.above.iter_mut().find(|one| one.id == id) {
                    Some(one) => one,
                    None => continue,
                },
            };
            match feed(one, fermenting, roll()) {
                Some(Fed::Split) => splits.push(Item {
                    above: Vec::new(),
                    ..one.clone()
                }),
                Some(Fed::OnTime { stepped_up: true }) => stepped += 1,
                _ => {}
            }
        }
        (splits, stepped)
    }
}
