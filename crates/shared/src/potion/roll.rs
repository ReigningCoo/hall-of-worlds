//! Contents rolled from a seed, and the bottles asked for: for Give Item (dev
//! tools) until brewing makes potions. Both sides roll the same from the same
//! seed.

use super::{BottleForm, Contents, Dose, Liquid, LiquidKind};
use crate::herb::Stat;
use crate::item_form::{Field, Refused, RollRequest};
use crate::juice::juice_fruit;
use crate::rng::Rng;

/// The most effects Give Item can ask a potion to have.
pub const MOST_EFFECTS: u8 = 4;

/// `effects` doses (at most `MOST_EFFECTS`) on different stats, as brewing's
/// rule would leave them (`research-herbs.md`): the first 4 to 8 steps, the
/// rest 2 to 6, each helpful two times in three.
pub fn roll_contents(rng: &mut Rng, effects: u8) -> Contents {
    let mut stats = Stat::ALL.to_vec();
    let mut doses = Vec::new();
    for n in 0..effects.min(MOST_EFFECTS) {
        let at = rng.range_u64(0, stats.len() as u64) as usize;
        let stat = stats.remove(at);
        let least = if n == 0 { 4 } else { 2 };
        let size = rng.range_u64(least, least + 5) as i16;
        let helpful = rng.range_u64(0, 3) < 2;
        let step = if helpful { size } else { -size };
        doses.push(Dose { stat, step });
    }
    Contents::new(doses)
}

/// The bottle Give Item asks for (`RollRequest::liquid` and `effects`): with
/// no liquid asked for, a potion of that many effects, or empty for none; a
/// potion asked for has 1 to `MOST_EFFECTS` unless told how many (never 0);
/// a water, an oil, a juice or a cordial (its fruit rolled) takes no effects.
pub fn bottle_to(rng: &mut Rng, request: &RollRequest, look: u64) -> Result<BottleForm, Refused> {
    let refused = Err(Refused(Field::Effects));
    let liquid = match (request.liquid, request.effects) {
        (_, Some(effects)) if effects > MOST_EFFECTS => return refused,
        (None, effects) => Some(Liquid::Potion(roll_contents(rng, effects.unwrap_or(0)))),
        (Some(LiquidKind::Potion), Some(0)) => return refused,
        (Some(LiquidKind::Potion), effects) => {
            let effects =
                effects.unwrap_or_else(|| rng.range_u64(1, MOST_EFFECTS as u64 + 1) as u8);
            Some(Liquid::Potion(roll_contents(rng, effects)))
        }
        (Some(_), Some(effects)) if effects > 0 => return refused,
        (Some(LiquidKind::Water(water)), _) => Some(Liquid::Water(water)),
        (Some(LiquidKind::Oil(oil)), _) => Some(Liquid::Oil(oil)),
        (Some(LiquidKind::Juice), _) => Some(Liquid::Juice(juice_fruit(rng))),
        (Some(LiquidKind::Cordial), _) => Some(Liquid::Cordial(juice_fruit(rng))),
        (Some(LiquidKind::Drink), _) => Some(crate::fermentation::rolled_drink(rng)),
        (Some(LiquidKind::Vinegar), _) => Some(crate::fermentation::rolled_vinegar(rng)),
        (Some(LiquidKind::Alcohol), _) => Some(Liquid::Alcohol),
    };
    // An empty potion is an empty bottle.
    Ok(BottleForm::new(liquid, look))
}
