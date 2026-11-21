//! `client.save_picture`: one item's picture drawn at a given size and saved as
//! a PNG with a clear background, for the players' wiki (`cargo xtask wiki
//! --pictures`). Drawn as the game draws it, but apart from the pictures kept
//! for the interface, so it can be read back off the graphics card.

use bevy::asset::RenderAssetUsages;
use bevy::prelude::*;
use bevy::render::gpu_readback::{ReadbackComplete, ReadbackOnce};
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat, TextureUsages};
use serde::Deserialize;
use server::debug_api::{BrpError, BrpResult, Value, json, params, refused};
use shared::creature::CreatureKind;
use shared::gender::Gender;
use shared::item::Item;
use shared::livestock::Stage;
use shared::sheet::Appearance;

use shared::fruit::{FruitForm, Fruits};
use shared::vegetable::{VegetableForm, Vegetables};

use super::fruit::{FRUIT_VARIANTS, FruitItemLook};
use super::herb::{HERB_VARIANTS, HerbLook};
use super::pictures::Waiting;
use super::vegetable::{VEGETABLE_VARIANTS, VegetableItemLook};
use super::{Batch, ItemPictures, Look, look_of};
use crate::common::HerbTable;

#[derive(Deserialize)]
struct Asked {
    /// Where to save it: absolute, or from the game's working folder.
    path: String,
    /// Pixels along its longest side, as `ItemPictures::picture` takes them.
    pixels: u32,
    /// A herb by its key, with its variant (0 if not given)...
    herb: Option<String>,
    variant: Option<u8>,
    /// ...or a fruit by its key, with its variant, whole or a slice...
    fruit: Option<String>,
    serving: Option<bool>,
    /// ...or a vegetable by its key, the same way...
    vegetable: Option<String>,
    /// ...or any item, as saved, drawn as its look...
    item: Option<Item>,
    /// ...or a creature of a kind, bare, as its token's model: of a gender
    /// (Neutral if not given) with these looks (each trait's first if not given),
    /// at a stage of its life (Adult if not given).
    creature: Option<CreatureKind>,
    gender: Option<Gender>,
    looks: Option<Appearance>,
    stage: Option<Stage>,
    /// ...or a cooked food's model (`{"Dish": "Pie"}`, `{"Rest": "Dough"}`)
    /// in sample colours.
    dish: Option<super::dish::Model>,
}

/// Pictures asked for and not yet read back.
#[derive(Resource, Default)]
pub(super) struct Saving(Vec<(Handle<Image>, UVec2, String)>);

pub(super) fn add(app: &mut App) {
    use server::debug_api::AddDebugMethod;
    app.init_resource::<Saving>()
        .add_systems(PostUpdate, read_back.after(super::draw_pictures))
        .add_debug_method("client.save_picture", save_picture);
}

/// `client.save_picture {path, pixels, herb?, fruit?, vegetable?, serving?, variant?, item?,
/// creature?, gender?, looks?, stage?, dish?}`:
/// answers the full path, the picture's `width` and `height`, and `wait_for_file`.
fn save_picture(
    In(request): In<Option<Value>>,
    mut pictures: ResMut<ItemPictures>,
    mut saving: ResMut<Saving>,
    mut images: ResMut<Assets<Image>>,
    herbs: Res<HerbTable>,
) -> BrpResult {
    let asked: Asked = params(request)?;
    let look = match (asked.herb, asked.item) {
        (None, None) if asked.creature.is_some() => creature_look(
            asked.creature.unwrap_or_default(),
            asked.gender.unwrap_or_default(),
            asked.stage.unwrap_or_default(),
            asked.looks.unwrap_or_default(),
        )?,
        (None, None) if asked.dish.is_some() => Look::Dish(super::DishLook::sample(
            asked
                .dish
                .unwrap_or(super::dish::Model::Dish(shared::cooking::DishForm::Bowl)),
            shared::herb::Rgb(205, 160, 95),
            shared::herb::Rgb(95, 140, 60),
        )),
        (None, None) if asked.fruit.is_some() => fruit_look(
            asked.fruit.unwrap_or_default(),
            asked.serving,
            asked.variant,
        )?,
        (None, None) if asked.vegetable.is_some() => vegetable_look(
            asked.vegetable.unwrap_or_default(),
            asked.serving,
            asked.variant,
        )?,
        (Some(herb), None) => {
            if herbs.0.get(&herb).is_none() {
                return Err(refused(format!("no herb has the key {herb:?}")));
            }
            let variant = asked.variant.unwrap_or(0);
            if variant >= HERB_VARIANTS {
                return Err(refused(format!("variant is 0 to {}", HERB_VARIANTS - 1)));
            }
            Look::Herb(HerbLook { herb, variant })
        }
        (None, Some(item)) => look_of(&item).ok_or_else(|| refused("that item has no picture"))?,
        _ => return Err(refused("give either herb or item")),
    };
    if !(16..=4096).contains(&asked.pixels) {
        return Err(refused("pixels is 16 to 4096"));
    }
    let path = std::path::absolute(&asked.path)
        .map_err(|error| refused(format!("can't use {}: {error}", asked.path)))?;
    let path = path.to_string_lossy().into_owned();
    if let Some(folder) = std::path::Path::new(&path).parent() {
        std::fs::create_dir_all(folder)
            .map_err(|error| refused(format!("can't make {}: {error}", folder.display())))?;
    }
    // An old file would look like the new one to whoever waits for it.
    let _ = std::fs::remove_file(&path);
    let size = look.picture_size(asked.pixels);
    let mut image = Image::new_target_texture(size.x, size.y, TextureFormat::Rgba8UnormSrgb, None);
    image.texture_descriptor.usage |= TextureUsages::COPY_SRC;
    let picture = images.add(image);
    // Straight onto the queue, not kept: the interface's own pictures can't be
    // read back, and this one is drawn at exactly the pixels asked for.
    pictures.waiting.push_back(Waiting {
        look,
        asked: asked.pixels,
        pixels: asked.pixels,
        picture: picture.clone(),
    });
    saving.0.push((picture, size, path.clone()));
    Ok(json!({
        "path": path,
        "width": size.x,
        "height": size.y,
        "wait_for_file": path,
    }))
}

