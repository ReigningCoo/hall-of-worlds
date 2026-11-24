//! The dungeon (and building) view: a grid of floor and wall cells in 3D,
//! seen through `view3d`'s camera with the same tokens as the world map. See
//! `docs/context/client/dungeon_map.md`.
//!
//! Only what the player (or anything sharing its sight) can see is drawn:
//! floors, stairs, and thin walls with only the faces someone sees (`walls`).
//! A seen side facing away from the camera is drawn half see-through. What
//! was seen before but is out of sight now is drawn dim, in sepia and
//! half see-through. Walls and floors use tile pictures chosen by the kind of place
//! (`theme`). Light is dull everywhere, and brighter near light sources, which
//! walls block. Left-click a seen walkable cell to travel there.

mod bench_ghost;
mod bench_picking;
mod bench_tools;
mod bolts;
mod border;
mod campfires;
mod daylight;
mod decor;
mod door_swing;
mod field;
mod fixed_lights;
mod fruit;
mod fruit_picking;
mod fruit_showcase;
mod fungi;
mod furniture;
mod herbs;
pub use field::CROP_STAGES;
pub use herbs::HERB_STAGES;
mod gate_picking;
mod ground_hit;
mod holes;
mod liquid;
mod mesh;
mod oasis;
mod other_floors;
mod outlined;
mod picking;
mod rack_bottles;
mod rack_casks;
mod rain;
mod rampart;
mod sight_lines;
mod spell_fx;
mod stairs;
mod theme;
mod town;
mod tree_picking;
mod trees;
mod vegetable_plants;
mod vegetable_showcase;
mod vegetables;
mod walls;

pub use theme::TOWN_ATLAS;
pub use town::door_picture;

use bevy::prelude::*;
use daylight::{OutsideLight, TownLights};
use mesh::Surface;
use sight_lines::SurfaceMaterial;

use crate::common::GameView;
use crate::player::PlayerSystems;
use crate::view3d::{TOKEN_DIAMETER, View3dSystems};

// Split into files (`cargo xtask lint` keeps files small).
mod basket_logs;
mod click;
mod face_point;
mod flowing;
mod hearth_fires;
mod hover;
mod inn_way_click;
mod lights;
mod parts;
mod pictures;
mod plain_pictures;
mod put_meshes;
mod showing;
mod spawn_scene;
mod tree_flash;
mod update_meshes;
mod viewers;
mod wild_ground;

use click::pick_on_click;
pub use click::{BenchClicked, DoorClicked};
use herbs::{DrawnPatches, HerbHeading, aim_at_herbs, pick_on_arrival, update_herbs};
pub use inn_way_click::InnWayClicked;
use lights::sync_lights;
pub use parts::Shown;
use parts::{
    DungeonScene, MemoryPart, SeeThroughPart, SolidPart, StairsThroughPart, SurfaceMaterials,
    TokenLight,
};
use pictures::{apply_theme, colour_ground, sepia_pictures};
use showing::{hide_scene, hide_unseen_tokens, show_scene};
use spawn_scene::{load_tile, spawn_scene};
use update_meshes::update_meshes;

/// Size of a cell in 3D units: about the size of a token's picture.
pub const CELL_SIZE: f32 = TOKEN_DIAMETER * 1.5;

/// How tall walls stand.
const WALL_HEIGHT: f32 = CELL_SIZE;

/// The dull light everywhere, as a fraction of full light.
const AMBIENT_LIGHT: f32 = 0.1;

/// Opacity of wall sides facing away from the camera.
const SEE_THROUGH_ALPHA: f32 = 0.5;

/// Opacity of what is remembered but out of sight.
const MEMORY_ALPHA: f32 = 0.5;

/// Draws a wall's fade across the floor after the floor, before the walls.
const BLEND_DEPTH_BIAS: f32 = -0.5;

/// Tokens are lit by real lights (the tiles' light is baked into their
/// colours): a point light this high above each light source, and a dim
/// ambient fill so tokens in the dark are still visible.
const TOKEN_LIGHT_HEIGHT: f32 = CELL_SIZE;
const TOKEN_LIGHT_LUMENS: f32 = 1.0e8;
const TOKEN_AMBIENT_BRIGHTNESS: f32 = 150.0;

pub struct DungeonMapPlugin;

