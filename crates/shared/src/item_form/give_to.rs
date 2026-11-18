//! Giving a new item its form as asked for: `give_form_to`, which every other way of
//! giving a form comes down to.

use super::armour::Piece;
use super::armour_ask::armour_form_to;
use super::gem::{GemAsk, GemForm, roll_gem};
use super::give::set_blade;
use super::necklace_ask::NecklaceAsk;
use super::necklace_roll::roll_necklace;
use super::request::{Field, Refused, RollRequest, keep_only};
use super::ring_ask::RingAsk;
use super::ring_roll::{roll_ring, roll_ring_size};
use super::{ItemForm, Limits};
use crate::blade;
use crate::creature::Size;
use crate::id::Id;
use crate::item::{Item, ItemKind, Made, MadeAt, Quality};
use crate::potion::bottle_to;
use crate::rng::Seed;

/// Gives a new item its form, for kinds that have one, going by its quality and
/// rolled from `seed`, keeping to what `request` asks; `RollRequest::NONE` rolls what
/// `give_form_from` does. `size` is what it must be; without one, it is rolled. A
/// blade within limits may become another family's kind. Anything inside it, such as
/// a ring's gems, gets its id from `new_id`. Refused, changing nothing, if the kind
/// doesn't take an option asked for or nothing can be made so.
pub fn give_form_to(
    item: &mut Item,
    size: Option<Size>,
    seed: Seed,
    request: &RollRequest,
    new_id: &mut impl FnMut() -> Id,
) -> Result<(), Refused> {
    if let Some(field) = request.not_for(item.kind) {
        return Err(Refused(field));
    }
    let mut rng = seed.derive("item form").rng();
    let quality = item.quality.unwrap_or_default();
    let (form, rolled_size) = match item.kind {
        kind if let Some(piece) = Piece::of(kind) => {
            let form = match *request == RollRequest::NONE {
                true => piece.roll(&mut rng, quality, None),
                false => armour_form_to(piece, &mut rng, quality, request)?,
            };
            (form, size.unwrap_or(Size::Medium))
        }
        // An apple: shops and mobs' gear are then suited to their place
        // (`shared::produce`); Give Item's fruit is chosen as herbs are
        // (`fruit::dev_fruit`).
        ItemKind::Fruit => {
            let def = crate::fruit::Fruits::built_in().get(crate::fruit::APPLE);
            let def = def.expect("the built-in fruits have the apple");
            let form = crate::fruit::FruitForm::new(def, seed);
            (ItemForm::Fruit(form), Size::Medium)
        }
        // Any vegetable, rolled from the seed (`vegetable::dev_vegetable`).
        ItemKind::Vegetable => {
            let def = crate::vegetable::dev_vegetable(None, seed);
            let def = def.expect("the built-in vegetables");
            let form = crate::vegetable::VegetableForm::new(def, seed);
            (ItemForm::Vegetable(form), Size::Medium)
        }
        // Any grain's flour, rolled from the seed (`flour::dev_flour`).
        ItemKind::Flour => {
            let def = crate::flour::dev_flour(None, seed);
            let def = def.expect("the built-in grains");
            let form = crate::flour::FlourForm::of(def).expect("grain mills");
            (ItemForm::Flour(form), Size::Medium)
        }
        // A log or kindling of the wood asked for, or rolled from the seed
        // (`woodcutting::dev_wood`).
        ItemKind::Log | ItemKind::Kindling => {
            let wood = crate::woodcutting::dev_wood(request.material, seed)?;
            let form = match item.kind {
                ItemKind::Log => ItemForm::Log(crate::woodcutting::LogForm { wood }),
                _ => ItemForm::Kindling(crate::woodcutting::KindlingForm { wood }),
            };
            (form, Size::Medium)
        }
        // Empty unless asked for a liquid or effects: potions are brewed
        // (later), not rolled.
        ItemKind::Bottle => {
            let look = seed.derive("bottle look").value();
            let bottle = bottle_to(&mut rng, request, look)?;
            (ItemForm::Bottle(bottle), Size::Medium)
        }
        // Size and wood rolled or as asked, empty or full of a water or oil.
        ItemKind::Cask => {
            let cask = crate::cask::cask_to(&mut rng, request)?;
            (ItemForm::Cask(cask), Size::Medium)
        }
        // Which vessel rolled or as asked, empty or full of a water or oil.
        ItemKind::Vessel => {
            let vessel = crate::vessel::vessel_to(&mut rng, request)?;
            (ItemForm::Vessel(vessel), Size::Medium)
        }
        // Rolled doses (1 to 4 effects, or as asked), its quality from them,
        // until Grind makes it (`brewing`).
        ItemKind::Ointment => {
            use crate::potion::{MOST_EFFECTS, roll_contents};
            let effects = request.effects.unwrap_or_else(|| rng.range_u64(1, 4) as u8);
            if !(1..=MOST_EFFECTS).contains(&effects) {
                return Err(Refused(Field::Effects));
            }
            let contents = roll_contents(&mut rng, effects);
            item.quality = Some(crate::brewing::quality(&contents));
            let form = crate::brewing::OintmentForm { contents };
            (ItemForm::Ointment(form), Size::Medium)
        }
        ItemKind::Gem => {
            let ask = GemAsk::of(request);
            ask.check(quality, true)?;
            let (gem, size) = roll_gem(&mut rng, quality, true, ask);
            (ItemForm::Gem(gem), size)
        }
        ItemKind::Ring => {
            let ask = RingAsk::of(request);
            ask.check(quality)?;
            let size = size.unwrap_or_else(|| roll_ring_size(&mut rng));
            let made_at = item.made_at;
            let mut new_gem =
                |gem, gem_size, quality| set_gem(new_id(), gem, gem_size, quality, made_at);
            let ring = roll_ring(&mut rng, quality, size, ask, &mut new_gem);
            (ItemForm::Ring(Box::new(ring)), size)
        }
        ItemKind::Necklace => {
            let ask = NecklaceAsk::of(request);
            ask.check(quality)?;
            let size = size.unwrap_or(Size::Medium);
            let made_at = item.made_at;
            let mut new_gem =
                |gem, gem_size, quality| set_gem(new_id(), gem, gem_size, quality, made_at);
            let necklace = roll_necklace(&mut rng, quality, size, ask, &mut new_gem);
            (ItemForm::Necklace(Box::new(necklace)), size)
        }
        kind => {
            let Some(family) = kind.blade_family() else {
                return Ok(());
            };
            if !keep_only(&mut blade::blade_metals(quality), request.material) {
                return Err(Refused(Field::Material));
            }
            let limits = Limits {
                metal: request.material,
                ..request.limits.unwrap_or(Limits::NONE)
            };
            if request.limits.is_some() {
                let limits = Limits {
                    wielder: limits.wielder.or(size),
                    ..limits
                };
                let rolled = blade::roll_blade_within(&mut rng, family, quality, &limits);
                set_blade(item, rolled.ok_or(Refused(Field::Limits))?);
                return Ok(());
            }
            let size = size.unwrap_or(Size::Medium);
            let blade = blade::roll_blade_at_within(&mut rng, family, quality, size, &limits)
                .ok_or(Refused(Field::Material))?;
            (ItemForm::Blade(Box::new(blade)), size)
        }
    };
    let size = size.unwrap_or(rolled_size);
    item.made = Made::for_body(item.kind, size, item.made.fingers, item.made.magical);
    item.form = Some(form);
    Ok(())
}

/// A gem for a setting, as a whole item.
fn set_gem(id: Id, gem: GemForm, size: Size, quality: Quality, made_at: Option<MadeAt>) -> Item {
    let mut stone = Item::new(id, ItemKind::Gem);
    stone.made = Made::for_body(ItemKind::Gem, size, None, false);
    stone.quality = Some(quality);
    stone.made_at = made_at;
    stone.form = Some(ItemForm::Gem(gem));
    stone
}
