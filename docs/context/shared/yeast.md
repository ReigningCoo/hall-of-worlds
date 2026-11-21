# shared: yeast

Yeast as items (drinks plan PR 4, `docs/context/drinks.md`, owner's decisions 26 to 31): `ItemKind::Yeast` with a `YeastForm` keeping its clock, progress and use; feeding with flour and water, splitting and starters (PR 8). Using it is `shared::ferment`'s (PR 6).

## Files

- `crates/shared/src/yeast.rs`: the module list and re-exports. Tests in `yeast/tests.rs`.
- `yeast/form.rs`: `YeastForm`, `FEED_AFTER`, `WEAKENS_EVERY`, `new_item`, `Item::yeast`, `yeast_mut`.
- `yeast/clock.rs`: time passing: `Settled`, `SECONDS_PER_HOUR`, `Item::{age_yeast, lay_yeast_down, settle_lying_yeast}`, `Inventory::age_yeast`.
- `yeast/shop.rs`: `shop_quality`.
- `yeast/feed.rs`: one yeast fed: `Window`, `Fed`, `feed`, `feed_points`, `step_up_chance`, `FEED_PRACTICE`, `BEST_BY_FEEDING`, `YeastForm::{window, feedable}`.
- `yeast/feed_stack.rs`: a stack fed: `Supplies`, `FeedRefused`, `FeedPlan`, `plan_feeding`, `Inventory::feed_yeasts`.
- `yeast/supplies.rs`: what feeding uses: `WATER_ML`, `fresh`, `water_portions`, `drawn`, `Inventory::{feed_supplies, water_source, empty_bottles, use_flour}`.
- `yeast/strain.rs`: `Strain` (cooking plan PR 6).
- `yeast/starter.rs`: `STARTS_IN`, `STARTER_PRACTICE`, `StartRefused`, `can_start`, `new_starter`. Tests also in `yeast/feed_tests.rs`.

## Public API

- `YeastForm { unfed, progress, used, lying_since, developing }` (`ItemForm::Yeast`): `unfed` seconds since last fed by its clock; `progress` from on-time feeds; `used` since last fed (Ferment sets it, `ferment::use_yeasts`; a split too); `lying_since` (Unix seconds) while on the ground; `developing`, a starter's seconds left before it is ready (its clock counts it down first; until then neither usable nor feedable, and it stacks apart). `feed_in()` (0 once a feed is on time), `weakens_in()`, `usable()`.
- `FEED_AFTER` (12 h): earlier feeds split, later ones are on time. `WEAKENS_EVERY` (24 h): a quality step lost each, down to Poor (`Quality::worse`).
- `new_item(quality, id, made_at)`: one fresh yeast. `give_form` gives a fresh form.
- `Item::age_yeast(secs, now)`: every yeast in it (itself, stacked on it, inside at any depth) lets `secs` of playtime pass, or, for one picked up from the ground, the real time it lay there until `now`. `Inventory::age_yeast` does it for everything carried and worn (not the bin). `Item::lay_yeast_down(now)`: stamps `lying_since` (kept if already lying). `Item::settle_lying_yeast(now)`: lets the real time pass for yeast still lying, moving its stamp on. Each returns `Settled { weakened, hour_passed }` (`seen()`): whether the player needs telling.
- Feeding (owner's decisions 29 to 31): `feed(one, fermenting, roll)` by `window()`: `Early` (under `FEED_AFTER`) splits (`Fed::Split`: progress reset, `used` set; refused while used, `feedable`); `OnTime` adds `feed_points` (10 + whole Fermenting points) and steps up by `step_up_chance` (progress in 100 at Poor, ÷ 2 at Common, ÷ 3 Uncommon..., none from `BEST_BY_FEEDING`, Legendary), progress reset on a step; `Late` only feeds. Each resets `unfed` and the use (bar a split). `plan_feeding(stack, Supplies)`: every feedable yeast, longest unfed first, a sack and 100 ml each (a split twice, and an empty bottle), skipping one the supplies can't cover; `FeedRefused` (`words()`). `Inventory::feed_yeasts(stack, plan, fermenting, roll)`: the splits' second yeasts (ids to give) and steps up.
- Supplies: `feed_supplies(flour)` (sacks in that carried stack, fresh water portions in carried bottles (whole) and vessels (100 ml), not casks, empty bottles), `water_source()` (bottles first), `empty_bottles()`, `use_flour(stack, n)`, `drawn(container)` (after 100 ml taken), `fresh` (any water but salt).
- Starters: `can_start(flour, bottle)` (flour on a bottle of fresh water; `StartRefused`), `new_starter(id, made_at)`: a Poor yeast developing for `STARTS_IN` (24 h of its clock), the bottle used up.
- `Strain` (`YeastForm::strain`, default Bread, as older saves load): Bread, Yoghurt (a culture taken from a yoghurt, `food::culture_from`; never fed, `fed`), Koji (fed as yeast); `name` ("Yeast", "Yoghurt Culture", "Koji", the item's name), `brews` (only bread yeast ferments casks: `ferment`'s choices and usable yeasts). Stacks key on it too.
- `shop_quality(rolled, rng)`: Common 95% of the time, else the shop's roll (`shop::catch_up`).

## Depends on

- `item` (`Item`, `Inventory`, `ItemKind::Yeast`, `Quality`), `item_form`, `rng`, `id`, `serde`; for feeding `flour` (`ItemKind::Flour`, counted), `potion` (`BottleForm`, `Liquid`, `Water`), `pour` (`BOTTLE_ML`), `vessel` (`VesselForm::holding`), `skill` (`VALUE_SCALE`).

## Used by

- `item` (the kind: Materials, 250 g, 1 × 1, with quality, not counted; stacks compare it without its form, so a stack keys on quality and whether it is still developing), `item_form`, `shop` (herbalists sell it), server `connections/playtime.rs` (carried yeast by playtime), `ground` (stamped by `drop_at`, settled in reach by `tell::sync`), client `item_art` (`yeast.rs`), `item_popup` (`yeast.rs`: also a starter's hours left), `ferment` (usable yeasts, marking them used), `protocol` (`YeastAsk`), server `connections/feed_yeast.rs`, client `inventory` (`hold/feed.rs`, `hold_items/feed.rs`: flour let go on yeast or a bottle).

## Notes

- Each yeast is its own item, so a stack keeps every one's clock (owner). When their clocks part, one may lose a step before the rest: the stack is then mixed (`is_even`), as any stack of differing items.
- The clock (owner): playtime while carried (the server's `Activity::played`, which stops after 5 minutes idle, and while logged out), real time on the ground. The server adds playtime as it counts it; a yeast picked up still stamped counts its time on the ground first. Ground yeast in someone's reach is settled every tick without marking the site unsaved (a save with an older stamp counts the same time later), and marked once a step or hour passes.
- Clients hear of it each hour on its clock: the popup shows whole hours.
- Feeding's amounts (a sack of flour and 100 ml of water a yeast, twice and an empty bottle for a split), the points and chances, Poor starters, the bottle used up by a starter or split, 4 practice each and no energy cost are Claude's first guesses for the owner to tune.
- No bakers in towns yet: herbalists sell it until there are (a question for the owner).
