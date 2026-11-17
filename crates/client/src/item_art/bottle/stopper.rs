//! What closes a bottle (there is always something): a cork, a cork sealed in
//! wax, a glass stopper or a wooden bung with a cloth tied over it. Built in the
//! bottle's own units on top of its mouth.

use bevy::prelude::*;
use std::f32::consts::FRAC_PI_2;

use super::Part;
use super::look::{Glass, Stopper};
use super::profiles::Profile;

/// How far the stopper reaches: its top, and how wide it is, for fitting.
pub(super) struct Reach {
    pub(super) top: f32,
    pub(super) radius: f32,
}

fn matte(colour: Color, materials: &mut Assets<StandardMaterial>) -> Handle<StandardMaterial> {
    materials.add(StandardMaterial {
        base_color: colour,
        perceptual_roughness: 0.9,
        reflectance: 0.1,
        ..default()
    })
}

fn at(mesh: Handle<Mesh>, material: &Handle<StandardMaterial>, y: f32) -> Part {
    Part {
        mesh,
        material: material.clone(),
        transform: Transform::from_xyz(0.0, y, 0.0),
    }
}

/// The stopper's parts on this outline's mouth, and how far they reach.
pub(super) fn stopper_parts(
    stopper: Stopper,
    glass: Glass,
    profile: &Profile,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) -> (Vec<Part>, Reach) {
    let (mouth, top) = (profile.mouth(), profile.top());
    let (lip, lip_bottom) = profile.lip();
    let cork_height = (mouth * 1.4).max(7.0);
    let cork_middle = top + cork_height / 2.0 - 3.0;
    let cork = |meshes: &mut Assets<Mesh>| {
        meshes.add(ConicalFrustum {
            radius_top: mouth * 1.15,
            radius_bottom: mouth * 0.95,
            height: cork_height,
        })
    };
    let cork_colour = Color::srgb(0.74, 0.56, 0.36);
    let cork_top = top + cork_height - 3.0;
    match stopper {
        Stopper::Cork => {
            let material = matte(cork_colour, materials);
            let parts = vec![at(cork(meshes), &material, cork_middle)];
            (
                parts,
                Reach {
                    top: cork_top,
                    radius: lip.max(mouth * 1.15),
                },
            )
        }
        Stopper::Waxed => {
            let wax = materials.add(StandardMaterial {
                base_color: Color::srgb(0.62, 0.06, 0.08),
                perceptual_roughness: 0.45,
                reflectance: 0.35,
                ..default()
            });
            let seal_bottom = lip_bottom - 1.5;
            let height = cork_top + 1.0 - seal_bottom;
            let mut parts = vec![at(
                meshes.add(ConicalFrustum {
                    radius_top: mouth * 1.25,
                    radius_bottom: lip + 1.2,
                    height,
                }),
                &wax,
                seal_bottom + height / 2.0,
            )];
            // The seal's rounded top, and drips down the front of the neck.
            parts.push(Part {
                transform: Transform::from_xyz(0.0, cork_top + 1.0, 0.0)
                    .with_scale(Vec3::new(1.0, 0.35, 1.0)),
                ..at(meshes.add(Sphere::new(mouth * 1.25)), &wax, 0.0)
            });
            for (angle, length) in [(-0.5_f32, 6.0), (0.15, 9.0), (0.7, 4.5)] {
                let out = profile.neck() + 0.8;
                parts.push(Part {
                    transform: Transform::from_xyz(
                        angle.sin() * out,
                        seal_bottom - length / 2.0 + 1.0,
                        angle.cos() * out,
                    ),
                    ..at(meshes.add(Capsule3d::new(1.3, length)), &wax, 0.0)
                });
            }
            (
                parts,
                Reach {
                    top: cork_top + 1.0 + mouth * 0.45,
                    radius: lip + 1.2,
                },
            )
        }
        Stopper::Glass => {
            let colour = Color::WHITE.mix(&super::glass::shade(glass), 0.6);
            let material = materials.add(StandardMaterial {
                base_color: colour,
                perceptual_roughness: 0.15,
                reflectance: 0.7,
                ..default()
            });
            let ball = (mouth * 1.3).max(5.0);
            let parts = vec![
                at(
                    meshes.add(Cylinder::new(mouth * 0.95, 6.0)),
                    &material,
                    top - 2.0,
                ),
                at(meshes.add(Cylinder::new(lip, 1.6)), &material, top + 1.5),
                at(
                    meshes.add(Cylinder::new(mouth * 0.5, 3.0)),
                    &material,
                    top + 3.5,
                ),
                at(
                    meshes.add(Sphere::new(ball)),
                    &material,
                    top + 4.0 + ball * 0.9,
                ),
            ];
            (
                parts,
                Reach {
                    top: top + 4.0 + ball * 1.9,
                    radius: lip.max(ball),
                },
            )
        }
        Stopper::Cloth => cloth(profile, meshes, materials),
    }
}

/// A wooden bung with linen tied over it by a string under the lip.
fn cloth(
    profile: &Profile,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) -> (Vec<Part>, Reach) {
    let (mouth, top) = (profile.mouth(), profile.top());
    let (lip, lip_bottom) = profile.lip();
    let linen = matte(Color::srgb(0.86, 0.8, 0.66), materials);
    let string = matte(Color::srgb(0.48, 0.3, 0.17), materials);
    let dome = top + mouth * 0.9 + 2.0;
    let tie = lip_bottom - 2.0;
    let wide = lip + 2.2;
    let tied = profile.neck() + 0.9;
    let mut parts = vec![
        // The cloth over the bung's top, then gathered in to the string.
        Part {
            transform: Transform::from_xyz(0.0, dome - 2.0, 0.0)
                .with_scale(Vec3::new(1.0, 0.45, 1.0)),
            ..at(meshes.add(Sphere::new(wide)), &linen, 0.0)
        },
        frustum(meshes, &linen, (wide, dome - 2.0), (tied, tie)),
        // Below the string, flaring out a little.
        frustum(meshes, &linen, (tied, tie), (tied + 2.2, tie - 5.0)),
        Part {
            transform: Transform::from_xyz(0.0, tie, 0.0),
            ..at(meshes.add(Torus::new(tied, tied + 1.6)), &string, 0.0)
        },
    ];
    // The string's two ends.
    for side in [-1.0_f32, 1.0] {
        let tilt = Quat::from_rotation_z(side * 0.5) * Quat::from_rotation_x(FRAC_PI_2 * 0.2);
        parts.push(Part {
            transform: Transform::from_xyz(side * 2.0, tie - 3.0, tied + 1.0).with_rotation(tilt),
            ..at(meshes.add(Capsule3d::new(0.7, 5.0)), &string, 0.0)
        });
    }
    (
        parts,
        Reach {
            top: dome - 2.0 + wide * 0.45,
            radius: wide,
        },
    )
}

/// A cone's slice from one radius and height to another.
fn frustum(
    meshes: &mut Assets<Mesh>,
    material: &Handle<StandardMaterial>,
    (upper, upper_y): (f32, f32),
    (lower, lower_y): (f32, f32),
) -> Part {
    let mesh = meshes.add(ConicalFrustum {
        radius_top: upper,
        radius_bottom: lower,
        height: upper_y - lower_y,
    });
    at(mesh, material, (upper_y + lower_y) / 2.0)
}
