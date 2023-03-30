use bevy::prelude::*;
use bevy_mod_billboard::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(BillboardPlugin)
        .add_startup_system(setup_scene)
        .run();
}

fn setup_scene(mut commands: Commands, asset_server: Res<AssetServer>) {
    let fira_sans_regular_handle = asset_server.load("FiraSans-Regular.ttf");
    commands
        .spawn(BillboardTextBundle {
            transform: Transform::from_scale(Vec3::splat(0.0085)),
            text: Text::from_sections([
                TextSection {
                    value: "IMPORTANT".to_string(),
                    style: TextStyle {
                        font_size: 60.0,
                        font: fira_sans_regular_handle.clone(),
                        color: Color::ORANGE,
                    }
                },
                TextSection {
                    value: " text".to_string(),
                    style: TextStyle {
                        font_size: 60.0,
                        font: fira_sans_regular_handle.clone(),
                        color: Color::WHITE,
                    }
                }
            ]).with_alignment(TextAlignment::Center),
            ..default()
        });

    commands.spawn(Camera3dBundle {
        transform: Transform::from_translation(Vec3::new(5., 0., 0.))
            .looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}