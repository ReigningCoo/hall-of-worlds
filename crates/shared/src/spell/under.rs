//! What PR 13's spells lying on a character change, read from their sheet
//! (effects that have ended are forgotten every tick on the server, and the
//! client gets the sheet again, so being there is enough): the light they
//! carry, floating, and teleporting.

use crate::bonus::{Source, Totals};
use crate::item::Inventory;
use crate::sheet::Sheet;

use super::part::Part;
use super::utility::{ANCHORED, DARKENED, LEVITATING, TELEPORTING};

impl Sheet {
    /// Whether a spell's `part` lies on it.
    pub fn has_spell(&self, part: Part) -> bool {
        (self.effects.0.iter()).any(|e| e.source == Source::Spell(part))
    }

    /// How far its light reaches, in cells, carrying `carrying`: a lit
    /// lantern's or a Light's, whichever is further, and gear's on top; none while Darkness lies
    /// on it, which swallows a lantern's too.
    pub fn light_reach(&self, carrying: &Inventory) -> u32 {
        if self.has_spell(DARKENED) {
            return 0;
        }
        let spells = self
            .effects
            .0
            .iter()
            .filter(|e| matches!(e.source, Source::Spell(_)));
        let light = Totals::of(spells.flat_map(|e| &e.bonuses)).light();
        // Gear's light reaches further than whichever light it has.
        carrying.light_reach().max(light) + carrying.worn_totals().light()
    }

    /// Whether it floats over holes and water (Levitate).
    pub fn levitating(&self) -> bool {
        self.has_spell(LEVITATING)
    }

    /// Whether its next move may go at once to a cell it chooses: a Teleport
    /// lies on it and nothing holds it.
    pub fn may_teleport(&self) -> bool {
        self.has_spell(TELEPORTING) && !self.has_spell(ANCHORED)
    }
}
