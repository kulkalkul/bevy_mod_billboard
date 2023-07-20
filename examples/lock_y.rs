use bevy::prelude::*;
use bevy::prelude::shape::{Plane, Quad};
use bevy_mod_billboard::{BillboardLockAxisBundle, BillboardLockAxis};
use bevy_mod_billboard::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(BillboardPlugin)
        .add_systems(Startup, setup_scene)
        .run();
}

fn setup_scene(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut billboard_textures: ResMut<Assets<BillboardTexture>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let image_handle = asset_server.load("tree.png");
    commands
        .spawn(BillboardLockAxisBundle {
            billboard_bundle: BillboardTextureBundle {
                transform: Transform::from_translation(Vec3::new(2.0, 2.0, 0.0)),
                texture: billboard_textures.add(BillboardTexture::Single(image_handle.clone())),
                mesh: BillboardMeshHandle(meshes.add(Quad::new(Vec2::new(2., 4.)).into())),
                ..default()
            },
            lock_axis: BillboardLockAxis {
                y_axis: true,
                ..Default::default()
            },
        });
    commands
        .spawn(BillboardTextureBundle {
            transform: Transform::from_translation(Vec3::new(-2.0, 2.0, 0.0)),
            texture: billboard_textures.add(BillboardTexture::Single(image_handle)),
            mesh: BillboardMeshHandle(meshes.add(Quad::new(Vec2::new(2., 4.)).into())),
            ..default()
        });
    commands.spawn(PbrBundle {
        mesh: meshes.add(Plane { size: 4.0, subdivisions: 0 }.into()),
        ..default()
    });

    commands.spawn(Camera3dBundle {
        transform: Transform::from_translation(Vec3::new(0., 15., 2.))
            .looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}