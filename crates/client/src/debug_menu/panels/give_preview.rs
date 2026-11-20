//! Give Item's picture of what it will give, rolled here from the same seed as the
//! server will roll it; how many it may give; and whether it can all be given: made
//! as asked, and with room for it.

use bevy::prelude::*;
use shared::creature::CreatureKind;
use shared::flour::dev_flour;
use shared::fruit::dev_fruit;
use shared::fungus::dev_fungus;
use shared::herb::{Herbs, dev_herb, new_item};
use shared::id::Id;
use shared::item::{Inventory, Item, ItemKind, StackLimits};
use shared::item_form::{Field, ItemOrder, Refused, RollRequest, batch_seed, try_make_item};
use shared::protocol::LevelKind;
use shared::vegetable::dev_vegetable;

use super::{GiveButton, GiveChoice, GiveCount, GiveForm, GivePreview, GiveTo, any};
use crate::common::{CurrentDungeon, GameView, HerbTable};
use crate::dom::Dom;
use crate::inventory::{Carried, Stacks};
use crate::widgets::{ButtonState, CELL, ItemLook, ItemName, TextBox, spawn_item_cells};

/// Draws what is to be given, keeps the count to the most allowed, and greys what
/// can't be used: Give where it can't all be given, Different / Identical for one.
#[allow(clippy::too_many_arguments, clippy::type_complexity)]
pub(in crate::debug_menu) fn show_give_preview(
    mut commands: Commands,
    mut form: ResMut<GiveForm>,
    to: Res<GiveTo>,
    carried: Res<Carried>,
    stacks: Res<Stacks>,
    view: Res<State<GameView>>,
    (dungeon, table): (Res<CurrentDungeon>, Res<HerbTable>),
    mut previews: Query<(Entity, &mut GivePreview, &mut Node)>,
    mut counts: Query<&mut TextBox, With<GiveCount>>,
    mut dom: Dom,
    mut buttons: Query<
        (&mut ButtonState, Option<&GiveChoice>),
        Or<(With<GiveButton>, With<GiveChoice>)>,
    >,
) {
    let Ok((entity, mut preview, mut node)) = previews.single_mut() else {
        return;
    };
    let fresh = preview.drawn.is_none();
    let moved = view.is_changed() || dungeon.is_changed();
    let changed =
        form.is_changed() || to.is_changed() || carried.is_changed() || table.is_changed();
    if !(fresh || moved || changed || stacks.is_changed()) {
        return;
    }
    let (order, seed) = (form.order(), form.seed.unwrap_or_default());
    // Made for whoever gets it.
    let wearer = to.0.map_or(carried.0.creature, |(_, kind)| kind);
    let chosen = form.herb.clone();
    let herb = Herb {
        herbs: &table.0,
        key: chosen.as_deref(),
    };
    let first = one_of(&order, (wearer, herb), seed, 0, form.identical);
    // What a refused request would have been: drawn, but not to be given.
    let item = first.clone().unwrap_or_else(|_| {
        let plain = order_as_usual(&order);
        one_of(&plain, (wearer, herb), seed, 0, true)
            .unwrap_or_else(|_| Item::new(Id::stand_in(0), order.kind))
    });
    let most = match form.identical {
        true => stacks.0.most_of(&item),
        false => stacks.0.most,
    };
    if form.count > most.max(1) {
        form.count = most.max(1);
        for mut count in &mut counts {
            count.text = form.count.to_string();
        }
    }
    let drawn = Some((order, seed, wearer, chosen.clone()));
    if preview.drawn != drawn {
        preview.drawn = drawn;
        draw(&mut commands, entity, &mut node, &item);
    }
    // A target's room is the server's to judge: mobs take anything given.
    let has_ground = to.0.is_some()
        || *view.get() == GameView::Dungeon
            && (dungeon.info.as_ref())
                .is_some_and(|info| !matches!(info.kind, LevelKind::Hall { .. }));
    let batch = Batch {
        order,
        wearer,
        herb,
        seed,
        count: form.count,
        identical: form.identical,
    };
    let problem = match first {
        Err(Refused(field)) => Some(format!("No such {}", &any(field)[4..])),
        Ok(_) => batch.problem(&carried.0, stacks.0, has_ground),
    };
    if let Some(words) = &problem {
        dom.set_text("debug.give_item/no_space", words.clone());
    }
    dom.set_flag("debug.give_item/no_space", "hidden", problem.is_none());
    for (mut state, choice) in &mut buttons {
        let usable = match choice {
            Some(GiveChoice::Identical) => form.count > 1,
            Some(_) => continue,
            None => problem.is_none() && form.seed.is_some(),
        };
        state.set_if_neq(if usable {
            ButtonState::Normal
        } else {
            ButtonState::Disabled
        });
    }
}

