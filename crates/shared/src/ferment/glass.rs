//! Drinks ageing sealed in glass (tavern furniture plan PR 2, owner: sealed
//! wine bottles and spirits age wherever they are, raising quality, a little
//! faster on a wine or spirit rack): each sealed wine bottle (`vessel`) and
//! bottle (`potion`) holding a fermented drink keeps a `GlassTime`, on the
//! same two clocks as casks (playtime carried, world time on the ground and on
//! furniture). The rule is `fermentation::glass_aged`.

use serde::{Deserialize, Serialize};

use crate::fermentation::glass_aged;
use crate::item::{Item, Quality};
use crate::item_form::ItemForm;
use crate::potion::Liquid;

/// A sealed drink's time in its glass.
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct GlassTime {
    /// Its quality when it was first aged in this glass: what ageing counts up
    /// from (none until then).
    #[serde(default)]
    pub bottled: Option<Quality>,
    /// Seconds aged sealed, already sped up on a rack.
    #[serde(default)]
    pub aged: u64,
    /// When it was put on the ground or on furniture (seconds since 1970),
    /// while it lies there: its clock then goes by world time.
    #[serde(default)]
    pub lying_since: Option<u64>,
}

impl GlassTime {
    /// Lets `secs` pass for `liquid` in its glass, aged `percent`% as fast,
    /// if it is a sealed drink; anything else keeps no time. Whether its
    /// quality rose.
    pub fn pass(
        &mut self,
        liquid: &mut Option<Liquid>,
        sealed: bool,
        secs: u64,
        percent: u64,
    ) -> bool {
        let Some(Liquid::Drink(drink)) = liquid else {
            return false;
        };
        if !sealed {
            return false;
        }
        let bottled = *self.bottled.get_or_insert(drink.quality);
        self.aged = self.aged.saturating_add(secs.saturating_mul(percent) / 100);
        let now = glass_aged(bottled, drink.base.kind(), self.aged);
        let rose = now != drink.quality;
        drink.quality = now;
        rose
    }
}

/// One glass's liquid, seal and clock, to let time pass on.
pub(super) struct Glass<'a> {
    pub(super) liquid: &'a mut Option<Liquid>,
    pub(super) sealed: bool,
    pub(super) time: &'a mut GlassTime,
}

impl Glass<'_> {
    /// Lets `secs` pass on it (`GlassTime::pass`).
    pub(super) fn pass(&mut self, secs: u64, percent: u64) -> bool {
        self.time.pass(self.liquid, self.sealed, secs, percent)
    }
}

impl Item {
    /// Every bottle and vessel in it: itself, those stacked on it and those
    /// inside it at any depth.
    pub(super) fn each_glass(&mut self, act: &mut impl FnMut(Glass<'_>)) {
        match &mut self.form {
            Some(ItemForm::Bottle(bottle)) => act(Glass {
                liquid: &mut bottle.liquid,
                sealed: bottle.sealed,
                time: &mut bottle.glass,
            }),
            Some(ItemForm::Vessel(vessel)) => act(Glass {
                liquid: &mut vessel.liquid,
                sealed: vessel.sealed,
                time: &mut vessel.glass,
            }),
            _ => {}
        }
        for above in &mut self.above {
            above.each_glass(act);
        }
        for placed in &mut self.contents {
            placed.item.each_glass(act);
        }
    }
}
