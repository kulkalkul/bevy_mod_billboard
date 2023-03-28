use bevy::prelude::*;
use bevy_mod_billboard::{BillboardPlugin, BillboardTextBundle};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(BillboardPlugin)
        .add_startup_system(spawn_camera)
        // By running the systems in this order we can actually see our billboard to verify that it's working
        .add_systems((despawn_billboard, spawn_billboard).chain())
        .run();
}

/// Avoid constantly reloading the font
#[derive(Resource)]
struct FontHandle(Handle<Font>);

fn spawn_camera(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_translation(Vec3::new(5., 0., 0.))
            .looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    commands.insert_resource(FontHandle(asset_server.load("FiraSans-Regular.ttf")));
}

fn spawn_billboard(mut commands: Commands, font: Res<FontHandle>) {
    commands.spawn(BillboardTextBundle {
        transform: Transform::from_scale(Vec3::splat(0.0085)),
        text: Text::from_sections([
            TextSection {
                value: "IMPORTANT".to_string(),
                style: TextStyle {
                    font_size: 60.0,
                    font: font.0.clone_weak(),
                    color: Color::ORANGE,
                },
            },
            TextSection {
                value: " text".to_string(),
                style: TextStyle {
                    font_size: 60.0,
                    font: font.0.clone_weak(),
                    color: Color::WHITE,
                },
            },
        ])
        .with_alignment(TextAlignment::Center),
        ..default()
    });
}

fn despawn_billboard(mut commands: Commands, query: Query<Entity, With<Text>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}
