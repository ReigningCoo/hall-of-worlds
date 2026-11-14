//! Armour's requirements, from how heavy it is for what it covers: milligrams
//! per square centimetre. That depends only on how it is made and what of, not
//! on the piece or its size, so every piece of a set asks the same.

use super::kind::Requirement;
use crate::armour::Construction;
use crate::item::Item;
use crate::item_form::ItemForm;
use crate::material::{Category, Material};
use crate::sheet::Attribute;

/// Steel plate's milligrams per cm² (about 1.1 mm thick)...
pub const REFERENCE_MG: u32 = 870;
/// ...asks for this much Strength, a little over the median 100 (first guess).
const REFERENCE_NEED: u32 = 110;
/// Pieces heavier than this for what they cover also need Strength, however
/// they bend: riveted mail (about 750) needs both, ring mail (400) only Agility
/// (first guess, owner OK'd).
pub const AGILITY_AND_STRENGTH_ABOVE: u32 = 600;
/// Milligrams per cm² of a metal construction made in something that isn't
/// metal (horn scales, leather lamellae, wooden slats), per tenth of a gram per
/// cm³ of the material: plates about 6 mm thick.
pub const NON_METAL_PLATE_MG: u32 = 60;
/// The density metal constructions are worked out against: steel's.
const STEEL: u32 = 78;

/// Milligrams per cm² of each construction at a reference density (tenths of a
/// gram per cm³), as arm armour's weights are worked out (`armour::arms`).
pub const fn mg_at_reference(construction: Construction) -> (u32, u32) {
    match construction {
        Construction::Padded => (220, 15),
        Construction::Leather => (400, 11),
        Construction::Woven => (480, 5),
        Construction::Plate => (REFERENCE_MG, STEEL),
        Construction::Mail => (750, STEEL),
        Construction::RingMail => (400, STEEL),
        Construction::MailAndPlate => (800, STEEL),
        Construction::Splint => (600, STEEL),
        // Scale, lamellar, laminar, plates in cloth, mirrors: overlapping plates.
        _ => (1000, STEEL),
    }
}

/// How heavy armour made this way, of this, is for what it covers.
pub fn milligrams_per_cm2(construction: Construction, material: Material) -> u32 {
    let (mg, reference) = mg_at_reference(construction);
    if reference == STEEL && material.category() != Category::Metal {
        material.density() * NON_METAL_PLATE_MG
    } else {
        mg * material.density() / reference
    }
}

/// Whether it bends with the body, so asks for Agility: cloth, leather, mail
/// and ring mail. Everything else holds its shape and asks for Strength.
pub fn needs_agility(construction: Construction) -> bool {
    matches!(
        construction,
        Construction::Padded | Construction::Leather | Construction::Mail | Construction::RingMail
    )
}

/// The attribute armour this heavy for what it covers asks for: in step with
/// it, steel plate's `REFERENCE_NEED` (rounded, at least 1).
pub fn requirement_for(mg_per_cm2: u32) -> u16 {
    let need = (mg_per_cm2 * REFERENCE_NEED + REFERENCE_MG / 2) / REFERENCE_MG;
    need.clamp(1, u32::from(u16::MAX)) as u16
}

/// Armour's requirements: Agility if it bends, Strength if it doesn't or is
/// heavier than `AGILITY_AND_STRENGTH_ABOVE`; both by `requirement_for`.
pub(super) fn armour_requirements(item: &Item) -> Vec<Requirement> {
    let Some((construction, material)) = made_of(item) else {
        return Vec::new();
    };
    let mg = milligrams_per_cm2(construction, material);
    let min = requirement_for(mg);
    let mut list = Vec::new();
    if needs_agility(construction) {
        list.push(Requirement::Attribute {
            attribute: Attribute::Agility,
            min,
        });
    }
    if !needs_agility(construction) || mg > AGILITY_AND_STRENGTH_ABOVE {
        list.push(Requirement::Attribute {
            attribute: Attribute::Strength,
            min,
        });
    }
    list
}

/// How an armour piece is made and of what, as its form says (a set's is its
/// torso's).
fn made_of(item: &Item) -> Option<(Construction, Material)> {
    Some(match item.form.as_ref()? {
        ItemForm::Hands(form) => (form.construction, form.material),
        ItemForm::Shoulders(form) => (form.construction, form.material),
        ItemForm::Torso(form) => (form.construction, form.material),
        ItemForm::Head(form) => (form.construction, form.material),
        ItemForm::Arms(form) => (form.construction, form.material),
        ItemForm::Legs(form) => (form.construction, form.material),
        ItemForm::Waist(form) => (form.construction, form.material),
        ItemForm::Feet(form) => (form.construction, form.material),
        ItemForm::Neck(form) => (form.construction, form.material),
        ItemForm::Set(form) => (form.construction(), form.material()),
        _ => return None,
    })
}