impl Plugin for DungeonMapPlugin {
    fn build(&self, app: &mut App) {
        let in_view = in_state(GameView::Dungeon);
        app.add_message::<DoorClicked>()
            .add_message::<InnWayClicked>()
            .add_message::<BenchClicked>()
            .init_resource::<bench_tools::DrawnTools>()
            .init_resource::<bench_tools::ToolModels>()
            .init_resource::<rack_casks::DrawnCasks>()
            .init_resource::<rack_casks::CaskModels>()
            .init_resource::<rack_bottles::DrawnBottles>()
            .init_resource::<rack_bottles::BottleModels>()
            .init_resource::<decor::DrawnDecor>()
            .init_resource::<decor::DecorModels>()
            .init_resource::<Shown>()
            .init_resource::<door_swing::DoorSwings>()
            .init_resource::<OutsideLight>()
            .init_resource::<TownLights>()
            .init_resource::<other_floors::Floors>()
            .init_resource::<wild_ground::GroundTint>()
            .init_resource::<DrawnPatches>()
            .init_resource::<HerbHeading>()
            .init_resource::<fruit::DrawnFruit>()
            .init_resource::<trees::DrawnTrees>()
            .init_resource::<tree_flash::TreeFlashes>()
            .init_resource::<vegetable_plants::DrawnVegetables>()
            .init_resource::<fungi::DrawnFungi>()
            .init_resource::<fixed_lights::FixedLights>()
            .init_resource::<fixed_lights::SpellLights>()
            .init_resource::<campfires::DrawnFires>()
            .init_resource::<hearth_fires::DrawnHearthFires>()
            .init_resource::<basket_logs::DrawnBasketLogs>()
            .init_resource::<spell_fx::DrawnBolts>()
            .init_resource::<spell_fx::FxKit>()
            .init_resource::<spell_fx::FxQueue>()
            .add_plugins(MaterialPlugin::<rain::RainMaterial>::default())
            .add_plugins(MaterialPlugin::<SurfaceMaterial>::default())
            .add_systems(Startup, spawn_scene)
            .add_systems(
                OnEnter(GameView::Dungeon),
                (show_scene, sight_lines::set_mask_active::<true>),
            )
            .add_systems(
                OnExit(GameView::Dungeon),
                (hide_scene, sight_lines::set_mask_active::<false>),
            )
            .add_systems(
                Update,
                pick_on_click
                    .in_set(PlayerSystems::Input)
                    .after(View3dSystems::Click)
                    .run_if(in_view.clone()),
            )
            .add_systems(
                Update,
                (
                    apply_theme,
                    colour_ground,
                    daylight::update_outside,
                    daylight::update_town_lights,
                    daylight::tint_outside,
                    daylight::token_ambient,
                    sight_lines::update_outlines,
                    sight_lines::place_mask,
                    wild_ground::work_out_tint,
                    door_swing::swing_doors,
                    (
                        fixed_lights::update_fixed_lights,
                        update_meshes,
                        other_floors::draw_others,
                        other_floors::follow_materials,
                        other_floors::aim_wedge,
                    )
                        .chain(),
                    outlined::apply_outlined,
                    (
                        update_herbs,
                        fruit::update_fruit,
                        (
                            trees::update_trees,
                            tree_flash::take_chops,
                            tree_flash::flash_trees,
                        )
                            .chain(),
                        vegetable_plants::update_vegetables,
                        fungi::update_fungi,
                        (
                            bench_tools::update_bench_tools,
                            rack_casks::update_rack_casks,
                            rack_bottles::update_rack_bottles,
                            basket_logs::update_basket_logs,
                            decor::update_decor,
                        ),
                        (
                            campfires::update_campfires,
                            hearth_fires::update_hearth_fires,
                            campfires::flicker_flames,
                        )
                            .chain(),
                        aim_at_herbs,
                        pick_on_arrival,
                    ),
                    hide_unseen_tokens,
                    sync_lights,
                    rain::update,
                    bolts::strike,
                    (
                        spell_fx::spawn_fx,
                        spell_fx::draw_bolts,
                        spell_fx::drift_sparkles,
                        spell_fx::fade_fx,
                    )
                        .chain(),
                    border::draw_borders,
                    flowing::run_flow,
                )
                    .chain()
                    .after(PlayerSystems::Travel)
                    .after(View3dSystems::Place)
                    .run_if(in_view),
            )
            .add_systems(
                Update,
                (bench_ghost::follow_pointer, bench_ghost::draw_ghost)
                    .chain()
                    .after(PlayerSystems::Travel)
                    .after(View3dSystems::Place)
                    .run_if(in_state(GameView::Dungeon)),
            )
            .add_systems(
                Update,
                (sepia_pictures, plain_pictures::plain_pictures).chain(),
            )
            .add_systems(Update, spell_fx::take_news)
            .add_systems(
                Update,
                fixed_lights::follow_spell_lights.before(fixed_lights::update_fixed_lights),
            )
            .add_systems(
                Update,
                (hover::name_hovered_place, face_point::face_pointed_at)
                    .after(PlayerSystems::Travel)
                    .after(View3dSystems::Place),
            );
    }
}
