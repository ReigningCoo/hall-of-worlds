//! `LiquidLook`: what a bottle's liquid looks like, worked out from the liquid
//! alone (owner, 2026-11-17), never the bottle's seed or quality, so the same
//! effects look the same in any bottle and at any rarity. Waters and oils have
//! their own colours (brewing plan, PR 1), and juices their fruit's (drinks
//! plan, PR 3).

use shared::fruit::Fruits;
use shared::herb::{Rgb, Stat};
use shared::potion::{BottleForm, Contents, Dose, Liquid, Oil, Water};
use shared::rng::Seed;

/// Milk's colour in a bottle.
const MILK: Rgb = Rgb(246, 242, 228);
/// Honey's.
const HONEY: Rgb = Rgb(222, 150, 30);

/// The most the steps added up (helpful minus harmful) count for lightness.
pub(super) const MOST_NET: i16 = 12;
/// The most all the steps together count for how see-through it is.
pub(super) const MOST_STRENGTH: i16 = 24;

/// Where a liquid's main colour comes from.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Base {
    /// A potion's strongest effect's stat (ties by `Stat`'s order).
    Stat(Stat),
    Water(Water),
    Oil(Oil),
    /// A juice (or, `cordial`, juice and water): its fruit's ripe colour.
    Juice {
        colour: Rgb,
        cordial: bool,
    },
}

impl Base {
    /// A number for it, for the seed.
    fn number(self) -> i64 {
        match self {
            Self::Stat(stat) => stat as i64,
            Self::Water(water) => 100 + water as i64,
            Self::Oil(oil) => 200 + oil as i64,
            Self::Juice { colour, cordial } => {
                let Rgb(r, g, b) = colour;
                let packed = (i64::from(r) << 16) | (i64::from(g) << 8) | i64::from(b);
                300 + packed * 2 + i64::from(cordial)
            }
        }
    }
}

/// A bottle's liquid. Equal liquids always give an equal look.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct LiquidLook {
    /// The main colour: a potion's strongest stat's, or a water's or an oil's.
    pub main: Base,
    /// The second strongest's: swirls and layers.
    pub second: Option<Stat>,
    /// The strongest effect of the other kind (harmful if the main one helps,
    /// or helpful if it harms), when both are in it: lava-lamp blobs.
    pub blobs: Option<Stat>,
    /// Steps added up, helpful minus harmful, up to `MOST_NET` either way:
    /// paler and brighter the more it helps, deeper and darker the more it harms.
    pub net: i16,
    /// All steps added up whichever way, up to `MOST_STRENGTH`: the stronger,
    /// the less see-through.
    pub strength: i16,
    /// Agility: fine fizz.
    pub fizz: bool,
    /// Strength: big bubbles.
    pub bubbles: bool,
    /// Mana or Aura, or Moon Water: sparkles.
    pub sparkles: bool,
    /// Intellect: a swirl of the second colour.
    pub swirl: bool,
    /// Toughness: still and layered.
    pub layered: bool,
}

impl LiquidLook {
    /// The liquid in this bottle, or none if it is empty.
    pub fn of_bottle(bottle: &BottleForm) -> Option<Self> {
        let plain = |main| Self {
            main,
            second: None,
            blobs: None,
            net: 0,
            strength: 0,
            fizz: false,
            bubbles: false,
            sparkles: main == Base::Water(Water::Moon),
            swirl: false,
            layered: false,
        };
        match bottle.liquid.as_ref()? {
            Liquid::Potion(contents) => Self::of(contents),
            Liquid::Water(water) => Some(plain(Base::Water(*water))),
            Liquid::Oil(oil) => Some(plain(Base::Oil(*oil))),
            Liquid::Juice(fruit) => Some(plain(juice(fruit, false))),
            Liquid::Cordial(fruit) => Some(plain(juice(fruit, true))),
            Liquid::Drink(drink) => Some(plain(brewed(&drink.base, false))),
            Liquid::Vinegar(base) => Some(plain(brewed(base, true))),
            // As clear as water.
            Liquid::Alcohol => Some(plain(Base::Water(Water::Plain))),
            // Drawn as a juice is, in their own colours: creamy milk, amber
            // honey (cooking plan PR 3).
            Liquid::Milk(_) => Some(plain(Base::Juice {
                colour: MILK,
                cordial: false,
            })),
            Liquid::Honey(_) => Some(plain(Base::Juice {
                colour: HONEY,
                cordial: false,
            })),
        }
    }

    /// The liquid in a bottle holding these, or none if it is empty.
    pub fn of(contents: &Contents) -> Option<Self> {
        let doses = contents.doses();
        let main = strongest(doses.iter())?;
        let second = strongest(doses.iter().filter(|dose| dose.stat != main.stat));
        let helps = main.step > 0;
        let blobs = strongest(doses.iter().filter(|dose| (dose.step > 0) != helps));
        let has = |stat: Stat| doses.iter().any(|dose| dose.stat == stat);
        let net: i32 = doses.iter().map(|dose| dose.step as i32).sum();
        let strength: i32 = doses.iter().map(|dose| (dose.step as i32).abs()).sum();
        Some(Self {
            main: Base::Stat(main.stat),
            second: second.map(|dose| dose.stat),
            blobs: blobs.map(|dose| dose.stat),
            net: net.clamp(-MOST_NET as i32, MOST_NET as i32) as i16,
            strength: strength.min(MOST_STRENGTH as i32) as i16,
            fizz: has(Stat::Agility),
            bubbles: has(Stat::Strength),
            sparkles: has(Stat::Mana) || has(Stat::Aura),
            swirl: has(Stat::Intellect),
            layered: has(Stat::Toughness),
        })
    }

    /// Where its bubbles, fizz and the like go, from the look alone.
    pub(super) fn seed(&self) -> Seed {
        let stat = |stat: Option<Stat>| stat.map_or(-1, |stat| stat as i64);
        let flags = [
            self.fizz,
            self.bubbles,
            self.sparkles,
            self.swirl,
            self.layered,
        ]
        .iter()
        .fold(0, |bits, &on| bits * 2 + on as i64);
        let value = Seed::new(0).derive("liquid").at(&[
            self.main.number(),
            stat(self.second),
            stat(self.blobs),
            self.net as i64,
            self.strength as i64,
            flags,
        ]);
        Seed::new(value)
    }
}

/// A juice's or cordial's base: its fruit's ripe colour (an orange's for a
/// fruit the table lacks).
fn juice(fruit: &str, cordial: bool) -> Base {
    let colour = Fruits::built_in()
        .get(fruit)
        .map_or(Rgb(240, 150, 40), |def| def.look.ripe);
    Base::Juice { colour, cordial }
}

/// A fermented drink: its fruit's colour, or amber for ale; vinegar paler
/// (drawn as a cordial is).
fn brewed(base: &shared::fermentation::Base, vinegar: bool) -> Base {
    match base {
        shared::fermentation::Base::Fruit(fruit) => juice(fruit, vinegar),
        shared::fermentation::Base::Grain(_) => Base::Juice {
            colour: Rgb(200, 130, 40),
            cordial: vinegar,
        },
    }
}

/// The dose moving its stat furthest, the first in `Stat`'s order on a tie.
fn strongest<'a>(doses: impl Iterator<Item = &'a Dose>) -> Option<&'a Dose> {
    doses.fold(None, |best: Option<&Dose>, dose| match best {
        Some(best) if best.step.abs() >= dose.step.abs() => Some(best),
        _ => Some(dose),
    })
}
