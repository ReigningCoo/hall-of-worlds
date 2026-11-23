//! What a spell is built from on the panel: the wheres, the three filters and
//! the effects, each drawn as an ability is (`widgets::spawn_ability`), with
//! its skill's icon on the colour of what that skill leans on.

use bevy::prelude::*;
use shared::sheet::Attribute;
use shared::spell::{Effect, Filters, Where};

use crate::skills_page::skill_icon;
use crate::widgets::{AbilityLook, Shade, attribute_colour};

/// One of the three filters.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(super) enum FilterKind {
    /// Self: the caster.
    Caster,
    Friend,
    Enemy,
}

impl FilterKind {
    pub(super) const ALL: [Self; 3] = [Self::Caster, Self::Friend, Self::Enemy];

    pub(super) const fn name(self) -> &'static str {
        match self {
            Self::Caster => "Self",
            Self::Friend => "Friend",
            Self::Enemy => "Enemy",
        }
    }

    const fn about(self) -> &'static str {
        match self {
            Self::Caster => "aims the first where at you, whatever is targeted",
            Self::Friend => {
                "aims at the target if a friend, else the nearest friend, and hits only friends"
            }
            Self::Enemy => {
                "aims at the target if an enemy, else the nearest enemy, and hits only enemies"
            }
        }
    }

    const fn icon(self) -> &'static str {
        match self {
            Self::Caster => "icons/delapouite/person.png",
            Self::Friend => "icons/delapouite/shaking-hands.png",
            Self::Enemy => "icons/delapouite/angry-eyes.png",
        }
    }

    /// Whether these filters have it.
    pub(super) const fn of(self, filters: Filters) -> bool {
        match self {
            Self::Caster => filters.caster,
            Self::Friend => filters.friend,
            Self::Enemy => filters.enemy,
        }
    }

    pub(super) fn set(self, filters: &mut Filters, on: bool) {
        match self {
            Self::Caster => filters.caster = on,
            Self::Friend => filters.friend = on,
            Self::Enemy => filters.enemy = on,
        }
    }
}

/// Anything along the top of the panel.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(super) enum Piece {
    Where(Where),
    Filter(FilterKind),
    Effect(Effect),
}

/// A where's or an effect's name, as the panel writes it.
pub(super) fn where_name(place: Where) -> &'static str {
    match place {
        Where::Touch => "Touch",
        Where::Missile => "Missile",
        Where::Surround => "Surround",
        Where::Beam => "Beam",
        Where::Point => "Point",
    }
}

pub(super) fn effect_name(effect: Effect) -> String {
    let key = effect.key();
    let mut letters = key.chars();
    letters
        .next()
        .map(|first| first.to_uppercase().chain(letters).collect())
        .unwrap_or_default()
}

const fn where_about(place: Where) -> &'static str {
    match place {
        Where::Touch => "touching distance in the direction faced; more make a wider arc",
        Where::Missile => "flies to a spot, hitting the first creature in its way; more fly faster",
        Where::Surround => "a ring round its centre, not the centre itself; more make it larger",
        Where::Beam => "a line to the target and everything along it; more add beams either side",
        Where::Point => {
            "exactly the target, at once, in clear sight; more add the nearest on its side"
        }
    }
}

impl Piece {
    /// Its tip: its name and what it does.
    pub(super) fn tip(self) -> String {
        match self {
            Self::Where(place) => format!("{}: {}", where_name(place), where_about(place)),
            Self::Filter(filter) => {
                format!("{}: {} (first step only)", filter.name(), filter.about())
            }
            Self::Effect(Effect::Void) => {
                "Void: reverses every effect after it in its step".to_owned()
            }
            Self::Effect(effect) => effect_name(effect),
        }
    }

    /// Its id's name and index, for the debug API.
    pub(super) fn id(self) -> (&'static str, usize) {
        match self {
            Self::Where(place) => ("where", place as usize),
            Self::Filter(filter) => ("filter", filter as usize),
            Self::Effect(effect) => ("effect", effect as usize),
        }
    }

    fn leans(self) -> &'static [Attribute] {
        match self {
            Self::Where(place) => place.skill().leans(),
            Self::Filter(_) => &[],
            Self::Effect(effect) => effect.skill().leans(),
        }
    }

    pub(super) fn icon_path(self) -> String {
        let skill = match self {
            Self::Where(place) => place.skill(),
            Self::Filter(filter) => return filter.icon().to_owned(),
            Self::Effect(effect) => effect.skill(),
        };
        skill_icon(skill).unwrap_or_else(|| "icons/lorc/magic-swirl.png".to_owned())
    }

    /// How it is drawn: as an ability, greyed when `usable` is false.
    pub(super) fn look(self, usable: bool, assets: &AssetServer) -> AbilityLook {
        AbilityLook {
            icon: assets.load(self.icon_path()),
            name: self.tip(),
            // Wheres round, filters octagons (`nodes.rs`), effects square.
            passive: matches!(self, Self::Where(_)),
            usable,
            colour: attribute_colour(self.leans(), Shade::Active),
            item: None,
        }
    }
}
