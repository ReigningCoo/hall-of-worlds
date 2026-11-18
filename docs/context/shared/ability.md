# shared: ability

Abilities, the named things a creature can do: active ones done when asked (and put on the action bar), toggles turned on and off there, passive ones always at work. It also holds the action bar's slots and the rules for changing them, which the client predicts with and the server checks by. How Attack works when used is `shared::combat`'s and the server's, Heal's `shared::magic`'s. A creature knows an ability by its body (its kind), from a skill it knows (`skill.rs`), or by having learned it (`Learned`, saved on its sheet); some need a tool carried (`tool.rs`). Attack uses the held weapon's skill node and Heal Healing's. Plan: `docs/context/abilities-skills.md`.

## Files

- `crates/shared/src/ability.rs`: `Ability` (`ALL`, `name`, `description` (a line for its tip), `leans` (the attributes it leans on, which colour it: Attack and Kick Strength and Agility, Trample Strength and Toughness, Flutter Agility, Night Vision Perception, Heal Empathy as Healing), `mana_cost`, `use_kind`, `is_active` (active or a toggle: it goes on the bar), `is_toggle`), `Use::{Active, Passive, Toggle}`. Tests.
- `crates/shared/src/ability/knowing.rs`: `known_by_kind`, `known`, `Ability::is_known`, `Ability::can_use`.
- `crates/shared/src/ability/learned.rs`: `Learned`.
- `crates/shared/src/ability/skill.rs`: `Ability::skill`, `ALCHEMY` (re-exported, also for `brewing`), and which skills' abilities everyone has for now.
- `crates/shared/src/ability/tool.rs`: `Ability::tool`, `Ability::has_tool`.
- `crates/shared/src/ability/bar.rs`: `ActionBar`, `BarChange`, `BarRefusal`, `BAR_SLOTS`.
- `crates/shared/src/ability/bar_slot.rs`: `BarThing`, `BarSlot` (with its two shapes: saves write `(ability: Attack, auto: false)` as before items, or `(item: Apple)`, by `is_human_readable`; messages carry it as it is), `usable_from_bar`.

## Public API

