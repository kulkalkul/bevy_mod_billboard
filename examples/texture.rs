use bevy::prelude::*;
use bevy::prelude::shape::Quad;
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
    let image_handle = asset_server.load("rust-logo-256x256.png");
    commands
        .spawn(BillboardTextureBundle {
            texture: billboard_textures.add(BillboardTexture::Single(image_handle)),
            mesh: BillboardMeshHandle(meshes.add(Quad::new(Vec2::new(2., 2.)).into())),
            ..default()
        });

    commands.spawn(Camera3dBundle {
        transform: Transform::from_translation(Vec3::new(5., 0., 0.))
            .looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}