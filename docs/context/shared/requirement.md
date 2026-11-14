# shared: requirement

What an item asks of whoever uses it, and how well it works for someone who falls short. Never a refusal for attributes: items work less well. Not the bonuses themselves (`bonus`) nor showing the lines (client `item_popup`).

## Files

- `crates/shared/src/requirement.rs`: the module list, re-exports. Tests.
- `requirement/kind.rs`: `Requirement::{Attribute { attribute, min }, Skill { id, min }, Kinds(..)}` (`met_by`, `suits(kind)`), `item_requirements(item)`.
- `requirement/armour.rs`: armour's, by milligrams per cm² covered: `mg_at_reference`, `milligrams_per_cm2`, `needs_agility`, `requirement_for`, `REFERENCE_MG`, `AGILITY_AND_STRENGTH_ABOVE`, `NON_METAL_PLATE_MG`.
- `requirement/effectiveness.rs`: `factor_for`, `effectiveness`, `effective_bonuses`, `FULL`.

## Public API

- `item_requirements(item)`: armour's (below), and `Kinds` for food that suits only some kinds (`bonus::food`: Dwarven Bread, Raw Meat). `Skill` is for later. `met_by` and `effectiveness` count `Skill` and `Kinds` as met; `suits(kind)` checks `Kinds` (anyone suits the others).
- `effectiveness(requirements, attributes)`: thousandths (`FULL` 1000); per attribute `factor_for(min, value)` = (value − min/2) ÷ (min/2), clamped 0..1000; the lowest counts.
- `effective_bonuses(item, attributes)`: `bonus::item_bonuses` with every positive amount × the factor, negative ones in full. `Sheet::totals` uses it, so players (server `connections/bonuses.rs`), mobs (`entities/made.rs`) and the client's sheet all follow it.

## Depends on

- `bonus` (`item_bonuses`, `Bonus`, `Amount`, `food`), `armour` (`Construction`), `item`, `item_form`, `material` (`density`, `Category`), `sheet` (`Attribute`, `Attributes`), `skill` (`SkillId`), `creature` (`CreatureKind`).

## Used by

- `sheet` (`Sheet::totals`, `attributes_without_worn`), client `item_popup` (requirement lines), server `entities/suited.rs` (mobs' armour made again when it works at under half for them).

## Notes

- Armour (owner, 2026-11-14): by mg per cm² covered, which depends only on construction and material, so every piece of a set and every size asks the same. Each construction's figure at a reference density is arm armour's table (plate 870, mail 750, ring mail 400, mail and plate 800, splint 600, other plates 1000 at steel; leather 400 at hardened leather; padding 220 at linen; woven 480 at rattan), scaled by the material's density. A metal construction in a non-metal (horn scales, leather lamellae, wooden slats) is that material 6 mm thick (`NON_METAL_PLATE_MG`). This replaced the brief's weight ÷ area per piece: torso plate is modelled at 2.5 mm against 1 mm on the limbs, so per-piece figures would split sets.
- Padding, leather, mail and ring mail ask Agility; anything else Strength; over 600 mg/cm² both. The amount is in step with mg/cm², steel plate's 870 asking 110 (first guesses).
- Requirements are checked against the wearer's attributes with effects but not other worn items (`Sheet::attributes_without_worn`), so nothing loops.
- Combat's armour stage is still 0, so armour reaches combat only through its Toughness bonus, which the factor already scales.
