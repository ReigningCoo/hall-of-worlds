//! `look_of`: what can be seen of an item, from its kind and form.

use shared::item::{Item, ItemKind};
use shared::item_form::ItemForm;
use shared::material::Material;

use super::look::{Look, RingLook, setting_looks};

/// How this item looks, if it has a form to go by.
pub fn look_of(item: &Item) -> Option<Look> {
    match item.kind {
        ItemKind::Parcel => return Some(Look::Parcel),
        ItemKind::Keepsake => return Some(Look::Keepsake),
        ItemKind::Bag => return Some(Look::Bag),
        ItemKind::Pouch => return Some(Look::Pouch),
        ItemKind::Keyring => return Some(Look::Keyring),
        ItemKind::Key => return Some(Look::Key(super::KeyLook::of(item))),
        ItemKind::HerbPouch => return Some(Look::HerbPouch),
        ItemKind::CopperCoin => return Some(Look::Coin(Material::Copper)),
        ItemKind::SilverCoin => return Some(Look::Coin(Material::Silver)),
        ItemKind::GoldCoin => return Some(Look::Coin(Material::Gold)),
        kind @ (ItemKind::Cauldron
        | ItemKind::MortarAndPestle
        | ItemKind::Retort
        | ItemKind::Alembic
        | ItemKind::OilPress
        | ItemKind::Tap) => {
            return Some(Look::Tool(kind));
        }
        ItemKind::Yeast => return Some(Look::Yeast),
        ItemKind::Salt => return Some(Look::Salt),
        kind @ (ItemKind::Bread | ItemKind::DwarvenBread | ItemKind::RawMeat) => {
            return Some(Look::Food(kind));
        }
        _ => {}
    }
    if let Some(flour) = super::FlourLook::of(item) {
        return Some(Look::Flour(flour));
    }
    if let Some(vessel) = item.vessel() {
        return Some(Look::Vessel(vessel.vessel));
    }
    if let Some(log) = item.log() {
        return Some(Look::Log(log.wood));
    }
    if let Some(kindling) = item.kindling() {
        return Some(Look::Kindling(kindling.wood));
    }
    if let Some(cask) = item.cask() {
        return Some(Look::Cask(cask.look()));
    }
    if let Some(bottle) = item.bottle() {
        return Some(Look::Bottle {
            bottle: super::BottleLook::of(bottle),
            liquid: super::LiquidLook::of_bottle(bottle),
        });
    }
    if let Some(ointment) = item.ointment() {
        let salve = super::LiquidLook::of(&ointment.contents);
        return Some(Look::Ointment(salve));
    }
    if let Some(paste) = item.paste() {
        return Some(Look::Paste(super::LiquidLook::of(&paste.contents)));
    }
    if let Some(fruit) = item.fruit() {
        return Some(Look::Fruit(super::FruitItemLook::of(fruit)));
    }
    if let Some(vegetable) = item.vegetable() {
        return Some(Look::Vegetable(super::VegetableItemLook::of(vegetable)));
    }
    if let Some(herb) = item.herb() {
        return Some(Look::Herb(super::HerbLook::of(herb)));
    }
    let quality = item.quality;
    match item.form.as_ref()? {
        ItemForm::Blade(form) => Some(super::looks::blade_look(item, form)),
        ItemForm::Gem(gem) => Some(Look::Gem { gem: *gem, quality }),
        ItemForm::Ring(ring) => Some(Look::Ring {
            ring: RingLook {
                metal: ring.metal,
                profile: ring.profile,
                width: ring.width,
                pattern: ring.pattern,
                settings: setting_looks(&ring.settings, item.made.size),
            },
            quality,
        }),
        ItemForm::Necklace(necklace) => Some(Look::Necklace {
            necklace: super::NecklaceLook::new(necklace, item.made.size),
            quality,
        }),
        ItemForm::Under(form) => {
            let shape = item.shape();
            Some(Look::Under {
                under: super::UnderLook::new(*form, (shape.width, shape.height)),
                quality,
            })
        }
        ItemForm::Clothing(form) => {
            let shape = item.shape();
            Some(Look::Clothing {
                clothing: super::ClothingLook::new(*form, (shape.width, shape.height))
                    .with_digits(shared::armour::hands::Hand::digits(item.made.fingers)),
                quality,
            })
        }
        ItemForm::Footwear(form) => {
            let shape = item.shape();
            Some(Look::Footwear {
                footwear: super::FootwearLook::new(*form, (shape.width, shape.height)),
                quality,
            })
        }
        _ => super::looks::armour_look(item),
    }
}
