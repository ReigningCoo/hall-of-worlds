//! Item Pictures' Clothes page: rolled pieces of each kind, two rows of tunics, robes,
//! skirts, belts, hoods, headcloths and hats; then each playable people wearing one of
//! each kind, and three rows of outfits (tunic, trousers, belt and gloves; bodice,
//! skirt and belt; robe, hood and hat).

use bevy::prelude::*;
use shared::clothing::{self, ClothingPiece};
use shared::creature::CreatureKind;
use shared::item::Quality;
use shared::item_form::ItemForm;
use shared::rng::Seed;
use shared::sheet::Appearance;
use shared::wear::WornLook;

use super::armour::spawn_row;
use super::{RING_COLUMNS, Shown};
use crate::item_art::{ClothingLook, Look, character_look};

/// The so-manyth piece of a kind rolled, alone or worn by a kind.
#[derive(Clone, Copy)]
pub(super) struct ClothesShown {
    piece: ClothingPiece,
    nth: u64,
    worn_by: Option<CreatureKind>,
    /// Worn with these too, rolled from the same seed.
    with: &'static [ClothingPiece],
}

/// Outfits shown worn together: with a tunic, with a bodice, and with a robe.
const OUTFITS: [&[ClothingPiece]; 3] = [
    &[
        ClothingPiece::Trousers,
        ClothingPiece::Belt,
        ClothingPiece::Gloves,
    ],
    &[ClothingPiece::Skirt, ClothingPiece::Belt],
    &[ClothingPiece::Hood, ClothingPiece::Hat],
];

impl ClothesShown {
    pub(super) fn look(self, quality: Quality) -> Option<Look> {
        let form = clothing::roll(&mut Seed::new(self.nth).rng(), self.piece, quality);
        let Some(kind) = self.worn_by else {
            let shape = form.shape();
            return Some(Look::Clothing {
                clothing: ClothingLook::new(form, (shape.width, shape.height)),
                quality: Some(quality),
            });
        };
        let wear = |form| WornLook {
            form: ItemForm::Clothing(form),
            quality: Some(quality),
            fingers: None,
        };
        let others = self.with.iter().map(|&piece| {
            wear(clothing::roll(
                &mut Seed::new(self.nth).rng(),
                piece,
                quality,
            ))
        });
        let worn: Vec<WornLook> = std::iter::once(wear(form)).chain(others).collect();
        character_look(
            kind,
            shared::gender::Gender::default(),
            &Appearance::default(),
            &worn,
        )
    }
}

/// A row for each kind, two for tunics, robes, skirts, belts, hoods, headcloths and
/// hats, then a row of each kind worn.
pub(super) fn spawn_clothes_rows(grid: &mut ChildSpawnerCommands) {
    let rows = [
        ("Tunics", ClothingPiece::Tunic, 0),
        ("", ClothingPiece::Tunic, 1),
        ("Robes", ClothingPiece::Robe, 0),
        ("", ClothingPiece::Robe, 1),
        ("Doublets", ClothingPiece::Doublet, 0),
        ("Bodices", ClothingPiece::Bodice, 0),
        ("Vests", ClothingPiece::Vest, 0),
        ("Trousers", ClothingPiece::Trousers, 0),
        ("Skirts", ClothingPiece::Skirt, 0),
        ("", ClothingPiece::Skirt, 1),
        ("Belts", ClothingPiece::Belt, 0),
        ("", ClothingPiece::Belt, 1),
        ("Gloves", ClothingPiece::Gloves, 0),
        ("Hoods", ClothingPiece::Hood, 0),
        ("", ClothingPiece::Hood, 1),
        ("Caps", ClothingPiece::Cap, 0),
        ("Headcloths", ClothingPiece::Headcloth, 0),
        ("", ClothingPiece::Headcloth, 1),
        ("Headdresses", ClothingPiece::Headdress, 0),
        ("Neckwear", ClothingPiece::Neckwear, 0),
        ("Shawls", ClothingPiece::Shawl, 0),
        ("Hats", ClothingPiece::Hat, 0),
        ("", ClothingPiece::Hat, 1),
    ];
    let columns = RING_COLUMNS as u64;
    for (label, piece, page) in rows {
        let pictures = (0..columns).map(move |cell| {
            Shown::Clothes(ClothesShown {
                piece,
                nth: page * columns + cell,
                worn_by: None,
                with: &[],
            })
        });
        spawn_row(grid, label, pictures);
    }
    let peoples: Vec<CreatureKind> = CreatureKind::ALL
        .into_iter()
        .filter(|kind| kind.def().playable)
        .collect();
    let rows = [
        ("Tunics worn", ClothingPiece::Tunic, &[][..]),
        ("Robes worn", ClothingPiece::Robe, &[][..]),
        ("Doublets worn", ClothingPiece::Doublet, &[][..]),
        ("Bodices worn", ClothingPiece::Bodice, &[][..]),
        ("Vests worn", ClothingPiece::Vest, &[][..]),
        ("Trousers worn", ClothingPiece::Trousers, &[][..]),
        ("Skirts worn", ClothingPiece::Skirt, &[][..]),
        ("Belts worn", ClothingPiece::Belt, &[][..]),
        ("Gloves worn", ClothingPiece::Gloves, &[][..]),
        ("Hoods worn", ClothingPiece::Hood, &[][..]),
        ("Caps worn", ClothingPiece::Cap, &[][..]),
        ("Headcloths worn", ClothingPiece::Headcloth, &[][..]),
        ("Headdresses worn", ClothingPiece::Headdress, &[][..]),
        ("Neckwear worn", ClothingPiece::Neckwear, &[][..]),
        ("Shawls worn", ClothingPiece::Shawl, &[][..]),
        ("Hats worn", ClothingPiece::Hat, &[][..]),
        ("Outfits worn", ClothingPiece::Tunic, OUTFITS[0]),
        ("", ClothingPiece::Bodice, OUTFITS[1]),
        ("", ClothingPiece::Robe, OUTFITS[2]),
    ];
    for (label, piece, with) in rows {
        let pictures = peoples
            .clone()
            .into_iter()
            .enumerate()
            .map(move |(nth, kind)| {
                Shown::Clothes(ClothesShown {
                    piece,
                    nth: nth as u64,
                    worn_by: Some(kind),
                    with,
                })
            });
        spawn_row(grid, label, pictures);
    }
}
