//! A cask's shape without Bevy, so the ground can draw it too: staves bowed
//! out at the belly, iron hoops, and a flat head at each end, standing on
//! `y = 0`, in metres, coloured by its wood.

use shared::cask::{CaskLook, CaskSize};
use shared::material::Material;
use std::f32::consts::{PI, TAU};

/// How many staves go round, and rings up it.
const STAVES: usize = 20;
const RINGS: usize = 24;
/// How much narrower the ends are than the belly.
const END_SHARE: f32 = 0.82;
/// Where the hoops are, as a share of the height, and how wide each is.
const HOOPS: [f32; 4] = [0.07, 0.24, 0.76, 0.93];
const HOOP_HALF: f32 = 0.025;
const IRON: [f32; 3] = [0.22, 0.22, 0.24];
/// How far the heads sit in from the staves' ends (the chime).
const CHIME: f32 = 0.03;

/// A cask's points and triangles, ready for a mesh.
#[derive(Default)]
pub struct CaskMesh {
    pub positions: Vec<[f32; 3]>,
    pub normals: Vec<[f32; 3]>,
    pub colours: Vec<[f32; 4]>,
    pub indices: Vec<u32>,
}

/// Its wood's colour (owner: oak brown, chestnut redder, acacia yellower,
/// cherry red-brown, pine pale).
pub fn wood_colour(wood: Material) -> [f32; 3] {
    match wood {
        Material::Chestnut => [0.48, 0.29, 0.17],
        Material::Acacia => [0.63, 0.5, 0.26],
        Material::Cherry => [0.56, 0.27, 0.18],
        Material::Pine => [0.8, 0.66, 0.44],
        _ => [0.55, 0.38, 0.22],
    }
}

/// Its belly's radius against its height: from its cells, so its picture
/// fills them (a Pin slim, a Tun stout).
pub fn radius_share(size: CaskSize) -> f32 {
    let shape = size.shape();
    f32::from(shape.width) / f32::from(shape.height) / 2.0
}

/// How tall one stands, in metres: what holds its litres at its shape.
pub fn height_of(size: CaskSize) -> f32 {
    let share = radius_share(size);
    // The gaugers' rule for a barrel: π h (2 R² + r²) / 3, R the belly's
    // radius and r the ends', with R = share × h.
    let bulk = PI * share * share * (2.0 + END_SHARE * END_SHARE) / 3.0;
    let cubic_metres = size.litres() as f32 / 1000.0;
    (cubic_metres / bulk).cbrt()
}

/// The radius at a height, as a share of the belly's: a smooth bow.
fn bow(up: f32) -> f32 {
    let from_middle = (up - 0.5) * 2.0;
    1.0 - (1.0 - END_SHARE) * from_middle * from_middle
}

/// A cask standing upright, its height `height_of` its size.
pub fn cask_mesh(look: CaskLook) -> CaskMesh {
    let height = height_of(look.size);
    let radius = radius_share(look.size) * height;
    let wood = wood_colour(look.wood);
    let mut mesh = CaskMesh::default();
    let row = STAVES * 2;
    for ring in 0..=RINGS {
        let up = ring as f32 / RINGS as f32;
        let hoop = HOOPS.iter().any(|at| (up - at).abs() <= HOOP_HALF + 0.001);
        let out = radius * bow(up) * if hoop { 1.02 } else { 1.0 };
        let slope = -(1.0 - END_SHARE) * 8.0 * (up - 0.5) * radius / height;
        for stave in 0..STAVES {
            // Each stave flat, with its own shade, so the joins show.
            let shade = 0.85 + 0.15 * ((stave * 7 % 5) as f32 / 4.0);
            let colour = match hoop {
                true => IRON,
                false => wood.map(|channel| channel * shade),
            };
            for side in 0..2 {
                let round = (stave + side) as f32 / STAVES as f32 * TAU;
                let middle = (stave as f32 + 0.5) / STAVES as f32 * TAU;
                let (x, z) = (round.cos(), round.sin());
                mesh.positions.push([out * x, up * height, out * z]);
                let normal = [middle.cos(), slope, middle.sin()];
                mesh.normals.push(normalised(normal));
                mesh.colours.push([colour[0], colour[1], colour[2], 1.0]);
            }
        }
    }
    for ring in 0..RINGS {
        for stave in 0..STAVES {
            let low = (ring * row + stave * 2) as u32;
            let high = low + row as u32;
            mesh.indices
                .extend([low, high, high + 1, low, high + 1, low + 1]);
        }
    }
    for (up, facing) in [(CHIME, -1.0), (1.0 - CHIME, 1.0)] {
        add_head(&mut mesh, radius * bow(up), up * height, facing, wood);
    }
    mesh
}

/// A flat head across the cask at `y`, facing up (1) or down (-1), with
/// boards across it shaded apart.
fn add_head(mesh: &mut CaskMesh, radius: f32, y: f32, facing: f32, wood: [f32; 3]) {
    let centre = mesh.positions.len() as u32;
    let dark = wood.map(|channel| channel * 0.8);
    mesh.positions.push([0.0, y, 0.0]);
    mesh.normals.push([0.0, facing, 0.0]);
    mesh.colours.push([dark[0], dark[1], dark[2], 1.0]);
    for step in 0..=STAVES {
        let round = step as f32 / STAVES as f32 * TAU;
        let (x, z) = (round.cos() * radius, round.sin() * radius);
        let board = ((x / radius + 1.0) * 2.5) as usize;
        let shade = if board.is_multiple_of(2) { 0.9 } else { 0.78 };
        mesh.positions.push([x, y, z]);
        mesh.normals.push([0.0, facing, 0.0]);
        mesh.colours
            .push([wood[0] * shade, wood[1] * shade, wood[2] * shade, 1.0]);
    }
    for step in 0..STAVES as u32 {
        let (a, b) = (centre + 1 + step, centre + 2 + step);
        match facing > 0.0 {
            true => mesh.indices.extend([centre, b, a]),
            false => mesh.indices.extend([centre, a, b]),
        }
    }
}

fn normalised([x, y, z]: [f32; 3]) -> [f32; 3] {
    let length = (x * x + y * y + z * z).sqrt().max(1e-6);
    [x / length, y / length, z / length]
}
