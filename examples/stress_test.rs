//! Tests the performance of the library
//! by rendering a large number of billboarded objects at once.
//!
//! This example uses command line flags to determine which type of billboards to render.
//! Run this example as `cargo run --example stress_test text` to render text billboards,
//! or `cargo run --example stress_test texture` to render image-based billboards.
//!
//! To test the performance of constantly recomputing billboards,
//! add the `recompute_text` or `recompute_texture` argument to your invocation above.
//! `recompute_text` trigger change detection to Text while `recompute_texture` triggers
//! change detection to BillboardTexture.
//! For example `cargo run --example stress_test text recompute_text` will render text billboards
//! and recompute them every frame.

use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::{shape::Quad, *},
};
use bevy_mod_billboard::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(BillboardPlugin)
        .add_plugins((
            FrameTimeDiagnosticsPlugin::default(),
            LogDiagnosticsPlugin::default(),
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, recompute_billboards)
        .run();
}

#[derive(Resource)]
struct Settings {
    recompute_text: bool,
    recompute_texture: bool,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut meshes: ResMut<Assets<Mesh>>) {
    let image_handle: Handle<Image> = asset_server.load("rust-logo-256x256.png");
    let billboard_texture = BillboardTexture(image_handle);
    let mesh_handle = meshes.add(Quad::new(Vec2::new(1., 1.)).into());
    let billboard_mesh = BillboardMesh(mesh_handle);
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
                        texture: billboard_texture.clone(),
                        mesh: billboard_mesh.clone(),
                        transform: Transform::from_translation(translation),
                        ..Default::default()
                    });
                }
            }
        }
    }

    commands.insert_resource(Settings {
        recompute_texture: std::env::args().any(|arg| arg == "recompute_texture"),
        recompute_text: std::env::args().any(|arg| arg == "recompute_text"),
    });
}

fn recompute_billboards(
    mut text_query: Query<&mut Text>,
    mut billboard_query: Query<&mut BillboardTexture>,
    settings: Res<Settings>,
) {
    if settings.recompute_text {
        for mut text in text_query.iter_mut() {
            // Simply setting changed on the text component will cause the billboard to be recomputed
            // This is expected as text is recalculated using change detection
            text.set_changed();
        }
    }

    if settings.recompute_texture {
        for mut billboard_texture in billboard_query.iter_mut() {
            // This should be negligible
            billboard_texture.set_changed();
        }
    }
}
