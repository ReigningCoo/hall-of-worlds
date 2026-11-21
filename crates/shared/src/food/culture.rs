//! Cultures in ferments (owner, 2026-11-21): a ferment forms by itself,
//! wild, poorer and slower; a matching culture does best, any other strain
//! helps (koji: ordinary yeast works, a koji culture does better). A culture
//! is taken from a good product as yeast is: a yoghurt gives a yoghurt
//! culture of its quality.

use serde::{Deserialize, Serialize};

use super::{Rest, curve::better};
use crate::id::Id;
use crate::item::{Item, MadeAt, Quality};
use crate::yeast::Strain;

/// A culture put into something to ferment: its strain and quality.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Culture {
    pub strain: Strain,
    pub quality: Quality,
}

/// How long a ferment takes with `culture`, in percent of the time with a
/// matching one: 200 wild, 150 with another strain. First guesses.
pub fn set_percent(culture: Option<Culture>, wants: Option<Strain>) -> u32 {
    match culture {
        None => 200,
        Some(c) if Some(c.strain) == wants => 100,
        Some(_) => 150,
    }
}

/// What a ferment made at `base` comes out as: wild, a step worse; with
/// another strain, as made; with a matching one, a step better when the
/// culture is better than what it went into.
pub fn turned_quality(base: Quality, culture: Option<Culture>, wants: Option<Strain>) -> Quality {
    match culture {
        None => base.worse(),
        Some(c) if Some(c.strain) == wants && c.quality as u8 > base as u8 => better(base),
        Some(_) => base,
    }
}

/// A culture sampled from `item`, a yoghurt: a fresh Yeast item of the
/// yoghurt strain at its quality. `None` from anything else.
pub fn culture_from(item: &Item, id: Id, made_at: Option<MadeAt>) -> Option<Item> {
    let rest = item.food()?.rest.as_ref()?;
    if rest.kind != Rest::Yoghurt {
        return None;
    }
    let mut culture = crate::yeast::new_item(item.quality.unwrap_or_default(), id, made_at);
    culture.yeast_mut()?.strain = Strain::Yoghurt;
    Some(culture)
}
