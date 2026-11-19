//! Reading a `CaskForm` back: messages (postcard) carry exactly what is
//! written; saves (RON) may be from before millilitres and opening
//! (2026-11-18), with whole `litres` and no `opened`: those load sealed.

use serde::{Deserialize, Deserializer};

use super::CaskForm;
use crate::cask::{CaskSize, CaskTime};
use crate::material::Material;
use crate::potion::Liquid;

impl<'de> Deserialize<'de> for CaskForm {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        if !deserializer.is_human_readable() {
            let wire = Wire::deserialize(deserializer)?;
            let cask = CaskForm::new(wire.size, wire.wood).holding(wire.liquid, wire.ml);
            return Ok(CaskForm {
                opened: wire.opened,
                time: wire.time,
                ..cask
            });
        }
        let saved = Saved::deserialize(deserializer)?;
        let ml = match saved.ml {
            ABSENT => saved.litres * 1000,
            ml => ml,
        };
        let cask = CaskForm::new(saved.size, saved.wood).holding(saved.liquid, ml);
        Ok(CaskForm {
            opened: saved.opened,
            time: saved.time,
            ..cask
        })
    }
}

/// A cask as messages carry it.
#[derive(Deserialize)]
#[serde(rename = "CaskForm")]
struct Wire {
    size: CaskSize,
    wood: Material,
    liquid: Option<Liquid>,
    ml: u32,
    opened: bool,
    time: Box<CaskTime>,
}

/// What `ml` reads as when a save has only `litres`.
const ABSENT: u32 = u32::MAX;

fn absent() -> u32 {
    ABSENT
}

/// A cask as saved, now or before millilitres.
#[derive(Deserialize)]
#[serde(rename = "CaskForm")]
struct Saved {
    size: CaskSize,
    wood: Material,
    #[serde(default)]
    liquid: Option<Liquid>,
    #[serde(default = "absent")]
    ml: u32,
    #[serde(default)]
    litres: u32,
    #[serde(default)]
    opened: bool,
    #[serde(default)]
    time: Box<CaskTime>,
}
