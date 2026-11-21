//! The Abilities page: every ability the character knows, flowing in rows, Attack
//! first. Drawn again when what they know, or can use now, changes: another
//! body, an ability learned, a tool picked up or put down.

use bevy::prelude::*;
use shared::ability::{Ability, known};

use super::icons::icon_path;
use crate::character::CharacterSheet;
use crate::common::AbilitiesPage;
use crate::common::BenchHere;
use crate::dom::{AbilityGridShown, AbilityThing, KnownAbilities};
use crate::inventory::Carried;
use crate::widgets::{AbilityLook, Shade, attribute_colour};

/// How many abilities a row holds before the next row starts.
const PER_ROW: u8 = 8;

/// The page's list of abilities.
#[derive(Component)]
pub(super) struct PageList;

/// An ability on the page, which can be dragged to the action bar.
#[derive(Component, Clone, Copy)]
pub(super) struct PageAbility(pub Ability);

/// What a page was drawn for: the abilities known, each with whether it can be
/// used now.
#[derive(Component, Clone, PartialEq, Eq)]
pub(super) struct DrawnFor(Vec<(Ability, bool)>);

/// Every ability, whether it is known and whether it can be used now.
fn standing(
    carried: &Carried,
    sheet: &CharacterSheet,
    here: &BenchHere,
) -> Vec<(Ability, bool, bool)> {
    let kind = carried.0.creature;
    Ability::ALL
        .into_iter()
        .map(|ability| {
            let known = ability.is_known(kind, &sheet.0);
            (ability, known, usable(ability, carried, sheet, here))
        })
        .collect()
}

/// Fills each Abilities page that is new, or was drawn before what is known or
/// usable changed.
pub(super) fn fill_pages(
    pages: Query<(Entity, Option<&DrawnFor>), With<AbilitiesPage>>,
    (carried, sheet, here): (Res<Carried>, Res<CharacterSheet>, Res<BenchHere>),
    assets: Res<AssetServer>,
    mut commands: Commands,
) {
    let now = DrawnFor(
        standing(&carried, &sheet, &here)
            .into_iter()
            .filter(|(_, known, _)| *known)
            .map(|(ability, _, usable)| (ability, usable))
            .collect(),
    );
    for (page, drawn) in &pages {
        if drawn == Some(&now) {
            continue;
        }
        let mut page = commands.entity(page);
        page.despawn_children().insert(now.clone());
        page.with_children(|page| spawn_contents(page, (&carried, &sheet, &here), &assets));
    }
}

/// Says how every ability looks for the character as they are, by name, for an
/// `<object type="ability">` anywhere: again when what can be used changes.
pub(super) fn name_abilities(
    (carried, sheet, here): (Res<Carried>, Res<CharacterSheet>, Res<BenchHere>),
    assets: Res<AssetServer>,
    mut known: ResMut<KnownAbilities>,
    mut named_for: Local<Option<Vec<(Ability, bool, bool)>>>,
) {
    let now = standing(&carried, &sheet, &here);
    if named_for.as_ref() == Some(&now) {
        return;
    }
    *named_for = Some(now);
    for ability in Ability::ALL {
        let look = look(ability, usable(ability, &carried, &sheet, &here), &assets);
        known.set(
            ability_id(ability),
            AbilityThing {
                thing: ability.into(),
                look,
            },
        );
    }
}

/// The list of what the character knows, which draws itself (`dom`).
fn spawn_contents(
    page: &mut ChildSpawnerCommands,
    (carried, sheet, here): (&Carried, &CharacterSheet, &BenchHere),
    assets: &AssetServer,
) {
    let known = known(carried.0.creature, &sheet.0);
    let things = known.into_iter().map(|ability| AbilityThing {
        thing: ability.into(),
        look: look(ability, usable(ability, carried, sheet, here), assets),
    });
    let shown = AbilityGridShown {
        columns: PER_ROW,
        things: things.collect(),
    };
    page.spawn((PageList, shown));
}

/// Whether the character can use it now (`Ability::can_use_at`, at the
/// crafting bench they are at if any), or it is passive,
/// which is never shown as unusable as it is never done.
pub(super) fn usable(
    ability: Ability,
    carried: &Carried,
    sheet: &CharacterSheet,
    here: &BenchHere,
) -> bool {
    !ability.is_active() || ability.can_use_at(&carried.0, &sheet.0, here.tools())
}

/// How an ability shows on the page, greyed unless `usable` (`usable`).
pub(super) fn look(ability: Ability, usable: bool, assets: &AssetServer) -> AbilityLook {
    AbilityLook {
        icon: assets.load(icon_path(ability)),
        name: format!(
            "{}
{}",
            ability.name(),
            ability.description()
        ),
        passive: !ability.is_active(),
        usable,
        colour: attribute_colour(ability.leans(), Shade::of_ability(!ability.is_active())),
        item: None,
    }
}

/// Its name in widget ids: `abilities/<id>`.
pub(super) fn ability_id(ability: Ability) -> &'static str {
    match ability {
        Ability::Attack => "attack",
        Ability::Heal => "heal",
        Ability::Flutter => "flutter",
        Ability::Kick => "kick",
        Ability::NightVision => "night_vision",
        Ability::Trample => "trample",
        Ability::Sneak => "sneak",
        Ability::PressOil => "press_oil",
        Ability::Alchemy => "alchemy",
        Ability::CollectWater => "collect_water",
        Ability::FellTree => "fell_tree",
        Ability::CutKindling => "cut_kindling",
        Ability::PressJuice => "press_juice",
        Ability::Ferment => "ferment",
        Ability::MakeCampfire => "make_campfire",
        Ability::Cook => "cook",
    }
}
