# shared: clothing

Clothes on the Middle layer: everything worn over underclothes and under cloaks and armour, footwear apart. So far tunics, robes, doublets, bodices and vests on the torso, trousers and skirts on the legs, belts and gloves (PRs 1 and 2 of 6; head, neck and shoulder pieces, then models, follow). Medieval and earlier first, later pieces (to 1700) rarer. What one is like is rolled once and saved, with its names, cells and weight. Not the rules for wearing (`wear`), nor underclothes (`underclothes`), nor models (client `item_art`, still to come).

## Files

- `crates/shared/src/clothing/mod.rs`: `ClothingForm` (`piece`), the module list and re-exports.
- `piece.rs`: `ClothingPiece` (one for each item kind, `of`, `kind`; in `ItemKind::ALL_CLOTHING`'s order).
- `garment.rs`: `Garment` (which garment and how it is cut: `piece`, `bare`), passing each question to the garment's own file.
- One file per garment, each with its options, grid, roll, names by signature and base weight: `tunic/` (`Tunic`, `Length`, `Cut`, `Sleeves`, `Closure`, `Neck`, `TunicDecoration`; tables in `tables.rs`, names and weight in `names.rs`), `robe.rs` (`Robe`, `RobeLength`, `Front`, `Fastening`, `RobeSleeves`, `Collar`, `Lining`, `RobeDecoration`), `doublet.rs` (`Doublet`, `DoubletLength`, `Padding`, `DoubletFastening`, `DoubletCollar`, `DoubletSleeves`, `DoubletDecoration`; `arming`), `bodice.rs` (`Bodice`, `BodiceLength`, `Lacing`, `BodiceNeck`, `BodiceSleeves`, `Stiffening`), `vest.rs` (`Vest`, `VestLength`, `VestMake`, `VestFastening`, `VestDecoration`), `trousers.rs` (`Trousers`, `TrouserLength`, `Fullness`, `TrouserWaist`, `LegEnds`, `TrouserFront`; leg wraps a flag), `skirt.rs` (`Skirt`, `SkirtLength`, `SkirtMake`, `SkirtWaist`, `SkirtFront`), `belt.rs` (`Belt`, `BeltForm`, `BeltWidth`, `BeltEnd`, `Fittings`, `BeltFastening`; pleated sashes a flag), `gloves.rs` (`Gloves`, `Fingers`, `Cuff`, `GloveMake`, `GloveDecoration`; fur lining a flag). The new garments keep their own materials lists (`materials`), which `cloth.rs` asks for.
- `cloth.rs`: `Weave` (plain, velvet, brocade, damask), `Pattern` (plain, striped, checked, tartan); materials, weaves, patterns, dyes and second colours by garment.
- `roll.rs`: `roll`. `grid.rs`: `allowed`. `names.rs`: `specific_name`, `name`. `weight.rs`: `grams`, `grams_at`, `shape` (fur linings and belts' metal added after the cloth). The weighted tables use `underclothes::table` (`has`, `when`).

## Public API

- `roll(rng, piece, quality)`: a `ClothingForm` for a `ClothingPiece::{Tunic, Robe, Doublet, Bodice, Vest, Trousers, Skirt, Belt, Gloves}` (the item kinds of the same names; `ClothingPiece::of(kind)`).
- `ClothingForm { garment, material, weave, pattern, dye, contrast }`: `piece`, `allowed`, `name` (a `String`), `specific_name`, `shape`, `grams`, `grams_at(size)`.
- `Garment::{Tunic(Tunic), Robe(..), Doublet(..), Bodice(..), Vest(..), Trousers(..), Skirt(..), Belt(..), Gloves(..)}`; `Garment::bare()`: the arms of a sleeveless (or one-shouldered) tunic or bodice, fingerless gloves' Fingers (so rings stay free), read through `Item::bare` / `Item::worn_locations`. `Doublet::arming()`.

## Depends on

- `shared::armour` (`Dye`), `shared::creature` (`Location`, `Size`), `shared::item`, `shared::material`, `shared::rng`, `shared::underclothes` (`table`), `serde`.

## Used by

- `shared::item_form` (`ItemForm::Clothing`, rolled by `give_form`; material and dye can be asked for), `shared::item` (`Item::bare`; the kinds' rows in `item/clothing_def.rs`). Not drawn yet: `ItemForm::drawn_worn` leaves clothing out, and client `item_art` has no look for it.

## Notes

- Every choice is rolled from what the earlier ones allow (garment, material, weave, pattern, dye, second colour); `allowed` is that grid, checked by a test, as are every name turning up, bare arms and weights.
- Owner's decisions (2026-11-15, the research's review): one garment on the torso on Middle, over-garments (gowns, jerkins over doublets, haori) left for an Outer research; tunics and robes list only the Torso and Arms, their length a property, so trousers, skirts and belts go with any; a doublet may be made for arming (quilted, with points) and goes under unlined plate as an arming vest does, which the layers already give (lined armour takes Middle and clashes); regional names kept as specific names, up to 1700 with later ones rarer; the dalmatic as the Byzantine court tunic, no liturgical names; velvet, brocade and damask are kinds of silk (velvet of wool too) and tartan a pattern like stripes; Middle clothes drop on death on Hard.
- PR 2 (owner, 2026-11-15): skirts, kilts, wraps and the dhoti are a Skirt on the Legs (Middle), clashing with trousers, not belts, leg armour over them; leg wraps are a trousers option, no item; one Belt kind rolled into strap, cord girdle, sash, obi and plates, every one with the belt's bag slots (`item::Inventory`, by kind and quality); one Gloves kind rolled into gloves, mittens and fingerless (Hands only); padded gloves stay armour; the baldric waits for sheaths. Trousers, Belt and Gloves were placeholder kinds and now roll forms (old items without one load as before); the old `FingerlessGloves` kind stays for saves but is no longer made or sold. Plated belts, elbow-length and perfumed gloves are rolled only at Rare and better. Kilts are mostly tartan wool, tube skirts often checked (the Longyi).
- Velvet, brocade and damask are a `Weave` on the form, not materials, so a velvet doublet is still silk or wool; Grass is a new `Material` (Plant) for the grass skirt to come. Buff is the existing Buff Leather.
- A tunic trailing to the floor (rank only) is rolled only at Rare and better; the grid allows it at any quality.
- A name comes from its garment's signature (a curved flap is a Deel); a pattern, a weave or an unusual material goes first ("Striped Silk Robe", "Velvet Doublet"), except where the name says it (a Jerkin and Lederhosen are leather, a Grass Skirt grass, a Kaunakes sheepskin (Fur), tartan is wool).
- Weights are worked out from cloth areas in plain linen, changed by material (buff leather 4.5 ×) and weave (velvet 1.6 ×), plus fur linings; by the square of the size, as underclothes.
- Research: `docs/context/research-clothing.md`. Players' page: `docs/clothing.md`, kept in step with this module.
