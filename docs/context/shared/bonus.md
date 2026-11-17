# shared: bonus

Bonuses: what worn items and effects (food, potions, dev tools) add to a creature's attributes, the most Health and Mana, skills, and things only bonuses give (restoring Health or Mana over time). Rules only: the server applying them to players (wearing, eating) and the client showing them are elsewhere.

## Files

- `crates/shared/src/bonus.rs`: the module list, re-exports, `MILLI`. Tests.
- `bonus/stat.rs`: `Stat::{Attribute(a), Most(vital), Skill(id), Restore(vital), Drain(vital)}` (new ones at the end), `Amount::{Flat, Percent}`, `Bonus { stat, amount }` (`flat`, `percent`), `PER_CENT`.
- `bonus/totals.rs`: `Totals` (`of`, `sum`, `added_milli`, `whole`, `attributes`, `restoring`, `draining`), `Sum { flat, percent }`, `round_milli`.
- `bonus/item.rs`: `item_bonuses(item)` (armour's, a hair shirt's), `quality_factor`, `HAIR_SHIRT_WILL` (−5%), `TOUGHNESS_PER_KG` 10, `AGILITY_PER_KG` 2, `MANA_PER_KG` 1.
- `bonus/material.rs`: `MaterialFactors { toughness, agility, mana }` (`USUAL`), `material_factors(material)`; an armour piece's material from its form.
- `bonus/effect.rs`: `Effect { source, bonuses, start, end, ends_on }` (`lasts`, `dev_bonus`), `Source::{Eaten(kind), Dev, Poison, RawHerb, EatenFruit(tag), FruitEffects(tag)}` (`replaced_by`, `is_eaten`; a fruit named by `fruit::FruitTag`), `EndsOn { moving, hurt }` (`NOTHING`, `by`), `Cancel::{Moving, Hurt}`.
- `bonus/effects.rs`: `Effects(Vec<Effect>)` (`add`, `cancel`, `cancelled_by`, `forget_ended`, `first_ended`, `next_end`, `bonuses(now)`).
- `bonus/food.rs`: `Food { restore, ticks, kinds, others }` (`suits`), `Others::{Refused, Poisoned}`, `food(kind)` (the table), `is_food(kind)`, `edible(kind)` (food or a herb, eaten raw), `may_eat(kind, eater)` (anyone may eat a herb or fruit). Fruit has no row: `is_food` counts it (the action bar) and `fruit::eating` says what it does.
- `bonus/eating.rs`: `eating(kind, eater, now)` (the effects, `None` if not food or refused), `food_effect(kind, now)` (the healing, for whoever it suits), `poison(now)`, `POISON_DRAIN`, `POISON_TICKS`.
- On `Sheet` (`sheet/bonuses.rs`): `Situation { worn, playtime, refill }`, `totals` (worn items through `requirement::effective_bonuses`), `attributes_without_worn`, `attributes_with`, `limits`, `rebase`, `change_effects`, `add_effect`, `eat(kind, eater, ..)`, `cancel_effects`, `forget_ended`; old saves' changes folded into effects by `settled`.

## Public API

- Everything above. Amounts are thousandths of the stat's unit (points for attributes and the most, a skill's value hundredths for skills, points a second for restoring); percentages are thousandths of a percent of the stat's base (what it is without bonuses).
- `Sheet::limits(situation, now)` is what rules go by for Health and Mana; `Sheet::attributes_with(worn, now)` for attributes.

## Depends on

- `sheet` (`Attribute`, `Attributes`), `vitals` (`Vital`, `PER_POINT`), `skill` (`SkillId`), `item`, `item_form`, `material`, `energy` (ticks a second), `serde`.

## Used by

