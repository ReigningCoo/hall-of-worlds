//! Which abilities a creature knows, and which it can use now.

use super::Ability;
use crate::creature::CreatureKind;
use crate::item::Inventory;
use crate::sheet::Sheet;

/// What every creature knows, whatever its kind.
const EVERYONE_KNOWS: [Ability; 3] = [Ability::Attack, Ability::Magic, Ability::Sneak];

/// The abilities a creature in a body of kind `kind` knows by that body alone,
/// in `Ability::ALL`'s order: Attack, Magic, Sneak, and its kind's own.
/// Shapeshifted, the form's.
pub fn known_by_kind(kind: CreatureKind) -> Vec<Ability> {
    Ability::ALL
        .into_iter()
        .filter(|ability| by_kind(*ability, kind))
        .collect()
}

/// Everything a creature in a body of kind `kind` with this sheet knows, in
/// `Ability::ALL`'s order: its body's (`known_by_kind`), its skills' (`skill.rs`)
/// and what it has learned (`Sheet::abilities`).
pub fn known(kind: CreatureKind, sheet: &Sheet) -> Vec<Ability> {
    Ability::ALL
        .into_iter()
        .filter(|ability| ability.is_known(kind, sheet))
        .collect()
}

/// Whether its body alone knows it.
fn by_kind(ability: Ability, kind: CreatureKind) -> bool {
    EVERYONE_KNOWS.contains(&ability)
        || kind.def().abilities.contains(&ability)
        // Anyone with legs can kick (owner, 2026-11-09).
        || (ability == Ability::Kick && crate::body::has_legs(kind))
}

impl Ability {
    /// Whether a creature in a body of kind `kind` with this sheet knows it: by
    /// its body, from a skill, or learned.
    pub fn is_known(self, kind: CreatureKind, sheet: &Sheet) -> bool {
        by_kind(self, kind) || self.known_from_skill(&sheet.skills) || sheet.abilities.knows(self)
    }

    /// Whether the creature carrying `carried` (in its body, `Inventory::creature`)
    /// with this sheet can do it now: active, known and with its tool carried
    /// (`tool`). The client greys out what fails it. Later also what it costs
    /// and how soon it can be done again.
    pub fn can_use(self, carried: &Inventory, sheet: &Sheet) -> bool {
        self.is_active() && self.is_known(carried.creature, sheet) && self.has_tool(carried)
    }

    /// The same for someone at a crafting bench (`crafting_bench::bench_faced`)
    /// holding `bench`'s tools: Alchemy can also be done with no apparatus
    /// carried when an Alchemy bench there holds one, as the server allows.
    pub fn can_use_at(
        self,
        carried: &Inventory,
        sheet: &Sheet,
        bench: Option<&crate::crafting_bench::BenchTools>,
    ) -> bool {
        let at_bench = bench.is_some_and(|bench| {
            bench.craft == crate::crafting_bench::Craft::Alchemy && !bench.tools().is_empty()
        });
        self.can_use(carried, sheet)
            || (self == Self::Alchemy && at_bench && self.is_known(carried.creature, sheet))
    }
}
