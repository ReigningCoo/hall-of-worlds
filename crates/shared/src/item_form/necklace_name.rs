//! A necklace's name: "Silver Chain", "Leather Cord with an Iron Leaf", "Gold
//! Pendant with a Ruby", "Silver Amulet with Three Sapphires", "Bronze Torc".

use crate::item_form::necklace::{CordStyle, NecklaceForm, Ornament, Strand, Thickness};
use crate::item_form::words::with_stones;

impl NecklaceForm {
    /// Named for what hangs from it if anything does, else for its strand; then what
    /// is in its settings, as it is now.
    pub(super) fn name(&self) -> String {
        let metal = self.metal.name();
        match self.ornament {
            Ornament::Pendant => with_stones(format!("{metal} Pendant"), &self.settings),
            Ornament::Amulet(_) => with_stones(format!("{metal} Amulet"), &self.settings),
            Ornament::Stations => with_stones(self.strand_name(), &self.settings),
            Ornament::Charm(charm) => match self.strand {
                // A cord isn't metal, so the charm's metal is named.
                Strand::Cord { .. } => {
                    let charm = format!("{metal} {}", charm.name());
                    format!("{} with {} {charm}", self.strand_name(), a(&charm))
                }
                // A chain's or torc's charm is of its metal, already named.
                _ => format!(
                    "{} with {} {}",
                    self.strand_name(),
                    a(charm.name()),
                    charm.name()
                ),
            },
            Ornament::None => self.strand_name(),
        }
    }

    /// "Braided Suede Cord", "Fine Gold Chain", "Bronze Torc".
    fn strand_name(&self) -> String {
        let metal = self.metal.name();
        match self.strand {
            Strand::Cord { leather, style } => match style {
                CordStyle::Plain => format!("{} Cord", leather.name()),
                CordStyle::Braided => format!("Braided {} Cord", leather.name()),
                CordStyle::Knotted => format!("Knotted {} Cord", leather.name()),
            },
            Strand::Chain { thickness, .. } => match thickness {
                Thickness::Fine => format!("Fine {metal} Chain"),
                Thickness::Medium => format!("{metal} Chain"),
                Thickness::Heavy => format!("Heavy {metal} Chain"),
            },
            Strand::Torc { .. } => format!("{metal} Torc"),
        }
    }
}

fn a(word: &str) -> &'static str {
    match word.starts_with(['A', 'E', 'I', 'O', 'U']) {
        true => "an",
        false => "a",
    }
}