/// A creature's bare body, as its token shows it.
fn creature_look(
    kind: CreatureKind,
    gender: Gender,
    stage: Stage,
    looks: Appearance,
) -> Result<Look, BrpError> {
    let looks = looks.fitted(kind, gender);
    super::character::character_look_at(kind, gender, stage, &looks, &[])
        .ok_or_else(|| refused(format!("{kind:?}'s body isn't drawn")))
}

/// A fruit's look by key, whole or a slice, in a variant (0 if not given).
fn fruit_look(key: String, serving: Option<bool>, variant: Option<u8>) -> Result<Look, BrpError> {
    let Some(def) = Fruits::built_in().get(&key) else {
        return Err(refused(format!("no fruit has the key {key:?}")));
    };
    let variant = variant.unwrap_or(0);
    if variant >= FRUIT_VARIANTS {
        return Err(refused(format!("variant is 0 to {}", FRUIT_VARIANTS - 1)));
    }
    let serving = serving.unwrap_or(false);
    let servings = u32::from(def.servings.max(1));
    let form = FruitForm {
        fruit: key,
        serving,
        look: u64::from(variant),
        grams: if serving {
            def.grams.div_ceil(servings)
        } else {
            def.grams
        },
    };
    Ok(Look::Fruit(FruitItemLook::of(&form)))
}

/// A vegetable's look by key, whole or a slice, in a variant (0 if not given).
fn vegetable_look(
    key: String,
    serving: Option<bool>,
    variant: Option<u8>,
) -> Result<Look, BrpError> {
    let Some(def) = Vegetables::built_in().get(&key) else {
        return Err(refused(format!("no vegetable has the key {key:?}")));
    };
    let variant = variant.unwrap_or(0);
    if variant >= VEGETABLE_VARIANTS {
        return Err(refused(format!(
            "variant is 0 to {}",
            VEGETABLE_VARIANTS - 1
        )));
    }
    let form = match serving.unwrap_or(false) {
        true => VegetableForm::serving(def, shared::rng::Seed::new(0)),
        false => VegetableForm::new(def, shared::rng::Seed::new(0)),
    };
    let form = VegetableForm {
        look: u64::from(variant),
        ..form
    };
    Ok(Look::Vegetable(VegetableItemLook::of(&form)))
}

/// Once a picture asked for is drawn and copied off its page (no longer waiting
/// nor on a page still being drawn), reads it back and saves it.
fn read_back(
    mut commands: Commands,
    mut saving: ResMut<Saving>,
    pictures: Res<ItemPictures>,
    batches: Query<&Batch>,
) {
    let drawing = |picture: &Handle<Image>| {
        pictures.waiting.iter().any(|w| w.picture == *picture)
            || batches
                .iter()
                .any(|batch| batch.cells.iter().any(|(_, cell)| cell == picture))
    };
    let (ready, still): (Vec<_>, Vec<_>) = saving
        .0
        .drain(..)
        .partition(|(picture, ..)| !drawing(picture));
    saving.0 = still;
    for (picture, size, path) in ready {
        commands.spawn(ReadbackOnce::texture(picture)).observe(
            move |done: On<ReadbackComplete>, mut commands: Commands| {
                if let Err(error) = save(&done.data, size, &path) {
                    error!("client.save_picture: {error}");
                }
                commands.entity(done.entity).despawn();
            },
        );
    }
}

/// Writes the picture's pixels (rows maybe padded, as the graphics card copies
/// them) to a PNG.
fn save(data: &[u8], size: UVec2, path: &str) -> Result<(), String> {
    let row = size.x as usize * 4;
    let stride = data.len() / size.y.max(1) as usize;
    if stride < row {
        return Err(format!("read back {} bytes, too few", data.len()));
    }
    let pixels: Vec<u8> = data
        .chunks(stride)
        .take(size.y as usize)
        .flat_map(|line| &line[..row])
        .copied()
        .collect();
    let image = Image::new(
        Extent3d {
            width: size.x,
            height: size.y,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        pixels,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::MAIN_WORLD,
    );
    let picture = image
        .try_into_dynamic()
        .map_err(|error| format!("can't convert the picture: {error:?}"))?;
    picture
        .save(path)
        .map_err(|error| format!("can't save {path}: {error}"))
}
