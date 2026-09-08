use bevy::color::palettes;
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
    let image_handle = asset_server.load("tree.png");

    commands.spawn((
        BillboardTexture(image_handle.clone()),
        BillboardMesh(meshes.add(Rectangle::new(2.0, 4.0))),
        Transform::from_xyz(2.0, 2.0, 0.0),
        BillboardLockAxis::from_lock_y(true),
    ));

    commands.spawn((
        BillboardTexture(image_handle),
        BillboardMesh(meshes.add(Rectangle::new(2.0, 4.0))),
        Transform::from_xyz(-2.0, 2.0, 0.0),
    ));
}

// Important bits are above, the code below is for camera, reference plane and rotation

#[derive(Component)]
#[require(Transform, Visibility)]
pub struct CameraHolder;

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::new(Vec3::Y, Vec2::splat(1.5)))),
        MeshMaterial3d(materials.add(Color::Srgba(palettes::css::SILVER))),
        Transform::from_scale(Vec3::splat(3.0)),
    ));

    commands.spawn(CameraHolder).with_children(|parent| {
        parent.spawn((
            Camera3d::default(),
            Transform::from_xyz(0., 15., 2.).looking_at(Vec3::ZERO, Vec3::Y),
        ));
    });
}

fn rotate_camera(mut camera: Query<&mut Transform, With<CameraHolder>>, time: Res<Time>) {
    let mut camera = camera.single_mut().unwrap();

    camera.rotate_y(time.delta_secs());
}