- `Ability`: `Attack`, `Heal` and `Sneak` (every creature; Heal as there is no way to learn spells yet; Sneak a toggle, holding moves on levels to a sneaking pace, `energy::sneak_move_energy`), and the kinds' own `Flutter`, `Kick`, `NightVision` (passive), `Trample`; and Alchemy's `PressOil`, `Grind` and `Brew` (brewing, `docs/context/brewing.md`; active, leaning on Intellect; Press Oil built: `potion`'s `press.rs`, server `connections/press_oil.rs`, client `abilities/press_oil.rs`; Grind built: `brewing`'s `order.rs`, server `connections/grind.rs`, client `abilities/grind/`; Brew built: `brewing`'s `brew_order.rs`, server `connections/brew.rs`, client `abilities/brew.rs`); `CollectWater` (learned, leaning on Perception as the Water skill: empty bottles filled one after another at the water faced, `potion`'s `collect.rs`, server `connections/collect_water.rs`). Also `sheet::Ability`. Messages carry it by its place, so new ones go at the end of the enum; `ALL` is the order players see (Attack first).
- `known_by_kind(kind)`: what a body alone knows: Attack, Heal, Sneak, Kick for any kind with legs (`body::has_legs`, owner 2026-11-09), and the kind's own (`CreatureDef::abilities`), for the body it is in now, so shapeshifting changes them.
- `known(kind, sheet)` and `Ability::is_known(kind, sheet)`: everything known, in `ALL`'s order: the body's, those from a skill the sheet knows (`Ability::skill`, a path: Press Oil, Grind and Brew from `ALCHEMY`, `crafting/alchemy`, given to everyone for now, TODO `todo-learn-alchemy`), and those learned (`Sheet::abilities`).
- `Learned` (`knows`, `learn` (true if new), `iter`): the abilities one character has learned, saved on `Sheet::abilities` as a list of names, empty in older saves. `learn_from(carried)`: Collect Water on a first empty bottle (`Inventory::empty_bottle`; the server asks each time it sends the inventory, sending the sheet when it is new).
- `Ability::tool()`: the item kind it needs carried (Press Oil an Oil Press, Grind a Mortar and Pestle, Brew a Cauldron); `has_tool(carried)`: anywhere in what is carried, bags too (`Inventory::count_of`); Collect Water needs an empty bottle instead, or a cask with room for water (`Inventory::cask_for_water`).
- `Ability::can_be_automatic()`: whether a slot may be made automatic, done whenever it can be (owner, 2026-11-07): Attack and Heal so far; `BarChange::ToggleAuto` refuses the rest (`BarRefusal::NotAutomatic`), though turning one off always works.
- `Ability::can_use(carried, sheet)`: active, known (`is_known`, the body being `carried.creature`) and with its tool carried. The client greys out what fails it, and what costs more Mana (`mana_cost`) than there is; the server refuses it.
- `BarThing::{Ability(Ability), Item(ItemKind), Potion(PotionKey)}` (`ability()`, `item()`, `potion()`, `can_be_automatic()`: abilities that may be, never items); `From` an `Ability`, an `ItemKind` or a `potion::PotionKey`. `usable_from_bar(kind)`: food (`bonus::is_food`) so far.
- `ActionBar { slots: [Option<BarSlot { thing, auto }>; 10], given }` (`BarSlot::new(thing)`, `BarSlot::ability()`): `Default` is a new character's (Heal in the first slot and Attack in the second, so healing comes first if both are automatic: Claude's suggestion, 2026-11-07; also for saves from before the bar); `apply(change, known)` (`known`: what the character knows, `ability::known`), `slot_of(ability or item kind)`, `catch_up` (a bar saved before an ability was added to new bars gets it once, in its first empty slot, counted by `given`; the server calls it on loading). Automatic slots take turns from the first: the first whose ability can be done is (owner, 2026-11-07).
- `BarChange::{Place { slot, ability }, Clear { slot }, Swap { a, b }, ToggleAuto { slot }, PlaceItem { slot, item }, PlacePotion { slot, potion }}` (an empty potion refused as `NotUsable`); `BarRefusal::{NoSlot, Passive, Unknown, Empty, NotAutomatic, NotUsable}`.

## Depends on

- `creature` (`CreatureKind::def().abilities`), `item` (`ItemKind`, `Inventory::count_of` for tools), `bonus` (`is_food`), `potion` (`Inventory::empty_bottle`), `sheet` (`Sheet::skills`, `Sheet::abilities`), `skill` (`SkillId::find`, `Skills::knows`).

## Used by

- `sheet` (re-exports `Ability`; `Sheet::abilities` is a `Learned`), `creature` (kinds' abilities), `protocol` (`ClientMessage::ActionBar`, `ServerMessage::ActionBar`), server `connections/action_bar.rs` and `storage` (`CharacterSave::action_bar`), client `abilities` (the Abilities page and the action bar).

## Notes

- Bar rules (owner): an ability is in at most one slot; placing it from the Abilities page onto a filled slot replaces what was there, and placing one already on the bar moves it with its `auto`; swapping keeps each slot's `auto` with its ability; taking one off (dropping it on the Abilities page) forgets its `auto`. Only active, known abilities can be placed. One no longer known (after shapeshifting) stays in its slot, shown as unusable.
- Items on the bar (owner, 2026-11-15): a slot may hold a kind of item with something to do when used (food, eaten), placed whoever may eat it (Raw Meat, Dwarven Bread for anyone); one slot per kind, like abilities; never automatic. The slot stands for every one carried; which stack is used is `Inventory::stack_to_use`, and using it is the existing `ClientMessage::Consume`, so the server checks nothing new.
- Potions on the bar (brewing decision 22): a slot holds a particular potion by its contents (`PotionKey`), saved as `(potion: [...])`; one slot per potion; never automatic; using it drinks a bottle of it (`Inventory::potion_to_use`) through `ClientMessage::Consume`, which the server checks as any drink (the cooldown too).
- Which of the kinds' abilities are active was Claude's guess, agreed by the owner for now (Night Vision passive, the rest active); see `decisions.md`.
