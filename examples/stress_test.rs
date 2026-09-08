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
    color::palettes,
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
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
    let billboard_texture = BillboardTexture(asset_server.load("rust-logo-256x256.png"));
    let billboard_mesh = BillboardMesh(meshes.add(Rectangle::from_size(Vec2::splat(1.0))));
    let text_font = TextFont::from(asset_server.load("FiraSans-Regular.ttf")).with_font_size(60.0);

    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0., 0., 50.).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    for x in -10..=10 {
        for y in -10..=10 {
            for z in -10..=10 {
                let translation = Vec3::new(x as f32, y as f32, z as f32);

                if std::env::args().any(|arg| arg == "text") {
                    commands.spawn((
                        BillboardText::new("STRESS"),
                        text_font.clone(),
                        TextColor(Color::Srgba(palettes::css::ORANGE)),
                        Transform::from_translation(translation).with_scale(Vec3::splat(0.0085)),
                    ));
                }

                if std::env::args().any(|arg| arg == "texture") {
                    commands.spawn((
                        billboard_texture.clone(),
                        billboard_mesh.clone(),
                        Transform::from_translation(translation),
                    ));
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
