//! `LiquidLook`: what a potion's liquid looks like, worked out from its contents
//! alone (owner, 2026-11-17), never the bottle's seed or quality, so the same
//! effects look the same in any bottle and at any rarity.

use shared::herb::Stat;
use shared::potion::{Contents, Dose};
use shared::rng::Seed;

/// The most the steps added up (helpful minus harmful) count for lightness.
pub(super) const MOST_NET: i16 = 12;
/// The most all the steps together count for how see-through it is.
pub(super) const MOST_STRENGTH: i16 = 24;

/// A potion's liquid. Equal contents always give an equal look.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct LiquidLook {
    /// The strongest effect's stat (ties by `Stat`'s order): the main colour.
    pub main: Stat,
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
    /// Mana or Aura: sparkles.
    pub sparkles: bool,
    /// Intellect: a swirl of the second colour.
    pub swirl: bool,
    /// Toughness: still and layered.
    pub layered: bool,
}

impl LiquidLook {
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
            main: main.stat,
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
            self.main as i64,
            stat(self.second),
            stat(self.blobs),
            self.net as i64,
            self.strength as i64,
            flags,
        ]);
        Seed::new(value)
    }
}

/// The dose moving its stat furthest, the first in `Stat`'s order on a tie.
fn strongest<'a>(doses: impl Iterator<Item = &'a Dose>) -> Option<&'a Dose> {
    doses.fold(None, |best: Option<&Dose>, dose| match best {
        Some(best) if best.step.abs() >= dose.step.abs() => Some(best),
        _ => Some(dose),
    })
}
