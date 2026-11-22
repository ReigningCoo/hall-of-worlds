//! Where the player is cooking, found the way the server finds it
//! (`connections/cook_kitchen.rs`): facing a hearth told to be lit, its
//! heat, Oven, tools and storage (one not lit: its tools, no heat); else, in
//! a town, the town's virtual hearth; else, facing a campfire, Fire; else no
//! heat. Water faced counts anywhere.

use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use shared::campfire::campfire_faced;
use shared::cooking::{Heat, Kitchen, Tool};
use shared::crafting_bench::{town_hearth, town_tools};
use shared::item::{Inventory, Item, ItemKind, Quality};
use shared::potion::water_faced;

use crate::common::{BenchHere, Benches, CurrentDungeon, Facing, GameView, SpotPosition};
use crate::player::Player;

/// The kitchen found, owning what `Kitchen` borrows.
#[derive(Clone, Debug, Default, PartialEq)]
pub(super) struct KitchenNow {
    pub(super) heat: Heat,
    pub(super) oven: bool,
    pub(super) tools: Vec<ItemKind>,
    /// The hearth's tools' qualities, by kind (a town's are Common).
    pub(super) qualities: Vec<(ItemKind, Quality)>,
    pub(super) storage: Vec<Item>,
    pub(super) water_faced: bool,
    /// The hearth worked at (its piece's north-west cell), sent with the
    /// order.
    pub(super) bench: Option<(i64, i64)>,
    /// What it is, for the words under the grid.
    pub(super) place: Place,
}

/// What kind of kitchen it is.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(super) enum Place {
    #[default]
    Nowhere,
    Hearth,
    ColdHearth,
    Town,
    Campfire,
}

impl KitchenNow {
    /// As the rules take it.
    pub(super) fn kitchen(&self) -> Kitchen<'_> {
        Kitchen {
            heat: self.heat,
            oven: self.oven,
            tools: &self.tools,
            storage: &self.storage,
            water_faced: self.water_faced,
        }
    }

    /// The quality of the `tool` a step would use, for its outline: the
    /// kitchen's first (the Oven, a town's tools Common), else the best
    /// carried; `None` when it isn't to hand (as `has_cooking_tool`).
    pub(super) fn used_quality(&self, tool: Tool, carried: &Inventory) -> Option<Quality> {
        if tool == Tool::Oven {
            return self.oven.then_some(Quality::Common);
        }
        let serves = |kind: ItemKind| Tool::of_item(kind) == Some(tool);
        if self.tools.iter().any(|&kind| serves(kind)) {
            let here = self.qualities.iter().filter(|(kind, _)| serves(*kind));
            return Some(here.map(|(_, q)| *q).max().unwrap_or(Quality::Common));
        }
        carried.best_quality_of(serves)
    }

    /// Whether `id` lies in the hearth's storage.
    pub(super) fn stored(&self, id: shared::id::Id) -> Option<&Item> {
        self.storage.iter().find(|item| item.id == id)
    }
}

/// What finding the kitchen reads.
#[derive(SystemParam)]
pub(super) struct KitchenHere<'w, 's> {
    view: Res<'w, State<GameView>>,
    dungeon: Res<'w, CurrentDungeon>,
    benches: Res<'w, Benches>,
    here: Res<'w, BenchHere>,
    player: Query<'w, 's, (&'static SpotPosition, &'static Facing), With<Player>>,
}

/// The kitchen the player is at, kept up to date while the panel is open
/// (changed only when it changes, for the rest to redraw).
#[derive(Resource, Clone, Debug, Default, PartialEq)]
pub(super) struct Kitchens(pub(super) KitchenNow);

/// Keeps `Kitchens` as the player moves and turns, while the Cooking panel
/// or the reading panel is open.
pub(super) fn keep_kitchen(
    bench: Res<super::bench::Bench>,
    reading: Res<super::reading::Reading>,
    here: KitchenHere,
    mut kitchens: ResMut<Kitchens>,
) {
    if bench.is_open() || reading.is_open() {
        kitchens.set_if_neq(Kitchens(here.now()));
    }
}

impl KitchenHere<'_, '_> {
    /// The kitchen the player is at now.
    pub(super) fn now(&self) -> KitchenNow {
        let mut now = KitchenNow::default();
        let Ok((at, facing)) = self.player.single() else {
            return now;
        };
        if *self.view.get() != GameView::Dungeon {
            return now;
        }
        let level = &self.dungeon.level;
        now.water_faced = water_faced(level, at.0, facing.0).is_some();
        let hearth = self.here.0.as_ref().and_then(|(piece, tools)| {
            let tools = tools.as_ref()?;
            tools.craft.is_hearth().then_some((piece.at, tools))
        });
        if let Some((cell, tools)) = hearth {
            let lit = self.benches.1.contains(&cell);
            now.heat = if lit { Heat::Hearth } else { Heat::Cold };
            now.oven = lit;
            now.tools = tools.tools().iter().map(|tool| tool.kind).collect();
            let quality = |tool: &Item| tool.quality.unwrap_or(Quality::Common);
            now.qualities = tools.tools().iter().map(|t| (t.kind, quality(t))).collect();
            now.storage = tools.storage().to_vec();
            now.bench = Some(cell);
            now.place = if lit {
                Place::Hearth
            } else {
                Place::ColdHearth
            };
            return now;
        }
        let poi = self.dungeon.info.as_ref().and_then(|info| info.kind.poi());
        if poi.is_some_and(town_hearth) {
            now.heat = Heat::Hearth;
            now.oven = true;
            now.tools = town_tools().to_vec();
            now.place = Place::Town;
            return now;
        }
        let burning = |x: i64, y: i64| level.campfire_at(x, y).is_some();
        if campfire_faced(level, at.0, facing.0, burning).is_some() {
            now.heat = Heat::Fire;
            now.place = Place::Campfire;
        }
        now
    }
}
