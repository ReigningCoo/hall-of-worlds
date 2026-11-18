//! Give Item's herb, fruit or vegetable (for the Herb, Fruit and Vegetable
//! kinds): a dropdown of the server's herbs, the fruits or the vegetables, by
//! name, "Any herb", "Any fruit" or "Any vegetable" first for one rolled from
//! the seed (`herb::dev_herb`, `fruit::dev_fruit`, `vegetable::dev_vegetable`),
//! on the Form line, which only these and kinds with a form show.

use bevy::prelude::*;
use shared::fruit::Fruits;
use shared::item::ItemKind;
use shared::vegetable::Vegetables;

use super::GiveForm;
use crate::common::HerbTable;
use crate::dom::Dom;
use crate::widgets::DropdownChosen;

/// The herb dropdown, with the keys of the herbs or fruits it lists after
/// "Any herb" or "Any fruit", and the kind they are for.
#[derive(Component, Default)]
pub(in crate::debug_menu) struct GiveHerb {
    keys: Vec<String>,
    listed: Option<ItemKind>,
}

/// Lists the herbs or fruits (again when the table or the kind changes), and
/// shows the dropdown and the Form line for them. Runs after `fill_give_options`, which hides that
/// line for kinds without a form.
pub(in crate::debug_menu) fn fill_give_herb(
    mut form: ResMut<GiveForm>,
    table: Res<HerbTable>,
    mut dropdowns: Query<&mut GiveHerb>,
    mut dom: Dom,
) {
    let Ok(mut dropdown) = dropdowns.single_mut() else {
        return;
    };
    let added = dropdown.is_added();
    if !(added || form.is_changed() || table.is_changed()) {
        return;
    }
    let shown = matches!(
        form.kind,
        ItemKind::Herb | ItemKind::Fruit | ItemKind::Vegetable
    );
    dom.set_flag("debug.give_item/herb", "hidden", !shown);
    if shown {
        dom.set_flag("debug.give_item/form_line", "hidden", false);
    }
    let kind = shown.then_some(form.kind);
    if kind.is_some() && (dropdown.listed != kind || table.is_changed()) {
        dropdown.listed = kind;
        let (mut named, any): (Vec<_>, _) = match form.kind {
            ItemKind::Fruit => {
                let fruits = Fruits::built_in().items();
                (
                    fruits.map(|def| (&def.name, &def.key)).collect(),
                    "Any fruit",
                )
            }
            ItemKind::Vegetable => {
                let vegetables = Vegetables::built_in().iter();
                (
                    vegetables.map(|def| (&def.name, &def.key)).collect(),
                    "Any vegetable",
                )
            }
            _ => (
                table.0.iter().map(|def| (&def.name, &def.key)).collect(),
                "Any herb",
            ),
        };
        named.sort();
        dropdown.keys = named.iter().map(|(_, key)| (*key).clone()).collect();
        let names = named.into_iter().map(|(name, _)| name.clone());
        let all = std::iter::once(any.to_owned()).chain(names).enumerate();
        dom.set_options(
            "debug.give_item/herb",
            all.map(|(at, name)| (at.to_string(), name)),
        );
    }
    // A herb's key chosen before the kind became Fruit, or the other way round.
    let stale = (form.herb.as_ref()).is_some_and(|key| !dropdown.keys.contains(key));
    if shown && stale {
        form.herb = None;
    }
    let chosen = form.herb.as_ref();
    let at = chosen.and_then(|key| dropdown.keys.iter().position(|it| it == key));
    let value = at.map_or(0, |at| at + 1);
    dom.set("debug.give_item/herb", "value", &value.to_string());
}

/// Follows the herb dropdown.
pub(in crate::debug_menu) fn choose_give_herb(
    mut chosen: MessageReader<DropdownChosen>,
    dropdowns: Query<&GiveHerb>,
    mut form: ResMut<GiveForm>,
) {
    for &DropdownChosen(dropdown, index) in chosen.read() {
        let Ok(dropdown) = dropdowns.get(dropdown) else {
            continue;
        };
        form.herb = index
            .checked_sub(1)
            .and_then(|at| dropdown.keys.get(at))
            .cloned();
    }
}
