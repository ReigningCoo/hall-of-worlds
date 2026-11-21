//! What fills an item's square in writing, as far as it is known: its picture
//! kept to its proportions, its kind's colour if it has none, a faint square
//! while it is asked after, or a grey "?" if nobody knows it. A kind of item is
//! drawn as its stand-in is, or by an icon if it has none (the Oven).

use bevy::prelude::*;
use shared::cooking::Tool;
use shared::item::{Item, ItemKind, Rotation};
use shared::item_text::KindToken;

use super::tiles::UNKNOWN;
use crate::common::KnownItem;
use crate::item_art::{Look, look_of};
use crate::widgets::{INK, PICTURE_PIXELS_PER_CELL, item_colour, item_picture, picture_in_square};

/// The tint of an item being asked after.
const LOADING: Color = Color::srgba(0.25, 0.17, 0.1, 0.18);

/// What a frame shows, to know when to change it.
#[derive(Component, Clone, Debug, Default, PartialEq)]
pub(super) enum Shows {
    #[default]
    Loading,
    Unknown,
    Plain(ItemKind),
    Picture(Look),
    /// An icon, by its path in the assets.
    Icon(&'static str),
}

impl Shows {
    pub fn of(known: Option<&KnownItem>) -> Self {
        match known {
            Some(KnownItem::Found(item, _)) => Self::of_item(item.top()),
            Some(KnownItem::Unknown(_)) => Self::Unknown,
            Some(KnownItem::Asked(_)) | None => Self::Loading,
        }
    }

    fn of_item(item: &Item) -> Self {
        look_of(item).map_or(Self::Plain(item.kind), Self::Picture)
    }

    /// A kind of item: its stand-in's picture, the Oven's icon, else a "?".
    pub fn of_kind(kind: &KindToken) -> Self {
        match (kind.stand_in(), kind) {
            (Some(item), _) => Self::of_item(&item),
            (None, KindToken::Tool(Tool::Oven)) => Self::Icon("icons/delapouite/chicken-oven.png"),
            (None, _) => Self::Unknown,
        }
    }

    /// Fills a frame with it.
    pub fn spawn(&self, frame: &mut ChildSpawnerCommands, assets: &AssetServer) {
        let filling = Node {
            width: percent(100),
            height: percent(100),
            ..default()
        };
        match self {
            Self::Loading => {
                frame.spawn((filling, BackgroundColor(LOADING)));
            }
            Self::Unknown => {
                frame.spawn((filling, ImageNode::new(UNKNOWN)));
            }
            Self::Plain(kind) => {
                frame.spawn((filling, BackgroundColor(item_colour(*kind))));
            }
            Self::Icon(path) => {
                let icon = ImageNode::new(assets.load(*path)).with_color(INK);
                frame.spawn((filling, icon));
            }
            Self::Picture(look) => {
                let node = picture_in_square(look);
                let pixels = PICTURE_PIXELS_PER_CELL;
                frame.spawn(item_picture(look.clone(), pixels, node, Rotation::None));
            }
        }
    }
}
