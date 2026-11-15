//! Item Pictures' Necklaces page: necklaces as the server would roll them, a row of
//! each strand and each ornament, then people wearing them, bare and under a shirt.

use bevy::prelude::*;
use shared::creature::{CreatureKind, Size};
use shared::id::Id;
use shared::item::{Item, ItemKind, Quality};
use shared::item_form::{ItemForm, NecklaceForm, NecklaceLength, Ornament, Strand, give_form};

use super::armour::spawn_row;
use super::hair::dressed;
use super::{RING_COLUMNS, Shown};
use crate::item_art::{Look, look_of};

/// The most numbers tried to find a necklace for a row.
const TRIES: u128 = 20_000;
/// Where the numbers for rows start, away from rings'.
const FIRST: u128 = 1 << 20;

/// Which necklaces a row shows.
#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum Row {
    Cords,
    Chains,
    Torcs,
    Pendants,
    Amulets,
    Stations,
    Charms,
}

impl Row {
    fn takes(self, form: &NecklaceForm) -> bool {
        match self {
            Self::Cords => matches!(form.strand, Strand::Cord { .. }),
            Self::Chains => matches!(form.strand, Strand::Chain { .. }),
            Self::Torcs => matches!(form.strand, Strand::Torc { .. }),
            Self::Pendants => form.ornament == Ornament::Pendant,
            Self::Amulets => matches!(form.ornament, Ornament::Amulet(_)),
            Self::Stations => form.ornament == Ornament::Stations,
            Self::Charms => matches!(form.ornament, Ornament::Charm(_)),
        }
    }
}

/// The so-manyth necklace rolled at the quality chosen that fits a row.
#[derive(Clone, Copy)]
pub(super) struct NecklaceShown(Row, u8);

/// A necklace rolled from this number at this quality.
fn rolled(seed: u128, quality: Quality) -> Item {
    let mut item = Item::new(Id::for_test(seed), ItemKind::Necklace);
    item.quality = Some(quality);
    let mut gems = seed << 32;
    give_form(&mut item, Some(Size::Medium), &mut || {
        gems += 1;
        Id::for_test(gems)
    });
    item
}

impl NecklaceShown {
    pub(super) fn look(self, quality: Quality) -> Option<Look> {
        let NecklaceShown(row, nth) = self;
        let item = (FIRST..FIRST + TRIES)
            .map(|seed| rolled(seed, quality))
            .filter(|item| matches!(&item.form, Some(ItemForm::Necklace(form)) if row.takes(form)))
            .nth(nth as usize)?;
        look_of(&item)
    }
}

/// The first number whose necklace is this long, for people to wear.
fn of_length(length: NecklaceLength, skip: usize) -> u128 {
    (FIRST..FIRST + TRIES)
        .filter(|&seed| {
            let item = rolled(seed, Quality::Rare);
            matches!(&item.form, Some(ItemForm::Necklace(form))
                if form.length == length && form.ornament != Ornament::None)
        })
        .nth(skip)
        .unwrap_or(FIRST)
}

/// A row of each, then people wearing one of each length, bare and in a shirt.
pub(super) fn spawn_necklace_rows(grid: &mut ChildSpawnerCommands) {
    let rows = [
        ("Cords", Row::Cords),
        ("Chains", Row::Chains),
        ("Torcs", Row::Torcs),
        ("Pendants", Row::Pendants),
        ("Amulets", Row::Amulets),
        ("Stations", Row::Stations),
        ("Charms", Row::Charms),
    ];
    for (label, row) in rows {
        let pictures =
            (0..RING_COLUMNS as u8).map(move |nth| Shown::Necklace(NecklaceShown(row, nth)));
        spawn_row(grid, label, pictures);
    }
    let lengths = [
        NecklaceLength::Choker,
        NecklaceLength::Collarbone,
        NecklaceLength::Chest,
        NecklaceLength::Long,
    ];
    let worn = lengths
        .into_iter()
        .flat_map(|length| (0..2).map(move |skip| (ItemKind::Necklace, of_length(length, skip))));
    let worn: Vec<(ItemKind, u128)> = worn.collect();
    let bare = worn
        .clone()
        .into_iter()
        .map(|necklace| dressed(CreatureKind::Human, (0, 0), [Some(necklace), None]));
    spawn_row(grid, "Worn", bare);
    let shirted = worn.into_iter().map(|necklace| {
        dressed(
            CreatureKind::Human,
            (0, 0),
            [Some(necklace), Some((ItemKind::Shirt, 3))],
        )
    });
    spawn_row(grid, "In a Shirt", shirted);
}