/// The same order, with its form rolled as usual.
fn order_as_usual(order: &ItemOrder) -> ItemOrder {
    ItemOrder {
        request: RollRequest::NONE,
        ..*order
    }
}

/// The herbs to choose from, and the herb or fruit chosen (`None`: from the
/// seed).
#[derive(Clone, Copy)]
struct Herb<'a> {
    herbs: &'a Herbs,
    key: Option<&'a str>,
}

/// The `n`th of a batch, as the server will make it; its ids only stand in.
fn one_of(
    order: &ItemOrder,
    (wearer, herb): (CreatureKind, Herb),
    seed: u64,
    n: u32,
    identical: bool,
) -> Result<Item, Refused> {
    let mut next = (n as u128 + 1) << 32;
    let mut new_id = || {
        next += 1;
        Id::for_test(next)
    };
    let id = new_id();
    let seed = batch_seed(seed, if identical { 0 } else { n });
    if order.kind == ItemKind::Herb {
        // As the server makes them (`herb::dev_herb`).
        let def = dev_herb(herb.herbs, herb.key, seed).ok_or(Refused(Field::Material))?;
        return Ok(new_item(def, 1, id, None));
    }
    if order.kind == ItemKind::Fruit {
        // As the server makes them (`fruit::dev_fruit`).
        let def = dev_fruit(herb.key, seed).ok_or(Refused(Field::Material))?;
        return Ok(shared::fruit::new_item(def, 1, id, None));
    }
    if order.kind == ItemKind::Vegetable {
        // As the server makes them (`vegetable::dev_vegetable`).
        let def = dev_vegetable(herb.key, seed).ok_or(Refused(Field::Material))?;
        return Ok(shared::vegetable::new_item(def, 1, id, None));
    }
    if order.kind == ItemKind::Fungus {
        // As the server makes them (`fungus::dev_fungus`).
        let def = dev_fungus(herb.key, seed).ok_or(Refused(Field::Material))?;
        return Ok(shared::fungus::new_item(def, 1, id, None));
    }
    if order.kind == ItemKind::Flour {
        // As the server makes them (`flour::dev_flour`).
        let def = dev_flour(herb.key, seed).ok_or(Refused(Field::Material))?;
        return shared::flour::new_item(def, 1, id, None).ok_or(Refused(Field::Material));
    }
    try_make_item(order, wearer, id, seed, None, &mut new_id)
}

/// What Give would ask for.
struct Batch<'a> {
    order: ItemOrder,
    wearer: CreatureKind,
    herb: Herb<'a>,
    seed: u64,
    count: u32,
    identical: bool,
}

impl Batch<'_> {
    /// Why the server would give none of it, as the panel says it: one of a batch of
    /// different ones can't be made as asked, or with no ground, not all of it fits.
    fn problem(
        &self,
        carried: &Inventory,
        limits: StackLimits,
        has_ground: bool,
    ) -> Option<String> {
        let mut inventory = carried.clone();
        let mut fits = true;
        // Only a trial, so stand-ins name any new coin stacks.
        let mut stand_ins = 0;
        let mut stand_in = || {
            stand_ins += 1;
            Id::stand_in(stand_ins)
        };
        for n in 0..self.count {
            if has_ground && (self.identical || n == 0) {
                continue;
            }
            let maker = (self.wearer, self.herb);
            let item = match one_of(&self.order, maker, self.seed, n, self.identical) {
                Ok(item) => item,
                Err(Refused(field)) => return Some(format!("No such {}", &any(field)[4..])),
            };
            fits = fits && (has_ground || inventory.give(item, limits, &mut stand_in).is_ok());
        }
        (!fits).then(|| "No space".to_owned())
    }
}

fn draw(commands: &mut Commands, entity: Entity, node: &mut Node, item: &Item) {
    let shape = item.shape();
    node.width = px(shape.width as f32 * CELL);
    node.height = px(shape.height as f32 * CELL);
    let name = ItemName(item.clone());
    commands.entity(entity).despawn_related::<Children>();
    commands.entity(entity).with_children(|parent| {
        let look = ItemLook::of(item);
        spawn_item_cells(parent, look, shape, (0.0, 0.0), false, |_, _| name.clone());
    });
}
