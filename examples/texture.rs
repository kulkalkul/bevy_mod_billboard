use bevy::prelude::shape::Quad;
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

fn setup_billboard(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let image_handle = asset_server.load("rust-logo-256x256.png");
    commands.spawn(BillboardTextureBundle {
        texture: BillboardTextureHandle(image_handle),
        mesh: BillboardMeshHandle(meshes.add(Quad::new(Vec2::new(2., 2.)))),
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
        mesh: meshes.add(shape::Cube::default()),
        material: materials.add(Color::GRAY),
        transform: Transform::from_translation(Vec3::NEG_Y * 2.),
        ..default()
    });
}

fn rotate_camera(mut camera: Query<&mut Transform, With<CameraHolder>>, time: Res<Time>) {
    let mut camera = camera.single_mut();

    camera.rotate_y(time.delta_seconds());
}
