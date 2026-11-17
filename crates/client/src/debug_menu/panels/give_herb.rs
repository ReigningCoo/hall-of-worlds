//! Give Item's herb (for the Herb kind): a dropdown of the server's herbs by
//! name, "Any herb" first for one rolled from the seed (`herb::dev_herb`), on
//! the Form line, which only herbs and kinds with a form show.

use bevy::prelude::*;
use shared::item::ItemKind;

use super::GiveForm;
use crate::common::HerbTable;
use crate::dom::Dom;
use crate::widgets::DropdownChosen;

/// The herb dropdown, with the keys of the herbs it lists after "Any herb".
#[derive(Component, Default)]
pub(in crate::debug_menu) struct GiveHerb {
    keys: Vec<String>,
}

/// Lists the herbs (again when the table changes), and shows the dropdown and
/// the Form line for herbs. Runs after `fill_give_options`, which hides that
/// line for kinds without a form.
pub(in crate::debug_menu) fn fill_give_herb(
    form: Res<GiveForm>,
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
    let herbs = form.kind == ItemKind::Herb;
    dom.set_flag("debug.give_item/herb", "hidden", !herbs);
    if herbs {
        dom.set_flag("debug.give_item/form_line", "hidden", false);
    }
    if added || table.is_changed() {
        let mut named: Vec<_> = table.0.iter().map(|def| (&def.name, &def.key)).collect();
        named.sort();
        dropdown.keys = named.iter().map(|(_, key)| (*key).clone()).collect();
        let names = named.into_iter().map(|(name, _)| name.clone());
        let all = std::iter::once("Any herb".to_owned())
            .chain(names)
            .enumerate();
        dom.set_options(
            "debug.give_item/herb",
            all.map(|(at, name)| (at.to_string(), name)),
        );
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
