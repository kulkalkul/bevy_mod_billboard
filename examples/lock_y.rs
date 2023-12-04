use bevy::prelude::*;
use bevy::prelude::shape::{Plane, Quad};
use bevy_mod_billboard::{BillboardLockAxisBundle, BillboardLockAxis};
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
    commands
        .spawn(BillboardLockAxisBundle {
            billboard_bundle: BillboardTextureBundle {
                transform: Transform::from_translation(Vec3::new(2.0, 2.0, 0.0)),
                texture: BillboardTexture(image_handle.clone()),
                mesh: BillboardMesh(meshes.add(Quad::new(Vec2::new(2., 4.)).into())),
                ..default()
            },
            lock_axis: BillboardLockAxis {
                y_axis: true,
                ..Default::default()
            },
        });
    commands
        .spawn(BillboardTextureBundle {
            transform: Transform::from_translation(Vec3::new(-2.0, 2.0, 0.0)),
            texture: BillboardTexture(image_handle),
            mesh: BillboardMesh(meshes.add(Quad::new(Vec2::new(2., 4.)).into())),
            ..default()
        });
}

// Important bits are above, the code below is for camera, reference plane and rotation

#[derive(Component)]
pub struct CameraHolder;

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(PbrBundle {
        mesh: meshes.add(Plane { size: 4.0, subdivisions: 0 }.into()),
        material: materials.add(Color::SILVER.into()),
        ..default()
    });

    commands.spawn((
        CameraHolder,
        Transform::IDENTITY,
        GlobalTransform::IDENTITY,
    )).with_children(|parent| {
        parent.spawn(Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(0., 15., 2.))
                .looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        });
    });
}

fn rotate_camera(
    mut camera: Query<&mut Transform, With<CameraHolder>>,
    time: Res<Time>,
) {
    let mut camera = camera.single_mut();

    camera.rotate_y(time.delta_seconds());
}