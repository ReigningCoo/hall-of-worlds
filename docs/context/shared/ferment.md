# shared: ferment

Ferment and vinegar (drinks plan PR 6, `docs/context/drinks.md`): applying the fermentation rules (`shared::fermentation`) to casks. Ferment's order and what it starts, the yeasts it uses, what its panel offers, and time passing on casks: fermenting, ageing sealed, opened drinks turning to vinegar, opened juice fermenting wild. Times are real seconds, a game day a real hour.

## Files

- `crates/shared/src/ferment.rs`: the module list and re-exports. Tests in `ferment/tests.rs`.
- `ferment/order.rs`: `FERMENTING`, `FERMENT_PRACTICE`, `ferment_ticks`, `FermentOrder`, `FermentRefused`, `FermentPlan`, `plan`.
- `ferment/yeasts.rs`: `Inventory::{usable_yeasts, use_yeasts, take_added}`.
- `ferment/choices.rs`: `FermentChoices`, `YeastStack`, `fermentable_cask`, `Inventory::ferment_choices`.
- `ferment/clock.rs`: `CaskSettled`, `days_left`, `CaskForm::{pass, pass_ageing, turns_in}`, `Item::{age_casks, lay_casks_down, settle_lying_casks, settle_lying_casks_ageing}`, `Inventory::age_casks`.

## Public API

- `FERMENTING` (`crafting/fermenting`): Ferment's skill, given to everyone for now (`ability`'s `GIVEN_FOR_NOW`). `FERMENT_PRACTICE` (8) a cask. `ferment_ticks()`: a 5 s wind-up.
- `FermentOrder { cask, add, yeast, yeasts }` (`ClientMessage::Ferment`): the cask (carried or lying in reach), a carried fruit or grain stack for water (all of it goes in), a carried yeast stack and how many of its usable yeasts.
- `plan(cask, add, yeasts) -> Result<FermentPlan, FermentRefused>`: the mash from the cask's juice or cordial (nothing added), or its fresh water with the fruit (`Mash::fruit`, the stack's grams) or grain sheaves (`Mash::grain`) added; strength by the best yeast, time by `ferment_secs`, quality by `drink_quality`; at most what the cask holds. `FermentRefused` (`words()`): NotACask, Fermenting, Nothing (salt water, empty, anything else), NeedsAdding, TakesNothing, WontFerment, NoYeast, TooWeak. `FermentPlan { drink, ml, secs }`, `started(cask)`: the cask sealed, holding the drink, `CaskTime::fermenting` and `made` set.
- `Inventory::usable_yeasts(stack, count)` (qualities, top first), `use_yeasts(stack, count)` (marks them `used`, owner: used once, not used up), `take_added(stack)`.
- `Inventory::ferment_choices()` (`FermentChoices { casks, adds, yeasts }`, carried only; `YeastStack { stack, quality, usable }`), `fermentable_cask(item)` (for casks lying in reach too).
- `CaskForm::pass(secs) -> CaskSettled { seen, warned }`: fermenting counts down first, then a sealed drink ages (`aged` from `made`), an opened drink counts `opened_for` to `best_before_secs` and becomes `Liquid::Vinegar` of its base (warned once in its last tenth: `notice_due`), opened juice or cordial becomes a Poor drink after `JUICE_TURNS_SECS` (`WILD_TOLERANCE`; a mash too weak stays juice). `seen` when a game day passes on a clock shown, it finishes, ages a step or turns. `turns_in()`: seconds to vinegar for an opened drink. `pass_ageing(secs, percent)`: the same with a sealed drink's ageing at `percent` (casks on a rack: `cask_rack::RACK_AGEING_PERCENT`, 110), fermenting and the opened clock unchanged. `days_left(secs)`: game days rounded up.
- The two clocks (owner's decision 2), as yeast's: `Inventory::age_casks(secs, now)` by playtime; `Item::lay_casks_down(now)` stamps `CaskTime::lying_since`; `settle_lying_casks(now)` lets real time pass for casks on the ground (`settle_lying_casks_ageing(now, percent)` for casks on a rack, in whole steps of `100 / gcd(percent, 100)` seconds so ageing loses nothing to rounding); one picked up counts its time lying first.

## Depends on

- `fermentation` (all of it), `cask` (`CaskForm`, `CaskTime`), `fruit`, `vegetable`, `yeast` (`YeastForm::usable`), `potion` (`Liquid`, `Water`), `item` (`Item`, `Inventory::{find, find_mut, take, is_worn, stacks_where}`), `energy`, `id`, `serde`.

## Used by

- `cask_rack` (`RackCasks::settle`), `ability` (`Ferment`: its skill, has_tool by a usable yeast), `protocol` (`ClientMessage::Ferment`), `pour` (refusing a fermenting cask), server `connections/ferment.rs`, `connections/playtime.rs`, `ground/bags.rs`, `ground/tell.rs`, client `abilities/ferment/`, `item_popup/cask.rs`.

## Notes

- Worked out when looked at (as shops catch up their stock): carried casks as the server counts playtime, ground ones in a player's reach each tick; nobody near, nothing changes until someone comes.
- Only casks keep clocks. A drink poured into a vessel or bottle doesn't age or turn yet; pouring cask to cask keeps the older opened clock (`pour`).
- Ageing only while sealed and only from `made` (a drink fermented in that cask); once opened a cask is never sealed again.
- First guesses for the owner (Claude's): the 5 s wind-up, 8 practice a cask, all of a fruit or grain stack going in, cordial turning wild like juice.
