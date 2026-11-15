//! Item Pictures' Clothes page: rolled pieces of each kind drawn so far, two rows of
//! tunics and robes; then each playable people wearing one of each kind.

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
}

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
        let worn = WornLook {
            form: ItemForm::Clothing(form),
            quality: Some(quality),
            fingers: None,
        };
        character_look(kind, &Appearance::default(), &[worn])
    }
}

/// A row for each kind, two for tunics and robes, then a row of each kind worn.
pub(super) fn spawn_clothes_rows(grid: &mut ChildSpawnerCommands) {
    let rows = [
        ("Tunics", ClothingPiece::Tunic, 0),
        ("", ClothingPiece::Tunic, 1),
        ("Robes", ClothingPiece::Robe, 0),
        ("", ClothingPiece::Robe, 1),
        ("Doublets", ClothingPiece::Doublet, 0),
        ("Bodices", ClothingPiece::Bodice, 0),
        ("Vests", ClothingPiece::Vest, 0),
    ];
    let columns = RING_COLUMNS as u64;
    for (label, piece, page) in rows {
        let pictures = (0..columns).map(move |cell| {
            Shown::Clothes(ClothesShown {
                piece,
                nth: page * columns + cell,
                worn_by: None,
            })
        });
        spawn_row(grid, label, pictures);
    }
    let peoples: Vec<CreatureKind> = CreatureKind::ALL
        .into_iter()
        .filter(|kind| kind.def().playable)
        .collect();
    for (label, piece) in [
        ("Tunics worn", ClothingPiece::Tunic),
        ("Robes worn", ClothingPiece::Robe),
        ("Doublets worn", ClothingPiece::Doublet),
        ("Bodices worn", ClothingPiece::Bodice),
        ("Vests worn", ClothingPiece::Vest),
    ] {
        let pictures = peoples
            .clone()
            .into_iter()
            .enumerate()
            .map(move |(nth, kind)| {
                Shown::Clothes(ClothesShown {
                    piece,
                    nth: nth as u64,
                    worn_by: Some(kind),
                })
            });
        spawn_row(grid, label, pictures);
    }
}
