use bevy::prelude::*;
use bevy::prelude::shape::Cube;
use bevy_mod_billboard::BillboardDepth;
use bevy_mod_billboard::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(BillboardPlugin)
        .add_startup_system(setup_scene)
        .run();
}

const TEXT_SCALE: Vec3 = Vec3::splat(0.0085);

fn setup_scene(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let fira_sans_regular_handle = asset_server.load("FiraSans-Regular.ttf");
    commands
        .spawn(PbrBundle {
            transform: Transform::from_translation(Vec3::new(1., 0., 0.)),
            mesh: meshes.add(Cube { size: 0.7 }.into()),
            ..default()
        });

    commands.spawn(BillboardTextBundle {
        transform: Transform::from_translation(Vec3::new(0., 0.5, 0.)).with_scale(TEXT_SCALE),
        text: Text::from_section("depth enabled", TextStyle {
            font_size: 60.0,
            font: fira_sans_regular_handle.clone(),
            color: Color::WHITE,
        }).with_alignment(TextAlignment::Center),
        ..default()
    });

    commands.spawn(BillboardTextBundle {
        transform: Transform::from_translation(Vec3::new(0., -0.5, 0.)).with_scale(TEXT_SCALE),
        text: Text::from_section("depth disabled", TextStyle {
            font_size: 60.0,
            font: fira_sans_regular_handle.clone(),
            color: Color::WHITE,
        }).with_alignment(TextAlignment::Center),
        billboard_depth: BillboardDepth(false),
        ..default()
    });

    commands.spawn(Camera3dBundle {
        transform: Transform::from_translation(Vec3::new(5., 0., 0.))
            .looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}