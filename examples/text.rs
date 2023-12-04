use bevy::prelude::*;
use bevy_mod_billboard::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(BillboardPlugin)
        .add_systems(Startup, (setup_billboard, setup_scene))
        .add_systems(Update, rotate_camera)
        .run();
}

fn setup_billboard(mut commands: Commands, asset_server: Res<AssetServer>) {
    let fira_sans_regular_handle = asset_server.load("FiraSans-Regular.ttf");
    commands.spawn(BillboardTextBundle {
        transform: Transform::from_scale(Vec3::splat(0.0085)),
        text: Text::from_sections([
            TextSection {
                value: "IMPORTANT".to_string(),
                style: TextStyle {
                    font_size: 60.0,
                    font: fira_sans_regular_handle.clone(),
                    color: Color::ORANGE,
                },
            },
            TextSection {
                value: " text".to_string(),
                style: TextStyle {
                    font_size: 60.0,
                    font: fira_sans_regular_handle.clone(),
                    color: Color::WHITE,
                },
            },
        ])
        .with_alignment(TextAlignment::Center),
        ..default()
    });
}

// Important bits are above, the code below is for camera, reference cube and rotation

#[derive(Component)]
pub struct CameraHolder;

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn((CameraHolder, Transform::IDENTITY, GlobalTransform::IDENTITY))
        .with_children(|parent| {
            parent.spawn(Camera3dBundle {
                transform: Transform::from_translation(Vec3::new(5., 0., 0.))
                    .looking_at(Vec3::ZERO, Vec3::Y),
                ..default()
            });
        });

    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Cube::default().into()),
        material: materials.add(Color::GRAY.into()),
        transform: Transform::from_translation(Vec3::NEG_Y),
        ..default()
    });
}

fn rotate_camera(mut camera: Query<&mut Transform, With<CameraHolder>>, time: Res<Time>) {
    let mut camera = camera.single_mut();

    camera.rotate_y(time.delta_seconds());
}