- `requirement` (`item_bonuses`, `Bonus`, `Amount`), `sheet` (`Sheet::effects`, `sheet/bonuses.rs`), `vitals` (`Totals` in `Vitals::most` and `limits_with`), `requirement` (`food`: a food's kinds), `server` (`connections/vitals.rs`, `bonuses.rs`, `consume.rs` and `ground/eat.rs` with `may_eat`, `entities/made.rs`), client `character` (the most's lines), `inventory` (`is_food`, `may_eat`, `food`: eating on the clothes panel and why not), `item_popup` (`item_bonuses`, `food_effect`, `food`, `poison`: the popup's lines).

## Notes

- Worn items give their bonuses by how well they work for the wearer (`requirement::effective_bonuses`): positive ones × the factor, negative ones in full, judged by `Sheet::attributes_without_worn` (own attributes with effects, never other worn items).
- Totals are always rebuilt from scratch from everything worn plus effects lasting at `now` (`Sheet::totals`), flat sum then percentage of the base, rounded once to the nearest (halves up); never added or taken away a piece at a time, so no rounding builds up. Attributes never go below 1.
- Item bonuses are a pure function of the item. Armour gives (owner, 2026-11-14): per kg of the piece's own weight (its form's, without contents), +10 Toughness × quality factor × material factor, −2 Agility × material factor, −1 most Mana × material factor. Quality factors (first guesses): Poor 0.75, Common 1, +0.25 a step to Legendary 2, Artifact 2.5; no quality counts as Common. Material factors are a table, all 1 for now (later Mithril costing less Agility, Adamantine giving more Toughness). A hair shirt (an underclothes shirt of haircloth; not a horsehair petticoat) gives −5% Will (owner, 2026-11-14: "a light Will bonus", other positive ones later). Nothing else gives anything yet.
- Effects: at most one per `Source`, so eating a second loaf starts it again; and only one food counts at a time (owner, 2026-11-15): eating anything replaces any earlier `Source::Eaten(_)` or `EatenFruit(_)`, whatever its kind (`Source::replaced_by`, in `Effects::add`), while Poison and Dev stay apart. The client shows the timed ones in `effect_bar`, by the server's tick (`ServerMessage::Clock`). They end at an action tick (real time, offline too) or by `Cancel` (`EndsOn`). `lasts` goes by the end only, so a client passing tick 0 counts everything the server still keeps. Fruit (owner, 2026-11-17, `shared::fruit`): 1 Health a second for a quarter of its kcal in seconds, its effects (`Source::FruitEffects`, their own source: the latest fruit's replace any earlier, nothing ends them early) as long. Placeholder foods (owner, 2026-11-14): Bread 1 a second for 45 s, anyone; Dwarven Bread 1 a second for 60 s, dwarves only, refused to others (no poison); Raw Meat 2 a second for 60 s for orcs and goblins, while anyone else eats it for no healing and poison. Food healing always ends by moving or being hurt.
- A herb eaten raw (`Source::RawHerb`, made by `herb::raw_dose`): its own source, so it runs alongside food and poison, the latest herb replacing any earlier; herbs aren't food (`is_food`), so they never go on the action bar.
- Poison (`Source::Poison`, `Stat::Drain`): 1 Health a second for 60 s, its own source so it runs alongside healing and eating again starts it again; never takes Health below 1 (owner: other future effects may); ended only by running out (a cure later), not by moving, harm or combat; unlike restoring it doesn't wait for refilling (`vitals::course`). A food's kinds are its `requirement::Requirement::Kinds`, which the popup shows.
- Restoring joins refilling's formula (`vitals::Rate`): extra points a second while refilling (so not while waiting after a fight), until `Limits::restore.until`, the next tick any effect ends. `Sheet::forget_ended` settles each ended effect at its own end with the limits from just before, so restoring stops exactly there however late it is called. `Sheet::limits` doesn't count an ended effect, so call `forget_ended` first (`change_effects` and `cancel_effects` do); cancel `Hurt` before harming.
- Changes to the most from bonuses keep the share (`Vitals::rebase`, owner, 2026-11-14): `Sheet::change_effects` and `Sheet::rebase` (for worn changes: limits before, then the change) do it.
- The server applies worn items and effects to players (`connections/bonuses.rs`, rebuilt on any change to what is worn) and worn items to mobs once, when they are made; players eat by dropping food on the clothes panel (`ClientMessage::Consume`). `is_food` says what can be eaten at all, `may_eat` whether by this kind (the server refuses others, and the client says why first); `Sheet::eat` takes the eater's kind (the body it is in now, `Inventory::creature`).
