//! Tests the performance of the library
//! by rendering a large number of billboarded objects at once.
//!
//! This example uses command line flags to determine which type of billboards to render.
//! Run this example as `cargo run --example stress_test text` to render text billboards,
//! or `cargo run --example stress_test texture` to render image-based billboards.
//!
//! To test the performance of constantly recomputing billboards,
//! add the `recompute` argument to your invocation above.
//! For example `cargo run --example stress_test text recompute` will render text billboards
//! and recompute them every frame.

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
        .add_system(recompute_billboards)
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
    let fira_sans_regular_handle = asset_server.load("FiraSans-Regular.ttf");

    commands.spawn(Camera3dBundle {
        transform: Transform::from_translation(Vec3::new(0., 0., 50.))
            .looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });

    for x in -10..=10 {
        for y in -10..=10 {
            for z in -10..=10 {
                let translation = Vec3::new(x as f32, y as f32, z as f32);

                if std::env::args().any(|arg| arg == "text") {
                    commands.spawn(BillboardTextBundle {
                        transform: Transform {
                            translation,
                            rotation: Quat::IDENTITY,
                            scale: Vec3::splat(0.0085),
                        },
                        text: Text::from_section(
                            "STRESS",
                            TextStyle {
                                font_size: 60.0,
                                font: fira_sans_regular_handle.clone(),
                                color: Color::ORANGE,
                            },
                        ),
                        ..default()
                    });
                }

                if std::env::args().any(|arg| arg == "texture") {
                    commands.spawn(BillboardTextureBundle {
                        texture: billboard_texture_handle.clone(),
                        mesh: billboard_mesh_handle.clone(),
                        transform: Transform::from_translation(translation),
                        ..Default::default()
                    });
                }
            }
        }
    }
}

fn recompute_billboards(
    mut text_query: Query<&mut Text>,
    mut billboard_query: Query<&mut Handle<BillboardTexture>>,
) {
    // Only do this work if we're testing performance of recomputing billboards
    if !std::env::args().any(|arg| arg == "recompute") {
        return;
    };

    for mut text in text_query.iter_mut() {
        // Simply setting changed on the text component will cause the billboard to be recomputed
        text.set_changed();
    }

    for mut billboard_texture_handle in billboard_query.iter_mut() {
        // Simply setting changed on the mesh component will cause the billboard to be recomputed
        billboard_texture_handle.set_changed();
    }
}
