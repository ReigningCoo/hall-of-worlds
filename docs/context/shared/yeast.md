# shared: yeast

Yeast as items (drinks plan PR 4, `docs/context/drinks.md`, owner's decisions 26 to 31): `ItemKind::Yeast` with a `YeastForm` keeping its clock, progress and use. Not feeding or splitting (PR 8, needs flour); using it is `shared::ferment`'s (PR 6).

## Files

- `crates/shared/src/yeast.rs`: the module list and re-exports. Tests in `yeast/tests.rs`.
- `yeast/form.rs`: `YeastForm`, `FEED_AFTER`, `WEAKENS_EVERY`, `new_item`, `Item::yeast`, `yeast_mut`.
- `yeast/clock.rs`: time passing: `Settled`, `SECONDS_PER_HOUR`, `Item::{age_yeast, lay_yeast_down, settle_lying_yeast}`, `Inventory::age_yeast`.
- `yeast/shop.rs`: `shop_quality`.

## Public API

- `YeastForm { unfed, progress, used, lying_since }` (`ItemForm::Yeast`): `unfed` seconds since last fed by its clock; `progress` from on-time feeds (PR 8 adds it by the feeder's Fermenting); `used` since last fed (Ferment sets it, `ferment::use_yeasts`; a split will too); `lying_since` (Unix seconds) while on the ground. `feed_in()` (0 once a feed is on time), `weakens_in()`, `usable()`.
- `FEED_AFTER` (12 h): earlier feeds split, later ones are on time. `WEAKENS_EVERY` (24 h): a quality step lost each, down to Poor (`Quality::worse`).
- `new_item(quality, id, made_at)`: one fresh yeast. `give_form` gives a fresh form.
- `Item::age_yeast(secs, now)`: every yeast in it (itself, stacked on it, inside at any depth) lets `secs` of playtime pass, or, for one picked up from the ground, the real time it lay there until `now`. `Inventory::age_yeast` does it for everything carried and worn (not the bin). `Item::lay_yeast_down(now)`: stamps `lying_since` (kept if already lying). `Item::settle_lying_yeast(now)`: lets the real time pass for yeast still lying, moving its stamp on. Each returns `Settled { weakened, hour_passed }` (`seen()`): whether the player needs telling.
- `shop_quality(rolled, rng)`: Common 95% of the time, else the shop's roll (`shop::catch_up`).

## Depends on

- `item` (`Item`, `Inventory`, `ItemKind::Yeast`, `Quality`), `item_form`, `rng`, `id`, `serde`.

## Used by

- `item` (the kind: Materials, 250 g, 1 × 1, with quality, not counted; stacks compare it without its form, so a stack keys on quality), `item_form`, `shop` (herbalists sell it), server `connections/playtime.rs` (carried yeast by playtime), `ground` (stamped by `drop_at`, settled in reach by `tell::sync`), client `item_art` (`yeast.rs`), `item_popup` (`yeast.rs`), `ferment` (usable yeasts, marking them used).

## Notes

- Each yeast is its own item, so a stack keeps every one's clock (owner). When their clocks part, one may lose a step before the rest: the stack is then mixed (`is_even`), as any stack of differing items.
- The clock (owner): playtime while carried (the server's `Activity::played`, which stops after 5 minutes idle, and while logged out), real time on the ground. The server adds playtime as it counts it; a yeast picked up still stamped counts its time on the ground first. Ground yeast in someone's reach is settled every tick without marking the site unsaved (a save with an older stamp counts the same time later), and marked once a step or hour passes.
- Clients hear of it each hour on its clock: the popup shows whole hours.
- No bakers in towns yet: herbalists sell it until there are (a question for the owner).
