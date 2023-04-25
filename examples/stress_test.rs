//! Tests the performance of the library
//! by rendering a large number of billboarded objects at once.

use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::{shape::Quad, *},
};
use bevy_mod_billboard::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(BillboardPlugin)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_startup_system(setup)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut billboard_textures: ResMut<Assets<BillboardTexture>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let image_handle: Handle<Image> = asset_server.load("rust-logo-256x256.png");
    let billboard_texture_handle = billboard_textures.add(BillboardTexture::Single(image_handle));
    let mesh_handle = meshes.add(Quad::new(Vec2::new(1., 1.)).into());
    let billboard_mesh_handle = BillboardMeshHandle(mesh_handle);

    commands.spawn(Camera3dBundle {
        transform: Transform::from_translation(Vec3::new(0., 0., 50.))
            .looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });

    for x in -10..=10 {
        for y in -10..=10 {
            for z in -10..=10 {
                commands.spawn(BillboardTextureBundle {
                    texture: billboard_texture_handle.clone(),
                    mesh: billboard_mesh_handle.clone(),
                    transform: Transform::from_translation(Vec3::new(x as f32, y as f32, z as f32)),
                    ..Default::default()
                });
            }
        }
    }
}
